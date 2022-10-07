use std::collections::{btree_map::Entry, BTreeMap};

use catastrophic_ir::ir::{Block, Builtin, Command, Instr, Value};
use dragon_tamer::{self as llvm, Builder};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum FunctionKey {
    Builtin(Builtin),
    Block(usize),
}

#[derive(Copy, Clone)]
struct Function {
    index: usize,
    offset: usize,
    value: llvm::Function<fn()>,
}

pub struct State {
    ir: Vec<Block>,
    queue: Vec<FunctionKey>,

    module: llvm::Module,
    pop_fn: llvm::Function<fn() -> i64>,
    push_fn: llvm::Function<fn(i64)>,
    call_fn: llvm::Function<fn(i64) -> (fn(), i64)>,
    stack: llvm::Value<*mut [i64; 256]>,
    index: llvm::Value<*mut u32>,
    functions: BTreeMap<FunctionKey, Function>,
}

impl FunctionKey {
    fn llvm_name(&self) -> String {
        match self {
            FunctionKey::Builtin(builtin) => format!(
                "builtin_{}",
                match builtin {
                    Builtin::Plus => "plus",
                    Builtin::Minus => "minus",
                    Builtin::Equals => "equals",
                    Builtin::GreaterThan => "greater_than",
                    Builtin::LessThan => "less_than",
                    Builtin::IfThenElse => "if_then_else",
                }
            ),
            FunctionKey::Block(index) => format!("block_{}", index),
        }
    }
}

impl Function {
    pub fn new(index: usize, offset: usize, value: llvm::Function<fn()>) -> Self {
        Self { index, offset, value }
    }
}

impl State {
    pub fn new(ir: Vec<Block>) -> Self {
        let module = llvm::Module::new("test", "test");
        let pop_fn = module.add_function("stack_pop");
        let push_fn = module.add_function("stack_push");
        let call_fn = module.add_function("call_index");
        let stack = module.add_array();
        let index = module.add_global(0);

        Self {
            ir,
            queue: Vec::new(),
            module,
            pop_fn,
            push_fn,
            call_fn,
            stack,
            index,
            functions: BTreeMap::new(),
        }
    }

    fn compile_push(&mut self) {
        let builder = Builder::new();
        let (value,) = self.push_fn.params();

        // TODO: Check for max stack size

        let entry = self.push_fn.add_block("entry");
        builder.set_block(&entry);
        let index = builder.build_load(&self.index);
        builder.build_index_store(&self.stack, &index, &value);
        let new_index = builder.build_add(&index, &llvm::Value::constant(1));
        builder.build_store(&self.index, &new_index);
        builder.build_void_ret();
    }

    fn compile_pop(&mut self) {
        let builder = Builder::new();

        let entry = self.pop_fn.add_block("entry");
        let zero = self.pop_fn.add_block("zero");
        let other = self.pop_fn.add_block("other");

        builder.set_block(&entry);
        let index = builder.build_load(&self.index);
        builder.build_conditional_jump(&index, &zero, &other);

        builder.set_block(&zero);
        builder.build_ret(&llvm::Value::constant(0i64));

        builder.set_block(&other);
        let new_index = builder.build_sub(&index, &llvm::Value::constant(1));
        let value = builder.build_index_load(&self.stack, &new_index);
        builder.build_store(&self.index, &new_index);
        builder.build_ret(&value);
    }

    fn compile_builtin(&mut self, builtin: Builtin) {
        let builder = Builder::new();

        let entry = self.functions[&FunctionKey::Builtin(builtin)].value.add_block("entry");
        builder.set_block(&entry);

        match builtin {
            Builtin::Plus => {
                let x = builder.build_call(&self.pop_fn, ());
                let y = builder.build_call(&self.pop_fn, ());
                let result = builder.build_add(&x, &y);
                builder.build_call(&self.push_fn, (result,));
                builder.build_void_ret();
            }
            Builtin::Minus => {
                let x = builder.build_call(&self.pop_fn, ());
                let y = builder.build_call(&self.pop_fn, ());
                let result = builder.build_sub(&x, &y);
                builder.build_call(&self.push_fn, (result,));
                builder.build_void_ret();
            }
            Builtin::Equals => {
                let x = builder.build_call(&self.pop_fn, ());
                let y = builder.build_call(&self.pop_fn, ());
                let result = builder.build_eq(&x, &y);
                builder.build_call(&self.push_fn, (result,));
                builder.build_void_ret();
            }
            Builtin::GreaterThan => todo!(),
            Builtin::LessThan => todo!(),
            Builtin::IfThenElse => {
                let v = builder.build_call(&self.pop_fn, ());
                let i = builder.build_call(&self.pop_fn, ());
                let e = builder.build_call(&self.pop_fn, ());

                let then_block = self.functions[&FunctionKey::Builtin(builtin)].value.add_block("then");
                let else_block = self.functions[&FunctionKey::Builtin(builtin)].value.add_block("else");

                builder.build_conditional_jump(&v, &else_block, &then_block);

                builder.set_block(&then_block);
                builder.build_call(&self.push_fn, (i,));
                builder.build_void_ret();

                builder.set_block(&else_block);
                builder.build_call(&self.push_fn, (e,));
                builder.build_void_ret();
            }
        };
    }

