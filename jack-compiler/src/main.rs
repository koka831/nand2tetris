use std::{
    env,
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::Result;
use jack_compiler::compiler;

fn help() -> ! {
    println!(
        "\
Usage: cargo run -p jack-compiler -- <jack file>
        "
    );
    exit(0);
}

fn collect_files<'s, P: AsRef<Path> + 's>(path: P) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let path = path.as_ref();

    if path.is_dir() {
        for entry in path.read_dir()?.filter_map(|p| p.map(|p| p.path()).ok()) {
            if entry.is_dir() {
                files.extend(collect_files(entry)?);
            } else {
                files.push(entry);
            }
        }
    } else {
        files.push(path.into());
    }

    Ok(files)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else { help() };
    let sources = collect_files(path)?;

    compiler::compile(sources);
    Ok(())
}
