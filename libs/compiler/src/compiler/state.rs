use std::collections::{btree_map::Entry, BTreeMap};

use catastrophic_mir::mir::{BinOp, Block, Command, Function, Instr, TriOp, Value};
use dragon_tamer as llvm;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum FunctionKey {
    Block(usize),
    BinOp(BinOp),
    TriOp(TriOp),
}

#[derive(Copy, Clone)]
struct FunctionInfo {
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
    functions: BTreeMap<FunctionKey, FunctionInfo>,
}

impl FunctionKey {
    fn from_function(function: &Function) -> Self {
        match function {
            Function::Block(index) => FunctionKey::Block(*index),
            Function::BinOp(bin_op) => FunctionKey::BinOp(*bin_op),
            Function::TriOp(tri_op) => FunctionKey::TriOp(*tri_op),
        }
    }
    fn llvm_name(&self) -> String {
        match self {
            FunctionKey::Block(index) => format!("block_{}", index),
            FunctionKey::BinOp(builtin) => format!(
                "builtin_{}",
                match builtin {
                    BinOp::Plus => "plus",
                    BinOp::Minus => "minus",
                    BinOp::Equals => "equals",
                    BinOp::GreaterThan => "greater_than",
                    BinOp::LessThan => "less_than",
                }
            ),
            FunctionKey::TriOp(builtin) => format!(
                "builtin_{}",
                match builtin {
                    TriOp::IfThenElse => "if_then_else",
                }
            ),
        }
    }
}

impl FunctionInfo {
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
        let (value,) = self.push_fn.params();

        // TODO: Check for max stack size

        let entry = self.push_fn.add_block("entry");

