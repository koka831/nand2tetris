// NOTE: These tests use a modified version of the given XMLs. The original XML represents multiple
// variable definitions (e.g., `var int i, sum ;`) in a single scope, but my implementation parses
// each definition into separate internal representations, making it impossible to retain the
// original formatting.
#![feature(box_patterns)]

use std::io::{self, BufWriter};

use anyhow::Result;
use jack_ast::*;
use jack_compiler::parser::parse;

struct XmlContext<'s, W: io::Write> {
    writer: &'s mut W,
    depth: usize,
}
impl<'s, W: io::Write> XmlContext<'s, W> {
    fn new(writer: &'s mut W) -> Self {
        XmlContext { writer, depth: 0 }
    }

    fn write(&mut self, xml: &str) -> Result<()> {
        let indent = " ".repeat(self.depth * 2);
        writeln!(self.writer, "{}{}", indent, xml)?;
        Ok(())
    }

    fn nest<F: FnOnce(&mut Self) -> Result<()>>(&mut self, tag: &str, f: F) -> Result<()> {
        self.open(tag)?;
        self.depth += 1;
        f(self)?;
        self.depth -= 1;
        self.close(tag)?;
        Ok(())
    }

    #[inline]
    fn with_paren<F: FnOnce(&mut Self) -> Result<()>>(&mut self, f: F) -> Result<()> {
        self.with("(", ")", f)
    }

    #[inline]
    fn with_brace<F: FnOnce(&mut Self) -> Result<()>>(&mut self, f: F) -> Result<()> {
        self.with("{", "}", f)
    }

    #[inline]
    fn with_bracket<F: FnOnce(&mut Self) -> Result<()>>(&mut self, f: F) -> Result<()> {
        self.with("[", "]", f)
    }

    fn with<F: FnOnce(&mut Self) -> Result<()>>(
        &mut self,
        open: &str,
        close: &str,
        f: F,
    ) -> Result<()> {
        self.symbol(open)?;
        f(self)?;
        self.symbol(close)?;
        Ok(())
    }

    #[inline]
    fn open(&mut self, tag: &str) -> Result<()> {
        self.write(&format!("<{}>", tag))
    }

    #[inline]
    fn close(&mut self, tag: &str) -> Result<()> {
        self.write(&format!("</{}>", tag))
    }

    fn keyword(&mut self, xml: &str) -> Result<()> {
        self.leaf("keyword", xml)
    }

    fn symbol(&mut self, xml: &str) -> Result<()> {
        self.leaf("symbol", xml)
    }

    fn ident(&mut self, xml: &str) -> Result<()> {
        self.leaf("identifier", xml)
    }

    fn leaf(&mut self, tag: &str, xml: &str) -> Result<()> {
        self.write(&format!("<{0}> {1} </{0}>", tag, xml))?;
        Ok(())
    }
}

trait Xml<'s, W: io::Write> {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()>;
}

impl<'s, W: io::Write> Xml<'s, W> for Class<'s> {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        ctx.nest("class", |ctx| {
            ctx.keyword("class")?;
            ctx.ident(self.name)?;
            ctx.with_brace(|ctx| {
                for def in self.variables.iter() {
                    ctx.nest("classVarDec", |ctx| {
                        def.kind.to_xml(ctx)?;
                        def.ty.to_xml(ctx)?;
                        ctx.ident(def.name)?;
                        ctx.symbol(";")
                    })?;
                }

                for f in self.functions.iter() {
                    f.to_xml(ctx)?;
                }
                Ok(())
            })
        })
    }
}

impl<'s, W: io::Write> Xml<'s, W> for VariableDef<'s> {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        ctx.nest("varDec", |ctx| {
            self.kind.to_xml(ctx)?;
            self.ty.to_xml(ctx)?;
            ctx.ident(self.name)?;
            ctx.symbol(";")?;
            Ok(())
        })
    }
}

impl<'s, W: io::Write> Xml<'s, W> for VariableDefKind {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        let kind = match self {
            VariableDefKind::Var => "var",
            VariableDefKind::Field => "field",
            VariableDefKind::Static => "static",
        };
        ctx.keyword(kind)
    }
}

