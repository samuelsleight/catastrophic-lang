#![cfg(test)]

use catastrophic_core::span::{Location, Span};

use crate::analyser::error::CompileError;

use super::*;

fn span<D>(data: D, from_line: usize, from_col: usize, to_line: usize, to_col: usize) -> Span<D> {
    Span::new(Location::new(from_line, from_col), Location::new(to_line, to_col), data)
}

fn analyser_test(input: ast::Block, expected: &[hir::Block]) {
    let result = Analyser::analyse_ast(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn analyse_empty_ast() {
    let input = ast::Block::no_args();

    let expected = hir::Block::new(vec![], None);

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_call_command() {
    let mut input = ast::Block::no_args();
    input.push_instruction(span(ast::Instruction::Command(ast::Command::Call), 0, 0, 0, 2));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_instr(span(hir::Instr::Command(hir::Command::Call), 0, 0, 0, 2));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_char_input_command() {
    let mut input = ast::Block::no_args();
    input.push_instruction(span(ast::Instruction::Command(ast::Command::InputChar), 0, 0, 0, 1));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_instr(span(hir::Instr::Command(hir::Command::InputChar), 0, 0, 0, 1));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_char_output_command() {
    let mut input = ast::Block::no_args();
    input.push_instruction(span(ast::Instruction::Command(ast::Command::OutputChar), 0, 0, 0, 1));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_instr(span(hir::Instr::Command(hir::Command::OutputChar), 0, 0, 0, 1));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_num_input_command() {
    let mut input = ast::Block::no_args();
    input.push_instruction(span(ast::Instruction::Command(ast::Command::InputNumber), 0, 0, 0, 1));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_instr(span(hir::Instr::Command(hir::Command::InputNumber), 0, 0, 0, 1));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_num_output_command() {
    let mut input = ast::Block::no_args();
    input.push_instruction(span(ast::Instruction::Command(ast::Command::OutputNumber), 0, 0, 0, 1));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_instr(span(hir::Instr::Command(hir::Command::OutputNumber), 0, 0, 0, 1));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_number_value() {
    let mut input = ast::Block::no_args();
    input.push_instruction(span(ast::Instruction::Push(ast::InstrValue::Number(13579)), 0, 0, 0, 5));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_instr(span(hir::Instr::Push(hir::Value::Number(13579)), 0, 0, 0, 5));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_builtin_plus() {
    let mut input = ast::Block::no_args();
    input.push_instruction(span(ast::Instruction::Push(ast::InstrValue::Builtin(ast::Builtin::Plus)), 0, 0, 0, 1));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_instr(span(
        hir::Instr::Push(hir::Value::Function(hir::Function::Builtin(hir::Builtin::Plus))),
        0,
        0,
        0,
        1,
    ));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_builtin_minus() {
    let mut input = ast::Block::no_args();
    input.push_instruction(span(ast::Instruction::Push(ast::InstrValue::Builtin(ast::Builtin::Minus)), 0, 0, 0, 1));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_instr(span(
        hir::Instr::Push(hir::Value::Function(hir::Function::Builtin(hir::Builtin::Minus))),
        0,
        0,
        0,
        1,
    ));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_builtin_less_than() {
    let mut input = ast::Block::no_args();
    input.push_instruction(span(ast::Instruction::Push(ast::InstrValue::Builtin(ast::Builtin::LessThan)), 0, 0, 0, 1));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_instr(span(
        hir::Instr::Push(hir::Value::Function(hir::Function::Builtin(hir::Builtin::LessThan))),
        0,
        0,
        0,
        1,
    ));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_builtin_greater_than() {
    let mut input = ast::Block::no_args();
    input.push_instruction(span(
        ast::Instruction::Push(ast::InstrValue::Builtin(ast::Builtin::GreaterThan)),
        0,
        0,
        0,
        1,
    ));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_instr(span(
        hir::Instr::Push(hir::Value::Function(hir::Function::Builtin(hir::Builtin::GreaterThan))),
        0,
        0,
        0,
        1,
    ));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_builtin_if_then_else() {
    let mut input = ast::Block::no_args();
    input.push_instruction(span(
        ast::Instruction::Push(ast::InstrValue::Builtin(ast::Builtin::IfThenElse)),
        0,
        0,
        0,
        1,
    ));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_instr(span(
        hir::Instr::Push(hir::Value::Function(hir::Function::Builtin(hir::Builtin::IfThenElse))),
        0,
        0,
        0,
        1,
    ));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_symbol_number() {
    let mut input = ast::Block::no_args();
    input
        .with_symbol("sym".to_string())
        .or_insert_with(|| ast::Symbol::new(span((), 0, 0, 0, 3), span(ast::SymbolValue::Number(24680), 0, 4, 0, 9)));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_symbol("sym".to_string(), hir::Value::Number(24680));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_symbol_builtin() {
    let mut input = ast::Block::no_args();
    input
        .with_symbol("sym".to_string())
        .or_insert_with(|| ast::Symbol::new(span((), 0, 0, 0, 3), span(ast::SymbolValue::Builtin(ast::Builtin::Equals), 0, 4, 0, 5)));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_symbol("sym".to_string(), hir::Value::Function(hir::Function::Builtin(hir::Builtin::Equals)));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_symbol_block() {
    let block = ast::Block::no_args();

    let mut input = ast::Block::no_args();
    input
        .with_symbol("sym".to_string())
        .or_insert_with(|| ast::Symbol::new(span((), 0, 0, 0, 3), span(ast::SymbolValue::Block(block), 0, 4, 0, 6)));

    let mut expected1 = hir::Block::new(vec![], None);
    expected1.push_symbol("sym".to_string(), hir::Value::Function(hir::Function::Block(1)));

    let mut expected2 = hir::Block::new(vec![], None);
    expected2.push_symbol("sym".to_string(), hir::Value::Function(hir::Function::Block(1)));

    analyser_test(input, &[expected1, expected2]);
}

#[test]
fn analyse_empty_block() {
    let block = ast::Block::no_args();

    let mut input = ast::Block::no_args();
    input.push_instruction(span(ast::Instruction::Push(ast::InstrValue::Block(block)), 0, 0, 0, 2));

    let mut expected1 = hir::Block::new(vec![], None);
    expected1.push_instr(span(hir::Instr::Push(hir::Value::Function(hir::Function::Block(1))), 0, 0, 0, 2));

    let expected2 = hir::Block::new(vec![], Some(&expected1));

    analyser_test(input, &[expected1, expected2]);
}

#[test]
fn analyse_symbol_and_valid_ident() {
    let mut input = ast::Block::no_args();
    input
        .with_symbol("sym".to_string())
        .or_insert_with(|| ast::Symbol::new(span((), 0, 0, 0, 3), span(ast::SymbolValue::Number(24680), 0, 4, 0, 9)));
    input.push_instruction(span(ast::Instruction::Push(ast::InstrValue::Ident("sym".to_string())), 1, 0, 1, 3));

    let mut expected = hir::Block::new(vec![], None);
    expected.push_symbol("sym".to_string(), hir::Value::Number(24680));
    expected.push_instr(span(hir::Instr::Push(hir::Value::Number(24680)), 1, 0, 1, 3));

    analyser_test(input, &[expected]);
}

#[test]
fn analyse_arg_block() {
    let block = ast::Block::with_args(vec![span("arg".to_string(), 0, 0, 0, 3)]);

    let mut input = ast::Block::no_args();
    input.push_instruction(span(ast::Instruction::Push(ast::InstrValue::Block(block)), 0, 0, 2, 1));

    let mut expected1 = hir::Block::new(vec![], None);
    expected1.push_instr(span(hir::Instr::Push(hir::Value::Function(hir::Function::Block(1))), 0, 0, 2, 1));

    let expected2 = hir::Block::new(vec![span("arg".to_string(), 0, 0, 0, 3)], Some(&expected1));

    analyser_test(input, &[expected1, expected2]);
}

#[test]
fn analyse_arg_block_and_valid_ident() {
    let mut block = ast::Block::with_args(vec![span("arg".to_string(), 0, 0, 0, 3)]);
    block.push_instruction(span(ast::Instruction::Push(ast::InstrValue::Ident("arg".to_string())), 1, 1, 1, 4));

    let mut input = ast::Block::no_args();
    input.push_instruction(span(ast::Instruction::Push(ast::InstrValue::Block(block)), 0, 0, 2, 1));

    let mut expected1 = hir::Block::new(vec![], None);
    expected1.push_instr(span(hir::Instr::Push(hir::Value::Function(hir::Function::Block(1))), 0, 0, 2, 1));

    let mut expected2 = hir::Block::new(vec![span("arg".to_string(), 0, 0, 0, 3)], Some(&expected1));
    expected2.push_instr(span(hir::Instr::Push(hir::Value::Arg(0)), 1, 1, 1, 4));

    analyser_test(input, &[expected1, expected2]);
}

#[test]
fn analyse_block_and_valid_parent_symbol() {
    let mut block = ast::Block::no_args();
    block.push_instruction(span(ast::Instruction::Push(ast::InstrValue::Ident("sym".to_string())), 3, 1, 3, 4));

    let mut input = ast::Block::no_args();
    input
        .with_symbol("sym".to_string())
        .or_insert_with(|| ast::Symbol::new(span((), 0, 0, 0, 3), span(ast::SymbolValue::Number(24680), 0, 4, 0, 9)));
    input.push_instruction(span(ast::Instruction::Push(ast::InstrValue::Block(block)), 2, 0, 4, 1));

    let mut expected1 = hir::Block::new(vec![], None);
    expected1.push_symbol("sym".to_string(), hir::Value::Number(24680));
    expected1.push_instr(span(hir::Instr::Push(hir::Value::Function(hir::Function::Block(1))), 2, 0, 4, 1));

    let mut expected2 = hir::Block::new(vec![], Some(&expected1));
    expected2.push_instr(span(hir::Instr::Push(hir::Value::Number(24680)), 3, 1, 3, 4));

    analyser_test(input, &[expected1, expected2]);
}

#[test]
fn analyse_undefined_symbol_fails() {
    let mut input = ast::Block::no_args();
    input.push_instruction(span(ast::Instruction::Push(ast::InstrValue::Ident("sym".to_string())), 0, 0, 0, 3));

    let result = Analyser::analyse_ast(input);

    assert_eq!(
        result,
        Err(CompileErrors {
            errors: vec![CompileError::UndefinedSymbolError(span("sym".to_string(), 0, 0, 0, 3))]
        })
    );
}
