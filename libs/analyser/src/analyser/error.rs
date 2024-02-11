use catastrophic_core::{
    error::{context::ErrorProvider, writer::ErrorWriter},
    span::Span,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompileErrors {
    pub errors: Vec<CompileError>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompileError {
    UndefinedSymbolError(Span<String>),
}

impl From<Vec<CompileError>> for CompileErrors {
    fn from(errors: Vec<CompileError>) -> Self {
        Self { errors }
    }
}

impl ErrorProvider for CompileErrors {
    fn write_errors(&self, writer: &mut dyn ErrorWriter) -> std::fmt::Result {
        for error in &self.errors {
            match error {
                CompileError::UndefinedSymbolError(ref symbol) => {
                    writer.error(Some(symbol.swap(())), &format!("Use of undefined symbol `{}`", symbol.data))?;
                }
            }
        }

        Ok(())
    }
}
