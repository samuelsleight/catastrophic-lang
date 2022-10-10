use catastrophic_core::span::Span;
use catastrophic_hir::hir;
use catastrophic_mir::mir;

fn convert_instr(hir: Span<hir::Instr>) -> Span<mir::Instr> {
    let span = hir.swap(());
    let mir = match hir.data {
        hir::Instr::Command(command) => mir::Instr::Command(command),
        hir::Instr::Push(value) => mir::Instr::Push(value),
    };

    span.swap(mir)
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
    }
}

pub fn convert_blocks(hir: Vec<hir::Block>) -> Vec<mir::Block> {
    hir.into_iter()
        .map(convert_block)
        .collect()
}
