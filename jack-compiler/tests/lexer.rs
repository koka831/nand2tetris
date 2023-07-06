use std::io::{self, BufWriter};

use anyhow::bail;
use jack_compiler::{lexer::Lexer, token::TokenKind};

fn into_xml<W: io::Write>(writer: &mut W, src: &str) -> anyhow::Result<()> {
    let lexer = Lexer::new(src);
    let indent = " ".repeat(2);
    writeln!(writer, "<tokens>")?;
    for token in lexer {
        let Ok(token) = token else { bail!("lex error {:?}", token) };
        let tag = match token.kind {
            TokenKind::Keyword(_) => "keyword",
            TokenKind::Integer(_) => "integerConstant",
            TokenKind::Str(_) => "stringConstant",
            TokenKind::Ident(_) => "identifier",
            _ => "symbol",
        };

        let name = match token.kind {
            TokenKind::And => "&amp;".into(),
            TokenKind::Lt => "&lt;".into(),
            TokenKind::Gt => "&gt;".into(),
            _ => token.name(),
        };

        writeln!(writer, "{0}<{1}> {2} </{1}>", indent, tag, name)?;
    }
    writeln!(writer, "</tokens>")?;
    Ok(())
}

fn remove_ws(s: String) -> String {
    s.chars().filter(|c| !matches!(c, ' ' | '\r')).collect()
}

macro_rules! assert_lexed_xml {
    ($input:literal) => {
        let jack = include_str!(concat!($input, ".jack"));
        let expected = include_str!(concat!($input, "T.xml"));

        let mut writer = BufWriter::new(Vec::new());
        into_xml(&mut writer, jack).unwrap();
        let xml = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        similar_asserts::assert_eq!(remove_ws(xml), remove_ws(expected.to_string()));
    };
}

#[test]
fn lex_array_test_in_xml() {
    assert_lexed_xml!("./fixtures/ArrayTest/Main");
}

#[test]
fn lex_square_in_xml() {
    assert_lexed_xml!("./fixtures/Square/Main");
    assert_lexed_xml!("./fixtures/Square/Square");
    assert_lexed_xml!("./fixtures/Square/SquareGame");
}

#[test]
fn lex_expression_less_square() {
    assert_lexed_xml!("./fixtures/ExpressionLessSquare/Main");
    assert_lexed_xml!("./fixtures/ExpressionLessSquare/Square");
    assert_lexed_xml!("./fixtures/ExpressionLessSquare/SquareGame");
}
