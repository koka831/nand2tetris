use rustc_hash::FxHashMap;
use std::{borrow::Cow, fmt};

use crate::{JackError, SemanticError, SemanticErrorKind as ErrorKind};
use hack_vm::Segment;
use jack_ast::*;

pub type Result<'s, T> = std::result::Result<T, JackError<'s>>;

#[derive(Debug, PartialEq, Eq)]
pub struct FnCtxt<'s> {
    pub class: &'s str,
    pub name: &'s str,
    pub ty: Type<'s>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(Clone, Copy))]
pub struct VariableCtxt<'s> {
    pub name: Ident<'s>,
    pub ty: Type<'s>,
    pub kind: VarKind,
    pub span: Span,
}

impl<'s> VariableCtxt<'s> {
    pub fn field(name: Ident<'s>, ty: Type<'s>, span: Span) -> Self {
        VariableCtxt {
            name,
            ty,
            kind: VarKind::Field,
            span,
        }
    }

    pub fn stat(name: Ident<'s>, ty: Type<'s>, span: Span) -> Self {
        VariableCtxt {
            name,
            ty,
            kind: VarKind::Static,
            span,
        }
    }

    pub fn var(name: Ident<'s>, ty: Type<'s>, span: Span) -> Self {
        VariableCtxt {
            name,
            ty,
            kind: VarKind::Var,
            span,
        }
    }

    pub fn this(ty: Type<'s>, span: Span) -> Self {
        VariableCtxt {
            name: "this",
            ty,
            kind: VarKind::Arg,
            span,
        }
    }
}

impl<'s> From<&VariableDef<'s>> for VariableCtxt<'s> {
    fn from(var: &VariableDef<'s>) -> Self {
        use VarKind::*;
        VariableCtxt {
            name: var.name,
            ty: var.ty,
            span: var.span,
            kind: match var.kind {
                VariableDefKind::Var => Var,
                VariableDefKind::Static => Static,
                VariableDefKind::Field => Field,
            },
        }
    }
}

impl<'s> From<&Parameter<'s>> for VariableCtxt<'s> {
    fn from(param: &Parameter<'s>) -> Self {
        VariableCtxt {
            name: param.name,
            ty: param.ty,
            span: param.span,
            kind: VarKind::Arg,
        }
    }
}

#[derive(Default)]
pub struct SymbolTable<'ctx: 's, 's> {
    // (class_name, fn_name) -> IdentCtxt
    functions: FxHashMap<(Ident<'ctx>, Ident<'ctx>), FnCtxt<'ctx>>,
    // holds symbol table of current scope
    ctx: ScopedContext<'s>,
    // current class_name, source
    current: Option<(Ident<'s>, &'s str)>,
    // program flows such as `for`, `while` will be interpreted with labels in Hack VM.
    // `label_counter` holds number of times to generate unique labels.
    label_counter: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum VarKind {
    Var,
    Arg,
    Static,
    Field,
}
impl VarKind {
    pub fn segment(&self) -> Segment {
        use VarKind::*;
        match self {
            Var => Segment::Local,
            Arg => Segment::Argument,
            Static => Segment::Static,
            Field => Segment::This,
        }
    }
}

impl fmt::Display for VarKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use VarKind::*;
        let s = match self {
            Var => "var",
            Arg => "function argument",
            Static => "static",
            Field => "field",
        };
        write!(f, "{}", s)
    }
}

// Holds a context of the current scope
#[derive(Default)]
struct ScopedContext<'ctx> {
    // If the ctx is belonging to a function, it must have a parent context,
    // since global function definitions are prohibited in Jack language.
    // (in other words, a context for a class won't have a parent context.)
    parent: Option<Box<ScopedContext<'ctx>>>,
    current_fn: Option<&'ctx FnDef<'ctx>>,
    // `idents` holds pairs of appeared identifiers and its kind to assign
    // unique register numbers for each variables.
    idents: FxHashMap<Ident<'ctx>, (VariableCtxt<'ctx>, usize)>,
    // holds a number of assigned registers for each `VarKind` variables.
    register_counter: FxHashMap<VarKind, usize>,
}

