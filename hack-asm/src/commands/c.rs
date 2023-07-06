/// Represents C(ompute) command
use anyhow::bail;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dest {
    Null = 0b000,
    M = 0b001,
    D = 0b010,
    MD = 0b011,
    A = 0b100,
    AM = 0b101,
    AD = 0b110,
    AMD = 0b111,
}

impl TryFrom<&str> for Dest {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use Dest::*;

        let dest = match s {
            "null" | "" => Null,
            "M" => M,
            "D" => D,
            "MD" => MD,
            "A" => A,
            "AM" => AM,
            "AD" => AD,
            "AMD" => AMD,
            _ => bail!("Unknown dest: {s}"),
        };

        Ok(dest)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Jump {
    Null = 0b000,
    JGT = 0b001,
    JEQ = 0b010,
    JGE = 0b011,
    JLT = 0b100,
    JNE = 0b101,
    JLE = 0b110,
    JMP = 0b111,
}

impl TryFrom<&str> for Jump {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use Jump::*;

        let jump = match s {
            "null" | "" => Null,
            "JGT" => JGT,
            "JEQ" => JEQ,
            "JGE" => JGE,
            "JLT" => JLT,
            "JNE" => JNE,
            "JLE" => JLE,
            "JMP" => JMP,
            _ => bail!("unknown jump: {s}"),
        };

        Ok(jump)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CCommand<'source> {
    pub dest: Dest,
    pub comp: &'source str,
    pub jump: Jump,
}

impl<'s> CCommand<'s> {
    /// code format: {dst=}cmp{;jmp}
    pub fn parse(code: &'s str) -> anyhow::Result<Self> {
        let equal_pos = code.find('=').map_or(0, |pos| pos + 1);
        let colon_pos = code.find(';').unwrap_or(code.len());

        let dest = if equal_pos != 0 {
            Dest::try_from(&code[..equal_pos - 1])?
        } else {
            Dest::Null
        };

        let comp = &code[equal_pos..colon_pos];
        let jump = if colon_pos != code.len() {
            Jump::try_from(&code[colon_pos + 1..])?
        } else {
            Jump::Null
        };

        let command = CCommand { dest, comp, jump };

        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_comp() {
        let code = "JLE";
        let command = CCommand::parse(code).unwrap();
        let expected = CCommand {
            dest: Dest::Null,
            comp: "JLE",
            jump: Jump::Null,
        };

        assert_eq!(command, expected);
    }

    #[test]
    fn parse_jmp() {
        let code = "M;JLE";
        let command = CCommand::parse(code).unwrap();
        let expected = CCommand {
            dest: Dest::Null,
            comp: "M",
            jump: Jump::JLE,
        };

        assert_eq!(command, expected);
    }

    #[test]
    fn parse_c_command() {
        let code = "M=D+M;JMP";
        let command = CCommand::parse(code).unwrap();
        let expected = CCommand {
            dest: Dest::M,
            comp: "D+M",
            jump: Jump::JMP,
        };

        assert_eq!(command, expected);
    }

    #[test]
    fn parse_unknown_jmp() {
        let code = "M;UNK";
        let command = CCommand::parse(code);
        assert!(command.is_err());
    }
}
