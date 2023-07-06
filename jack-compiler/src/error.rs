use std::{borrow::Cow, io};

use jack_ast::{Span, Type};
use thiserror::Error;

use crate::token::{KwKind, Token};

#[derive(Error, Debug)]
pub enum JackError<'source> {
    #[error("Internal compiler error: {0}")]
    InternalCompilerError(Cow<'source, str>),

    #[error("could not read file: {0}")]
    IoError(#[from] io::Error),

    #[error(transparent)]
    SemanticError(SemanticError<'source>),

    #[error(transparent)]
    ParseError(Box<ParseError<'source>>),

    #[error(transparent)]
    LexError(LexError<'source>),
}

#[derive(Error, Debug)]
#[error("{kind}")]
pub struct SemanticError<'s> {
    pub kind: SemanticErrorKind<'s>,
    pub src: &'s str,
    pub span: Span,
}

#[derive(Error, Debug)]
pub enum SemanticErrorKind<'s> {
    #[error("undefined variable `{0}` found")]
    UndefinedVariable(&'s str),

    #[error("{0}")]
    InvalidSyntax(Cow<'s, str>),

    #[error("mismatched types")]
    TypeMismatch {
        expected: Type<'s>,
        actual: Type<'s>,
    },

    #[error("unused variable `{0}` found")]
    UnusedVariable(&'s str),

    #[error("`{name}` is defined multiple times")]
    AlreadyDefinedIdent { name: &'s str, original: Span },
}

#[derive(Error, Debug)]
#[error("{kind}")]
pub struct ParseError<'s> {
    pub kind: ParseErrorKind<'s>,
    pub span: Span,
    pub src: &'s str,
    pub help: Option<Cow<'s, str>>,
}

impl<'s> ParseError<'s> {
    pub fn new(kind: ParseErrorKind<'s>, src: &'s str, span: Span) -> Self {
        ParseError {
            kind,
            span,
            src,
            help: None,
        }
    }

    // maybe better to move into diagnostics...
    pub fn with_help(self, help: Cow<'s, str>) -> Self {
        ParseError {
            help: Some(help),
            ..self
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseErrorKind<'s> {
    #[error(transparent)]
    LexError(LexError<'s>),
    #[error("{0} is a reserved keyword")]
    ReservedKeyword(KwKind),
    #[error("unexpected ident found: {0}")]
    UnexpectedIdent(Cow<'s, str>),
    #[error("unexpected token {0} found")]
    UnexpectedToken(Token<'s>),
    #[error("unexpected end of file")]
    UnexpectedEOF,
    #[error("internal compiler error: {0}")]
    InternalCompilerError(Cow<'s, str>),
}

#[derive(Error, Debug, Clone)]
#[error("{kind}")]
pub struct LexError<'s> {
    pub src: &'s str,
    pub span: Span,
    pub kind: LexErrorKind<'s>,
}

#[derive(Error, Debug, Clone)]
pub enum LexErrorKind<'s> {
    #[error("unexpected character {0} found")]
    UnexpectedCharacter(char),
    #[error("cannot parse given number: {0}")]
    InvalidNumberFormat(#[from] std::num::ParseIntError),
    #[error("undefined keyword {0} found")]
    UndefinedKeyword(&'s str),
    #[error("could not find matching string quotation")]
    UnterminatedQuote,
    #[error("unterminated comment")]
    UnterminatedComment,
}

// `#[from]` attribute could not handle lifetime
impl<'s> From<ParseError<'s>> for JackError<'s> {
    fn from(value: ParseError<'s>) -> Self {
        match value.kind {
            ParseErrorKind::LexError(e) => JackError::LexError(e),
            _ => JackError::ParseError(Box::new(value)),
        }
    }
}

impl<'s> From<Box<ParseError<'s>>> for JackError<'s> {
    fn from(value: Box<ParseError<'s>>) -> Self {
        match value.kind {
            ParseErrorKind::LexError(e) => JackError::LexError(e),
            _ => JackError::ParseError(value),
        }
    }
}

impl<'s> From<LexError<'s>> for ParseError<'s> {
    fn from(e: LexError<'s>) -> Self {
        ParseError {
            span: e.span,
            src: e.src,
            kind: ParseErrorKind::LexError(e),
            help: None,
        }
    }
}
