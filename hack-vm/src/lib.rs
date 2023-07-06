#![forbid(unsafe_code)]
#![feature(let_chains)]

use std::{fs, path::Path};

pub mod insn;
pub mod parser;
pub mod segment;
pub mod symbol;
pub mod vm;

pub use insn::*;
pub use segment::*;
pub use vm::HackVm;

pub fn is_vm_file<P: AsRef<Path>>(p: P) -> bool {
    let path = p.as_ref();
    path.is_file()
        && path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("vm"))
}

pub fn read_vm_file<P: AsRef<Path>>(fname: P) -> anyhow::Result<String> {
    assert!(is_vm_file(fname.as_ref()));

    let content = fs::read_to_string(fname)?;
    Ok(content)
}

pub fn extract_vm_name(path: &Path) -> anyhow::Result<&str> {
    path.file_stem()
        .and_then(|name| name.to_str())
        .ok_or(anyhow::anyhow!("failed to parse"))
}