impl<'ctx: 's, 's> SymbolTable<'ctx, 's> {
    pub fn new() -> Self {
        SymbolTable {
            functions: load_stl(),
            ctx: ScopedContext::new(),
            current: None,
            label_counter: 0,
        }
    }

    // starts new session
    pub fn sess(&mut self, class: Ident<'s>, source: &'s str) {
        self.current = Some((class, source));
        self.ctx = ScopedContext::new();
    }

    pub fn current_class(&self) -> Result<'s, Ident<'s>> {
        match self.current {
            Some((class, _)) => Ok(class),
            None => Err(JackError::InternalCompilerError(Cow::Borrowed(
                "SymbolTable: current class is not set",
            ))),
        }
    }

    pub fn current_fn(&self) -> Option<&FnDef<'s>> {
        self.ctx.current_fn
    }

    /// creates a new ScopedContext that is used while executing the given `f`.
    /// After the closure has finished, the context is restored to its previous state.
    /// the closure `f` expect to acquire an argument to avoid the ownership problem.
    pub fn scoped<F>(&mut self, current_fn: &'s FnDef<'s>, f: F) -> Result<'s, ()>
    where
        F: FnOnce(&mut SymbolTable<'_, 's>) -> Result<'s, ()>,
    {
        let current = std::mem::take(&mut self.ctx);
        self.ctx = ScopedContext {
            parent: Some(Box::new(current)),
            current_fn: Some(current_fn),
            ..Default::default()
        };
        f(self)?;

        // safety: assume that the parent of the created context must be present
        self.ctx = *self.ctx.parent.take().unwrap();

        Ok(())
    }

    pub fn lookup_variable<'a>(
        &'a self,
        ident: Ident<'s>,
    ) -> Option<&'a (VariableCtxt<'s>, usize)> {
        self.ctx
            .lookup_ident(ident)
            .or_else(|| self.ctx.parent.as_ref().and_then(|p| p.lookup_ident(ident)))
    }

    pub fn lookup_fn<'a>(
        &'a self,
        receiver: Option<Ident<'s>>,
        fn_name: Ident<'s>,
    ) -> Option<&'a FnCtxt<'s>> {
        let Some(receiver) = receiver.or(self.current_class().ok()) else {
            return None;
        };

        let class_name = if let Some((
            VariableCtxt {
                ty: Type::Class(name),
                ..
            },
            _,
        )) = self.lookup_variable(receiver)
        {
            name
        } else {
            receiver
        };

        self.functions.get(&(class_name, fn_name))
    }

    pub fn register_fn(
        &mut self,
        class: Ident<'ctx>,
        name: Ident<'ctx>,
        ty: Type<'ctx>,
        span: Span,
    ) -> Result<'s, ()> {
        if let Some(FnCtxt { span: original, .. }) = self.lookup_fn(Some(class), name) {
            let original = *original;
            return self.error(ErrorKind::AlreadyDefinedIdent { name, original }, span);
        }

        let ctxt = FnCtxt {
            class,
            name,
            ty,
            span,
        };
        self.functions.insert((class, name), ctxt);
        Ok(())
    }

    // register appeared variable and assign unique register number
    pub fn register_variable(&mut self, var: VariableCtxt<'s>) -> Result<'s, ()> {
        let kind = var.kind;
        // validates variable scope
        use VarKind::*;
        match (&self.ctx.parent, kind) {
            (None, Var | Arg) => {
                let msg = format!("cannot use {kind} to define class-scoped variables");
                self.error(ErrorKind::InvalidSyntax(Cow::Owned(msg)), var.span)
            }
            (Some(_), Static | Field) => {
                let msg = format!("cannot use {kind} to define function-scoped variables",);
                self.error(ErrorKind::InvalidSyntax(Cow::Owned(msg)), var.span)
            }
            _ => {
                let register = self.assign_register(kind);
                self.ctx.idents.insert(var.name, (var, register));
                Ok(())
            }
        }
    }

    fn assign_register(&mut self, kind: VarKind) -> usize {
        let register = *self
            .ctx
            .register_counter
            .entry(kind)
            .and_modify(|c| *c += 1)
            .or_insert(1);
        register - 1
    }

    #[cfg(test)]
    fn count(&mut self, kind: VarKind) -> usize {
        *self.ctx.register_counter.entry(kind).or_insert(0)
    }

    pub fn label(&mut self) -> String {
        self.label_counter += 1;
        format!("LABEL_{}", self.label_counter)
    }

    fn error(&self, kind: ErrorKind<'ctx>, span: Span) -> Result<'s, ()> {
        let Some((_, src)) = self.current else {
            let msg = Cow::Borrowed("current source is not set");
            return Err(JackError::InternalCompilerError(msg));
        };
        Err(JackError::SemanticError(SemanticError { src, span, kind }))
    }
}

impl<'ctx> ScopedContext<'ctx> {
    pub fn new() -> Self {
        ScopedContext {
            parent: None,
            current_fn: None,
            idents: FxHashMap::default(),
            register_counter: FxHashMap::default(),
        }
    }

    pub fn lookup_ident<'a>(
        &'a self,
        ident: Ident<'ctx>,
    ) -> Option<&'a (VariableCtxt<'ctx>, usize)> {
        self.idents.get(ident)
    }
}

macro_rules! define_stl {
    ( $( ($class:ident, $fn_name:ident, $ty:expr) ),* ) => {[
        $(
            (
                (stringify!($class), stringify!($fn_name)),
                FnCtxt {
                    class: stringify!($class),
                    name: stringify!($fn_name),
                    ty: $ty,
                    span: Span::new(0, 0)
                }
            )
        ),*
    ]}
}

