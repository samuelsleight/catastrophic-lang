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

pub type FunctionMetadata = (fn(), i64);

pub struct State {
    ir: Vec<Block>,
    queue: Vec<FunctionKey>,

    module: llvm::Module,

    putchar_fn: llvm::Function<fn(i32)>,

    printf_str: llvm::Value<String>,
    printf_fn: llvm::Function<fn(String, llvm::Variadic)>,

    getchar_fn: llvm::Function<fn() -> i32>,

    pop_fn: llvm::Function<fn() -> i64>,
    push_fn: llvm::Function<fn(i64)>,
    call_fn: llvm::Function<fn(i64) -> FunctionMetadata>,

    closure_push_fn: llvm::Function<fn(i64)>,
    closure_offset_fn: llvm::Function<fn(i64, i64)>,

    closure_stack: llvm::Value<*mut [i64; 2048]>,
    closure_stack_index: llvm::Value<*mut u32>,

    stack: llvm::Value<*mut [i64; 1024]>,
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
            FunctionKey::Block(index) => format!("block_{index}"),
            FunctionKey::BinOp(builtin) => format!(
                "builtin_{}",
                match builtin {
                    BinOp::Plus => "plus",
                    BinOp::Minus => "minus",
                    BinOp::Multiply => "multiply",
                    BinOp::Divide => "divide",
                    BinOp::Equals => "equals",
                    BinOp::GreaterThan => "greater_than",
                    BinOp::LessThan => "less_than",
                    BinOp::Random => "random",
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
        let putchar_fn = module.add_function("putchar");
        let printf_str = module.add_string("%lld");
        let printf_fn = module.add_function("printf");
        let getchar_fn = module.add_function("getchar");
        let pop_fn = module.add_function("stack_pop");
        let push_fn = module.add_function("stack_push");
        let call_fn = module.add_function("call_index");
        let closure_push_fn = module.add_function("closure_push");
        let closure_offset_fn = module.add_function("closure_offset");
        let closure_stack = module.add_array();
        let closure_stack_index = module.add_global(0);
        let stack = module.add_array();
        let index = module.add_global(0);

        Self {
            ir,
            queue: Vec::new(),
            module,
            putchar_fn,
            printf_str,
            printf_fn,
            getchar_fn,
            pop_fn,
            push_fn,
            call_fn,
            closure_push_fn,
            closure_offset_fn,
            closure_stack,
            closure_stack_index,
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

    fn compile_closure_push(&mut self) {
        let (value,) = self.closure_push_fn.params();

        // TODO: Check for max stack size

        let entry = self.closure_push_fn.add_block("entry");

        let (index, builder) = entry
            .build()
            .build_load(&self.closure_stack_index);
        let (index, builder) = builder
            .build_index_store(&self.closure_stack, &index, &value)
            .build_add(&index, &llvm::Value::constant(1));
        builder
            .build_store(&self.closure_stack_index, &index)
            .build_void_ret();
    }

    fn compile_closure_offset(&mut self) {
        let (index, offset) = self.closure_offset_fn.params();

        let entry = self
            .closure_offset_fn
            .add_block("entry");

        let cont = self.closure_offset_fn.add_block("cont");

        let fin = self.closure_offset_fn.add_block("fin");
        fin.build().build_void_ret();

        entry
            .build()
            .build_conditional_jump(&offset, &fin, &cont);

        // Get the arg from the closure stack
        let (arg, builder) = cont
            .build()
            .build_index_load(&self.closure_stack, &index);

        // Push the arg to the main stack
        let builder = builder
            .build_call(&self.push_fn, (arg,))
            .1;

        // Adjust the parameters
        let (offset, builder) = builder.build_sub(&offset, &llvm::Value::constant(1));
        let (index, builder) = builder.build_add(&index, &llvm::Value::constant(1));

        builder
            .build_call(&self.closure_offset_fn, (index, offset))
            .1
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

        let (result, builder) = build_bin_op(builder, bin_op, x, y);

        builder
            .build_call(&self.push_fn, (result,))
            .1
            .build_void_ret();
    }

    fn compile_tri_op(&mut self, tri_op: TriOp) {
        let entry = self.functions[&FunctionKey::TriOp(tri_op)]
            .value
            .add_block("entry");

        let builder = entry.build();

        let (x, builder) = builder.build_call(&self.pop_fn, ());
        let (y, builder) = builder.build_call(&self.pop_fn, ());
        let (z, builder) = builder.build_call(&self.pop_fn, ());

        let (result, builder) = build_tri_op(builder, tri_op, x, y, z);

        builder
            .build_call(&self.push_fn, (result,))
            .1
            .build_void_ret();
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

                build_bin_op(builder, *bin_op, x, y)
            }
            Value::ImmediateTriOp(tri_op, ref x, ref y, ref z) => {
                let (x, builder) = self.build_value(builder, args, x);
                let (y, builder) = self.build_value(builder, args, y);
                let (z, builder) = self.build_value(builder, args, z);

                build_tri_op(builder, *tri_op, x, y, z)
            }
        }
    }

    fn build_call_command(&self, builder: llvm::Builder) -> llvm::Builder {
        // Load the closre pointer from the main stack
        let (closure_index, builder) = builder.build_call(&self.pop_fn, ());

        // Load the block index from the closure stack
        let (block_to_call_index, builder) = builder.build_index_load(&self.closure_stack, &closure_index);

        // Get the information for the block we are calling
        let ((f, offset), builder) = builder.build_call(&self.call_fn, (block_to_call_index,));

        // Increment the closure pointer to point at the args
        let (arg_index, builder) = builder.build_add(&closure_index, &llvm::Value::constant(1));

        // Iteratively push all the closure arguments onto the stack
        let builder = builder
            .build_call(&self.closure_offset_fn, (arg_index, offset))
            .1;

        // Call the function!
        builder.build_call(&f, ()).1
    }

    fn build_command_instr(&self, builder: llvm::Builder, command: Command) -> llvm::Builder {
        match command {
            Command::Call => self.build_call_command(builder),
            Command::OutputChar => {
                let (value, builder) = builder.build_call(&self.pop_fn, ());
                let (value, builder) = builder.build_int_cast(&value);
                builder
                    .build_call(&self.putchar_fn, (value,))
                    .1
            }
            Command::OutputNumber => {
                let (value, builder) = builder.build_call(&self.pop_fn, ());
                builder
                    .build_variadic_call(&self.printf_fn, (self.printf_str.clone(),), &[value.untyped()])
                    .1
            }
            Command::InputChar => {
                let (value, builder) = builder.build_call(&self.getchar_fn, ());
                let (value, builder) = builder.build_int_cast(&value);
                builder
                    .build_call(&self.push_fn, (value,))
                    .1
            }
            Command::InputNumber => unimplemented!(),
        }
    }

    fn build_push_instr(&mut self, builder: llvm::Builder, args: &[llvm::Value<i64>], value: &Value) -> llvm::Builder {
        match value {
            Value::Arg(arg) => {
                builder
                    .build_call(&self.push_fn, (args[*arg],))
                    .1
            }
            Value::Number(number) => {
                builder
                    .build_call(&self.push_fn, (llvm::Value::constant(*number),))
                    .1
            }
            Value::Function(function) => {
                let block_info = self.queue_function(FunctionKey::from_function(function));

                // Save closure stack index
                let (closure_index, builder) = builder.build_load(&self.closure_stack_index);

                // Push block index to closure stack
                let builder = builder
                    .build_call(&self.closure_push_fn, (llvm::Value::constant(block_info.index as i64),))
                    .1;

                // Push closure capture (offset args) to closure stack
                let builder = (0..block_info.offset)
                    .rev()
                    .fold(builder, |builder, arg| {
                        builder
                            .build_call(&self.closure_push_fn, (*args.get(arg).unwrap(),))
                            .1
                    });

                // Push closure stack value to main stack
                let (closure_index, builder) = builder.build_int_cast(&closure_index);

                builder
                    .build_call(&self.push_fn, (closure_index,))
                    .1
            }
            Value::ImmediateBinOp(bin_op, x, y) => {
                let (x, builder) = self.build_value(builder, args, x);
                let (y, builder) = self.build_value(builder, args, y);

                let (result, builder) = build_bin_op(builder, *bin_op, x, y);

                builder
                    .build_call(&self.push_fn, (result,))
                    .1
            }
            Value::ImmediateTriOp(tri_op, x, y, z) => {
                let (x, builder) = self.build_value(builder, args, x);
                let (y, builder) = self.build_value(builder, args, y);
                let (z, builder) = self.build_value(builder, args, z);

                let (result, builder) = build_tri_op(builder, *tri_op, x, y, z);

                builder
                    .build_call(&self.push_fn, (result,))
                    .1
            }
        }
    }

    fn build_immediate_call_instr(&mut self, mut builder: llvm::Builder, args: &[llvm::Value<i64>], function: &Function) -> llvm::Builder {
        let info = self.queue_function(FunctionKey::from_function(function));

        for arg in args.iter().take(info.offset) {
            builder = builder
                .build_call(&self.push_fn, (*arg,))
                .1;
        }

        builder.build_call(&info.value, ()).1
    }

    fn build_immediate_conditional_call_instr(
        &mut self,
        block_builder: llvm::Builder,
        args: &[llvm::Value<i64>],
        function_info: FunctionInfo,
        value: &Value,
        x: &Function,
        y: &Function,
    ) -> llvm::Builder {
        let x = self.queue_function(FunctionKey::from_function(x));
        let y = self.queue_function(FunctionKey::from_function(y));

        let cont = function_info.value.add_block("cont");

        let x_block = function_info.value.add_block("x");

        let mut builder = x_block.build();

        for arg in args.iter().take(x.offset).rev() {
            builder = builder
                .build_call(&self.push_fn, (*arg,))
                .1;
        }

        builder
            .build_call(&x.value, ())
            .1
            .build_jump(&cont);

        let y_block = function_info.value.add_block("y");

        let mut builder = y_block.build();

        for arg in args.iter().take(y.offset).rev() {
            builder = builder
                .build_call(&self.push_fn, (*arg,))
                .1;
        }

        builder
            .build_call(&y.value, ())
            .1
            .build_jump(&cont);

        let (value, builder) = self.build_value(block_builder, args, value);
        builder.build_conditional_jump(&value, &y_block, &x_block);

        cont.build()
    }

    fn compile_block(&mut self, block_index: usize) {
        let function_info = self.functions[&FunctionKey::Block(block_index)];
        let entry = function_info.value.add_block("entry");

        let mut block_builder = entry.build();

        let block = self.ir[block_index].clone();

        let (args, builder) =
            (0..block.args + block.offset).fold((Vec::with_capacity(block.args + block.offset), block_builder), |(mut vec, builder), _| {
                let (value, builder) = builder.build_call(&self.pop_fn, ());
                vec.push(value);
                (vec, builder)
            });

        block_builder = builder;

        for instr in &block.instrs {
            block_builder = match &instr.data {
                Instr::Command(command) => self.build_command_instr(block_builder, *command),
                Instr::Push(ref value) => self.build_push_instr(block_builder, &args, value),
                Instr::ImmediateCall(function) => self.build_immediate_call_instr(block_builder, &args, function),
                Instr::ImmediateConditionalCall(value, x, y) => {
                    self.build_immediate_conditional_call_instr(block_builder, &args, function_info, value, x, y)
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
            .values()
            .map(|f| {
                let block = self
                    .call_fn
                    .add_block(format!("block_{}", f.index));
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
        self.compile_closure_push();
        self.compile_closure_offset();
        self.queue_function(FunctionKey::Block(0));

        while let Some(function) = self.queue.pop() {
            self.compile_function(function);
        }

        self.compile_call();
        self.compile_main();

        println!("{:?}", self.module);
    }
}

fn build_tri_op(
    builder: llvm::Builder,
    tri_op: TriOp,
    x: llvm::Value<i64>,
    y: llvm::Value<i64>,
    z: llvm::Value<i64>,
) -> (llvm::Value<i64>, llvm::Builder) {
    match tri_op {
        TriOp::IfThenElse => builder.build_conditional_value(&x, &y, &z),
    }
}

fn build_bin_op(builder: llvm::Builder, bin_op: BinOp, x: llvm::Value<i64>, y: llvm::Value<i64>) -> (llvm::Value<i64>, llvm::Builder) {
    match bin_op {
        BinOp::Plus => builder.build_add(&x, &y),
        BinOp::Minus => builder.build_sub(&x, &y),
        BinOp::Multiply => builder.build_mul(&x, &y),
        BinOp::Divide => builder.build_sdiv(&x, &y),
        BinOp::Equals => builder.build_eq(&x, &y),
        BinOp::GreaterThan => builder.build_gt(&x, &y),
        BinOp::LessThan => builder.build_lt(&x, &y),
        BinOp::Random => unimplemented!(),
    }
}