    fn compile_block(&mut self, block_index: usize) {
        let builder = Builder::new();

        let mut entry = self.functions[&FunctionKey::Block(block_index)].value.add_block("entry");
        builder.set_block(&entry);

        let block = self.ir[block_index].clone();

        let args: Vec<_> = (0..block.args + block.offset)
            .into_iter()
            .map(|_| builder.build_call(&self.pop_fn, ()))
            .collect();

        for instr in &block.instrs {
            match instr.data {
                Instr::Command(command) => match command {
                    Command::Call => {
                        let index = builder.build_call(&self.pop_fn, ());
                        let (f, offset) = builder.build_call(&self.call_fn, (index,));

                        let cont = self.functions[&FunctionKey::Block(block_index)].value.add_block("cont");

                        let mut last_block = cont;

                        let mut table = builder.build_jump_table(&offset, &cont);

                        for (i, arg) in args.iter().enumerate() {
                            let new_block = self.functions[&FunctionKey::Block(block_index)].value.add_block(&format!("block_{}", i));
                            builder.set_block(&new_block);
                            builder.build_call(&self.push_fn, (*arg,));
                            builder.build_jump(&last_block);

                            table = table.case(&llvm::Value::constant((i + 1) as i64), &new_block);
                            last_block = new_block;
                        }

                        builder.set_block(&entry);
                        table.finish();

                        entry = cont;

                        builder.set_block(&entry);
                        builder.build_call(&f, ());
                    }
                    Command::OutputChar => (),
                    Command::OutputNumber => (),
                    Command::InputChar => (),
                    Command::InputNumber => (),
                },
                Instr::Push(value) => match value {
                    Value::Arg(arg) => builder.build_call(&self.push_fn, (args[arg],)),
                    Value::Number(number) => builder.build_call(&self.push_fn, (llvm::Value::constant(number),)),
                    Value::Block(index) => {
                        let index = self.queue_function(FunctionKey::Block(index)).index as i64;
                        builder.build_call(&self.push_fn, (llvm::Value::constant(index),));
                    }
                    Value::Builtin(builtin) => {
                        let index = self.queue_function(FunctionKey::Builtin(builtin)).index as i64;
                        builder.build_call(&self.push_fn, (llvm::Value::constant(index),));
                    }
                },
            }
        }

        builder.build_void_ret();
    }

    fn compile_function(&mut self, function: FunctionKey) {
        match function {
            FunctionKey::Builtin(builtin) => self.compile_builtin(builtin),
            FunctionKey::Block(index) => self.compile_block(index),
        }
    }

    fn queue_function(&mut self, function: FunctionKey) -> Function {
        let count = self.functions.len();

        match self.functions.entry(function) {
            Entry::Vacant(entry) => {
                self.queue.push(function);

                let llvm_function = self.module.add_function(function.llvm_name());
                let offset = match function {
                    FunctionKey::Builtin(_) => 0,
                    FunctionKey::Block(idx) => self.ir[idx].offset,
                };

                *entry.insert(Function::new(count, offset, llvm_function))
            }
            Entry::Occupied(occupied) => *occupied.get(),
        }
    }

    fn compile_call(&mut self) {
        let builder = Builder::new();
        let (value,) = self.call_fn.params();

        let entry = self.call_fn.add_block("entry");
        builder.set_block(&entry);

        let blocks: Vec<_> = self
            .functions
            .iter()
            .map(|(_, f)| {
                let block = self.call_fn.add_block(&format!("block_{}", f.index));
                builder.set_block(&block);
                let value = builder.build_struct(&f.value.as_value(), &llvm::Value::constant(f.offset as i64));
                let value = builder.build_load(&value);
                builder.build_ret(&value);
                (f.index, block)
            })
            .collect();

        let default = self.call_fn.add_block("default");
        builder.set_block(&default);
        builder.build_unreachable();

        builder.set_block(&entry);
        let mut table = builder.build_jump_table(&value, &default);

        for (index, block) in &blocks {
            table = table.case(&llvm::Value::constant(*index as i64), block);
        }

        table.finish();
    }

    fn compile_main(&mut self) {
        let builder = Builder::new();
        let main: llvm::Function<fn() -> i64> = self.module.add_function("main");
        let entry = main.add_block("entry");
        builder.set_block(&entry);
        builder.build_call(&self.functions[&FunctionKey::Block(0)].value, ());
        let result = builder.build_call(&self.pop_fn, ());
        builder.build_ret(&result);
    }

    pub fn compile(&mut self) {
        self.compile_pop();
        self.compile_push();
        self.queue_function(FunctionKey::Block(0));

        while let Some(function) = self.queue.pop() {
            self.compile_function(function);
        }

        self.compile_call();
        self.compile_main();

        println!("{:?}", self.module)
    }
}
