use anyhow::{anyhow, bail};

use crate::segment::Segment;

pub type Symbol<'a> = &'a str;

#[derive(Debug)]
pub struct Instruction<'source> {
    pub kind: InsnKind<'source>,
    pub line: usize,
    // hold original code
    #[cfg(debug_assertions)]
    pub lexeme: &'source str,
}

#[derive(Debug)]
pub enum InsnKind<'source> {
    // Arithmetic
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    // Stack
    Push(Segment, u16),
    Pop(Segment, u16),
    // Program Flow
    Label(Symbol<'source>),
    Goto(Symbol<'source>),
    IfGoto(Symbol<'source>),
    // Function call
    // Function definition(name, num of local variables)
    DefFn(Symbol<'source>, u16),
    // Function call(name, num of arguments)
    CallFn(Symbol<'source>, u16),
    Return,
}

impl<'s> Instruction<'s> {
    pub fn parse(code: &'s str, line: usize) -> anyhow::Result<Self> {
        use InsnKind::*;

        let mut ops = code.split_whitespace();
        let kind = ops
            .next()
            .ok_or(anyhow!("expect vm instruction, nothing found"))?;
        let kind = match kind {
            // Arithmetic
            "add" => Add,
            "sub" => Sub,
            "neg" => Neg,
            "eq" => Eq,
            "gt" => Gt,
            "lt" => Lt,
            "and" => And,
            "or" => Or,
            "not" => Not,
            // Stack
            "push" | "pop" => {
                let segment = ops
                    .next()
                    .ok_or(anyhow!("expect target segment to push or pop"))?
                    .try_into()?;
                let index = ops
                    .next()
                    .ok_or(anyhow!("expect index of the segment"))?
                    .parse()?;

                match kind {
                    "push" => Push(segment, index),
                    "pop" => Pop(segment, index),
                    _ => unreachable!(),
                }
            }
            "label" | "goto" | "if-goto" => {
                let symbol = ops.next().ok_or(anyhow!("expect label symbol"))?;
                if !validate(symbol) {
                    bail!("invalid label symbol: {symbol}");
                }

                match kind {
                    "label" => Label(symbol),
                    "goto" => Goto(symbol),
                    "if-goto" => IfGoto(symbol),
                    _ => unreachable!(),
                }
            }
            "function" => {
                let name = ops.next().ok_or(anyhow!("expect function name"))?;
                if !validate(name) {
                    bail!("invalid function name: {name}");
                }

                let n_locals = ops
                    .next()
                    .ok_or(anyhow!("expect num of local variables"))?
                    .parse()?;

                DefFn(name, n_locals)
            }
            "call" => {
                let name = ops.next().ok_or(anyhow!("expect function name"))?;
                let n_args = ops
                    .next()
                    .ok_or(anyhow!("expect num of arguments"))?
                    .parse()?;

                CallFn(name, n_args)
            }
            "return" => Return,
            _ => bail!("unknown vm instruction: {kind}"),
        };

        Ok(Instruction {
            kind,
            line,
            #[cfg(debug_assertions)]
            lexeme: code,
        })
    }
}

fn validate(symbol: &str) -> bool {
    !symbol.starts_with(|c: char| c.is_ascii_digit())
        && symbol
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | ':'))
}
