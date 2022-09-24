use std::collections::VecDeque;

use crate::parser::ast;

use super::ir;

pub struct QueuedBlock {
    block: ast::Block,
    parent: Option<usize>,
}

pub struct State {
    queue: VecDeque<QueuedBlock>,
    ir: Vec<ir::Block>,
}

impl QueuedBlock {
    pub fn new(block: ast::Block) -> Self {
        Self { block, parent: None }
    }

    pub fn new_with_parent(block: ast::Block, parent: usize) -> Self {
        Self { block, parent: Some(parent) }
    }
}

impl State {
    pub fn new(top_level: ast::Block) -> Self {
        Self {
            queue: VecDeque::from([QueuedBlock::new(top_level)]),
            ir: Vec::new(),
        }
    }

    fn queue_block(&mut self, block: ast::Block, parent: usize) -> usize {
        self.queue.push_front(QueuedBlock::new_with_parent(block, parent));
        self.queue.len() + self.ir.len()
    }

    fn analyse_block(&mut self, block: QueuedBlock, index: usize) -> ir::Block {
        let mut ir = ir::Block::new(block.block.args, block.parent.map(|index| &self.ir[index]));

        for (name, symbol) in block.block.symbols {
            let symbol = match symbol.value.data {
                ast::SymbolValue::Number(value) => ir::Value::Number(value),
                ast::SymbolValue::Block(block) => ir::Value::Block(self.queue_block(block, index)),
                ast::SymbolValue::Builtin(builtin) => ir::Value::Builtin(builtin),
            };

            ir.push_symbol(name, symbol);
        }

        for instr in block.block.instrs.into_iter().rev() {
            let instr = match instr.data {
                ast::Instruction::Call => ir::Instr::Call,
                ast::Instruction::Push(value) => {
                    let value = match value {
                        ast::InstrValue::Number(value) => ir::Value::Number(value),
                        ast::InstrValue::Ident(ref name) => ir.lookup_symbol(name).expect("TODO: undefined symbol"),
                        ast::InstrValue::Block(block) => ir::Value::Block(self.queue_block(block, index)),
                        ast::InstrValue::Builtin(builtin) => ir::Value::Builtin(builtin),
                    };
                    ir::Instr::Push(value)
                }
            };

            ir.push_instr(instr);
        }

        ir
    }

    pub fn analyse(mut self) -> Vec<ir::Block> {
        while let Some(block) = self.queue.pop_back() {
            let ir = self.analyse_block(block, self.ir.len());
            self.ir.push(ir);
        }

        self.ir
    }
}
