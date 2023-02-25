use std::fmt::Write;

use catastrophic_core::pretty::{PrettyDebug, PrettyFormatter};

use crate::mir::{Block, Command, Function, Instr, Value};

impl PrettyDebug for Block {
    fn pretty_debug(&self, fmt: &mut PrettyFormatter) -> std::fmt::Result {
        fmt.write_indent()?;
        writeln!(fmt, "{{")?;
        fmt.indent();

        for instr in &self.instrs {
            fmt.write_indent()?;

            match &instr.data {
                Instr::Command(command) => match command {
                    Command::Call => writeln!(fmt, "Call")?,
                    Command::OutputChar => writeln!(fmt, "OutputChar")?,
                    Command::OutputNumber => writeln!(fmt, "OutputNumber")?,
                    Command::InputChar => writeln!(fmt, "InputChar")?,
                    Command::InputNumber => writeln!(fmt, "InputNumber")?,
                },
                Instr::Push(value) => {
                    write!(fmt, "Push[")?;
                    write_value(fmt, value)?;
                    writeln!(fmt, "]")?;
                }
                Instr::ImmediateCall(function) => {
                    write!(fmt, "Call[")?;
                    write_function(fmt, function)?;
                    writeln!(fmt, "]")?;
                }
                Instr::ImmediateConditionalCall(value, x, y) => {
                    write!(fmt, "(")?;
                    write_value(fmt, value)?;
                    write!(fmt, " ? ")?;
                    write_function(fmt, x)?;
                    write!(fmt, "() : ")?;
                    write_function(fmt, y)?;
                    writeln!(fmt, "())")?;
                }
            }
        }

        fmt.deindent();
        fmt.write_indent()?;
        writeln!(fmt, "}}")
    }
}

fn write_function(fmt: &mut PrettyFormatter, function: &Function) -> std::fmt::Result {
    match function {
        Function::Block(index) => write!(fmt, "Block({})", index),
        Function::BinOp(binop) => write!(fmt, "{:?}", binop),
        Function::TriOp(triop) => write!(fmt, "{:?}", triop),
    }
}

fn write_value(fmt: &mut PrettyFormatter, value: &Value) -> std::fmt::Result {
    match value {
        Value::Arg(index) => write!(fmt, "Arg({})", index),
        Value::Number(value) => write!(fmt, "{}", value),
        Value::Function(function) => write_function(fmt, function),
        Value::ImmediateBinOp(binop, x, y) => {
            write!(fmt, "{:?}(", binop)?;
            write_value(fmt, x)?;
            write!(fmt, ", ")?;
            write_value(fmt, y)?;
            write!(fmt, ")")
        }
        Value::ImmediateTriOp(triop, x, y, z) => {
            write!(fmt, "{:?}(", triop)?;
            write_value(fmt, x)?;
            write!(fmt, ", ")?;
            write_value(fmt, y)?;
            write!(fmt, ", ")?;
            write_value(fmt, z)?;
            write!(fmt, ")")
        }
    }
}
