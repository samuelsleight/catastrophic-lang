use catastrophic_ir::ir::{self, Builtin};

#[derive(Debug, Copy, Clone)]
enum Value {
    Builtin(Builtin),
    Block(usize),
    Number(u64),
}

#[derive(Debug, Copy, Clone)]
enum Function {
    Builtin(Builtin),
    Block(usize),
}

struct Env<'a> {
    blocks: &'a [ir::Block],
    stack: &'a mut Vec<Value>,
    args: Vec<Value>,
    block: usize,
    instr: usize,
}

#[derive(Debug, Clone)]
pub struct State {
    blocks: Vec<ir::Block>,
    stack: Vec<Value>,
}

impl<'a> Env<'a> {
    fn new(blocks: &'a [ir::Block], stack: &'a mut Vec<Value>, args: Vec<Value>, block: usize) -> Self {
        Self {
            blocks,
            stack,
            args,
            block,
            instr: 0,
        }
    }

    fn call_builtin(&mut self, args: &[Value], builtin: Builtin) {
        match builtin {
            Builtin::Plus => {
                if let [Value::Number(a), Value::Number(b)] = args[..] {
                    self.stack.push(Value::Number(a + b));
                } else {
                    todo!("Invalid args for +");
                }
            }
            Builtin::Minus => {
                if let [Value::Number(a), Value::Number(b)] = args[..] {
                    self.stack.push(Value::Number(a - b));
                } else {
                    todo!("Invalid args for -");
                }
            }
            Builtin::Equals => {
                if let [Value::Number(a), Value::Number(b)] = args[..] {
                    self.stack.push(Value::Number(u64::from(a == b)));
                } else {
                    todo!("Invalid args for =");
                }
            }
            Builtin::IfThenElse => {
                if let [Value::Number(i), t, e] = args[..] {
                    self.stack.push(if i == u64::from(false) { e } else { t });
                } else {
                    todo!("Invalid args for +");
                }
            }
        }
    }

    fn call_block(&mut self, args: Vec<Value>, block: usize) {
        Env::new(self.blocks, self.stack, args, block).run();
    }

    fn call_instr(&mut self) {
        let function = match self.stack.pop() {
            None => todo!("Empty stack"),
            Some(value) => match value {
                Value::Builtin(builtin) => Function::Builtin(builtin),
                Value::Block(block) => Function::Block(block),
                Value::Number(_) => todo!("Attempted to call a number"),
            },
        };

        let (offset_count, args_count) = match function {
            Function::Builtin(builtin) => (
                0,
                match builtin {
                    Builtin::Plus | Builtin::Minus | Builtin::Equals => 2,
                    Builtin::IfThenElse => 3,
                },
            ),
            Function::Block(block) => match self.blocks.get(block) {
                Some(block) => (block.offset, block.args),
                None => todo!("Invalid block index"),
            },
        };

        let mut args = self.args[0..offset_count].to_owned();

        for _ in 0..args_count {
            match self.stack.pop() {
                Some(value) => args.push(value),
                None => todo!("Not enough arguments on stack"),
            }
        }

        match function {
            Function::Builtin(builtin) => self.call_builtin(&args, builtin),
            Function::Block(block) => self.call_block(args, block),
        }
    }

    fn run(&mut self) {
        while let Some(instr) = self.blocks.get(self.block).and_then(|block| block.instrs.get(self.instr)) {
            match *instr {
                ir::Instr::Call => self.call_instr(),
                ir::Instr::Push(value) => match value {
                    ir::Value::Arg(index) => self.stack.push(self.args[index]),
                    ir::Value::Block(index) => self.stack.push(Value::Block(index)),
                    ir::Value::Number(value) => self.stack.push(Value::Number(value as u64)),
                    ir::Value::Builtin(builtin) => self.stack.push(Value::Builtin(builtin)),
                },
            };

            self.instr += 1;
        }
    }
}

impl State {
    pub fn new(blocks: Vec<ir::Block>) -> Self {
        Self { blocks, stack: Vec::new() }
    }

    pub fn interpret(&mut self) {
        Env::new(&self.blocks, &mut self.stack, Vec::new(), 0).run();
        println!("{:?}", self.stack);
    }
}
