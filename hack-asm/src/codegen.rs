use anyhow::Context;
use rustc_hash::FxHashMap;
use std::sync::LazyLock;

use crate::{
    commands::{ACommand, Command},
    symbol::SymbolTable,
};

#[rustfmt::skip]
static OPCODE: LazyLock<FxHashMap<&'static str, i8>> = LazyLock::new(|| FxHashMap::from_iter([
    ("0",   0b0101010),
    ("1",   0b0111111),
    ("-1",  0b0111010),
    ("D",   0b0001100),
    ("A",   0b0110000),
    ("!D",  0b0001101),
    ("!A",  0b0110001),
    ("-D",  0b0001111),
    ("-A",  0b0110011),
    ("D+1", 0b0011111),
    ("A+1", 0b0110111),
    ("D-1", 0b0001110),
    ("A-1", 0b0110010),
    ("D+A", 0b0000010),
    ("A+D", 0b0000010),
    ("D-A", 0b0010011),
    ("A-D", 0b0000111),
    ("D&A", 0b0000000),
    ("A&D", 0b0000000),
    ("D|A", 0b0010101),
    ("A|D", 0b0010101),
    ("M",   0b1110000),
    ("!M",  0b1110001),
    ("-M",  0b1110011),
    ("M+1", 0b1110111),
    ("M-1", 0b1110010),
    ("D+M", 0b1000010),
    ("M+D", 0b1000010),
    ("D-M", 0b1010011),
    ("M-D", 0b1000111),
    ("D&M", 0b1000000),
    ("M&D", 0b1000000),
    ("D|M", 0b1010101),
    ("M|D", 0b1010101),
]));

pub fn generate<'s>(
    commands: &[Command<'s>],
    table: &SymbolTable<'s>,
) -> anyhow::Result<Vec<String>> {
    use Command::*;

    let mut mcode = Vec::new();
    for command in commands {
        match command {
            A(a) => {
                let address = match a {
                    ACommand::Value(v) => v,
                    ACommand::Symbol(s) => table.address(s).context("unknown symbol: {s}")?,
                };
                let code = format!("0{:015b}", address);
                mcode.push(code);
            }
            C(c) => {
                let code = format!(
                    "111{:07b}{:03b}{:03b}",
                    OPCODE[c.comp], c.dest as i16, c.jump as i16
                );

                mcode.push(code);
            }
            L(_) => { /* noop */ }
        }
    }

    Ok(mcode)
}
