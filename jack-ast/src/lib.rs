//! Abstract Syntax Tree of Jack
//!
//! Legend:
//!
//! 'xxx'   -- token (terminal symbol)
//! xxx :== -- definition of xxx (non-terminal)
//! (..)    -- group
//! x|y     -- alternation (x or y)
//! x?      -- zero or one repetition
//! x*      -- zero or more repetition
//!
//! note that each *.hack file contains only one class definition.
//!
//! ## Structure
//!
//! class :== 'class' class_name '{' class_variable_def* fn_def* '}'
//! class_variable_def :== ('static' | 'field') type variable_name (',' variable_name)* ';'
//! type :== 'int' | 'char' | 'boolean' | class_name
//!
//! fn_kind :== 'constructor' | 'function' | 'method'
//! fn_def :== fn_kind ('void' | type) fn_name '(' parameter_list ')' '{' fn_body '}'
//! parameter_list :== ((type arg_name) (',' type arg_name)*)?
//! fn_body :== variable_def* statement*
//! variable_def :== 'var' type variable_name (',' variable_name)* ';'
//!
//! class_name :== identifier
//! fn_name :== identifier
//! variable_name :== identifier
//!
//! ## Statement
//!
//! statement :== let_stmt | if_stmt | while_stmt | do_stmt | return_stmt
//! let_stmt :== 'let' variable_name ('[' expression ']')? '=' expression ';'
//! if_stmt :== 'if' '(' expression ')' '{' statement* '}' ('else' '{' statements* '}')?
//! while_stmt :== 'while' '(' expression ')' '{' statement* '}'
//! do_stmt := 'do' fn_call ';'
//! return_stmt := 'return' expression? ';'
//!
//! ## Expression
//!
//! expression :== term (op term)*
//! term :== integer_const | string_const | keyword_const | variable_name
//!          | variable_name '[' expression ']' | fn_call | '(' expression ')'
//!          | unary_op term
//! keyword_const :== 'true' | 'false' | 'null' | 'this'
//!
//! fn_call :== fn_name '(' arg_list ')'
//!             | (class_name | variable_name) '.' fn_name '(' arg_list ')'
//! arg_list :== (expression (',' expression)* )?
//!
//! op :== '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '~'
//! unary_op :== '-' | '~'
#![forbid(unsafe_code)]
#![feature(box_patterns)]

pub mod span;

use std::fmt;

pub use span::*;
pub type Ident<'s> = &'s str;

#[derive(Debug, PartialEq)]
pub struct Class<'s> {
    pub name: Ident<'s>,
    // class ClassName {
    //       [--------) span
    pub span: Span,
    pub variables: Vec<VariableDef<'s>>,
    pub functions: Vec<FnDef<'s>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Type<'s> {
    Int,
    Char,
    Boolean,
    Class(Ident<'s>),
    // Void only appears in return type of FnDef
    Void,
}

#[derive(Debug, PartialEq)]
pub struct FnDef<'s> {
    pub name: Ident<'s>,
    // {fn kind} {ret} fn_name( .. ) {}
    //                 [------) span
    pub span: Span,
    pub kind: FnKind,
    pub ret: Type<'s>,
    pub params: Vec<Parameter<'s>>,
    pub body: FnBody<'s>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FnKind {
    Ctor,
    Function,
    Method,
}

#[derive(Debug, PartialEq)]
pub struct Parameter<'s> {
    pub name: Ident<'s>,
    pub ty: Type<'s>,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct FnBody<'s> {
    pub variables: Vec<VariableDef<'s>>,
    pub statements: Vec<Stmt<'s>>,
}

