use std::fmt;

use crate::error::LexErrorKind;
use jack_ast::Span;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token<'s> {
    pub kind: TokenKind<'s>,
    pub span: Span,
}

impl<'s> Token<'s> {
    pub fn name(&self) -> String {
        use TokenKind::*;
        match self.kind {
            Keyword(ref kind) => kind.name().to_string(),
            Integer(v) => v.to_string(),
            Str(s) => s.to_string(),
            Ident(ident) => ident.to_string(),
            LParen => "(".into(),
            RParen => ")".into(),
            LBrace => "{".into(),
            RBrace => "}".into(),
            LBracket => "[".into(),
            RBracket => "]".into(),
            Comma => ",".into(),
            Semicolon => ";".into(),
            Equal => "=".into(),
            Plus => "+".into(),
            Minus => "-".into(),
            Dot => ".".into(),
            And => "&".into(),
            Mul => "*".into(),
            Or => "|".into(),
            Slash => "/".into(),
            Tilde => "~".into(),
            Lt => "<".into(),
            Gt => ">".into(),
        }
    }
}

impl<'s> fmt::Display for Token<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        let kind = match self.kind {
            Keyword(ref kind) => format!("Keyword({kind})"),
            Integer(v) => format!("Integer({v})"),
            Str(s) => format!("Str({s})"),
            Ident(ident) => format!("Ident({ident})"),
            LParen => "Lparen (".into(),
            RParen => "RParen )".into(),
            LBrace => "LBrace {".into(),
            RBrace => "RBrace }".into(),
            LBracket => "LBracket [".into(),
            RBracket => "RBracket ]".into(),
            Comma => "Comma ,".into(),
            Semicolon => "Semicolon ;".into(),
            Equal => "Equal =".into(),
            Plus => "Plus +".into(),
            Minus => "Minus -".into(),
            Dot => "Dot .".into(),
            And => "And &".into(),
            Mul => "Mul *".into(),
            Or => "Or |".into(),
            Slash => "Slash /".into(),
            Tilde => "Tilde ~".into(),
            Lt => "Lt <".into(),
            Gt => "Gt >".into(),
        };

        // consider adding Location info
        write!(f, "{kind}")
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenKind<'s> {
    Keyword(KwKind),
    Integer(u32),
    Str(&'s str),
    Ident(&'s str),
    /// (
    LParen,
    /// )
    RParen,
    /// {
    LBrace,
    /// }
    RBrace,
    /// [
    LBracket,
    /// ]
    RBracket,
    /// ,
    Comma,
    /// ;
    Semicolon,
    /// =
    Equal,
    /// +
    Plus,
    /// -
    Minus,
    /// .
    Dot,
    /// &
    And,
    /// *
    Mul,
    /// |
    Or,
    /// /
    Slash,
    /// ~
    Tilde,
    /// <
    Lt,
    /// >
    Gt,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum KwKind {
    Class,
    Ctor,
    Method,
    Function,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

impl KwKind {
    pub fn name(&self) -> &'static str {
        use KwKind::*;

        match self {
            Class => "class",
            Ctor => "constructor",
            Method => "method",
            Function => "function",
            Int => "int",
            Boolean => "boolean",
            Char => "char",
            Void => "void",
            Var => "var",
            Static => "static",
            Field => "field",
            Let => "let",
            Do => "do",
            If => "if",
            Else => "else",
            While => "while",
            Return => "return",
            True => "true",
            False => "false",
            Null => "null",
            This => "this",
        }
    }
}

impl<'s> TryFrom<&'s str> for KwKind {
    type Error = LexErrorKind<'s>;

    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        use KwKind::*;

        let kind = match s {
            "class" => Class,
            "constructor" => Ctor,
            "method" => Method,
            "function" => Function,
            "int" => Int,
            "boolean" => Boolean,
            "char" => Char,
            "void" => Void,
            "var" => Var,
            "static" => Static,
            "field" => Field,
            "let" => Let,
            "do" => Do,
            "if" => If,
            "else" => Else,
            "while" => While,
            "return" => Return,
            "true" => True,
            "false" => False,
            "null" => Null,
            "this" => This,
            _ => return Err(LexErrorKind::UndefinedKeyword(s)),
        };

        Ok(kind)
    }
}

impl fmt::Display for KwKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
