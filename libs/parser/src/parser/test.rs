#![cfg(test)]

use std::collections::hash_map::Entry;

use catastrophic_ast::ast::{Block, Builtin, Command, InstrValue, Instruction, Symbol, SymbolValue};
use catastrophic_core::{
    defines::ValueType,
    span::{Location, Span},
};

use crate::lexer::error::LexError;

use super::{error::ParseError, *};

fn span<D>(data: D, from_line: usize, from_col: usize, to_line: usize, to_col: usize) -> Span<D> {
    Span::new(Location::new(from_line, from_col), Location::new(to_line, to_col), data)
}

fn block(mut block: Block, f: impl FnOnce(&mut Block)) -> Block {
    f(&mut block);
    block
}

fn symbol(block: &mut Block, name: Span<String>, value: Span<SymbolValue>) {
    let name_span = name.swap(());

    match block.with_symbol(name.data) {
        Entry::Occupied(_) => panic!("Unexpected existing symbol"),
        Entry::Vacant(entry) => entry.insert(Symbol::new(name_span, value)),
    };
}

fn parser_test(input: &str, expected: &Block) {
    let parser = Parser::with_str(input);
    let result = parser.parse().unwrap();

    assert_eq!(&result.ast, expected);
}

macro_rules! test_cases {
        ($($name:ident($input:expr, $expected:expr))+) => {
            test_cases!(expand_single $(($name, $input, $expected))+);
        };

        (expand_single $(($name:ident, $input:expr, $expected:expr))+) => {
            $(
                paste::paste! {
                    #[test]
                    fn [<parse_ $name>]() {
                        parser_test($input, $expected);
                    }
                }
            )+
        };
    }

