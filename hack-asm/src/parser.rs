//! Hack Assembly parser
use anyhow::anyhow;

use crate::{
    commands::{ACommand, Command, LCommand},
    symbol::SymbolTable,
};

const COMMENT: &str = "//";

pub type ParseResult<'source> = anyhow::Result<(Vec<Command<'source>>, Vec<anyhow::Error>)>;

pub fn parse(program: &str) -> ParseResult {
    let mut commands = Vec::new();
    let mut errors = Vec::new();

    for (row, line) in program.lines().enumerate() {
        let valid_code_range = line.find(COMMENT).unwrap_or(line.len());
        let asm = line[..valid_code_range].trim();
        if asm.is_empty() {
            continue;
        }

        match Command::parse(asm) {
            Ok(command) => commands.push(command),
            Err(e) => {
                let ctx = anyhow!("failed to parse line: {}", row + 1);
                errors.push(e.context(ctx));
            }
        }
    }

    Ok((commands, errors))
}

pub fn load_symbol<'s>(commands: &[Command<'s>], table: &mut SymbolTable<'s>) {
    // program counter
    let mut pc = 0;

    for command in commands {
        if let Command::L(LCommand(symbol)) = command {
            if !table.contains(symbol) {
                table.register_label(symbol, pc);
            }
        } else {
            pc += 1;
        }
    }

    for command in commands {
        if let Command::A(ACommand::Symbol(symbol)) = command {
            if table.address(symbol).is_none() {
                table.register_symbol(symbol);
            }
        }
    }
}
