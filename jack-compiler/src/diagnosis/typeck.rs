use std::borrow::Cow;

use crate::{
    symbol::{FnCtxt, SymbolTable},
    JackError,
};

use jack_ast::{Constant, Expr, Term, TermKind, Type, Variable};

pub fn validate_variable_ty(var: &Variable<'_>, ty: &Type<'_>, table: &SymbolTable) -> bool {
    let Some((ctxt, .. )) = table.lookup_variable(var.name) else {
        return false;
    };

    if var.index_accessor.is_some() && ctxt.ty == Type::Class("Array") {
        return true;
    }

    // type coercion:
    // - boolean <-> int
    // - boolean <- any
    // - int     <- char (codepoint)
    match (ctxt.ty, ty) {
        (Type::Boolean, _) => true,
        (Type::Int, Type::Boolean) => true,
        (Type::Int, Type::Char) => true,
        (Type::Char, Type::Class("String")) => true,
        _ => ctxt.ty == *ty,
    }
}

pub fn infer_expr_ty<'s>(
    expr: &Expr<'s>,
    table: &SymbolTable<'_, 's>,
) -> Result<Type<'s>, JackError<'s>> {
    infer_term_ty(&expr.lhs, table)
        .or(expr
            .rhs
            .as_ref()
            .and_then(|box (_, rhs)| infer_term_ty(rhs, table)))
        .ok_or(JackError::InternalCompilerError(Cow::Owned(format!(
            "could not infer the type of `{expr}`"
        ))))
}

fn infer_term_ty<'s>(term: &Term<'s>, table: &SymbolTable<'_, 's>) -> Option<Type<'s>> {
    use TermKind::*;
    match &term.kind {
        Const(c) => match c {
            Constant::Integer(_) => Some(Type::Int),
            Constant::Str(s) => {
                if s.len() == 1 {
                    Some(Type::Char)
                } else {
                    Some(Type::Class("String"))
                }
            }
            Constant::True | Constant::False => Some(Type::Boolean),
            Constant::This => table.current_class().ok().map(Type::Class),
            Constant::Null => None,
        },
        Variable(v) => match table.lookup_variable(v.name) {
            Some((var, ..)) => Some(var.ty),
            _ => None,
        },
        FnCall(f) => match table.lookup_fn(f.receiver, f.fn_name) {
            Some(FnCtxt { ty, .. }) => Some(*ty),
            _ => None,
        },
        Expr(expr) => infer_expr_ty(expr, table).ok(),
        Unary { term, .. } => infer_term_ty(term, table),
    }
}
