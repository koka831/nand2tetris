//! construct syntax tree from lexed Jack program.
use std::borrow::Cow;
use std::iter::Peekable;

use crate::{
    lexer::Lexer,
    token::{KwKind, Token, TokenKind},
    LexError, ParseError, ParseErrorKind as ErrorKind,
};
use jack_ast::*;

pub fn parse(input: &str) -> ParseResult<'_, Class<'_>> {
    let mut parser = Parser::new(input);
    parser.parse()
}

type ParseResult<'s, T> = std::result::Result<T, Box<ParseError<'s>>>;

// Wrap `Peekable` and provide `peek` method, since we cannot convert errors inside `peek`ed(referenced) values.
struct TokenStream<'source> {
    tokens: Peekable<Lexer<'source>>,
    peeked: Option<Result<Token<'source>, LexError<'source>>>,
    current_span: Span,
}

impl<'s> TokenStream<'s> {
    pub fn new(lexer: Lexer<'s>) -> Self {
        let mut tokens = lexer.peekable();
        let peeked = tokens.next();
        let current_span = match peeked {
            Some(Ok(ref token)) => token.span,
            _ => Span::new(0, 0),
        };

        TokenStream {
            tokens,
            peeked,
            current_span,
        }
    }

    pub fn peek(&self) -> Option<&Result<Token<'s>, LexError<'s>>> {
        self.peeked.as_ref()
    }

    pub fn next(&mut self) -> Option<Result<Token<'s>, LexError<'s>>> {
        let next = self.peeked.take().or_else(|| self.tokens.next());
        if let Some(Ok(ref token)) = next {
            self.current_span = token.span;
        }
        self.peeked = self.tokens.next();

        next
    }
}

pub(crate) struct Parser<'source> {
    input: &'source str,
    tokens: TokenStream<'source>,
}

impl<'s> Parser<'s> {
    pub fn new(input: &'s str) -> Parser<'s> {
        let tokens = TokenStream::new(Lexer::new(input));
        Parser { tokens, input }
    }

    pub fn parse<P: Parse<'s>>(&mut self) -> ParseResult<'s, P> {
        Parse::<'s>::parse(self)
    }

    fn error<T>(&self, kind: ErrorKind<'s>, span: Span) -> ParseResult<'s, T> {
        Err(Box::new(ParseError::new(kind, self.input, span)))
    }

    fn unexpected_token<T>(&self, token: &Token<'s>) -> ParseResult<'s, T> {
        let token = token.clone();
        let span = token.span;
        self.error(ErrorKind::UnexpectedToken(token), span)
    }

    fn eat_token(&mut self) -> ParseResult<'s, Token<'s>> {
        let span = self.tokens.current_span;
        match self.tokens.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(Box::new(e.into())),
            None => self.error(ErrorKind::UnexpectedEOF, span),
        }
    }

    fn peek_token(&self) -> ParseResult<'s, &Token<'s>> {
        let span = self.current_span();
        match self.tokens.peek() {
            Some(Ok(ref token)) => Ok(token),
            Some(Err(e)) => Err(Box::new(e.clone().into())),
            None => self.error(ErrorKind::UnexpectedEOF, span),
        }
    }

    fn parse_vec<T: Parse<'s>>(&mut self, separator: &TokenKind<'s>) -> ParseResult<'s, Vec<T>> {
        let mut args = Vec::new();
        if let Ok(expr) = self.parse() {
            args.push(expr);
            while self.eat_if_matches(separator).is_ok() {
                let expr = self.parse()?;
                args.push(expr);
            }
        }

        Ok(args)
    }

    fn eat_if_matches(&mut self, want: &TokenKind<'s>) -> ParseResult<'s, Span> {
        let token = self.peek_token()?;
        let span = token.span;
        if token.kind == *want {
            self.eat_token()?;
            Ok(span)
        } else {
            self.unexpected_token(token)
        }
    }

    fn eat_by(&mut self, expected: TokenKind<'s>) -> ParseResult<'s, Span> {
        let span = self.current_span();
        match self.eat_token() {
            Ok(t) if t.kind == expected => Ok(t.span),
            Ok(actual) => {
                let msg = format!(
                    "unexpected kind of token given: eat_by(expected: {:?}), actual: {:?}",
                    expected, actual
                );
                self.error(ErrorKind::InternalCompilerError(Cow::Owned(msg)), span)
            }
            Err(e) => Err(e),
        }
    }

    fn current_span(&self) -> Span {
        self.tokens.current_span
    }
}

pub(crate) trait Parse<'s>: Sized {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self>;
}

impl<'s> Parse<'s> for Ident<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let token = parser.peek_token()?;
        match token.kind {
            TokenKind::Ident(ident) => {
                parser.eat_token()?;
                Ok(ident)
            }
            TokenKind::Keyword(ref kind) => {
                return parser.error(ErrorKind::ReservedKeyword(kind.clone()), token.span);
            }
            _ => parser.unexpected_token(token),
        }
    }
}

