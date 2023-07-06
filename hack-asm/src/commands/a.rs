//! Represents A(ddressing) command
use crate::symbol::Symbol;

#[derive(Debug, PartialEq, Eq)]
pub enum ACommand<'source> {
    Value(i16),
    Symbol(Symbol<'source>),
}

impl<'s> ACommand<'s> {
    /// code format: @{value,symbol}
    pub fn parse(code: &'s str) -> anyhow::Result<Self> {
        assert!(&code[0..1] == "@");
        let code = &code[1..];

        let command = if let Ok(v) = code.parse::<i16>() {
            assert!(v >= 0, "negative numbers are not supported");
            Self::Value(v)
        } else {
            Self::Symbol(code)
        };

        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_address() {
        let code = "@123";
        let command = ACommand::parse(code).unwrap();

        assert_eq!(command, ACommand::Value(123));
    }

    #[test]
    fn parse_symbol_address() {
        let code = "@some";
        let command = ACommand::parse(code).unwrap();

        assert_eq!(command, ACommand::Symbol("some"));
    }
}
