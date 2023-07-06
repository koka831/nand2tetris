//! VM Environment implementation for Hack platform
use std::io;

use anyhow::bail;

use crate::{segment::Segment, symbol::SymbolTable, InsnKind, Instruction};

struct VmContext<'s> {
    pub symbol_table: SymbolTable<'s>,
    pub current_file_name: Option<&'s str>,
    pub current_fn_name: Option<&'s str>,
}

pub struct HackVm<'s, W: io::Write> {
    w: &'s mut W,
    ctx: VmContext<'s>,
}

impl<'s, W: io::Write> HackVm<'s, W> {
    pub fn new(writer: &'s mut W) -> Self {
        let ctx = VmContext {
            symbol_table: SymbolTable::new(),
            current_file_name: None,
            current_fn_name: None,
        };

        HackVm { w: writer, ctx }
    }

    /// Interpret given `instructions` into HackAsm.
    pub fn interpret(
        &mut self,
        instructions: Vec<(&'s str, Vec<Instruction<'s>>)>,
    ) -> anyhow::Result<()> {
        use InsnKind::*;

        // If Sys.init function defined, include bootstrap procedure.
        let sys_init_defined = instructions.iter().any(|(_, is)| {
            is.iter().any(|i| match i.kind {
                DefFn(name, ..) => name == "Sys.init",
                _ => false,
            })
        });

        if sys_init_defined {
            #[cfg(debug_assertions)]
            writeln!(self.w, "// inject bootstrap")?;

            // Init stack pointer to 0x100 (=256), and call `Sys.init` in the beginning
            writeln!(self.w, "@256\nD = A\n@SP\nM = D\n")?;
            self.call_fn("Sys.init", 0)?;
        }

        for (name, is) in instructions {
            self.ctx.current_file_name = Some(name);

            for insn in is {
                // include the original representation in the comment
                #[cfg(debug_assertions)]
                writeln!(self.w, "// {}", insn.lexeme)?;

                match insn.kind {
                    // arithmetic
                    Add => self.add()?,
                    Sub => self.sub()?,
                    And => self.and()?,
                    Or => self.or()?,
                    Neg => self.neg()?,
                    Not => self.not()?,
                    Eq => self.eq()?,
                    Gt => self.gt()?,
                    Lt => self.lt()?,
                    // stack
                    Push(ref segment, index) => self.push(segment, index)?,
                    Pop(ref segment, index) => self.pop(segment, index)?,
                    // program flow
                    Label(label) => writeln!(self.w, "({})", self.label(label))?,
                    Goto(label) => self.goto(&self.label(label))?,
                    IfGoto(label) => self.if_goto(&self.label(label))?,
                    // function
                    DefFn(name, n_params) => self.define_fn(name, n_params)?,
                    CallFn(name, n_args) => self.call_fn(name, n_args)?,
                    Return => self.ret_fn()?,
                };
            }
        }

        self.w.flush()?;

        Ok(())
    }

    /// Construct symbole of static variable from the vm name and given index
    pub fn static_variable(&self, index: u16) -> anyhow::Result<String> {
        let Some(file_name) = self.ctx.current_file_name else { bail!("current file name is not set"); };
        let symbol = format!("{file_name}.{index}");
        Ok(symbol)
    }

    pub fn label(&self, label: &str) -> String {
        let fn_name = match self.ctx.current_fn_name {
            Some(fn_name) => format!("{fn_name}$"),
            None => String::new(),
        };

        format!("{fn_name}{label}")
    }

    /// Push the given value to the top of the stack
    pub fn push(&mut self, segment: &Segment, index: u16) -> anyhow::Result<()> {
        use Segment::*;

        match segment {
            // constant(immediate value) will be loaded to the A
            Constant => writeln!(self.w, "@{}\nD = A\n{PUSH}", index)?,
            Static => writeln!(self.w, "@{}\nD = M\n{PUSH}", self.static_variable(index)?)?,
            _ => {
                let src = self.address(segment, index)?;
                writeln!(self.w, "{src}\nD = M\n{PUSH}")?;
            }
        };
        Ok(())
    }