impl<'s> Parse<'s> for Class<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        parser.eat_if_matches(&TokenKind::Keyword(KwKind::Class))?;
        let name = match parser.parse() {
            Ok(name) => name,
            Err(e) => {
                if let ErrorKind::ReservedKeyword(ref kind) = e.kind {
                    let msg = format!("you cannot use the keyword `{kind}` for a class name");
                    return Err(Box::new(e.with_help(Cow::Owned(msg))));
                }
                return Err(e);
            }
        };
        let span = parser.current_span();
        parser.eat_by(TokenKind::LBrace)?;

        let mut variables = Vec::new();
        let mut functions = Vec::new();

        loop {
            match parser.parse::<Vec<_>>() {
                Ok(vs) => variables.extend(vs),
                Err(e) => match e.kind {
                    ErrorKind::LexError(_) => return Err(e),
                    _ => break,
                },
            }
        }

        loop {
            match parser.parse() {
                Ok(function) => functions.push(function),
                Err(e) => match e.kind {
                    ErrorKind::LexError(_) => return Err(e),
                    _ => break,
                },
            }
        }

        parser.eat_by(TokenKind::RBrace)?;

        Ok(Class {
            name,
            span,
            variables,
            functions,
        })
    }
}

impl<'s> Parse<'s> for Vec<VariableDef<'s>> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let kind = parser.parse()?;
        let ty = parser.parse()?;

        let mut vars = Vec::new();
        let name = parser.parse()?;

        vars.push(VariableDef {
            kind,
            ty,
            name,
            span: parser.current_span(),
        });

        while parser.eat_if_matches(&TokenKind::Comma).is_ok() {
            let name = parser.parse()?;
            vars.push(VariableDef {
                kind,
                ty,
                name,
                span: parser.current_span(),
            });
        }

        parser.eat_by(TokenKind::Semicolon)?;
        Ok(vars)
    }
}

impl<'s> Parse<'s> for VariableDefKind {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let token = parser.peek_token()?;
        let TokenKind::Keyword(ref kind) = token.kind else { return parser.unexpected_token(token) };

        let kind = match kind {
            KwKind::Static => VariableDefKind::Static,
            KwKind::Field => VariableDefKind::Field,
            KwKind::Var => VariableDefKind::Var,
            _ => return parser.unexpected_token(token),
        };

        parser.eat_token()?;
        Ok(kind)
    }
}

impl<'s> Parse<'s> for Type<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let token = parser.peek_token()?;
        let ty = match token.kind {
            TokenKind::Keyword(ref kind) => match kind {
                KwKind::Int => Type::Int,
                KwKind::Char => Type::Char,
                KwKind::Boolean => Type::Boolean,
                KwKind::Void => Type::Void,
                _ => return parser.unexpected_token(token),
            },
            TokenKind::Ident(class_name) => Type::Class(class_name),
            _ => return parser.unexpected_token(token),
        };

        parser.eat_token()?;
        Ok(ty)
    }
}

impl<'s> Parse<'s> for FnDef<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let kind = parser.parse()?;
        let ret = parser.parse()?;
        let name = parser.parse()?;
        let span = parser.current_span();
        parser.eat_by(TokenKind::LParen)?;
        let params = parser.parse()?;
        parser.eat_by(TokenKind::RParen)?;
        parser.eat_by(TokenKind::LBrace)?;
        let body = parser.parse()?;
        parser.eat_by(TokenKind::RBrace)?;

        Ok(FnDef {
            name,
            span,
            kind,
            ret,
            params,
            body,
        })
    }
}

impl<'s> Parse<'s> for FnKind {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let token = parser.peek_token()?;
        let TokenKind::Keyword(ref kind) = token.kind else { return parser.unexpected_token(token) };
        let kind = match kind {
            KwKind::Ctor => FnKind::Ctor,
            KwKind::Function => FnKind::Function,
            KwKind::Method => FnKind::Method,
            _ => return parser.unexpected_token(token),
        };

        parser.eat_token()?;
        Ok(kind)
    }
}

impl<'s> Parse<'s> for Parameter<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let ty = parser.parse()?;
        let mut span = parser.current_span();
        let name = parser.parse()?;
        span = span.with_hi(parser.current_span().hi());

        Ok(Parameter { name, ty, span })
    }
}

