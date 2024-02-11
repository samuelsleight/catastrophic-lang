use std::fmt::Write;

use catastrophic_core::pretty::{PrettyDebug, PrettyFormatter};

use crate::ast::{Block, Command, InstrValue, Instruction, SymbolValue};

impl PrettyDebug for Block {
    fn pretty_debug(&self, fmt: &mut PrettyFormatter) -> std::fmt::Result {
        writeln!(fmt, "{{")?;
        fmt.indent();

        write_block(self, fmt)?;

        fmt.deindent();
        writeln!(fmt, "}}")
    }
}

fn write_block(block: &Block, fmt: &mut PrettyFormatter) -> std::fmt::Result {
    for (name, value) in &block.symbols {
        fmt.write_indent()?;
        write!(fmt, "{name}: ")?;

        match &value.value.data {
            SymbolValue::Number(value) => writeln!(fmt, "{value}")?,
            SymbolValue::Block(block) => {
                writeln!(fmt, "{{")?;
                fmt.indent();

                write_block(block, fmt)?;

                fmt.deindent();
                fmt.write_indent()?;
                writeln!(fmt, "}}")?;
            }
            SymbolValue::Builtin(builtin) => writeln!(fmt, "{builtin}")?,
        }
    }

    if !block.symbols.is_empty() {
        writeln!(fmt)?;
    }

    for instr in block.instrs.iter().rev() {
        fmt.write_indent()?;

        match &instr.data {
            Instruction::Command(command) => match command {
                Command::Call => writeln!(fmt, "Call")?,
                Command::OutputChar => writeln!(fmt, "OutputChar")?,
                Command::OutputNumber => writeln!(fmt, "OutputNumber")?,
                Command::InputChar => writeln!(fmt, "InputChar")?,
                Command::InputNumber => writeln!(fmt, "InputNumber")?,
            },
            Instruction::Push(value) => match value {
                InstrValue::Number(value) => writeln!(fmt, "Push[{value}]")?,
                InstrValue::Ident(ident) => writeln!(fmt, "Push[{ident}]")?,
                InstrValue::Block(block) => {
                    writeln!(fmt, "Push {{")?;
                    fmt.indent();

                    write_block(block, fmt)?;

                    fmt.deindent();
                    fmt.write_indent()?;
                    writeln!(fmt, "}}")?;
                }
                InstrValue::Builtin(builtin) => writeln!(fmt, "Push[{builtin:?}]")?,
            },
        }
    }

    Ok(())
}
