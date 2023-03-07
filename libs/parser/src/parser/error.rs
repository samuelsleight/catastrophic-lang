use catastrophic_core::{
    error::{context::ErrorProvider, writer::ErrorWriter},
    span::Span,
};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar(Span<char>),
    BlockClosedWithoutOpening(Span<()>),
    BlockWithoutClosing(Span<()>),
    LabelWithoutName(Span<()>),
    LabelWithoutValue(Span<()>),
    ArrowWithoutArg(Span<()>),
    ArrowWithoutBlock(Span<()>),
    DuplicateSymbolError { first: Span<()>, duplicate: Span<()> },
}

impl ErrorProvider for ParseError {
    fn write_errors(&self, writer: &mut dyn ErrorWriter) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedChar(span) => writer.error(Some(span.swap(())), &format!("Encountered unexpected `{}`", span.data))?,
            ParseError::BlockClosedWithoutOpening(span) => writer.error(Some(*span), "Encountered `}` with no corresponding `{`")?,
            ParseError::BlockWithoutClosing(span) => writer.error(Some(*span), "Encountered `{` without corresponding `}`")?,
            ParseError::LabelWithoutName(span) => writer.error(Some(*span), "Encountered `:` without an accompanying symbol name")?,
            ParseError::LabelWithoutValue(span) => writer.error(Some(*span), "Encountered `:` without a corresponding symbol value")?,
            ParseError::ArrowWithoutArg(span) => writer.error(Some(*span), "Encountered `->` without a corresponding argument")?,
            ParseError::ArrowWithoutBlock(span) => writer.error(Some(*span), "Encountered `->` without a corresponding block")?,
            ParseError::DuplicateSymbolError { first, duplicate } => {
                writer.error(Some(*duplicate), "Encountered a duplicate symbol definition")?;
                writer.note(*first, "Symbol was previously defined here:")?;
            }
        }

        Ok(())
    }
}