impl<'s, W: io::Write> Xml<'s, W> for FnDef<'s> {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        ctx.nest("subroutineDec", |ctx| {
            self.kind.to_xml(ctx)?;
            self.ret.to_xml(ctx)?;
            ctx.ident(self.name)?;
            ctx.with_paren(|ctx| {
                ctx.nest("parameterList", |ctx| {
                    for (i, param) in self.params.iter().enumerate() {
                        param.to_xml(ctx)?;
                        if i + 1 != self.params.len() {
                            ctx.symbol(",")?;
                        }
                    }
                    Ok(())
                })
            })?;
            self.body.to_xml(ctx)
        })
    }
}

impl<'s, W: io::Write> Xml<'s, W> for FnKind {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        use FnKind::*;
        let kind = match self {
            Ctor => "constructor",
            Function => "function",
            Method => "method",
        };
        ctx.keyword(kind)
    }
}

impl<'s, W: io::Write> Xml<'s, W> for Parameter<'s> {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        self.ty.to_xml(ctx)?;
        ctx.ident(self.name)?;
        Ok(())
    }
}

impl<'s, W: io::Write> Xml<'s, W> for FnBody<'s> {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        ctx.nest("subroutineBody", |ctx| {
            ctx.with_brace(|ctx| {
                for var in self.variables.iter() {
                    var.to_xml(ctx)?;
                }
                ctx.nest("statements", |ctx| {
                    for stmt in self.statements.iter() {
                        stmt.to_xml(ctx)?;
                    }
                    Ok(())
                })
            })
        })
    }
}

impl<'s, W: io::Write> Xml<'s, W> for Stmt<'s> {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        match &self.kind {
            StmtKind::Let { lhs, rhs } => {
                ctx.nest("letStatement", |ctx| {
                    ctx.keyword("let")?;
                    lhs.to_xml(ctx)?;
                    ctx.symbol("=")?;
                    rhs.to_xml(ctx)?;
                    ctx.symbol(";")?;
                    Ok(())
                })?;
            }
            StmtKind::If {
                cond,
                then_branch,
                else_branch,
            } => {
                ctx.nest("ifStatement", |ctx| {
                    ctx.keyword("if")?;
                    ctx.with_paren(|ctx| cond.to_xml(ctx))?;
                    ctx.with_brace(|ctx| {
                        ctx.nest("statements", |ctx| {
                            for stmt in then_branch.iter() {
                                stmt.to_xml(ctx)?;
                            }
                            Ok(())
                        })
                    })?;
                    if let Some(else_branch) = else_branch {
                        ctx.keyword("else")?;
                        ctx.with_brace(|ctx| {
                            ctx.nest("statements", |ctx| {
                                for stmt in else_branch.iter() {
                                    stmt.to_xml(ctx)?;
                                }
                                Ok(())
                            })
                        })?;
                    }
                    Ok(())
                })?;
            }
            StmtKind::While { cond, body } => {
                ctx.nest("whileStatement", |ctx| {
                    ctx.keyword("while")?;
                    ctx.with_paren(|ctx| cond.to_xml(ctx))?;
                    ctx.with_brace(|ctx| {
                        ctx.nest("statements", |ctx| {
                            for stmt in body.iter() {
                                stmt.to_xml(ctx)?;
                            }
                            Ok(())
                        })
                    })
                })?;
            }
            StmtKind::Do(fncall) => {
                ctx.nest("doStatement", |ctx| {
                    ctx.keyword("do")?;
                    fncall.to_xml(ctx)?;
                    ctx.symbol(";")
                })?;
            }
            StmtKind::Return(retval) => {
                ctx.nest("returnStatement", |ctx| {
                    ctx.keyword("return")?;
                    if let Some(expr) = retval {
                        expr.to_xml(ctx)?;
                    }
                    ctx.symbol(";")
                })?;
            }
        }
        Ok(())
    }
}

impl<'s, W: io::Write> Xml<'s, W> for Expr<'s> {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        ctx.nest("expression", |ctx| {
            self.lhs.to_xml(ctx)?;
            if let Some(box (ref op, ref rhs)) = self.rhs {
                op.to_xml(ctx)?;
                rhs.to_xml(ctx)?;
            }
            Ok(())
        })
    }
}

