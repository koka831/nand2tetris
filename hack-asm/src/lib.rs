#![forbid(unsafe_code)]
#![feature(let_chains)]
#![feature(lazy_cell)]

use std::{fs, path::Path};

use symbol::SymbolTable;

pub mod codegen;
pub mod commands;
pub mod parser;
pub mod symbol;

pub fn read_and_format<P: AsRef<Path>>(fname: P) -> anyhow::Result<String> {
    let program = fs::read_to_string(fname)?
        .chars()
        .filter(|c| *c != ' ')
        .collect();

    Ok(program)
}

pub fn compile(program: &str) -> anyhow::Result<Vec<String>> {
    let mut table = SymbolTable::new();
    let (commands, errors) = parser::parse(program)?;
    parser::load_symbol(&commands, &mut table);

    if !errors.is_empty() {
        for error in errors {
            eprintln!("{}\n\tCaused by {}", error, error.root_cause());
        }
    }

    codegen::generate(&commands, &table)
}