    /// Resolve the address of the given segment by the index and assign the
    /// result to the A register.
    fn address(&self, segment: &Segment, index: u16) -> anyhow::Result<String> {
        use Segment::*;

        let align_offset = |s: &str, offset: u16| {
            let increment = "A = A + 1\n".repeat(offset as usize);
            format!("@{s}\nA = M\n{increment}")
        };

        let dest = match segment {
            Local => align_offset("LCL", index),
            Argument => align_offset("ARG", index),
            Static => format!("@{}", self.static_variable(index)?),
            This => align_offset("THIS", index),
            That => align_offset("THAT", index),
            Pointer if index == 0 => "@THIS".into(),
            Pointer => "@THAT".into(),
            Temp => format!("@R{}", index + 5),
            Constant => bail!("Cannot pop value into the constant segment"),
        };

        Ok(dest)
    }

    /// Pop value in the top of the stack and store into the given segment[index]
    pub fn pop(&mut self, segment: &Segment, index: u16) -> anyhow::Result<()> {
        let dest = self.address(segment, index)?;
        writeln!(self.w, "{POP}\n{dest}\nM = D")?;
        Ok(())
    }

    /// Define the function with given name.
    /// - define function label
    /// - initialize segment for local variable with `0` (intent to be undefined)
    pub fn define_fn(&mut self, name: &'s str, n_params: u16) -> anyhow::Result<()> {
        self.ctx.current_fn_name = Some(name);

        let init_local_segment = format!("D = 0\n{}", PUSH.repeat(n_params as usize));
        writeln!(self.w, "({name})\n{init_local_segment}")?;
        Ok(())
    }

    /// `call_fn` prepares function call and jump to the function label.
    /// when `call_fn` completes, the memory alignment will be like:
    ///
    /// +------------------+
    /// |     ...          |
    /// |     ARG_0        |
    /// |     ARG_1        |
    /// |     ...          |
    /// |     ARG_n-1      |
    /// |------------------|
    /// |  return address  |
    /// |------------------|
    /// |  preserved LCL   |
    /// |  .. ARG ..       |
    /// |  .. THIS ..      |
    /// |  .. THAT ..      |
    /// |------------------|
    /// |     LCL_0        |
    /// |     LCL_1        |
    /// |     ...          |
    /// |     LCL_k-1      |
    /// |------------------|
    /// |                  |  <-- @SP
    /// +------------------+
    pub fn call_fn(&mut self, name: &'s str, n_args: u16) -> anyhow::Result<()> {
        // push the value of the given `label` address to the stack
        let push_label_addr = |label: &str| format!("@{label}\nD = M\n{PUSH}");
        let ret = self.ctx.symbol_table.ret_addr("return-address");

        // ARG = SP - n_args - 5
        let caller_save_args = format!(
            "\
@{}
D = A
@SP
D = M - D
@ARG
M = D
",
            n_args + 5
        );

        writeln!(self.w, "@{ret}\nD = A\n{PUSH}")?;
        writeln!(
            self.w,
            "{}\n{}\n{}\n{}\n{}",
            push_label_addr("LCL"),
            push_label_addr("ARG"),
            push_label_addr("THIS"),
            push_label_addr("THAT"),
            caller_save_args,
        )?;

        // LCL = SP
        writeln!(self.w, "@SP\nD = M\n@LCL\nM = D")?;

        self.goto(name)?;
        writeln!(self.w, "({ret})")?;

        Ok(())
    }

    /// Generates `return` statement
    #[inline]
    pub fn ret_fn(&mut self) -> anyhow::Result<()> {
        writeln!(self.w, "{RET}")?;
        Ok(())
    }

    #[inline]
    pub fn eq(&mut self) -> anyhow::Result<()> {
        self.compare("JEQ")?;
        Ok(())
    }

    #[inline]
    pub fn gt(&mut self) -> anyhow::Result<()> {
        self.compare("JGT")?;
        Ok(())
    }

    #[inline]
    pub fn lt(&mut self) -> anyhow::Result<()> {
        self.compare("JLT")?;
        Ok(())
    }

    #[inline]
    pub fn add(&mut self) -> anyhow::Result<()> {
        self.binary_calc("+")?;
        Ok(())
    }