#[rustfmt::skip]
fn load_stl<'ctx>() -> FxHashMap<(Ident<'ctx>, Ident<'ctx>), FnCtxt<'ctx>> {
    FxHashMap::from_iter(define_stl![
        // Array
        (Array, new,        Type::Class("Array")),
        (Array, dispose,    Type::Void),
        // Keyboard
        (Keyboard, init,        Type::Void),
        (Keyboard, keyPressed,  Type::Char),
        (Keyboard, readChar,    Type::Char),
        (Keyboard, readLine,    Type::Class("String")),
        (Keyboard, readInt,     Type::Int),
        // Math
        (Math, init,        Type::Void),
        (Math, abs,         Type::Int),
        (Math, multiply,    Type::Int),
        (Math, divide,      Type::Int),
        (Math, min,         Type::Int),
        (Math, max,         Type::Int),
        (Math, sqrt,        Type::Int),
        // Memory
        (Memory, init,      Type::Void),
        (Memory, peek,      Type::Int),
        (Memory, poke,      Type::Void),
        (Memory, alloc,     Type::Class("Array")),
        (Memory, deAlloc,   Type::Void),
        // Output
        (Output, init,          Type::Void),
        (Output, moveCursor,    Type::Void),
        (Output, printChar,     Type::Void),
        (Output, printString,   Type::Void),
        (Output, printInt,      Type::Void),
        (Output, println,       Type::Void),
        (Output, backSpace,     Type::Void),
        // Screen
        (Screen, init,          Type::Void),
        (Screen, clearScreen,   Type::Void),
        (Screen, setColor,      Type::Void),
        (Screen, drawPixel,     Type::Void),
        (Screen, drawLine,      Type::Void),
        (Screen, drawRectangle, Type::Void),
        (Screen, drawCircle,    Type::Void),
        // String
        (String, new,           Type::Class("String")),
        (String, length,        Type::Int),
        (String, charAt,        Type::Char),
        (String, setCharAt,     Type::Char),
        (String, appendChar,    Type::Class("String")),
        (String, eraseLastChar, Type::Void),
        (String, intValue,      Type::Int),
        (String, setInt,        Type::Void),
        // Sys
        (Sys, init,     Type::Void),
        (Sys, halt,     Type::Void),
        (Sys, error,    Type::Void),
        (Sys, wait,     Type::Void)
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fndef() -> FnDef<'static> {
        FnDef {
            name: "function",
            span: Span::new(0, 0),
            kind: FnKind::Method,
            ret: Type::Void,
            params: vec![],
            body: FnBody {
                variables: vec![],
                statements: vec![],
            },
        }
    }

    #[test]
    fn symbol_table_scoped() {
        let mut table = SymbolTable::default();
        let fndef = fndef();
        let some_field = VariableCtxt::field("some_field", Type::Int, Span::new(0, 10));
        table.register_variable(some_field).unwrap();
        table
            .scoped(&fndef, |t| {
                let some_var = VariableCtxt::var("some_var", Type::Char, Span::new(0, 8));
                t.register_variable(some_var).unwrap();
                assert_eq!(t.count(VarKind::Var), 1);
                Ok(())
            })
            .unwrap();

        table
            .scoped(&fndef, |t| {
                let some_var =
                    VariableCtxt::var("some_var_in_other_scope", Type::Char, Span::new(0, 23));

                t.register_variable(some_var).unwrap();
                assert_eq!(t.count(VarKind::Var), 1);
                Ok(())
            })
            .unwrap();

        assert_eq!(table.count(VarKind::Field), 1);
        // because it's out of the scope above,
        // there're no registered local variable.
        assert_eq!(table.count(VarKind::Var), 0);
    }

    #[test]
    fn symbol_table_lookup() {
        let fndef = fndef();
        let span = Span::new(0, 0);

        let mut table = SymbolTable::default();
        assert_eq!(table.lookup_variable("foo"), None);

        let foo = VariableCtxt::field("foo", Type::Char, span);
        table.register_variable(foo).unwrap();
        assert_eq!(table.lookup_variable("foo").unwrap(), &(foo, 0));
        table
            .scoped(&fndef, |t| {
                assert_eq!(t.lookup_variable("foo").unwrap(), &(foo, 0));
                let bar = VariableCtxt::var("bar", Type::Char, span);
                let piyo = VariableCtxt::var("piyo", Type::Char, span);
                t.register_variable(bar).unwrap();
                t.register_variable(piyo).unwrap();
                assert_eq!(t.lookup_variable("bar").unwrap(), &(bar, 0));
                assert_eq!(t.lookup_variable("piyo").unwrap(), &(piyo, 1));
                Ok(())
            })
            .unwrap();

        assert_eq!(table.lookup_variable("foo").unwrap(), &(foo, 0));

        assert_eq!(table.lookup_variable("bar"), None);
        assert_eq!(table.lookup_variable("piyo"), None);
    }
}
