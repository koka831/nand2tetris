use crate::{LexError, ParseError, SemanticError, SemanticErrorKind};
use miette::{LabeledSpan, MietteDiagnostic, Severity};

pub(crate) trait Report {
    fn report(&self) -> miette::Report;
}

impl<'s> Report for SemanticError<'s> {
    fn report(&self) -> miette::Report {
        let mut diag = MietteDiagnostic::new(self.to_string());
        match self.kind {
            SemanticErrorKind::AlreadyDefinedIdent { name, original } => {
                let label = Some(format!("`{name}` redefined here"));
                let hint = Some(format!("previous definition of `{name}` here"));
                diag = diag
                    .and_label(LabeledSpan::new_with_span(label, self.span))
                    .and_label(LabeledSpan::new_with_span(hint, original));
            }
            SemanticErrorKind::TypeMismatch { expected, actual } => {
                let label = Some(format!("expected `{expected}`, found `{actual}`"));
                // types in Jack language are not strict well, so set severity to warn instead of error
                diag = diag
                    .and_label(LabeledSpan::new_with_span(label, self.span))
                    .with_severity(Severity::Warning);
            }
            SemanticErrorKind::UnusedVariable(name) => {
                let label = Some(format!("variable `{name}` is defined here"));
                diag = diag
                    .and_label(LabeledSpan::new_with_span(label, self.span))
                    .with_severity(Severity::Warning);
            }
            _ => {
                let label = Some(self.to_string());
                diag = diag.and_label(LabeledSpan::new_with_span(label, self.span));
            }
        }

        miette::Report::new(diag).with_source_code(self.src.to_string())
    }
}

impl<'s> Report for ParseError<'s> {
    fn report(&self) -> miette::Report {
        let label = Some(self.to_string());
        let mut diag = MietteDiagnostic::new(self.to_string())
            .with_label(LabeledSpan::new_with_span(label, self.span));

        if let Some(help) = &self.help {
            diag = diag.with_help(help.to_string());
        }

        miette::Report::new(diag).with_source_code(self.src.to_string())
    }
}

impl<'s> Report for LexError<'s> {
    fn report(&self) -> miette::Report {
        let label = Some(self.to_string());
        let diag = MietteDiagnostic::new(self.to_string())
            .with_label(LabeledSpan::new_with_span(label, self.span));

        miette::Report::new(diag).with_source_code(self.src.to_string())
    }
}
