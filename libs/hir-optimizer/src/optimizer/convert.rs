use catastrophic_core::span::Span;
use catastrophic_hir::hir;
use catastrophic_mir::mir;

fn convert_instr(higher_ir: Span<hir::Instr>) -> Span<mir::Instr> {
    let span = higher_ir.swap(());
    let middle_ir = match higher_ir.data {
        hir::Instr::Command(command) => mir::Instr::Command(command),
        hir::Instr::Push(value) => mir::Instr::Push(match value {
            hir::Value::Arg(arg) => mir::Value::Arg(arg),
            hir::Value::Number(value) => mir::Value::Number(value),
            hir::Value::Function(function) => mir::Value::Function(match function {
                hir::Function::Block(block) => mir::Function::Block(block),
                hir::Function::Builtin(builtin) => match builtin {
                    hir::Builtin::Plus => mir::Function::BinOp(mir::BinOp::Plus),
                    hir::Builtin::Minus => mir::Function::BinOp(mir::BinOp::Minus),
                    hir::Builtin::Multiply => mir::Function::BinOp(mir::BinOp::Multiply),
                    hir::Builtin::Divide => mir::Function::BinOp(mir::BinOp::Divide),
                    hir::Builtin::Equals => mir::Function::BinOp(mir::BinOp::Equals),
                    hir::Builtin::GreaterThan => mir::Function::BinOp(mir::BinOp::GreaterThan),
                    hir::Builtin::LessThan => mir::Function::BinOp(mir::BinOp::LessThan),
                    hir::Builtin::Random => mir::Function::BinOp(mir::BinOp::Random),
                    hir::Builtin::IfThenElse => mir::Function::TriOp(mir::TriOp::IfThenElse),
                },
            }),
        }),
    };

    span.swap(middle_ir)
}

fn convert_block(hir: hir::Block) -> mir::Block {
    mir::Block {
        offset: hir.offset,
        args: hir.args,
        instrs: hir
            .instrs
            .into_iter()
            .map(convert_instr)
            .collect(),
        name: hir.name,
    }
}

pub fn convert_blocks(hir: Vec<hir::Block>) -> Vec<mir::Block> {
    hir.into_iter()
        .map(convert_block)
        .collect()
}