test_cases! {
    empty_input("", &Block::no_args())
    empty_block(
        "{}",
        &block(
            Block::no_args(),
            |block| block.push_instruction(span(Instruction::Push(InstrValue::Block(Block::no_args())), 0, 0, 0, 2))))
    single_arg_block(
        "a -> {}",
        &block(
            Block::no_args(),
            |block| block.push_instruction(span(Instruction::Push(InstrValue::Block(Block::with_args(vec![span("a".to_owned(), 0, 0, 0, 1)]))), 0, 5, 0, 7))))
    multi_arg_block(
        "a2 -> a1 -> {}",
        &block(
            Block::no_args(),
            |block| block.push_instruction(span(Instruction::Push(InstrValue::Block(Block::with_args(vec![span("a1".to_owned(), 0, 6, 0, 8), span("a2".to_owned(), 0, 0, 0, 2)]))), 0, 12, 0, 14))))
    single_string(
        "\"hello\"",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Push(InstrValue::Number('h' as ValueType)), 0, 0, 0, 7));
                block.push_instruction(span(Instruction::Push(InstrValue::Number('e' as ValueType)), 0, 0, 0, 7));
                block.push_instruction(span(Instruction::Push(InstrValue::Number('l' as ValueType)), 0, 0, 0, 7));
                block.push_instruction(span(Instruction::Push(InstrValue::Number('l' as ValueType)), 0, 0, 0, 7));
                block.push_instruction(span(Instruction::Push(InstrValue::Number('o' as ValueType)), 0, 0, 0, 7));
            }))
    single_emoji_string(
        "\"🐉\"",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Push(InstrValue::Number(240 as ValueType)), 0, 0, 0, 3));
                block.push_instruction(span(Instruction::Push(InstrValue::Number(159 as ValueType)), 0, 0, 0, 3));
                block.push_instruction(span(Instruction::Push(InstrValue::Number(144 as ValueType)), 0, 0, 0, 3));
                block.push_instruction(span(Instruction::Push(InstrValue::Number(137 as ValueType)), 0, 0, 0, 3));
            }))
    single_number(
        "12345",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Push(InstrValue::Number(12345)), 0, 0, 0, 5));
            }))
    single_ident(
        "hello",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Push(InstrValue::Ident("hello".to_owned())), 0, 0, 0, 5));
            }))
    single_call_command(
        "()",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Command(Command::Call), 0, 0, 0, 2));
            }))
    single_char_in_command(
        "~",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Command(Command::InputChar), 0, 0, 0, 1));
            }))
    single_char_out_command(
        ",",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Command(Command::OutputChar), 0, 0, 0, 1));
            }))
    single_num_in_command(
        "&",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Command(Command::InputNumber), 0, 0, 0, 1));
            }))
    single_num_out_command(
        ".",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Command(Command::OutputNumber), 0, 0, 0, 1));
            }))
    single_plus_builtin(
        "+",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Push(InstrValue::Builtin(Builtin::Plus)), 0, 0, 0, 1));
            }))
    single_minus_builtin(
        "-",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Push(InstrValue::Builtin(Builtin::Minus)), 0, 0, 0, 1));
            }))
    single_mul(
        "*",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Push(InstrValue::Builtin(Builtin::Multiply)), 0, 0, 0, 1));
            }))
    single_div(
        "/",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Push(InstrValue::Builtin(Builtin::Divide)), 0, 0, 0, 1));
            }))
    single_equals_builtin(
        "=",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Push(InstrValue::Builtin(Builtin::Equals)), 0, 0, 0, 1));
            }))
    single_lt_builtin(
        "<",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Push(InstrValue::Builtin(Builtin::LessThan)), 0, 0, 0, 1));
            }))
    single_gt_builtin(
        ">",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Push(InstrValue::Builtin(Builtin::GreaterThan)), 0, 0, 0, 1));
            }))
    single_random_builtin(
        "!",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Push(InstrValue::Builtin(Builtin::Random)), 0, 0, 0, 1));
            }))
    single_ite_builtin(
        "?",
        &block(
            Block::no_args(),
            |block| {
                block.push_instruction(span(Instruction::Push(InstrValue::Builtin(Builtin::IfThenElse)), 0, 0, 0, 1));
            }))
    named_empty_block(
        "fn: {}",
        &block(
            Block::no_args(), |block| {
                symbol(block, span("fn".to_owned(), 0, 0, 0, 2), span(SymbolValue::Block(Block::no_args()), 0, 4, 0, 6))
            }))
    named_number(
        "num: 12345",
        &block(
            Block::no_args(), |block| {
                symbol(block, span("num".to_owned(), 0, 0, 0, 3), span(SymbolValue::Number(12345), 0, 5, 0, 10))
            }))
    named_builtin(
        "add: +",
        &block(
            Block::no_args(), |block| {
                symbol(block, span("add".to_owned(), 0, 0, 0, 3), span(SymbolValue::Builtin(Builtin::Plus), 0, 5, 0, 6))
            }))
    blocked_block(
        "{\n\t{}\n}",
        &block(
            Block::no_args(), |outer| {
                let inner = block(Block::no_args(), |block| block.push_instruction(span(Instruction::Push(InstrValue::Block(Block::no_args())), 1, 1, 1, 3)));
                outer.push_instruction(span(Instruction::Push(InstrValue::Block(inner)), 0, 0, 2, 1));
            }))
    blocked_number(
        "{\n\t13579\n}",
        &block(
            Block::no_args(), |outer| {
                let inner = block(Block::no_args(), |block| block.push_instruction(span(Instruction::Push(InstrValue::Number(13579)), 1, 1, 1, 6)));
                outer.push_instruction(span(Instruction::Push(InstrValue::Block(inner)), 0, 0, 2, 1));
            }))
    blocked_ident(
        "{\n\thello\n}",
        &block(
            Block::no_args(), |outer| {
                let inner = block(Block::no_args(), |block| block.push_instruction(span(Instruction::Push(InstrValue::Ident("hello".to_owned())), 1, 1, 1, 6)));
                outer.push_instruction(span(Instruction::Push(InstrValue::Block(inner)), 0, 0, 2, 1));
            }))
    blocked_string(
        "{\n\t\"hello\"\n}",
        &block(
            Block::no_args(), |outer| {
                let inner = block(
                    Block::no_args(), |block| {
                        block.push_instruction(span(Instruction::Push(InstrValue::Number('h' as ValueType)), 1, 1, 1, 8));
                        block.push_instruction(span(Instruction::Push(InstrValue::Number('e' as ValueType)), 1, 1, 1, 8));
                        block.push_instruction(span(Instruction::Push(InstrValue::Number('l' as ValueType)), 1, 1, 1, 8));
                        block.push_instruction(span(Instruction::Push(InstrValue::Number('l' as ValueType)), 1, 1, 1, 8));
                        block.push_instruction(span(Instruction::Push(InstrValue::Number('o' as ValueType)), 1, 1, 1, 8));
                    });

                outer.push_instruction(span(Instruction::Push(InstrValue::Block(inner)), 0, 0, 2, 1));
            }))
    blocked_command(
        "{\n\t()\n}",
        &block(
            Block::no_args(), |outer| {
                let inner = block(Block::no_args(), |block| block.push_instruction(span(Instruction::Command(Command::Call), 1, 1, 1, 3)));
                outer.push_instruction(span(Instruction::Push(InstrValue::Block(inner)), 0, 0, 2, 1));
            }))
    blocked_builtin(
        "{\n\t+\n}",
        &block(
            Block::no_args(), |outer| {
                let inner = block(Block::no_args(), |block| block.push_instruction(span(Instruction::Push(InstrValue::Builtin(Builtin::Plus)), 1, 1, 1, 2)));
                outer.push_instruction(span(Instruction::Push(InstrValue::Block(inner)), 0, 0, 2, 1));
            }))
    blocked_label(
        "{\n\tnum: 98765\n}",
        &block(
            Block::no_args(), |outer| {
                let inner = block(Block::no_args(), |block| symbol(block, span("num".to_owned(), 1, 1, 1, 4), span(SymbolValue::Number(98765), 1, 6, 1, 11)));
                outer.push_instruction(span(Instruction::Push(InstrValue::Block(inner)), 0, 0, 2, 1));
            }))
}