        let (index, builder) = entry.build().build_load(&self.index);
        let (index, builder) = builder
            .build_index_store(&self.stack, &index, &value)
            .build_add(&index, &llvm::Value::constant(1));
        builder
            .build_store(&self.index, &index)
            .build_void_ret();
    }

    fn compile_pop(&mut self) {
        let entry = self.pop_fn.add_block("entry");
        let zero = self.pop_fn.add_block("zero");
        let other = self.pop_fn.add_block("other");

        let (index, builder) = entry.build().build_load(&self.index);
        builder.build_conditional_jump(&index, &zero, &other);

        zero.build()
            .build_ret(&llvm::Value::constant(0i64));

        let (index, builder) = other
            .build()
            .build_sub(&index, &llvm::Value::constant(1));
        let (value, builder) = builder.build_index_load(&self.stack, &index);
        builder
            .build_store(&self.index, &index)
            .build_ret(&value);
    }

    fn compile_bin_op(&mut self, bin_op: BinOp) {
        let entry = self.functions[&FunctionKey::BinOp(bin_op)]
            .value
            .add_block("entry");
        let builder = entry.build();

        let (x, builder) = builder.build_call(&self.pop_fn, ());
        let (y, builder) = builder.build_call(&self.pop_fn, ());

        let (result, builder) = self.build_bin_op(builder, bin_op, x, y);

        builder
            .build_call(&self.push_fn, (result,))
            .1
            .build_void_ret();
    }

    fn build_bin_op(&self, builder: llvm::Builder, bin_op: BinOp, x: llvm::Value<i64>, y: llvm::Value<i64>) -> (llvm::Value<i64>, llvm::Builder) {
        match bin_op {
            BinOp::Plus => builder.build_add(&x, &y),
            BinOp::Minus => builder.build_sub(&x, &y),
            BinOp::Equals => builder.build_eq(&x, &y),
            BinOp::GreaterThan => todo!(),
            BinOp::LessThan => builder.build_lt(&x, &y),
        }
    }

    fn compile_tri_op(&mut self, tri_op: TriOp) {
        let entry = self.functions[&FunctionKey::TriOp(tri_op)]
            .value
            .add_block("entry");

        match tri_op {
            TriOp::IfThenElse => {
                let (value, builder) = entry
                    .build()
                    .build_call(&self.pop_fn, ());

                let (then_result, builder) = builder.build_call(&self.pop_fn, ());
                let (else_result, builder) = builder.build_call(&self.pop_fn, ());

                let then_block = self.functions[&FunctionKey::TriOp(tri_op)]
                    .value
                    .add_block("then");
                let else_block = self.functions[&FunctionKey::TriOp(tri_op)]
                    .value
                    .add_block("else");

                builder.build_conditional_jump(&value, &else_block, &then_block);

                then_block
                    .build()
                    .build_call(&self.push_fn, (then_result,))
                    .1
                    .build_void_ret();

                else_block
                    .build()
                    .build_call(&self.push_fn, (else_result,))
                    .1
                    .build_void_ret();
            }
        }
    }

    fn build_value(&mut self, builder: llvm::Builder, args: &[llvm::Value<i64>], value: &Value) -> (llvm::Value<i64>, llvm::Builder) {
        match value {
            Value::Arg(arg) => (args[*arg], builder),
            Value::Number(number) => (llvm::Value::constant(*number), builder),
            Value::Function(function) => (
                llvm::Value::constant(
                    self.queue_function(FunctionKey::from_function(function))
                        .index as i64,
                ),
                builder,
            ),
            Value::ImmediateBinOp(bin_op, ref x, ref y) => {
                let (x, builder) = self.build_value(builder, args, x);
                let (y, builder) = self.build_value(builder, args, y);

                self.build_bin_op(builder, *bin_op, x, y)
            }
        }
    }

    fn compile_block(&mut self, block_index: usize) {
        let entry = self.functions[&FunctionKey::Block(block_index)]
            .value
            .add_block("entry");

        let mut block_builder = entry.build();

        let block = self.ir[block_index].clone();

        let (args, builder) = (0..block.args + block.offset)
            .into_iter()
            .fold((Vec::with_capacity(block.args + block.offset), block_builder), |(mut vec, builder), _| {
                let (value, builder) = builder.build_call(&self.pop_fn, ());
                vec.push(value);
                (vec, builder)
            });

        block_builder = builder;

        for instr in &block.instrs {
            match &instr.data {
                Instr::Command(command) => match command {
                    Command::Call => {
                        let (index, builder) = block_builder.build_call(&self.pop_fn, ());
                        let ((f, offset), builder) = builder.build_call(&self.call_fn, (index,));

                        let cont = self.functions[&FunctionKey::Block(block_index)]
                            .value
                            .add_block("cont");

                        let mut last_block = cont;

                        let mut table = builder.build_jump_table(&offset, &cont);

                        for (i, arg) in args.iter().enumerate() {
                            let new_block = self.functions[&FunctionKey::Block(block_index)]
                                .value
                                .add_block(&format!("block_{}", i));

                            new_block
                                .build()
                                .build_call(&self.push_fn, (*arg,))
                                .1
                                .build_jump(&last_block);

                            table = table.case(&llvm::Value::constant((i + 1) as i64), &new_block);
                            last_block = new_block;
                        }

                        table.finish();

                        block_builder = cont.build().build_call(&f, ()).1;
                    }
                    Command::OutputChar => (),
                    Command::OutputNumber => (),
                    Command::InputChar => (),
                    Command::InputNumber => (),
                },
                Instr::Push(ref value) => match value {
                    Value::Arg(arg) => {
                        block_builder = block_builder
                            .build_call(&self.push_fn, (args[*arg],))
                            .1
                    }
                    Value::Number(number) => {
                        block_builder = block_builder
                            .build_call(&self.push_fn, (llvm::Value::constant(*number),))
                            .1
                    }
                    Value::Function(function) => {
                        let index = self
                            .queue_function(FunctionKey::from_function(function))
                            .index as i64;
                        block_builder = block_builder
                            .build_call(&self.push_fn, (llvm::Value::constant(index),))
                            .1;
                    }
                    Value::ImmediateBinOp(bin_op, x, y) => {
                        let (x, builder) = self.build_value(block_builder, &args, x);
                        let (y, builder) = self.build_value(builder, &args, y);

                        let (result, builder) = self.build_bin_op(builder, *bin_op, x, y);

                        block_builder = builder
                            .build_call(&self.push_fn, (result,))
                            .1;
                    }
                },
                Instr::ImmediateCall(function) => {
                    let info = self.queue_function(FunctionKey::from_function(function));

                    for arg in args.iter().take(info.offset) {
                        block_builder = block_builder
                            .build_call(&self.push_fn, (*arg,))
                            .1;
                    }

                    block_builder = block_builder
                        .build_call(&info.value, ())
                        .1;
                }
            }
        }

        block_builder.build_void_ret();
    }

    fn compile_function(&mut self, function: FunctionKey) {
        match function {
            FunctionKey::Block(index) => self.compile_block(index),
            FunctionKey::BinOp(bin_op) => self.compile_bin_op(bin_op),
            FunctionKey::TriOp(tri_op) => self.compile_tri_op(tri_op),
        }
    }

    fn queue_function(&mut self, function: FunctionKey) -> FunctionInfo {
        let count = self.functions.len();

        match self.functions.entry(function) {
            Entry::Vacant(entry) => {
                self.queue.push(function);

                let llvm_function = self
                    .module
                    .add_function(function.llvm_name());
                let offset = match function {
                    FunctionKey::Block(idx) => self.ir[idx].offset,
                    _ => 0,
                };

                *entry.insert(FunctionInfo::new(count, offset, llvm_function))
            }
            Entry::Occupied(occupied) => *occupied.get(),
        }
    }

    fn compile_call(&mut self) {
        let (value,) = self.call_fn.params();

        let entry = self.call_fn.add_block("entry");
        let builder = entry.build();

        let blocks: Vec<_> = self
            .functions
            .iter()
            .map(|(_, f)| {
                let block = self
                    .call_fn
                    .add_block(&format!("block_{}", f.index));
                let (value, builder) = block
                    .build()
                    .build_struct(&f.value.as_value(), &llvm::Value::constant(f.offset as i64));
                let (value, builder) = builder.build_load(&value);
                builder.build_ret(&value);
                (f.index, block)
            })
            .collect();

        let default = self.call_fn.add_block("default");
        let mut table = builder.build_jump_table(&value, &default);

        for (index, block) in &blocks {
            table = table.case(&llvm::Value::constant(*index as i64), block);
        }

        table.finish();

        default.build().build_unreachable();
    }

    fn compile_main(&mut self) {
        let main: llvm::Function<fn() -> i64> = self.module.add_function("main");
        let entry = main.add_block("entry");
        let (result, builder) = entry
            .build()
            .build_call(&self.functions[&FunctionKey::Block(0)].value, ())
            .1
            .build_call(&self.pop_fn, ());
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