#[derive(Debug, PartialEq)]
pub struct Stmt<'s> {
    pub kind: StmtKind<'s>,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub enum StmtKind<'s> {
    /// let `name`([`index_accessor`])? = `expr`;
    Let {
        lhs: Variable<'s>,
        rhs: Expr<'s>,
    },
    /// if (`cond`) { `then_branch` } (else { `else_branch` })
    If {
        cond: Expr<'s>,
        then_branch: Vec<Stmt<'s>>,
        else_branch: Option<Vec<Stmt<'s>>>,
    },
    /// while (`cond`) { `body` }
    While {
        cond: Expr<'s>,
        body: Vec<Stmt<'s>>,
    },
    Do(FnCall<'s>),
    Return(Option<Expr<'s>>),
}

#[derive(Debug, PartialEq)]
pub struct Expr<'s> {
    pub lhs: Box<Term<'s>>,
    pub rhs: Option<Box<(BinOp, Term<'s>)>>,
}
impl<'s> Expr<'s> {
    pub fn span(&self) -> Span {
        match self.rhs {
            Some(box (_, ref term)) => self.lhs.span.with_hi(term.span.hi()),
            None => self.lhs.span,
        }
    }

    pub fn is_null(&self) -> bool {
        // Null cannot come in rhs since Jack does not support any operations with null
        // like (term `op` null)
        self.lhs.kind == TermKind::Const(Constant::Null)
    }
}

#[derive(Debug, PartialEq)]
pub struct Term<'s> {
    pub kind: TermKind<'s>,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub enum TermKind<'s> {
    Const(Constant<'s>),
    // name [ index_accessor ]?
    Variable(Variable<'s>),
    // name ( args )
    FnCall(FnCall<'s>),
    // ( expr )
    Expr(Box<Expr<'s>>),
    Unary { op: UnaryOp, term: Box<Term<'s>> },
}

#[derive(Debug, PartialEq)]
pub struct FnCall<'s> {
    pub receiver: Option<Ident<'s>>,
    pub fn_name: Ident<'s>,
    pub args: Vec<Expr<'s>>,
}

#[derive(Debug, PartialEq)]
pub struct VariableDef<'s> {
    pub name: Ident<'s>,
    pub kind: VariableDefKind,
    pub ty: Type<'s>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VariableDefKind {
    Static,
    Field,
    // Var only appears in function
    Var,
}

#[derive(Debug, PartialEq)]
pub struct Variable<'s> {
    pub name: &'s str,
    pub index_accessor: Option<Expr<'s>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Plus,
    Minus,
    Mul,
    Div,
    And,
    Or,
    Equal,
    Lt,
    Gt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Minus,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum Constant<'s> {
    Integer(u32),
    Str(&'s str),
    True,
    False,
    Null,
    This,
}

impl fmt::Display for Stmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            StmtKind::Let { lhs, rhs } => write!(f, "let {lhs} = {rhs}"),
            StmtKind::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let else_branch = match else_branch {
                    Some(els) => format!(" else {{ ({} statements...) }}", els.len()),
                    None => "".to_owned(),
                };
                write!(
                    f,
                    "if ({cond}) {{ ({} statements...) }}{else_branch}",
                    then_branch.len(),
                )
            }
            StmtKind::While { cond, body } => {
                write!(f, "while ({cond}) {{ ({} statements...) }}", body.len())
            }
            StmtKind::Do(call) => call.fmt(f),
            StmtKind::Return(retval) => match retval {
                Some(v) => write!(f, "return {v}"),
                None => write!(f, "return"),
            },
        }
    }
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.rhs {
            Some(box (ref op, ref term)) => write!(f, "{} {op} {term}", self.lhs),
            None => write!(f, "{}", self.lhs),
        }
    }
}

impl fmt::Display for Term<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TermKind::*;
        match &self.kind {
            Const(c) => c.fmt(f),
            Variable(v) => v.fmt(f),
            FnCall(call) => call.fmt(f),
            Expr(e) => e.fmt(f),
            Unary { op, term } => write!(f, "{op}{term}"),
        }
    }
}

impl fmt::Display for FnCall<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.receiver {
            Some(receiver) => format!("{}.{}", receiver, self.fn_name),
            None => self.fn_name.to_owned(),
        };
        let args = self
            .args
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}({})", name, args)
    }
}

impl fmt::Display for VariableDefKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariableDefKind::Static => "static",
            VariableDefKind::Field => "field",
            VariableDefKind::Var => "var",
        }
        .fmt(f)
    }
}

impl fmt::Display for Variable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suffix = match self.index_accessor {
            Some(ref expr) => format!("[{}]", expr),
            None => "".to_owned(),
        };

        write!(f, "{}{}", self.name, suffix)
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BinOp::*;
        match self {
            Plus => "+",
            Minus => "-",
            Mul => "*",
            Div => "/",
            And => "&",
            Or => "|",
            Equal => "=",
            Lt => "<",
            Gt => ">",
        }
        .fmt(f)
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Minus => "-",
            UnaryOp::Not => "~",
        }
        .fmt(f)
    }
}

impl fmt::Display for Constant<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Constant::*;
        match self {
            Integer(v) => return write!(f, "{v}"),
            Str(ref s) => return write!(f, "{s}"),
            True => "true",
            False => "false",
            Null => "null",
            This => "this",
        }
        .fmt(f)
    }
}

impl fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => "int",
            Type::Char => "char",
            Type::Boolean => "boolean",
            Type::Void => "void",
            Type::Class(ident) => return write!(f, "class({ident})"),
        }
        .fmt(f)
    }
}
