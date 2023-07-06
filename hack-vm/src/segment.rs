use std::fmt;

use anyhow::bail;

#[derive(Debug)]
pub enum Segment {
    // base address of `local` segment in the a function
    // RAM[1]
    Local,
    // base address of `argument` segment in the a function
    // RAM[2]
    Argument,
    Static,
    // no-mapping
    Constant,
    // RAM[3]
    This,
    // RAM[4]
    That,
    // RAM[3, 4]
    Pointer,
    // RAM[5 - 12]
    Temp,
}

impl TryFrom<&str> for Segment {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use Segment::*;

        let segment = match s {
            "local" => Local,
            "argument" => Argument,
            "static" => Static,
            "constant" => Constant,
            "this" => This,
            "that" => That,
            "pointer" => Pointer,
            "temp" => Temp,
            _ => bail!("Unknown segment: {s}"),
        };

        Ok(segment)
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Segment::*;

        let seg = match self {
            Local => "local",
            Argument => "argument",
            Static => "static",
            Constant => "constant",
            This => "this",
            That => "that",
            Pointer => "pointer",
            Temp => "temp",
        };

        write!(f, "{seg}")
    }
}
