use std::{
    env,
    io::BufWriter,
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::{bail, Context as _, Result};
use hack_vm::{extract_vm_name, is_vm_file, parser, read_vm_file, HackVm};

/// Collects file paths from the given path.
fn collect_file_path<'s, P: AsRef<Path> + 's>(path: P) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let path = path.as_ref();

    if path.is_dir() {
        for p in path.read_dir()?.filter_map(|p| p.map(|p| p.path()).ok()) {
            if is_vm_file(&p) {
                paths.push(p);
            }
        }
    } else if is_vm_file(path) {
        paths.push(path.to_path_buf());
    } else {
        bail!("Could not read {}", path.display());
    }

    Ok(paths)
}

fn help() -> ! {
    println!(
        "\
vm file name or directory is not given.

Usage: cargo run -p hack-vm -- <vm filename or directory>
"
    );
    exit(0);
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else { help(); };
    let file_paths = collect_file_path(path).context("could not retrieve given vm files")?;

    let mut sources = Vec::new();
    for path in &file_paths {
        let name = extract_vm_name(path)?;
        let vm = read_vm_file(path).unwrap_or_else(|e| {
            eprintln!("Could not load vm files properly. {e}");
            exit(1);
        });

        sources.push((name, vm));
    }

    let mut instructions = Vec::new();
    for (name, program) in &sources {
        let (is, errors) = parser::parse(program)?;
        if !errors.is_empty() {
            eprintln!("Failed to parse given program file : {name}");
            eprintln!("{:?}", errors);
            exit(1);
        }

        instructions.push((*name, is));
    }

    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let mut vm = HackVm::new(&mut writer);

    vm.interpret(instructions)?;

    Ok(())
}