impl<'s, W: io::Write> Xml<'s, W> for Term<'s> {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        ctx.nest("term", |ctx| {
            match &self.kind {
                TermKind::Const(constant) => match constant {
                    Constant::Integer(n) => ctx.leaf("integerConstant", &n.to_string())?,
                    Constant::Str(s) => ctx.leaf("stringConstant", s)?,
                    Constant::True => ctx.keyword("true")?,
                    Constant::False => ctx.keyword("false")?,
                    Constant::Null => ctx.keyword("null")?,
                    Constant::This => ctx.keyword("this")?,
                },
                TermKind::Variable(variable) => variable.to_xml(ctx)?,
                TermKind::Unary { op, term } => {
                    op.to_xml(ctx)?;
                    term.to_xml(ctx)?;
                }
                TermKind::FnCall(fncall) => fncall.to_xml(ctx)?,
                TermKind::Expr(expr) => ctx.with_paren(|ctx| expr.to_xml(ctx))?,
            }
            Ok(())
        })
    }
}

impl<'s, W: io::Write> Xml<'s, W> for FnCall<'s> {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        if let Some(receiver) = self.receiver {
            ctx.ident(receiver)?;
            ctx.symbol(".")?;
        }
        ctx.ident(self.fn_name)?;

        ctx.with_paren(|ctx| {
            ctx.nest("expressionList", |ctx| {
                for (i, expr) in self.args.iter().enumerate() {
                    expr.to_xml(ctx)?;
                    if i + 1 != self.args.len() {
                        ctx.symbol(",")?;
                    }
                }
                Ok(())
            })
        })
    }
}

impl<'s, W: io::Write> Xml<'s, W> for Variable<'s> {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        ctx.ident(self.name)?;
        if let Some(ref expr) = self.index_accessor {
            ctx.with_bracket(|ctx| expr.to_xml(ctx))?;
        }
        Ok(())
    }
}

impl<'s, W: io::Write> Xml<'s, W> for Type<'s> {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        use Type::*;
        let kind = match self {
            Int => "int",
            Char => "char",
            Boolean => "boolean",
            Void => "void",
            Class(name) => return ctx.ident(name),
        };
        ctx.keyword(kind)
    }
}

impl<'s, W: io::Write> Xml<'s, W> for BinOp {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        let sym = match self {
            BinOp::Plus => "+",
            BinOp::Minus => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::And => "&amp;",
            BinOp::Or => "|",
            BinOp::Equal => "=",
            BinOp::Lt => "&lt;",
            BinOp::Gt => "&gt;",
        };
        ctx.symbol(sym)
    }
}

impl<'s, W: io::Write> Xml<'s, W> for UnaryOp {
    fn to_xml(&self, ctx: &mut XmlContext<'s, W>) -> anyhow::Result<()> {
        let sym = match self {
            UnaryOp::Not => "~",
            UnaryOp::Minus => "-",
        };
        ctx.symbol(sym)
    }
}

fn remove_ws(s: String) -> String {
    s.chars().filter(|c| !matches!(c, '\r')).collect()
}

macro_rules! assert_parsed_xml {
    ($input:literal) => {
        let jack = include_str!(concat!($input, ".jack"));
        let expected = include_str!(concat!($input, ".xml"));

        let mut writer = BufWriter::new(Vec::new());
        let mut ctx = XmlContext::new(&mut writer);
        parse(jack).unwrap().to_xml(&mut ctx).unwrap();
        let xml = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        similar_asserts::assert_eq!(remove_ws(xml), remove_ws(expected.to_string()));
    };
}

#[test]
fn parse_array_test_in_xml() {
    assert_parsed_xml!("./fixtures/ArrayTest/Main");
}

#[test]
fn parse_expression_less_square() {
    assert_parsed_xml!("./fixtures/ExpressionLessSquare/Main");
    assert_parsed_xml!("./fixtures/ExpressionLessSquare/Square");
    assert_parsed_xml!("./fixtures/ExpressionLessSquare/SquareGame");
}

#[test]
fn parse_square() {
    assert_parsed_xml!("./fixtures/Square/Main");
    assert_parsed_xml!("./fixtures/Square/Square");
    assert_parsed_xml!("./fixtures/Square/SquareGame");
}
