//! Hack VM parser
use anyhow::anyhow;

use crate::insn::Instruction;

pub const COMMENT: &str = "//";

pub type ParseResult<'s> = anyhow::Result<(Vec<Instruction<'s>>, Vec<anyhow::Error>)>;

pub fn parse(program: &str) -> ParseResult {
    let mut instructions = Vec::new();
    let mut errors = Vec::new();

    for (row, line) in program.lines().enumerate() {
        let valid_code_range = line.find(COMMENT).unwrap_or(line.len());
        let vmcode = line[..valid_code_range].trim();
        if vmcode.is_empty() {
            continue;
        }

        match Instruction::parse(vmcode, row) {
            Ok(insn) => instructions.push(insn),
            Err(e) => {
                let ctx = anyhow!("failed to parse line {}", row + 1);
                errors.push(e.context(ctx));
            }
        }
    }
    Ok((instructions, errors))
}
