//! Jack language compiler
use std::{
    borrow::Cow,
    fmt, fs,
    io::{self, BufWriter},
    path::{Path, PathBuf},
    process,
};

use hack_vm::Segment;
use jack_ast::*;

use crate::{
    diagnosis::{typeck, unused_variable::UnusedVariableVisitor, DiagnosticReporter},
    is_jack_file, parser,
    symbol::{FnCtxt, SymbolTable, VariableCtxt},
    JackError, SemanticError, SemanticErrorKind,
};

type Result<'a, T> = std::result::Result<T, JackError<'a>>;

#[derive(Debug)]
struct Program {
    fname: PathBuf,
    source: String,
}

fn read_program<'a, P: AsRef<Path> + 'a>(path: &'a P) -> Result<'a, Program> {
    let fname = path.as_ref().with_extension("vm");
    let source = std::fs::read_to_string(path)?;

    Ok(Program { fname, source })
}

fn abort_on_failure<'a, T>(result: Result<'a, T>, sess: &CompilerSession<'_, 'a>) -> T {
    match result {
        Ok(r) => r,
        Err(e) => {
            sess.reporter.report(&e);
            process::exit(1);
        }
    }
}

#[derive(Default)]
pub struct CompilerSession<'sess: 's, 's> {
    pub classes: Vec<(&'sess PathBuf, &'sess str, Class<'sess>)>,
    pub table: SymbolTable<'sess, 's>,
    pub reporter: DiagnosticReporter,
    pub has_error: bool,
}
impl<'sess, 's> CompilerSession<'sess, 's> {
    pub fn new() -> Self {
        CompilerSession {
            classes: Vec::new(),
            table: SymbolTable::new(),
            reporter: DiagnosticReporter::new(),
            has_error: false,
        }
    }
}

pub fn compile<P: AsRef<Path>>(programs: Vec<P>) {
    let mut sess = CompilerSession::new();

    let programs = abort_on_failure(
        programs
            .iter()
            .filter(is_jack_file)
            .map(read_program)
            .collect::<Result<Vec<_>>>(),
        &sess,
    );

    if programs.is_empty() {
        eprintln!("Jack program is not given");
        process::exit(0);
    }

    for program in programs.iter() {
        let class = abort_on_failure(parser::parse(&program.source).map_err(|e| e.into()), &sess);
        sess.classes
            .push((&program.fname, program.source.as_str(), class));
    }

    // register global scope information
    for (_, src, class) in sess.classes.iter() {
        sess.table.sess(class.name, src);
        for f in class.functions.iter() {
            abort_on_failure(
                sess.table.register_fn(class.name, f.name, f.ret, f.span),
                &sess,
            );
        }
    }

    // lint
    UnusedVariableVisitor::new().check(&sess);

    // codegen
    process::exit(codegen(&mut sess).unwrap_or(false).into());
}

fn codegen<'s>(sess: &'s mut CompilerSession<'_, 's>) -> Result<'s, bool> {
    let mut unwrap = |result: Result<()>| {
        if result.is_err() {
            sess.has_error = true;
        }
    };

    for (fname, src, class) in sess.classes.iter() {
        sess.table.sess(class.name, src);
        let mut writer = BufWriter::new(fs::File::create(fname)?);
        let mut generator = Codegen::new(&mut writer, src, &sess.reporter);

        for var in &class.variables {
            unwrap(sess.table.register_variable(var.into()));
        }

        for f in &class.functions {
            sess.table.scoped(f, |table| {
                unwrap(generator.fn_def(class, f));

                if f.kind == FnKind::Method {
                    // padding an argument register for `this` receiver
                    // HACK: span for `this` does not exist actually
                    let ctxt = VariableCtxt::this(Type::Class(class.name), f.span);
                    unwrap(table.register_variable(ctxt));
                }

                for param in &f.params {
                    unwrap(table.register_variable(param.into()));
                }

                for var in &f.body.variables {
                    unwrap(table.register_variable(var.into()));
                }

                for stmt in &f.body.statements {
                    unwrap(generator.statement(stmt, table));
                }

                Ok(())
            })?;
        }
    }

    Ok(sess.has_error)
}

