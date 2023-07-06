use jack_ast::{Class, Expr, FnCall, Ident, Span, Stmt, StmtKind, Term, TermKind};
use rustc_hash::FxHashMap;

use crate::{compiler::CompilerSession, JackError, SemanticError, SemanticErrorKind};

#[derive(Default)]
pub struct UnusedVariableVisitor<'s> {
    // ident, (is ident used, src, span)
    used: FxHashMap<Ident<'s>, (bool, &'s str, Span)>,
    // `used` for current scope. once check completed, current scope will be merged into `used`
    // and `current` will be reset.
    current: FxHashMap<Ident<'s>, (bool, &'s str, Span)>,
}

impl<'s> UnusedVariableVisitor<'s> {
    pub fn new() -> Self {
        UnusedVariableVisitor {
            used: FxHashMap::default(),
            current: FxHashMap::default(),
        }
    }

    pub fn check(&mut self, sess: &'s CompilerSession<'_, 's>) {
        let classes: Vec<(&str, &Class)> = sess
            .classes
            .iter()
            .map(|(_, src, class)| (*src, class))
            .collect();

        self.walk(&classes);
        self.emit(sess);
    }

    fn walk(&mut self, classes: &[(&'s str, &'s Class<'s>)]) {
        for (src, class) in classes.iter() {
            for local in class.variables.iter() {
                self.used.insert(local.name, (false, src, local.span));
            }

            for f in class.functions.iter() {
                // reset for each functions' scope
                self.used.extend(self.current.iter());
                self.current.clear();

                for param in f.params.iter() {
                    self.current.insert(param.name, (false, src, param.span));
                }

                for var in f.body.variables.iter() {
                    self.current.insert(var.name, (false, src, var.span));
                }

                for stmt in f.body.statements.iter() {
                    self.check_stmt(stmt);
                }
            }
        }
    }

    fn emit(&self, sess: &'s CompilerSession<'_, 's>) {
        for (ident, (used, src, span)) in self.used.iter() {
            if !used {
                let span = *span;
                let err = JackError::SemanticError(SemanticError {
                    kind: SemanticErrorKind::UnusedVariable(ident),
                    src,
                    span,
                });
                sess.reporter.report(&err);
            }
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt<'s>) {
        match &stmt.kind {
            StmtKind::Let { rhs, .. } => {
                self.check_expr(rhs);
            }
            StmtKind::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.check_expr(cond);
                for stmt in then_branch.iter() {
                    self.check_stmt(stmt);
                }

                if let Some(else_branch) = else_branch {
                    for stmt in else_branch.iter() {
                        self.check_stmt(stmt);
                    }
                }
            }
            StmtKind::While { cond, body } => {
                self.check_expr(cond);
                for stmt in body.iter() {
                    self.check_stmt(stmt);
                }
            }
            StmtKind::Do(f) => self.check_fncall(f),
            StmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    self.check_expr(expr);
                }
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr<'s>) {
        self.check_term(&expr.lhs);
        if let Some(box (_, term)) = &expr.rhs {
            self.check_term(term);
        }
    }

    fn check_term(&mut self, term: &Term<'s>) {
        match &term.kind {
            TermKind::Variable(v) => self.mark_used(v.name),
            TermKind::Expr(expr) => self.check_expr(expr),
            TermKind::Unary { term, .. } => self.check_term(term),
            TermKind::FnCall(f) => self.check_fncall(f),
            TermKind::Const(..) => {}
        }
    }

    fn check_fncall(&mut self, fncall: &FnCall<'s>) {
        if let Some(receiver) = fncall.receiver {
            self.mark_used(receiver);
        }

        for arg in fncall.args.iter() {
            self.check_expr(arg);
        }
    }

    fn mark_used(&mut self, ident: Ident<'s>) {
        self.current.entry(ident).and_modify(|state| state.0 = true);
        // HACK: assume that there're no variables with the same name in both
        // function-scoped and class-scoped.
        self.used.entry(ident).and_modify(|state| state.0 = true);
    }
}
