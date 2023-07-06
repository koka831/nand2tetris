/// Represents L(abel) command
use anyhow::Context as _;

use crate::symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LCommand<'source>(pub Symbol<'source>);

impl<'s> LCommand<'s> {
    /// code format: (Xxx)
    pub fn parse(code: &'s str) -> anyhow::Result<Self> {
        let left = code.find('(').context("could not find left brace")?;
        let right = code.find(')').context("could not find right brace")?;
        let command = LCommand(&code[left + 1..right]);
        Ok(command)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_label_command() {
        let code = "(Xxx)";
        let command = LCommand::parse(code).unwrap();
        let expected = LCommand("Xxx");

        assert_eq!(command, expected);
    }
}