impl<'s> Parse<'s> for Vec<Parameter<'s>> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let mut params = Vec::new();
        if let Ok(first) = parser.parse() {
            params.push(first);
            while parser.eat_if_matches(&TokenKind::Comma).is_ok() {
                let param = parser.parse()?;
                params.push(param);
            }
        }

        Ok(params)
    }
}

impl<'s> Parse<'s> for FnBody<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let mut variables = Vec::new();
        let mut statements = Vec::new();
        // read tokens until closing brace found
        while parser
            .peek_token()
            .is_ok_and(|token| token.kind != TokenKind::RBrace)
        {
            if let Ok(vs) = parser.parse::<Vec<VariableDef>>() {
                variables.extend(vs);
                continue;
            }

            if let Ok(stmt) = parser.parse() {
                statements.push(stmt);
                continue;
            }

            break;
        }

        Ok(FnBody {
            variables,
            statements,
        })
    }
}

impl<'s> Parse<'s> for Stmt<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let token = parser.peek_token()?;
        let TokenKind::Keyword(ref kind) = token.kind else { return parser.unexpected_token(token) };
        let span = token.span;

        let stmt = match kind {
            KwKind::Let => {
                // consume `let`
                parser.eat_token()?;
                let lhs = parser.parse()?;
                parser.eat_by(TokenKind::Equal)?;
                let rhs = parser.parse()?;
                parser.eat_by(TokenKind::Semicolon)?;

                Stmt {
                    kind: StmtKind::Let { lhs, rhs },
                    span: span.with_hi(parser.current_span().hi()),
                }
            }
            KwKind::If => {
                // consume `if`
                parser.eat_token()?;
                parser.eat_by(TokenKind::LParen)?;
                let cond = parser.parse()?;
                parser.eat_by(TokenKind::RParen)?;

                parser.eat_by(TokenKind::LBrace)?;
                let then_branch = parser.parse()?;
                parser.eat_by(TokenKind::RBrace)?;

                let else_branch = if parser
                    .eat_if_matches(&TokenKind::Keyword(KwKind::Else))
                    .is_ok()
                {
                    parser.eat_by(TokenKind::LBrace)?;
                    let else_branch = parser.parse()?;
                    parser.eat_by(TokenKind::RBrace)?;
                    Some(else_branch)
                } else {
                    None
                };

                Stmt {
                    kind: StmtKind::If {
                        cond,
                        then_branch,
                        else_branch,
                    },
                    span: span.with_hi(parser.current_span().hi()),
                }
            }
            KwKind::While => {
                // consume `while`
                parser.eat_token()?;
                parser.eat_by(TokenKind::LParen)?;
                let cond = parser.parse()?;
                parser.eat_by(TokenKind::RParen)?;

                parser.eat_by(TokenKind::LBrace)?;
                let body = parser.parse()?;
                parser.eat_by(TokenKind::RBrace)?;

                Stmt {
                    kind: StmtKind::While { cond, body },
                    span: span.with_hi(parser.current_span().hi()),
                }
            }
            KwKind::Do => {
                // consume `do`
                parser.eat_token()?;
                let fn_call = parser.parse()?;
                parser.eat_by(TokenKind::Semicolon)?;

                Stmt {
                    kind: StmtKind::Do(fn_call),
                    span: span.with_hi(parser.current_span().hi()),
                }
            }
            KwKind::Return => {
                // consume `return`
                parser.eat_token()?;
                let maybe_semi = parser.peek_token()?;
                let retval = if maybe_semi.kind == TokenKind::Semicolon {
                    None
                } else {
                    let expr = parser.parse()?;
                    Some(expr)
                };

                parser.eat_by(TokenKind::Semicolon)?;

                Stmt {
                    kind: StmtKind::Return(retval),
                    span: span.with_hi(parser.current_span().hi()),
                }
            }
            _ => return parser.unexpected_token(token),
        };

        Ok(stmt)
    }
}

impl<'s> Parse<'s> for Vec<Stmt<'s>> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let terminator = TokenKind::RBrace;
        let mut statements = Vec::new();
        // parse Stmt until terminator appear
        while parser.peek_token().is_ok_and(|t| t.kind != terminator) {
            let stmt = parser.parse()?;
            statements.push(stmt);
        }

        Ok(statements)
    }
}

impl<'s> Parse<'s> for Expr<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let lhs = Box::new(parser.parse()?);
        let rhs = if let Ok(op) = parser.parse() {
            let rhs = parser.parse()?;
            Some(Box::new((op, rhs)))
        } else {
            None
        };

        Ok(Expr { lhs, rhs })
    }
}

