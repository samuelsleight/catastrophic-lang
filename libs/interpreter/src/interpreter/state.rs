use std::io::{stdin, stdout, Read, Write};

use rand::prelude::*;

use catastrophic_core::{defines::ValueType, span::Span};
use catastrophic_hir::hir::{self, Builtin, Command};

use super::error::RuntimeError;

#[derive(Debug, Copy, Clone)]
enum Value {
    Builtin(Builtin),
    Block(usize),
    Number(ValueType),
}

#[derive(Debug, Copy, Clone)]
enum Function {
    Builtin(Builtin),
    Block(usize),
}

#[derive(Debug, Clone)]
struct Stack {
    stack: Vec<Value>,
}

struct Env<'a> {
    blocks: &'a [hir::Block],
    stack: &'a mut Stack,
    args: Vec<Value>,
    block: usize,
    instr: usize,
}

#[derive(Debug, Clone)]
pub struct State {
    blocks: Vec<hir::Block>,
    stack: Stack,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    pub fn pop(&mut self) -> Value {
        self.stack
            .pop()
            .unwrap_or(Value::Number(0))
    }
}

impl<'a> Env<'a> {
    fn new(blocks: &'a [hir::Block], stack: &'a mut Stack, args: Vec<Value>, block: usize) -> Self {
        Self {
            blocks,
            stack,
            args,
            block,
            instr: 0,
        }
    }

    fn call_builtin(&mut self, span: Span<()>, args: &[Value], builtin: Builtin) -> Result<(), RuntimeError> {
        let result = match builtin {
            Builtin::Plus => {
                if let [Value::Number(a), Value::Number(b)] = args[..] {
                    Ok(Value::Number(a + b))
                } else {
                    Err(())
                }
            }
            Builtin::Minus => {
                if let [Value::Number(a), Value::Number(b)] = args[..] {
                    Ok(Value::Number(a - b))
                } else {
                    Err(())
                }
            }
            Builtin::Multiply => {
                if let [Value::Number(a), Value::Number(b)] = args[..] {
                    Ok(Value::Number(a * b))
                } else {
                    Err(())
                }
            }
            Builtin::Divide => {
                if let [Value::Number(a), Value::Number(b)] = args[..] {
                    Ok(Value::Number(a / b))
                } else {
                    Err(())
                }
            }
            Builtin::LessThan => {
                if let [Value::Number(a), Value::Number(b)] = args[..] {
                    Ok(Value::Number(ValueType::from(a < b)))
                } else {
                    Err(())
                }
            }
            Builtin::GreaterThan => {
                if let [Value::Number(a), Value::Number(b)] = args[..] {
                    Ok(Value::Number(ValueType::from(a > b)))
                } else {
                    Err(())
                }
            }
            Builtin::Equals => {
                if let [Value::Number(a), Value::Number(b)] = args[..] {
                    Ok(Value::Number(ValueType::from(a == b)))
                } else {
                    Err(())
                }
            }
            Builtin::Random => {
                if let [Value::Number(a), Value::Number(b)] = args[..] {
                    let mut rng = thread_rng();
                    Ok(Value::Number(ValueType::from(rng.gen_range(a..=b))))
                } else {
                    Err(())
                }
            }
            Builtin::IfThenElse => {
                if let [Value::Number(i), t, e] = args[..] {
                    Ok(if i == ValueType::from(false) { e } else { t })
                } else {
                    Err(())
                }
            }
        }
        .map_err(|_| RuntimeError::InvalidArgsForBuiltin(span, builtin))?;

        self.stack.push(result);
        Ok(())
    }

    fn call_block(&mut self, args: Vec<Value>, block: usize) -> Result<(), RuntimeError> {
        Env::new(self.blocks, self.stack, args, block).run()
    }

    fn call_instr(&mut self, span: Span<()>) -> Result<(), RuntimeError> {
        let function = match self.stack.pop() {
            Value::Builtin(builtin) => Function::Builtin(builtin),
            Value::Block(block) => Function::Block(block),
            Value::Number(_) => return Err(RuntimeError::CalledNumber(span)),
        };

        let (offset_count, args_count) = match function {
            Function::Builtin(builtin) => (
                0,
                match builtin {
                    Builtin::Plus
                    | Builtin::Minus
                    | Builtin::Multiply
                    | Builtin::Divide
                    | Builtin::LessThan
                    | Builtin::GreaterThan
                    | Builtin::Equals
                    | Builtin::Random => 2,

                    Builtin::IfThenElse => 3,
                },
            ),
            Function::Block(block) => match self.blocks.get(block) {
                Some(block) => (block.offset, block.args),
                None => return Err(RuntimeError::CalledInvalidBlock(span)),
            },
        };

        let mut args = self.args[0..offset_count].to_owned();

        for _ in 0..args_count {
            args.push(self.stack.pop())
        }

        match function {
            Function::Builtin(builtin) => self.call_builtin(span, &args, builtin),
            Function::Block(block) => self.call_block(args, block),
        }
    }

    fn output_char_instr(&mut self, span: Span<()>) -> Result<(), RuntimeError> {
        match self.stack.pop() {
            Value::Number(value) => {
                let value = char::try_from(u32::try_from(value).unwrap_or(0)).unwrap_or('\0');
                print!("{}", value);
                Ok(())
            }
            _ => Err(RuntimeError::OutputFunction(span)),
        }
    }

    fn output_number_instr(&mut self, span: Span<()>) -> Result<(), RuntimeError> {
        match self.stack.pop() {
            Value::Number(value) => {
                print!("{}", value);
                Ok(())
            }
            _ => Err(RuntimeError::OutputFunction(span)),
        }
    }

    fn input_char_instr(&mut self) -> Result<(), RuntimeError> {
        // TODO: Error handling here
        stdout().flush().unwrap();
        let mut buffer = [b'\0'];
        stdin().read_exact(&mut buffer).unwrap();
        self.stack
            .push(Value::Number(buffer[0] as char as ValueType));
        Ok(())
    }

    fn input_number_instr(&mut self) -> Result<(), RuntimeError> {
        // TODO: Error handling here
        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        self.stack
            .push(Value::Number(buffer.trim().parse().unwrap()));
        Ok(())
    }

    fn run(&mut self) -> Result<(), RuntimeError> {
        while let Some(instr) = self
            .blocks
            .get(self.block)
            .and_then(|block| block.instrs.get(self.instr))
        {
            let instr_span = instr.swap(());
            match instr.data {
                hir::Instr::Command(command) => match command {
                    Command::Call => self.call_instr(instr_span)?,
                    Command::OutputChar => self.output_char_instr(instr_span)?,
                    Command::OutputNumber => self.output_number_instr(instr_span)?,
                    Command::InputChar => self.input_char_instr()?,
                    Command::InputNumber => self.input_number_instr()?,
                },
                hir::Instr::Push(value) => match value {
                    hir::Value::Arg(index) => self.stack.push(self.args[index]),
                    hir::Value::Number(value) => self.stack.push(Value::Number(value)),
                    hir::Value::Function(function) => match function {
                        hir::Function::Builtin(builtin) => self.stack.push(Value::Builtin(builtin)),
                        hir::Function::Block(index) => self.stack.push(Value::Block(index)),
                    },
                },
            };

            self.instr += 1;
        }

        Ok(())
    }
}

impl State {
    pub fn new(blocks: Vec<hir::Block>) -> Self {
        Self { blocks, stack: Stack::new() }
    }

    pub fn interpret(&mut self) -> Result<(), RuntimeError> {
        Env::new(&self.blocks, &mut self.stack, Vec::new(), 0).run()
    }
}
