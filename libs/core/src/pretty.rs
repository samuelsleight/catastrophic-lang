use std::fmt::{self, Formatter, Write};

pub struct PrettyFormatter<'a, 'b> {
    indent: usize,
    fmt: &'a mut Formatter<'b>,
}

pub trait PrettyDebug {
    fn pretty_debug(&self, formatter: &mut PrettyFormatter) -> fmt::Result;
}

pub struct PrettyDebugger<'a, T>(pub &'a T);

impl<'a, 'b> PrettyFormatter<'a, 'b> {
    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn deindent(&mut self) {
        self.indent -= 1;
    }

    pub fn write_indent(&mut self) -> fmt::Result {
        for _ in 0..self.indent {
            write!(self.fmt, "    ")?;
        }

        Ok(())
    }
}

impl<'a, T: PrettyDebug> fmt::Display for PrettyDebugger<'a, T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut pretty_formatter = PrettyFormatter { indent: 0, fmt: formatter };

        self.0
            .pretty_debug(&mut pretty_formatter)
    }
}

impl<'a, 'b> Write for PrettyFormatter<'a, 'b> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.fmt.write_str(s)
    }
}

impl<T: PrettyDebug> PrettyDebug for Vec<T> {
    fn pretty_debug(&self, fmt: &mut PrettyFormatter) -> fmt::Result {
        writeln!(fmt, "[")?;
        fmt.indent();

        for item in self {
            T::pretty_debug(item, fmt)?
        }

        fmt.deindent();
        writeln!(fmt, "]")
    }
}
