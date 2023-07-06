//! Lexical analyzer of Jack language
use std::{iter::Peekable, str::CharIndices};

use crate::{
    error::{LexError, LexErrorKind},
    token::{Token, TokenKind},
};
use jack_ast::{BytePos, Span};

type Lexed<'s> = Option<Token<'s>>;
type LexResult<'s> = std::result::Result<Lexed<'s>, LexError<'s>>;

pub struct Lexer<'source> {
    input: &'source str,
    chars: Peekable<CharIndices<'source>>,
    bytepos: BytePos,
}

impl<'s> Lexer<'s> {
    pub fn new(input: &str) -> Lexer<'_> {
        Lexer {
            input,
            chars: input.char_indices().peekable(),
            bytepos: 0,
        }
    }

    fn next(&mut self) -> Option<(usize, char)> {
        let next = self.chars.next();
        if next.is_some() {
            self.bytepos += 1;
        }

        next
    }

    #[inline]
    fn peek(&mut self) -> Option<&(usize, char)> {
        self.chars.peek()
    }

    // - lex_xxx: if eat_xxx success, return lexed result [`Lexed<'s>`].
    // - eat_xxx: peek and if satisfy the condition, consume iter and return its offset.
    pub fn lex(&mut self) -> LexResult<'s> {
        while self.eat_ws()? { /* consume whitespaces and comments */ }

        if let Some(lexed) = self.lex_symbol() {
            return Ok(Some(lexed));
        }

        if let Some(lexed) = self.lex_str()? {
            return Ok(Some(lexed));
        }

        if let Some(lexed) = self.lex_integer()? {
            return Ok(Some(lexed));
        }

        if let Some(lexed) = self.lex_ident()? {
            return Ok(Some(lexed));
        }

        if let Some((base, unknown_char)) = self.next() {
            return self.error(LexErrorKind::UnexpectedCharacter(unknown_char), base, 1);
        }

        // EOF
        Ok(None)
    }

    fn error<T>(
        &self,
        kind: LexErrorKind<'s>,
        base: usize,
        offset: usize,
    ) -> Result<T, LexError<'s>> {
        Err(LexError {
            kind,
            span: Span::from_len(base, offset),
            src: self.input,
        })
    }

    fn lex_symbol(&mut self) -> Lexed<'s> {
        use TokenKind::*;

        let pos = self.bytepos;
        let Some((_, c)) = self.peek() else { return None };

        let kind = match c {
            '(' => LParen,
            ')' => RParen,
            '{' => LBrace,
            '}' => RBrace,
            '[' => LBracket,
            ']' => RBracket,
            ',' => Comma,
            ';' => Semicolon,
            '=' => Equal,
            '+' => Plus,
            '-' => Minus,
            '.' => Dot,
            '&' => And,
            '*' => Mul,
            '|' => Or,
            '/' => Slash,
            '~' => Tilde,
            '<' => Lt,
            '>' => Gt,
            _ => return None,
        };

        self.next();
        Some(Token {
            kind,
            span: Span::from_len(pos, 1),
        })
    }

    fn lex_ident(&mut self) -> LexResult<'s> {
        let pos = self.bytepos;
        // ident must not start with number
        let ident_first_char = |c: char| c.is_ascii_alphabetic() || c == '_';
        let Some(from) = self.eat_char_matches(ident_first_char) else { return Ok(None) };
        let until = self
            .eat_while(|c| c.is_ascii_alphanumeric() || c == '_')
            // actually idents do not continue till EOF; just avoid to crash by fuzzing.
            .unwrap_or(self.input.len() - 1);

        let ident_str = &self.input[from..=until];
        let kind = match ident_str.try_into() {
            Ok(kw) => TokenKind::Keyword(kw),
            _ => TokenKind::Ident(ident_str),
        };

        let span = Span::from_len(pos, ident_str.len());
        Ok(Some(Token { kind, span }))
    }

    fn lex_str(&mut self) -> LexResult<'s> {
        let pos = self.bytepos;

        let Some(from) = self.eat_char('"') else { return Ok(None) };
        let Some(until) = self.eat_while(|c| c != '"') else {
            return self.error(LexErrorKind::UnterminatedQuote, from, self.bytepos - 1);
        };

        // Consume the closing quote.
        // Since `eat_while` have consumed characters where c != (quote),
        // it's guaranteed that next char is `"`.
        self.eat_char('"').unwrap();
        let kind = TokenKind::Str(&self.input[from + 1..=until]);

        // including quotes
        let span = Span::new(pos, until + 2);
        Ok(Some(Token { kind, span }))
    }

    fn lex_integer(&mut self) -> LexResult<'s> {
        let pos = self.bytepos;

        let is_integer = |c: char| c.is_ascii_digit();
        let Some(from) = self.eat_char_matches(is_integer) else { return Ok(None) };
        let num_str = match self.eat_while(is_integer) {
            Some(until) => &self.input[from..=until],
            // EOF
            None => &self.input[from..],
        };

        let span = Span::from_len(pos, num_str.len());
        let kind = num_str
            .parse()
            .map(TokenKind::Integer)
            .map_err(|e| LexError {
                span,
                src: self.input,
                kind: LexErrorKind::InvalidNumberFormat(e),
            })?;

        Ok(Some(Token { kind, span }))
    }

    fn eat_ws(&mut self) -> Result<bool, LexError<'s>> {
        Ok(self.eat_char_matches(|c| c.is_whitespace()).is_some()
            || self.eat_line_comment()
            || self.eat_block_comment()?)
    }

    /// Consume the iterator while the given condition `cond` is satisfied.
    /// Once the condition is no longer satisfied, return the *last offset* that met the condition.
    /// If the iterator is empty, return `None`.
    fn eat_while<F: Fn(char) -> bool>(&mut self, cond: F) -> Option<usize> {
        let offset = loop {
            let Some((i, c)) = self.peek() else { return None };
            if !cond(*c) {
                break i - 1;
            } else {
                self.next();
            }
        };

        Some(offset)
    }

    // if `c` is `self.chars.peek()`, eat `c` and return its offset
    fn eat_char(&mut self, want: char) -> Option<usize> {
        match self.peek() {
            Some((offset, c)) if *c == want => {
                let offset = *offset;
                self.next();
                Some(offset)
            }
            _ => None,
        }
    }

    fn eat_char_matches<F: Fn(char) -> bool>(&mut self, matcher: F) -> Option<usize> {
        match self.peek() {
            Some((offset, c)) if matcher(*c) => {
                let offset = *offset;
                self.next();
                Some(offset)
            }
            _ => None,
        }
    }

    fn eat_str(&mut self, pat: &str) -> Option<usize> {
        let offset = self.bytepos;
        if self.input[offset..].starts_with(pat) {
            self.chars.nth(pat.len() - 1);
            self.bytepos += pat.len();
            Some(offset)
        } else {
            None
        }
    }

    fn eat_line_comment(&mut self) -> bool {
        if self.eat_str("//").is_none() {
            return false;
        }

        while let Some((_, c)) = self.next() {
            if c == '\n' {
                break;
            }
        }

        true
    }

    fn eat_block_comment(&mut self) -> Result<bool, LexError<'s>> {
        let Some(base) = self.eat_str("/*") else { return Ok(false); };

        loop {
            if self.eat_str("*/").is_some() {
                return Ok(true);
            }

            // increment cursor
            if self.next().is_none() {
                return self.error(LexErrorKind::UnterminatedComment, base, self.bytepos - 1);
            }
        }
    }
}