impl<'s> Parse<'s> for Term<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        if let Ok(constant) = parser.parse::<Constant>() {
            return Ok(Term {
                kind: TermKind::Const(constant),
                span: parser.current_span(),
            });
        }

        // Unary
        if let Ok(op) = parser.parse() {
            let span = parser.current_span();
            let term = parser.parse::<Term>()?;
            let span = span.with_hi(term.span.hi());
            return Ok(Term {
                kind: TermKind::Unary {
                    op,
                    term: Box::new(term),
                },
                span,
            });
        }

        // Expr
        if let Ok(span) = parser.eat_if_matches(&TokenKind::LParen) {
            let expr = parser.parse()?;
            let hi = parser.eat_by(TokenKind::RParen)?.hi();

            return Ok(Term {
                kind: TermKind::Expr(Box::new(expr)),
                span: span.with_hi(hi),
            });
        }

        if let Ok(name) = parser.parse() {
            let span = parser.current_span();
            if parser.eat_if_matches(&TokenKind::LParen).is_ok() {
                // FnCall
                let args = parser.parse_vec(&TokenKind::Comma)?;
                let hi = parser.eat_by(TokenKind::RParen)?.hi();
                let span = span.with_hi(hi);
                return Ok(Term {
                    kind: TermKind::FnCall(FnCall {
                        receiver: None,
                        fn_name: name,
                        args,
                    }),
                    span,
                });
            } else if parser.eat_if_matches(&TokenKind::Dot).is_ok() {
                // FnCall with receiver
                let fn_name = parser.parse()?;
                parser.eat_by(TokenKind::LParen)?;
                let args = parser.parse_vec(&TokenKind::Comma)?;
                let hi = parser.eat_by(TokenKind::RParen)?.hi();
                let span = span.with_hi(hi);
                return Ok(Term {
                    kind: TermKind::FnCall(FnCall {
                        receiver: Some(name),
                        fn_name,
                        args,
                    }),
                    span,
                });
            } else {
                // Variable
                let mut span = parser.current_span();
                // since ident token has been eaten, try parsing Variable manually here
                let index_accessor = if parser.eat_if_matches(&TokenKind::LBracket).is_ok() {
                    let accessor = parser.parse::<Expr>()?;
                    let hi = parser.eat_by(TokenKind::RBracket)?.hi();
                    span = span.with_hi(hi);

                    Some(accessor)
                } else {
                    None
                };

                return Ok(Term {
                    kind: TermKind::Variable(Variable {
                        name,
                        index_accessor,
                        span,
                    }),
                    span,
                });
            }
        }
        parser.unexpected_token(parser.peek_token()?)
    }
}

impl<'s> Parse<'s> for FnCall<'s> {
    // (receiver.)?fn_name( args.. )
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let mut fn_name = parser.parse()?;
        let receiver = if parser.eat_if_matches(&TokenKind::Dot).is_ok() {
            let name = parser.parse()?;
            let receiver = fn_name;
            fn_name = name;
            Some(receiver)
        } else {
            None
        };
        parser.eat_by(TokenKind::LParen)?;
        let args = parser.parse_vec(&TokenKind::Comma)?;
        parser.eat_by(TokenKind::RParen)?;
        Ok(FnCall {
            receiver,
            fn_name,
            args,
        })
    }
}

impl<'s> Parse<'s> for Variable<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let name = parser.parse()?;
        let mut span = parser.current_span();
        let index_accessor = if parser.eat_if_matches(&TokenKind::LBracket).is_ok() {
            let accessor = parser.parse()?;
            let hi = parser.eat_by(TokenKind::RBracket)?.hi();
            span = span.with_hi(hi);

            Some(accessor)
        } else {
            None
        };

        Ok(Variable {
            name,
            index_accessor,
            span,
        })
    }
}

impl<'s> Parse<'s> for BinOp {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        use TokenKind::*;

        let token = parser.peek_token()?;
        let op = match token.kind {
            Plus => BinOp::Plus,
            Minus => BinOp::Minus,
            Mul => BinOp::Mul,
            Slash => BinOp::Div,
            And => BinOp::And,
            Or => BinOp::Or,
            Equal => BinOp::Equal,
            Lt => BinOp::Lt,
            Gt => BinOp::Gt,
            _ => return parser.unexpected_token(token),
        };

        parser.eat_token()?;
        Ok(op)
    }
}

impl<'s> Parse<'s> for UnaryOp {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let token = parser.peek_token()?;
        let op = match token.kind {
            TokenKind::Minus => UnaryOp::Minus,
            TokenKind::Tilde => UnaryOp::Not,
            _ => return parser.unexpected_token(token),
        };

        parser.eat_token()?;
        Ok(op)
    }
}