struct Codegen<'w, 's, W: io::Write> {
    writer: &'w mut W,
    src: &'s str,
    reporter: &'s DiagnosticReporter,
}
impl<'w, 's, W: io::Write> Codegen<'w, 's, W> {
    fn new(writer: &'w mut W, src: &'s str, reporter: &'s DiagnosticReporter) -> Self {
        Codegen {
            writer,
            src,
            reporter,
        }
    }

    fn write<T: fmt::Display>(&mut self, v: T) -> Result<'s, ()> {
        writeln!(self.writer, "{v}")?;
        Ok(())
    }

    #[inline]
    fn comment<T: fmt::Display>(&mut self, v: T) -> Result<'s, ()> {
        #[cfg(debug_assertions)]
        self.write(format!("// {v}"))?;

        Ok(())
    }

    #[inline]
    fn push<T: fmt::Display>(&mut self, segment: Segment, v: T) -> Result<'s, ()> {
        self.write(format!("push {segment} {v}"))
    }

    #[inline]
    fn pop<T: fmt::Display>(&mut self, segment: Segment, v: T) -> Result<'s, ()> {
        self.write(format!("pop {segment} {v}"))
    }

    fn fn_def(&mut self, class: &Class<'s>, def: &FnDef<'s>) -> Result<'s, ()> {
        let n_locals = def.body.variables.len();
        self.write(format!("function {}.{} {}", class.name, def.name, n_locals))?;
        match def.kind {
            FnKind::Ctor => {
                self.push(Segment::Constant, class.variables.len())?;
                self.write("call Memory.alloc 1")?;
                // set `this` regiter to the beginning of the allocated memory.
                self.pop(Segment::Pointer, 0)?;
            }
            FnKind::Method => {
                // function caller is required to push `this` as first argument
                self.push(Segment::Argument, 0)?;
                self.pop(Segment::Pointer, 0)?;
            }
            FnKind::Function => { /* function does not allocate memory nor handle `this` */ }
        }

        Ok(())
    }

    #[inline]
    fn ret(&mut self) -> Result<'s, ()> {
        self.write("return")
    }

    #[inline]
    fn label(&mut self, label: &str) -> Result<'s, ()> {
        self.write(format!("label {label}"))
    }

    #[inline]
    fn if_goto(&mut self, label: &str) -> Result<'s, ()> {
        self.write(format!("if-goto {label}"))
    }

    #[inline]
    fn goto(&mut self, label: &str) -> Result<'s, ()> {
        self.write(format!("goto {label}"))
    }

    fn error(&self, kind: SemanticErrorKind<'s>, span: Span) -> Result<'s, ()> {
        let err = JackError::SemanticError(SemanticError {
            src: self.src,
            kind,
            span,
        });

        self.reporter.report(&err);
        Err(err)
    }

    fn warning(&self, kind: SemanticErrorKind<'s>, span: Span) -> Result<'s, ()> {
        let err = JackError::SemanticError(SemanticError {
            src: self.src,
            kind,
            span,
        });

        self.reporter.report(&err);
        Ok(())
    }

    fn statement(&mut self, stmt: &Stmt<'s>, table: &mut SymbolTable<'_, 's>) -> Result<'s, ()> {
        self.comment(stmt)?;
        match &stmt.kind {
            StmtKind::Let { lhs, rhs } => {
                let Some((var, register)) = table.lookup_variable(lhs.name) else {
                    return self.error(
                        SemanticErrorKind::UndefinedVariable(lhs.name),
                        lhs.span
                    );
                };

                if !rhs.is_null() {
                    let rhs_ty = typeck::infer_expr_ty(rhs, table)?;
                    if !typeck::validate_variable_ty(lhs, &rhs_ty, table) {
                        self.warning(
                            SemanticErrorKind::TypeMismatch {
                                expected: var.ty,
                                actual: rhs_ty,
                            },
                            stmt.span,
                        )?;
                    }
                }
                self.expr(rhs, table)?;

                match &lhs.index_accessor {
                    Some(offset) => {
                        self.expr(offset, table)?;
                        self.push(var.kind.segment(), register)?;
                        self.write("add")?;
                        self.pop(Segment::Pointer, 1)?; // ptr <- *(lhs + offset)
                        self.pop(Segment::That, 0)?;
                    }
                    None => self.pop(var.kind.segment(), register)?,
                }
            }
            StmtKind::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let label_then = table.label();
                let label_fi = table.label();

                self.expr(cond, table)?;
                self.if_goto(&label_then)?;
                if let Some(else_branch) = else_branch {
                    for stmt in else_branch {
                        self.statement(stmt, table)?;
                    }
                }
                self.goto(&label_fi)?;

                self.label(&label_then)?;
                for stmt in then_branch {
                    self.statement(stmt, table)?;
                }

                self.label(&label_fi)?;
            }
            StmtKind::While { cond, body } => {
                let label_while = table.label();
                let label_quit = table.label();

                self.label(&label_while)?;
                self.expr(cond, table)?;
                // HACK: inverse condition to align signatures
                self.write("not")?;
                self.if_goto(&label_quit)?;
                for stmt in body {
                    self.statement(stmt, table)?;
                }
                self.goto(&label_while)?;
                self.label(&label_quit)?;
            }
            StmtKind::Do(f) => {
                let Some(FnCtxt { ty, .. }) = table.lookup_fn(f.receiver, f.fn_name) else {
                    return self.error(
                        SemanticErrorKind::InvalidSyntax(
                            Cow::Borrowed("called function is not defined")
                        ),
                        stmt.span
                    );
                };
                if *ty != Type::Void {
                    self.warning(
                        SemanticErrorKind::TypeMismatch {
                            expected: Type::Void,
                            actual: *ty,
                        },
                        stmt.span,
                    )?;
                }
                self.fncall(f, table)?;
                self.comment("discard return value of the void function")?;
                self.pop(Segment::Temp, 0)?
            }
            StmtKind::Return(retval) => {
                let Some(current_fn) = table.current_fn() else {
                    return self.error(
                        SemanticErrorKind::InvalidSyntax(
                            Cow::Borrowed("statements can't be declared outside functions")
                        ),
                        stmt.span
                    );
                };
                match (retval, current_fn.ret) {
                    (Some(expr), ty) => {
                        // HACK: some Jack program returns the pointer of `this` in its ctor by using the
                        // address of the first local variable. since the compiler does not support pointer
                        // types so we have to skip typeck in that case.
                        if current_fn.kind != FnKind::Ctor {
                            let retval_ty = typeck::infer_expr_ty(expr, table)?;
                            if retval_ty != ty {
                                self.warning(
                                    SemanticErrorKind::TypeMismatch {
                                        expected: ty,
                                        actual: retval_ty,
                                    },
                                    expr.span(),
                                )?;
                            }
                        }

                        self.expr(expr, table)?;
                        self.ret()?;
                    }
                    (None, Type::Void) => {
                        // return 0 as a return value for vm convention
                        self.push(Segment::Constant, 0)?;
                        self.ret()?;
                    }
                    (None, ty) => {
                        // we can't continue because it cannot cast void to anything else.
                        return self.error(
                            SemanticErrorKind::TypeMismatch {
                                expected: ty,
                                actual: Type::Void,
                            },
                            stmt.span,
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn fncall(&mut self, fn_call: &FnCall<'s>, table: &SymbolTable<'_, 's>) -> Result<'s, ()> {
        let mut args_len = fn_call.args.len();
        // if a called function is a member method, `FnCall` struct does not have its class specifier
        // (and it's not allowed to call a member method with `this` like `this.method`); so we
        // should treat a fn call as a method call when it does not have its class specifier.
        let Some(FnCtxt { class, .. }) = table.lookup_fn(fn_call.receiver, fn_call.fn_name) else {
            return self.error(
                SemanticErrorKind::UndefinedVariable(fn_call.fn_name),
                // TODO point span
                Span::new(0, 0)
            );
        };

        match fn_call.receiver {
            Some(receiver) => {
                if let Some((var, register)) = table.lookup_variable(receiver) {
                    match var.ty {
                        Type::Class(_) => {
                            // push `this` as an argument.
                            // `instance.method()` will be converted into `Class.method(instance)`.
                            self.push(var.kind.segment(), register)?;
                            args_len += 1;
                        }
                        _ => {
                            return self.error(
                                SemanticErrorKind::InvalidSyntax(Cow::Borrowed(
                                    "could not call methods of builtin types",
                                )),
                                var.span,
                            );
                        }
                    }
                }
            }
            None => {
                // assume `this` is omitted
                self.push(Segment::Pointer, 0)?;
                args_len += 1;
            }
        }

        for arg in &fn_call.args {
            self.expr(arg, table)?;
        }

        self.write(format!("call {class}.{} {}", fn_call.fn_name, args_len))
    }

    fn expr(&mut self, expr: &Expr<'s>, table: &SymbolTable<'_, 's>) -> Result<'s, ()> {
        self.term(&expr.lhs, table)?;
        if let Some(box (op, rhs)) = &expr.rhs {
            self.term(rhs, table)?;
            let operator = match op {
                BinOp::Plus => "add",
                BinOp::Minus => "sub",
                // call function manually
                BinOp::Mul => return self.write("call Math.multiply 2"),
                BinOp::Div => return self.write("call Math.divide 2"),
                BinOp::And => "and",
                BinOp::Or => "or",
                BinOp::Equal => "eq",
                BinOp::Lt => "lt",
                BinOp::Gt => "gt",
            };
            self.write(operator)?;
        }
        Ok(())
    }

    fn term(&mut self, term: &Term<'s>, table: &SymbolTable<'_, 's>) -> Result<'s, ()> {
        match &term.kind {
            TermKind::Const(c) => match c {
                Constant::This => self.push(Segment::Pointer, 0)?,
                Constant::Integer(n) => self.push(Segment::Constant, n)?,
                // `null` and `false` are treated as `0`
                Constant::Null | Constant::False => self.push(Segment::Constant, 0)?,
                // `true` will be interpreted to `-1`
                Constant::True => {
                    self.push(Segment::Constant, 1)?;
                    self.write("neg")?;
                }
                Constant::Str(s) => {
                    self.push(Segment::Constant, s.len())?;
                    self.write("call String.new 1")?;

                    for c in s.bytes() {
                        self.push(Segment::Constant, c)?;
                        self.write("call String.appendChar 2")?;
                    }
                }
            },
            TermKind::Variable(v) => self.variable(v, table)?,
            TermKind::FnCall(f) => self.fncall(f, table)?,
            TermKind::Expr(e) => self.expr(e, table)?,
            TermKind::Unary { op, term } => {
                self.term(term, table)?;
                let op = match op {
                    UnaryOp::Not => "not",
                    UnaryOp::Minus => "neg",
                };
                self.write(op)?;
            }
        }
        Ok(())
    }

    fn variable(&mut self, variable: &Variable<'s>, table: &SymbolTable<'_, 's>) -> Result<'s, ()> {
        match table.lookup_variable(variable.name) {
            Some((var, register)) => {
                match &variable.index_accessor {
                    Some(offset) => {
                        self.expr(offset, table)?;
                        self.push(var.kind.segment(), register)?;
                        self.write("add")?;
                        self.pop(Segment::Pointer, 1)?; // ptr = (var + offset)
                        self.push(Segment::That, 0) // push *(var + offset)
                    }
                    None => self.push(var.kind.segment(), register),
                }
            }
            _ => {
                return self.error(
                    SemanticErrorKind::UndefinedVariable(variable.name),
                    variable.span,
                );
            }
        }
    }
}
