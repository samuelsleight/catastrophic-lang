use std::{
    fmt::{Formatter, Write},
    io::{Read, Seek},
};

use crate::span::Span;

use super::context::ErrorContext;

pub struct ErrorWriter<'ctx, 'fmt, 'a, R: Read + Seek> {
    context: &'ctx mut ErrorContext<R>,
    fmt: &'fmt mut Formatter<'a>,
}

impl<'ctx, 'fmt, 'a, R: Read + Seek> ErrorWriter<'ctx, 'fmt, 'a, R> {
    pub(crate) fn new(context: &'ctx mut ErrorContext<R>, fmt: &'fmt mut Formatter<'a>) -> Self {
        Self { context, fmt }
    }

    fn write_span(&mut self, span: Span<()>) -> std::fmt::Result {
        if span.start.line == span.end.line {
            write!(
                self.fmt,
                "\n\t> {}\n\t> ",
                self.context.line(span.start.line).map_err(|_| std::fmt::Error::default())?,
            )?;

            for _ in 0..span.start.col {
                self.fmt.write_char(' ')?;
            }

            for _ in span.start.col..span.end.col {
                self.fmt.write_char('^')?;
            }

            writeln!(self.fmt, "\n")?;
        }

        Ok(())
    }

    fn unspanned_error(&mut self, message: &str) -> std::fmt::Result {
        writeln!(self.fmt, "error: {}", message)
    }

    fn spanned_error(&mut self, span: Span<()>, message: &str) -> std::fmt::Result {
        writeln!(self.fmt, "error: {}:{}: {}", span.start.line, span.start.col, message)?;
        self.write_span(span)
    }

    pub fn error<S: Into<Option<Span<()>>>>(&mut self, span: S, message: &str) -> std::fmt::Result {
        match span.into() {
            Some(span) => self.spanned_error(span, message),
            None => self.unspanned_error(message),
        }
    }

    pub fn note(&mut self, span: Span<()>, message: &str) -> std::fmt::Result {
        writeln!(self.fmt, "note: {}:{}: {}", span.start.line, span.start.col, message)?;
        self.write_span(span)
    }
}