#[test]
fn parse_unterminated_string_fails() {
    let parser = Parser::with_str("\"hello");
    let result = parser.parse();

    if let Err(RuinousError::LexError(ruinous::lexer::Error::LexError(err))) = result {
        match err {
            LexError::UnterminatedString(s) if s == span((), 0, 0, 0, 1) => return,
            _ => (),
        }
    }

    panic!()
}

#[test]
fn parse_unexpected_char_fails() {
    let parser = Parser::with_str("(");
    let result = parser.parse();

    if let Err(err) = result {
        match err {
            RuinousError::ParseErrors(errs) if errs.errors.len() == 1 => match errs.errors[0] {
                ParseError::UnexpectedChar(s) if s == span('(', 0, 0, 0, 1) => return,
                _ => (),
            },
            _ => (),
        }
    }

    panic!()
}

#[test]
fn parse_unclosed_block_fails() {
    let parser = Parser::with_str("{");
    let result = parser.parse();

    if let Err(err) = result {
        match err {
            RuinousError::ParseErrors(errs) if errs.errors.len() == 1 => match errs.errors[0] {
                ParseError::BlockWithoutClosing(s) if s == span((), 0, 0, 0, 1) => return,
                _ => (),
            },
            _ => (),
        }
    }

    panic!()
}

#[test]
fn parse_unopened_block_fails() {
    let parser = Parser::with_str("}");
    let result = parser.parse();

    if let Err(err) = result {
        match err {
            RuinousError::ParseErrors(errs) if errs.errors.len() == 1 => match errs.errors[0] {
                ParseError::BlockClosedWithoutOpening(s) if s == span((), 0, 0, 0, 1) => return,
                _ => (),
            },
            _ => (),
        }
    }

    panic!()
}

#[test]
fn parse_solo_arrow_fails() {
    let parser = Parser::with_str("->");
    let result = parser.parse();

    if let Err(err) = result {
        match err {
            RuinousError::ParseErrors(errs) if errs.errors.len() == 1 => match errs.errors[0] {
                ParseError::ArrowWithoutArg(s) if s == span((), 0, 0, 0, 2) => return,
                _ => (),
            },
            _ => (),
        }
    }

    panic!()
}

#[test]
fn parse_value_prefixed_arrow_fails() {
    let parser = Parser::with_str("5 ->");
    let result = parser.parse();

    if let Err(err) = result {
        match err {
            RuinousError::ParseErrors(errs) if errs.errors.len() == 1 => match errs.errors[0] {
                ParseError::ArrowWithoutArg(s) if s == span((), 0, 2, 0, 4) => return,
                _ => (),
            },
            _ => (),
        }
    }

    panic!()
}

#[test]
fn parse_unblocked_arrow_fails() {
    let parser = Parser::with_str("a ->");
    let result = parser.parse();

    if let Err(err) = result {
        match err {
            RuinousError::ParseErrors(errs) if errs.errors.len() == 1 => match errs.errors[0] {
                ParseError::ArrowWithoutBlock(s) if s == span((), 0, 0, 0, 1) => return,
                _ => (),
            },
            _ => (),
        }
    }

    panic!()
}

#[test]
fn parse_solo_colon_fails() {
    let parser = Parser::with_str(":");
    let result = parser.parse();

    if let Err(err) = result {
        match err {
            RuinousError::ParseErrors(errs) if errs.errors.len() == 1 => match errs.errors[0] {
                ParseError::LabelWithoutName(s) if s == span((), 0, 0, 0, 1) => return,
                _ => (),
            },
            _ => (),
        }
    }

    panic!()
}

#[test]
fn parse_value_prefixed_name_fails() {
    let parser = Parser::with_str("5 :");
    let result = parser.parse();

    if let Err(err) = result {
        match err {
            RuinousError::ParseErrors(errs) if errs.errors.len() == 1 => match errs.errors[0] {
                ParseError::LabelWithoutName(s) if s == span((), 0, 2, 0, 3) => return,
                _ => (),
            },
            _ => (),
        }
    }

    panic!()
}

#[test]
fn parse_unvalued_label_fails() {
    let parser = Parser::with_str("bad:");
    let result = parser.parse();

    if let Err(err) = result {
        match err {
            RuinousError::ParseErrors(errs) if errs.errors.len() == 1 => match errs.errors[0] {
                ParseError::LabelWithoutValue(s) if s == span((), 0, 0, 0, 3) => return,
                _ => (),
            },
            _ => (),
        }
    }

    panic!()
}

#[test]
fn parse_duplicate_label_fails() {
    let parser = Parser::with_str("a: 5\na: 6");
    let result = parser.parse();

    if let Err(err) = result {
        match err {
            RuinousError::ParseErrors(errs) if errs.errors.len() == 1 => match errs.errors[0] {
                ParseError::DuplicateSymbolError { first, duplicate } if first == span((), 0, 0, 0, 1) && duplicate == span((), 1, 0, 1, 1) => return,
                _ => (),
            },
            _ => (),
        }
    }

    panic!()
}