impl<'s> Parse<'s> for Constant<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let token = parser.peek_token()?;
        let constant = match token.kind {
            TokenKind::Integer(n) => Constant::Integer(n),
            TokenKind::Str(s) => Constant::Str(s),
            TokenKind::Keyword(ref kwkind) => match kwkind {
                KwKind::True => Constant::True,
                KwKind::False => Constant::False,
                KwKind::Null => Constant::Null,
                KwKind::This => Constant::This,
                _ => return parser.unexpected_token(token),
            },
            _ => return parser.unexpected_token(token),
        };

        parser.eat_token()?;
        Ok(constant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    fn assert_parse<'a, P>(input: &'a str, expect: P)
    where
        P: Parse<'a> + PartialEq + fmt::Debug,
    {
        let parsed = Parser::new(input).parse::<P>().unwrap();
        assert_eq!(parsed, expect);
    }

    #[test]
    fn parse_reserved_keyword() {
        match parse("class class {}").unwrap_err().kind {
            ErrorKind::ReservedKeyword(_) => {}
            e => panic!("did not match to expected error kind {e:?}"),
        }
    }

    #[test]
    fn handle_lex_error() {
        match parse("class Main { /* */ ? }").unwrap_err().kind {
            ErrorKind::LexError(_) => {}
            e => panic!("did not match to expected error kind {e:?}"),
        }
    }

    #[test]
    fn parse_variable_defs() {
        assert_parse(
            "var int foo;",
            vec![VariableDef {
                name: "foo",
                kind: VariableDefKind::Var,
                ty: Type::Int,
                span: Span::new(8, 11),
            }],
        );
        assert_parse(
            "field SomeClass instance;",
            vec![VariableDef {
                name: "instance",
                kind: VariableDefKind::Field,
                ty: Type::Class("SomeClass"),
                span: Span::new(16, 24),
            }],
        );

        assert_parse(
            "static int a, xy;",
            vec![
                VariableDef {
                    name: "a",
                    kind: VariableDefKind::Static,
                    ty: Type::Int,
                    span: Span::new(11, 12),
                },
                VariableDef {
                    name: "xy",
                    kind: VariableDefKind::Static,
                    ty: Type::Int,
                    span: Span::new(14, 16),
                },
            ],
        );
    }

    #[test]
    fn parse_fn_def() {
        assert_parse(
            "method void dispose() {}",
            FnDef {
                name: "dispose",
                span: Span::new(12, 19),
                kind: FnKind::Method,
                ret: Type::Void,
                params: vec![],
                body: FnBody {
                    variables: vec![],
                    statements: vec![],
                },
            },
        );
    }

    #[test]
    fn parse_variable_kind() {
        assert_parse("static", VariableDefKind::Static);
        assert_parse("field", VariableDefKind::Field);
        assert_parse("var", VariableDefKind::Var);
    }

    #[test]
    fn parse_fn_kind() {
        assert_parse("constructor", FnKind::Ctor);
        assert_parse("function", FnKind::Function);
        assert_parse("method", FnKind::Method);
    }

    #[test]
    fn parse_parameter() {
        assert_parse(
            "int a",
            Parameter {
                name: "a",
                ty: Type::Int,
                span: Span::new(0, 5),
            },
        );
        assert_parse(
            "SomeClass s",
            Parameter {
                name: "s",
                ty: Type::Class("SomeClass"),
                span: Span::new(0, 11),
            },
        );
    }

    #[test]
    fn parse_parameters() {
        assert_parse(
            "int a, char b",
            vec![
                Parameter {
                    name: "a",
                    ty: Type::Int,
                    span: Span::new(0, 5),
                },
                Parameter {
                    name: "b",
                    ty: Type::Char,
                    span: Span::new(7, 13),
                },
            ],
        )
    }

    #[test]
    fn parse_let_stmt() {
        assert_parse(
            "let a = 10;",
            Stmt {
                kind: StmtKind::Let {
                    lhs: Variable {
                        name: "a",
                        index_accessor: None,
                        span: Span::new(4, 5),
                    },
                    rhs: Expr {
                        lhs: Box::new(Term {
                            kind: TermKind::Const(Constant::Integer(10)),
                            span: Span::new(8, 10),
                        }),
                        rhs: None,
                    },
                },
                span: Span::new(0, 11),
            },
        );

        assert_parse(
            "let g = Fraction.gcd(numerator, denominator);",
            Stmt {
                kind: StmtKind::Let {
                    lhs: Variable {
                        name: "g",
                        index_accessor: None,
                        span: Span::new(4, 5),
                    },
                    rhs: Expr {
                        lhs: Box::new(Term {
                            kind: TermKind::FnCall(FnCall {
                                receiver: Some("Fraction"),
                                fn_name: "gcd",
                                args: vec![
                                    Expr {
                                        lhs: Box::new(Term {
                                            kind: TermKind::Variable(Variable {
                                                name: "numerator",
                                                index_accessor: None,
                                                span: Span::new(21, 30),
                                            }),
                                            span: Span::new(21, 30),
                                        }),
                                        rhs: None,
                                    },
                                    Expr {
                                        lhs: Box::new(Term {
                                            kind: TermKind::Variable(Variable {
                                                name: "denominator",
                                                index_accessor: None,
                                                span: Span::new(32, 43),
                                            }),
                                            span: Span::new(32, 43),
                                        }),
                                        rhs: None,
                                    },
                                ],
                            }),
                            span: Span::new(8, 44),
                        }),
                        rhs: None,
                    },
                },
                span: Span::new(0, 45),
            },
        );
        assert_parse(
            "\
let data = car;
let next = cdr;
",
            vec![
                Stmt {
                    kind: StmtKind::Let {
                        lhs: Variable {
                            name: "data",
                            index_accessor: None,
                            span: Span::new(4, 8),
                        },
                        rhs: Expr {
                            lhs: Box::new(Term {
                                kind: TermKind::Variable(Variable {
                                    name: "car",
                                    index_accessor: None,
                                    span: Span::new(11, 14),
                                }),
                                span: Span::new(11, 14),
                            }),
                            rhs: None,
                        },
                    },
                    span: Span::new(0, 15),
                },
                Stmt {
                    kind: StmtKind::Let {
                        lhs: Variable {
                            name: "next",
                            index_accessor: None,
                            span: Span::new(20, 24),
                        },
                        rhs: Expr {
                            lhs: Box::new(Term {
                                kind: TermKind::Variable(Variable {
                                    name: "cdr",
                                    index_accessor: None,
                                    span: Span::new(27, 30),
                                }),
                                span: Span::new(27, 30),
                            }),
                            rhs: None,
                        },
                    },
                    span: Span::new(16, 31),
                },
            ],
        );
    }

    #[test]
    fn parse_if_stmt() {
        assert_parse(
            "\
if (a) {
    let a[0] = 10;
}
            ",
            Stmt {
                kind: StmtKind::If {
                    cond: Expr {
                        lhs: Box::new(Term {
                            kind: TermKind::Variable(Variable {
                                name: "a",
                                index_accessor: None,
                                span: Span::new(4, 5),
                            }),
                            span: Span::new(4, 5),
                        }),
                        rhs: None,
                    },
                    then_branch: vec![Stmt {
                        kind: StmtKind::Let {
                            lhs: Variable {
                                name: "a",
                                span: Span::new(17, 21),
                                index_accessor: Some(Expr {
                                    lhs: Box::new(Term {
                                        kind: TermKind::Const(Constant::Integer(0)),
                                        span: Span::new(19, 20),
                                    }),
                                    rhs: None,
                                }),
                            },
                            rhs: Expr {
                                lhs: Box::new(Term {
                                    kind: TermKind::Const(Constant::Integer(10)),
                                    span: Span::new(24, 26),
                                }),
                                rhs: None,
                            },
                        },
                        span: Span::new(13, 27),
                    }],
                    else_branch: None,
                },
                span: Span::new(0, 29),
            },
        );
        assert_parse(
            "\
if (a) {
    let a[0] = 42;
} else {
    let a = 0;
}
            ",
            Stmt {
                kind: StmtKind::If {
                    cond: Expr {
                        lhs: Box::new(Term {
                            kind: TermKind::Variable(Variable {
                                name: "a",
                                index_accessor: None,
                                span: Span::new(4, 5),
                            }),
                            span: Span::new(4, 5),
                        }),
                        rhs: None,
                    },
                    then_branch: vec![Stmt {
                        kind: StmtKind::Let {
                            lhs: Variable {
                                name: "a",
                                span: Span::new(17, 21),
                                index_accessor: Some(Expr {
                                    lhs: Box::new(Term {
                                        kind: TermKind::Const(Constant::Integer(0)),
                                        span: Span::new(19, 20),
                                    }),
                                    rhs: None,
                                }),
                            },
                            rhs: Expr {
                                lhs: Box::new(Term {
                                    kind: TermKind::Const(Constant::Integer(42)),
                                    span: Span::new(24, 26),
                                }),
                                rhs: None,
                            },
                        },
                        span: Span::new(13, 27),
                    }],
                    else_branch: Some(vec![Stmt {
                        kind: StmtKind::Let {
                            lhs: Variable {
                                name: "a",
                                index_accessor: None,
                                span: Span::new(45, 46),
                            },
                            rhs: Expr {
                                lhs: Box::new(Term {
                                    kind: TermKind::Const(Constant::Integer(0)),
                                    span: Span::new(49, 50),
                                }),
                                rhs: None,
                            },
                        },
                        span: Span::new(41, 51),
                    }]),
                },
                span: Span::new(0, 53),
            },
        );
    }

    #[test]
    fn parse_while_stmt() {
        assert_parse(
            "\
while (true) {
    let a = a + 1;
}
",
            Stmt {
                kind: StmtKind::While {
                    cond: Expr {
                        lhs: Box::new(Term {
                            kind: TermKind::Const(Constant::True),
                            span: Span::new(7, 11),
                        }),
                        rhs: None,
                    },
                    body: vec![Stmt {
                        kind: StmtKind::Let {
                            lhs: Variable {
                                name: "a",
                                index_accessor: None,
                                span: Span::new(23, 24),
                            },
                            rhs: Expr {
                                lhs: Box::new(Term {
                                    kind: TermKind::Variable(Variable {
                                        name: "a",
                                        index_accessor: None,
                                        span: Span::new(27, 28),
                                    }),
                                    span: Span::new(27, 28),
                                }),
                                rhs: Some(Box::new((
                                    BinOp::Plus,
                                    Term {
                                        kind: TermKind::Const(Constant::Integer(1)),
                                        span: Span::new(31, 32),
                                    },
                                ))),
                            },
                        },
                        span: Span::new(19, 33),
                    }],
                },
                span: Span::new(0, 35),
            },
        );
    }

    #[test]
    fn parse_do_stmt() {
        assert_parse(
            "do some_method();",
            Stmt {
                kind: StmtKind::Do(FnCall {
                    receiver: None,
                    fn_name: "some_method",
                    args: vec![],
                }),
                span: Span::new(0, 17),
            },
        );
        assert_parse(
            "do cat.meow();",
            Stmt {
                kind: StmtKind::Do(FnCall {
                    receiver: Some("cat"),
                    fn_name: "meow",
                    args: vec![],
                }),
                span: Span::new(0, 14),
            },
        );
        assert_parse(
            "do add(a, b);",
            Stmt {
                kind: StmtKind::Do(FnCall {
                    receiver: None,
                    fn_name: "add",
                    args: vec![
                        Expr {
                            lhs: Box::new(Term {
                                kind: TermKind::Variable(Variable {
                                    name: "a",
                                    index_accessor: None,
                                    span: Span::new(7, 8),
                                }),
                                span: Span::new(7, 8),
                            }),
                            rhs: None,
                        },
                        Expr {
                            lhs: Box::new(Term {
                                kind: TermKind::Variable(Variable {
                                    name: "b",
                                    index_accessor: None,
                                    span: Span::new(10, 11),
                                }),
                                span: Span::new(10, 11),
                            }),
                            rhs: None,
                        },
                    ],
                }),
                span: Span::new(0, 13),
            },
        )
    }

    #[test]
    fn parse_return_stmt() {
        assert_parse(
            "return;",
            Stmt {
                kind: StmtKind::Return(None),
                span: Span::new(0, 7),
            },
        );
        assert_parse(
            "return 0;",
            Stmt {
                kind: StmtKind::Return(Some(Expr {
                    lhs: Box::new(Term {
                        kind: TermKind::Const(Constant::Integer(0)),
                        span: Span::new(7, 8),
                    }),
                    rhs: None,
                })),
                span: Span::new(0, 9),
            },
        );
    }

    #[test]
    fn parse_expr() {
        assert_parse(
            "123",
            Expr {
                lhs: Box::new(Term {
                    kind: TermKind::Const(Constant::Integer(123)),
                    span: Span::new(0, 3),
                }),
                rhs: None,
            },
        );
        assert_parse(
            "123 + 456",
            Expr {
                lhs: Box::new(Term {
                    kind: TermKind::Const(Constant::Integer(123)),
                    span: Span::new(0, 3),
                }),
                rhs: Some(Box::new((
                    BinOp::Plus,
                    Term {
                        kind: TermKind::Const(Constant::Integer(456)),
                        span: Span::new(6, 9),
                    },
                ))),
            },
        );

        assert_parse(
            "a * (b[1] + -123)",
            Expr {
                lhs: Box::new(Term {
                    kind: TermKind::Variable(Variable {
                        name: "a",
                        index_accessor: None,
                        span: Span::new(0, 1),
                    }),
                    span: Span::new(0, 1),
                }),
                rhs: Some(Box::new((
                    BinOp::Mul,
                    Term {
                        kind: TermKind::Expr(Box::new(Expr {
                            lhs: Box::new(Term {
                                kind: TermKind::Variable(Variable {
                                    name: "b",
                                    index_accessor: Some(Expr {
                                        lhs: Box::new(Term {
                                            kind: TermKind::Const(Constant::Integer(1)),
                                            span: Span::new(7, 8),
                                        }),
                                        rhs: None,
                                    }),
                                    span: Span::new(5, 9),
                                }),
                                span: Span::new(5, 9),
                            }),
                            rhs: Some(Box::new((
                                BinOp::Plus,
                                (Term {
                                    kind: TermKind::Unary {
                                        op: UnaryOp::Minus,
                                        term: Box::new(Term {
                                            kind: TermKind::Const(Constant::Integer(123)),
                                            span: Span::new(13, 16),
                                        }),
                                    },
                                    span: Span::new(12, 16),
                                }),
                            ))),
                        })),
                        span: Span::new(4, 17),
                    },
                ))),
            },
        );
    }

    #[test]
    fn parse_term() {
        assert_parse(
            "12345",
            Term {
                kind: TermKind::Const(Constant::Integer(12345)),
                span: Span::new(0, 5),
            },
        );
        assert_parse(
            r##""some string""##,
            Term {
                kind: TermKind::Const(Constant::Str("some string")),
                span: Span::new(0, 13),
            },
        );
        assert_parse(
            "abc",
            Term {
                kind: TermKind::Variable(Variable {
                    name: "abc",
                    index_accessor: None,
                    span: Span::new(0, 3),
                }),
                span: Span::new(0, 3),
            },
        );
        assert_parse(
            "-123",
            Term {
                kind: TermKind::Unary {
                    op: UnaryOp::Minus,
                    term: Box::new(Term {
                        kind: TermKind::Const(Constant::Integer(123)),
                        span: Span::new(1, 4),
                    }),
                },
                span: Span::new(0, 4),
            },
        );
        assert_parse(
            "~(next = null)",
            Term {
                kind: TermKind::Unary {
                    op: UnaryOp::Not,
                    term: Box::new(Term {
                        kind: TermKind::Expr(Box::new(Expr {
                            lhs: Box::new(Term {
                                kind: TermKind::Variable(Variable {
                                    name: "next",
                                    index_accessor: None,
                                    span: Span::new(2, 6),
                                }),
                                span: Span::new(2, 6),
                            }),
                            rhs: Some(Box::new((
                                BinOp::Equal,
                                Term {
                                    kind: TermKind::Const(Constant::Null),
                                    span: Span::new(9, 13),
                                },
                            ))),
                        })),
                        span: Span::new(1, 14),
                    }),
                },
                span: Span::new(0, 14),
            },
        );
        assert_parse(
            "a[123 + 456]",
            Term {
                kind: TermKind::Variable(Variable {
                    name: "a",
                    index_accessor: Some(Expr {
                        lhs: Box::new(Term {
                            kind: TermKind::Const(Constant::Integer(123)),
                            span: Span::new(2, 5),
                        }),
                        rhs: Some(Box::new((
                            BinOp::Plus,
                            Term {
                                kind: TermKind::Const(Constant::Integer(456)),
                                span: Span::new(8, 11),
                            },
                        ))),
                    }),
                    span: Span::new(0, 12),
                }),
                span: Span::new(0, 12),
            },
        );
        assert_parse(
            "some_fn(this, a, 1 + 2)",
            Term {
                kind: TermKind::FnCall(FnCall {
                    receiver: None,
                    fn_name: "some_fn",
                    args: vec![
                        Expr {
                            lhs: Box::new(Term {
                                kind: TermKind::Const(Constant::This),
                                span: Span::new(8, 12),
                            }),
                            rhs: None,
                        },
                        Expr {
                            lhs: Box::new(Term {
                                kind: TermKind::Variable(Variable {
                                    name: "a",
                                    index_accessor: None,
                                    span: Span::new(14, 15),
                                }),
                                span: Span::new(14, 15),
                            }),
                            rhs: None,
                        },
                        Expr {
                            lhs: Box::new(Term {
                                kind: TermKind::Const(Constant::Integer(1)),
                                span: Span::new(17, 18),
                            }),
                            rhs: Some(Box::new((
                                BinOp::Plus,
                                Term {
                                    kind: TermKind::Const(Constant::Integer(2)),
                                    span: Span::new(21, 22),
                                },
                            ))),
                        },
                    ],
                }),
                span: Span::new(0, 23),
            },
        );
    }

    #[test]
    fn parse_constant() {
        assert_parse("12345", Constant::Integer(12345));
        assert_parse(r##""example""##, Constant::Str("example"));
        assert_parse("true", Constant::True);
    }
}
