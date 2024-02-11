use std::io::{stdin, stdout, Read, Write};

use rand::prelude::*;

use catastrophic_core::{defines::ValueType, span::Span};
use catastrophic_hir::hir::{self, Builtin, Command};

use super::error::RuntimeError;

#[derive(Debug, Copy, Clone)]
enum Value {
    Builtin(Builtin),
    Closure(usize),
    Number(ValueType),
}

#[derive(Debug, Copy, Clone)]
enum CallableFunction {
    Builtin(Builtin),
    Block(usize),
}

#[derive(Debug, Copy, Clone)]
enum StackFunction {
    Builtin(Builtin),
    Closure(usize),
}

#[derive(Debug, Clone)]
struct Closure {
    block: usize,
    args: Vec<Value>,
}

#[derive(Debug, Clone)]
struct Stack {
    stack: Vec<Value>,
}

struct Env<'a> {
    blocks: &'a [hir::Block],
    stack: &'a mut Stack,
    closures: &'a mut Closures,
    args: Vec<Value>,
    block: usize,
    instr: usize,
}

#[derive(Debug, Clone)]
struct Closures {
    closures: Vec<Closure>,
}

#[derive(Debug, Clone)]
pub struct State {
    blocks: Vec<hir::Block>,
    closures: Closures,
    stack: Stack,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack
            .pop()
            .unwrap_or(Value::Number(0))
    }
}

impl Closures {
    pub fn new() -> Self {
        Self { closures: Vec::new() }
    }

    pub fn push(&mut self, block: usize, args: Vec<Value>) -> usize {
        let len = self.closures.len();

        self.closures
            .push(Closure { block, args });

        len
    }

    pub fn get(&self, index: usize) -> Option<&Closure> {
        self.closures.get(index)
    }
}

impl<'a> Env<'a> {
    fn new(blocks: &'a [hir::Block], stack: &'a mut Stack, closures: &'a mut Closures, args: Vec<Value>, block: usize) -> Self {
        Self {
            blocks,
            stack,
            closures,
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
        .map_err(|()| RuntimeError::InvalidArgsForBuiltin(span, builtin))?;

        self.stack.push(result);
        Ok(())
    }

    fn call_block(&mut self, args: Vec<Value>, block: usize) -> Result<(), RuntimeError> {
        Env::new(self.blocks, self.stack, self.closures, args, block).run()
    }

    fn call_instr(&mut self, span: Span<()>) -> Result<(), RuntimeError> {
        let function = match self.stack.pop() {
            Value::Builtin(builtin) => StackFunction::Builtin(builtin),
            Value::Closure(closure) => StackFunction::Closure(closure),
            Value::Number(_) => return Err(RuntimeError::CalledNumber(span)),
        };

        let (parent_args, args_count, callable) = match function {
            StackFunction::Builtin(builtin) => (
                Vec::new(),
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
                CallableFunction::Builtin(builtin),
            ),
            StackFunction::Closure(closure) => match self.closures.get(closure) {
                Some(closure) => match self.blocks.get(closure.block) {
                    Some(block) => (closure.args.clone(), block.args, CallableFunction::Block(closure.block)),
                    None => return Err(RuntimeError::CalledInvalidBlock(span)),
                },
                None => return Err(RuntimeError::CalledInvalidBlock(span)),
            },
        };

        let mut args = parent_args;

        for _ in 0..args_count {
            args.push(self.stack.pop());
        }

        match callable {
            CallableFunction::Builtin(builtin) => self.call_builtin(span, &args, builtin),
            CallableFunction::Block(block) => self.call_block(args, block),
        }
    }

    fn output_char_instr(&mut self, span: Span<()>) -> Result<(), RuntimeError> {
        // TODO: Error handling here
        match self.stack.pop() {
            Value::Number(value) => {
                let _ = stdout().write(&[value as u8]).unwrap();
                Ok(())
            }
            _ => Err(RuntimeError::OutputFunction(span)),
        }
    }

    fn output_number_instr(&mut self, span: Span<()>) -> Result<(), RuntimeError> {
        // TODO: Error handling here
        match self.stack.pop() {
            Value::Number(value) => {
                print!("{value}");
                Ok(())
            }
            _ => Err(RuntimeError::OutputFunction(span)),
        }
    }

    fn input_char_instr(&mut self) {
        // TODO: Error handling here
        stdout().flush().unwrap();
        let mut buffer = [b'\0'];
        stdin().read_exact(&mut buffer).unwrap();
        self.stack
            .push(Value::Number(buffer[0] as char as ValueType));
    }

    fn input_number_instr(&mut self) {
        // TODO: Error handling here
        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        self.stack
            .push(Value::Number(buffer.trim().parse().unwrap()));
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
                    Command::InputChar => self.input_char_instr(),
                    Command::InputNumber => self.input_number_instr(),
                },
                hir::Instr::Push(value) => match value {
                    hir::Value::Arg(index) => self.stack.push(self.args[index]),
                    hir::Value::Number(value) => self.stack.push(Value::Number(value)),
                    hir::Value::Function(function) => match function {
                        hir::Function::Builtin(builtin) => self.stack.push(Value::Builtin(builtin)),
                        hir::Function::Block(index) => {
                            let args = match self.blocks.get(index) {
                                Some(block) => self.args[0..block.offset].to_owned(),
                                None => return Err(RuntimeError::CalledInvalidBlock(instr_span)),
                            };

                            self.stack
                                .push(Value::Closure(self.closures.push(index, args)));
                        }
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
        Self {
            blocks,
            stack: Stack::new(),
            closures: Closures::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<(), RuntimeError> {
        Env::new(&self.blocks, &mut self.stack, &mut self.closures, Vec::new(), 0).run()
    }
}