impl<'s> Iterator for Lexer<'s> {
    type Item = Result<Token<'s>, LexError<'s>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex().transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::KwKind;
    use TokenKind::*;

    fn lex(s: &str) -> Result<Vec<Token<'_>>, LexError<'_>> {
        Lexer::new(s).collect()
    }

    // compare tokens without its location
    macro_rules! assert_lex {
        ($input:expr, $expect:expr) => {
            let lexed = lex($input)
                .unwrap()
                .iter()
                .map(|t| t.kind.clone())
                .collect::<Vec<_>>();
            assert_eq!(lexed, $expect);
        };
    }

    #[test]
    fn lex_comment() {
        assert!(lex("/* block */").is_ok());
        assert!(lex("// line comment").is_ok());

        let input = r#"
        // line comment
        // line comment 2
        /* block comment */
        /* multi-line block comment
         */
        /** document comment */
        /**
         * multi-line document comment
         */
        "#;
        assert!(lex(input).is_ok());
    }

    macro_rules! assert_lex_error {
        // $errkind: LexErrorKind
        ($input:expr, $errkind:pat) => {
            match lex($input).unwrap_err().kind {
                $errkind => { /* ok */ }
                e => panic!("did not match to error kind {}", e),
            }
        };
    }

    #[test]
    fn lex_comment_err() {
        use LexErrorKind::*;

        assert_lex_error!("/* unterminated ", UnterminatedComment);
        assert_lex_error!("/** *", UnterminatedComment);
        assert_lex_error!("/* valid */ /** invalid *", UnterminatedComment);
        assert_lex_error!("/** /", UnterminatedComment);
    }