    #[inline]
    pub fn sub(&mut self) -> anyhow::Result<()> {
        self.binary_calc("-")?;
        Ok(())
    }

    #[inline]
    pub fn and(&mut self) -> anyhow::Result<()> {
        self.binary_calc("&")?;
        Ok(())
    }

    #[inline]
    pub fn or(&mut self) -> anyhow::Result<()> {
        self.binary_calc("|")?;
        Ok(())
    }

    #[inline]
    pub fn neg(&mut self) -> anyhow::Result<()> {
        self.unary_calc("-")?;
        Ok(())
    }

    #[inline]
    pub fn not(&mut self) -> anyhow::Result<()> {
        self.unary_calc("!")?;
        Ok(())
    }

    #[inline]
    pub fn goto(&mut self, label: &str) -> anyhow::Result<()> {
        writeln!(self.w, "@{label}\n1; JNE")?;

        Ok(())
    }

    /// Pop value on top of the stack, and if `value != 0` jump to the given `label`.
    pub fn if_goto(&mut self, label: &str) -> anyhow::Result<()> {
        writeln!(
            self.w,
            "\
{POP}
@{label}
D; JNE
"
        )?;

        Ok(())
    }

    /// Apply the given operand to the top two values on the stack and push the
    /// result back onto the stack.
    /// Actually it will pop only the top value and replace the second value with
    /// the result instead of pop both of them and push the result.
    fn binary_calc(&mut self, operand: &str) -> anyhow::Result<()> {
        writeln!(
            self.w,
            "\
{POP}
@SP
A = M - 1
M = M {operand} D
"
        )?;

        Ok(())
    }

    /// Apply the given operand to the value on the top of the stack
    fn unary_calc(&mut self, operand: &str) -> anyhow::Result<()> {
        writeln!(
            self.w,
            "\
@SP
A = M - 1
M = {operand}M
"
        )?;

        Ok(())
    }

    /// Compare the top values using the `operation` and push the result back.
    /// If the result is `true`, the value will be `-1`, otherwise it will be `0`.
    fn compare(&mut self, operation: &'s str) -> anyhow::Result<()> {
        let ret_label = self.ctx.symbol_table.ret_addr(operation);
        writeln!(
            self.w,
            "\
{POP}
@SP
A = M - 1
D = M - D
M = -1
@{ret_label}
D; {operation}
@SP
A = M - 1
M = 0
({ret_label})"
        )?;
        Ok(())
    }
}

/// Push value in the D register to the top of the stack.
/// - Load the address of the stack pointer @SP into the A register
/// - Set the top of the stack to the value of the D
/// - Increment @SP
#[doc(hidden)]
static PUSH: &str = "\
@SP
A = M
M = D
@SP
M = M + 1
";

/// Pop value in the top of the stack and store them in the D register.
/// - Decrement @SP and load the address of the new @SP into the A (and M)
/// - Store the value at the top of the stack in the D
#[doc(hidden)]
static POP: &str = "\
@SP
AM = M - 1
D = M
";

/// Generate return procedure from the current function.
/// use @R13 for @FRAME, @R14 for @RET and @R15 for return value.
#[doc(hidden)]
static RET: &str = "\
// FRAME(= @R13) = LCL
@LCL
D = M
@R13
M = D

// FRAME - 5
@5
A = D - A
D = M
// RET = *(FRAME - 5)
@R14
M = D

@SP
A = M - 1
D = M
@R15
M = D

@ARG
D = M + 1
@SP
M = D

// SP - 1 = RET
@R15
D = M
@SP
A = M - 1
M = D

// THAT = *(FRAME - 1)
@R13
A = M - 1
D = M
@THAT
M = D

// THIS = *(FRAME - 2)
@2
D = A
@R13
A = M - D
D = M
@THIS
M = D

// ARG = *(FRAME - 3)
@3
D = A
@R13
A = M - D
D = M
@ARG
M = D

// LCL = *(FRAME - 4)
@4
D = A
@R13
A = M - D
D = M
@LCL
M = D

// goto RET
@R14
A = M
1;JNE
";
