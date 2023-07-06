pub mod report;
pub mod typeck;
pub mod unused_variable;

use crate::JackError;
use report::Report;

#[derive(Default)]
pub struct DiagnosticReporter;
impl DiagnosticReporter {
    pub fn new() -> Self {
        miette::set_hook(Box::new(|_| {
            Box::new(miette::MietteHandlerOpts::new().unicode(false).build())
        }))
        .unwrap_or_else(|ie| panic!("failed to setup miette: {ie}"));

        DiagnosticReporter
    }

    pub fn report(&self, e: &JackError<'_>) {
        let report = match e {
            JackError::SemanticError(e) => e.report(),
            JackError::ParseError(e) => e.report(),
            JackError::LexError(ref e) => e.report(),
            _ => return eprintln!("{e:?}"),
        };

        eprintln!("{report:?}");
    }
}