    #[test]
    fn lexed_tokens_span() {
        let input = r#"(
12345; "hello"
        "#;
        let lexed = lex(input).unwrap();
        #[rustfmt::skip]
        assert_eq!(
            lexed,
            vec![
                Token { kind: LParen,         span: Span { base: 0, len: 1 }},
                Token { kind: Integer(12345), span: Span { base: 2, len: 5 }},
                Token { kind: Semicolon,      span: Span { base: 7, len: 1 }},
                // including surrounding quotes
                Token { kind: Str("hello"),   span: Span { base: 9, len: 7 }},
            ]
        );
    }

    #[test]
    fn lex_symbol() {
        let input = r#"()[]{}"#;
        assert_lex!(
            input,
            vec![LParen, RParen, LBracket, RBracket, LBrace, RBrace]
        );
    }

    #[test]
    fn lex_string() {
        let input = r##"
"string"
"string2"
"##;
        assert_lex!(input, vec![Str("string"), Str("string2")]);
        // empty string
        assert_lex!(r##" "" "##, vec![Str("")]);
    }

    #[test]
    fn lex_number() {
        let input = r#"12345
67890"#;
        assert_lex!(input, vec![Integer(12345), Integer(67890)]);
    }

    #[test]
    fn lex_invalid_ident() {
        let input = r#"
class Main {}
// invalid ident (?) will appear */
?...
        "#;
        match lex(input).unwrap_err() {
            LexError {
                kind: LexErrorKind::UnexpectedCharacter(c),
                span,
                ..
                // error source points where unexpected character appeared
            } if c == '?' && input[span.base..].starts_with('?') => {}
            e => panic!("did not match to error condition {:?}", e),
        }
    }

    #[test]
    fn lex_hello() {
        use KwKind::*;

        let input = r#"
        /** Hello World program */
        class Main {
            function void main() {
                do Output.printString("Hello world");
                do Output.println();
                return;
            }
        }
        "#;
        assert_lex!(
            input,
            vec![
                Keyword(Class),
                Ident("Main"),
                LBrace,
                Keyword(Function),
                Keyword(Void),
                Ident("main"),
                LParen,
                RParen,
                LBrace,
                Keyword(Do),
                Ident("Output"),
                Dot,
                Ident("printString"),
                LParen,
                Str("Hello world"),
                RParen,
                Semicolon,
                Keyword(Do),
                Ident("Output"),
                Dot,
                Ident("println"),
                LParen,
                RParen,
                Semicolon,
                Keyword(Return),
                Semicolon,
                RBrace,
                RBrace,
            ]
        );
    }
}
