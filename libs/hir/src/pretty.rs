use std::fmt::Write;

use catastrophic_core::pretty::{PrettyDebug, PrettyFormatter};

use crate::hir::{Block, Command, Function, Instr, Value};

impl PrettyDebug for Block {
    fn pretty_debug(&self, fmt: &mut PrettyFormatter) -> std::fmt::Result {
        fmt.write_indent()?;
        writeln!(fmt, "{{")?;
        fmt.indent();

        for instr in &self.instrs {
            fmt.write_indent()?;

            match instr.data {
                Instr::Command(command) => match command {
                    Command::Call => writeln!(fmt, "Call")?,
                    Command::OutputChar => writeln!(fmt, "OutputChar")?,
                    Command::OutputNumber => writeln!(fmt, "OutputNumber")?,
                    Command::InputChar => writeln!(fmt, "InputChar")?,
                    Command::InputNumber => writeln!(fmt, "InputNumber")?,
                },
                Instr::Push(value) => match value {
                    Value::Arg(index) => writeln!(fmt, "Push[Arg({})]", index)?,
                    Value::Number(value) => writeln!(fmt, "Push[{}]", value)?,
                    Value::Function(function) => match function {
                        Function::Block(index) => writeln!(fmt, "Push[Block({})]", index)?,
                        Function::Builtin(builtin) => writeln!(fmt, "Push[{}]", builtin)?,
                    },
                },
            }
        }

        fmt.deindent();
        fmt.write_indent()?;
        writeln!(fmt, "}}")
    }
}
