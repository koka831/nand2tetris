use std::env;

use anyhow::{Context as _, Result};

use hack_asm::{compile, read_and_format};

/// Usage: `cargo run -- sample.asm > sample.hack`
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let fname = args.get(1).context("asm file is not given")?;

    let program = read_and_format(fname)?;

    let binary = compile(&program)?;
    for line in binary {
        println!("{line}");
    }

    Ok(())
}
