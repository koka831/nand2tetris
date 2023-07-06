use anyhow::bail;

pub mod a;
pub mod c;
pub mod l;

pub use a::*;
pub use c::*;
pub use l::*;

/// Represents Hack Assembly instrument
#[derive(Debug, PartialEq, Eq)]
pub enum Command<'s> {
    A(ACommand<'s>),
    C(CCommand<'s>),
    L(LCommand<'s>),
}

impl<'s> Command<'s> {
    pub fn parse(code: &'s str) -> anyhow::Result<Self> {
        let Some(first) = code.chars().next() else { bail!("failed to parse: {code}") };

        let command = match first {
            '@' => Command::A(ACommand::parse(code)?),
            '(' => Command::L(LCommand::parse(code)?),
            _ => Command::C(CCommand::parse(code)?),
        };

        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error() {
        let code = "";
        let command = Command::parse(code);
        assert!(command.is_err());
    }

    #[test]
    fn test_parse_a_command() {
        let code = "@sym";
        let command = Command::parse(code).unwrap();
        assert!(matches!(command, Command::A(_)));
    }

    #[test]
    fn test_parse_value() {
        let code = "M;JMP";
        let command = Command::parse(code).unwrap();

        assert!(matches!(command, Command::C(_)));
    }
}
