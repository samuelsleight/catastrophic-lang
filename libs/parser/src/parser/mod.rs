use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
    path::Path,
};

use catastrophic_ast::ast;
use ruinous::parser::{Error as RuinousError, Parser as RuinousParser};

use crate::lexer::State as Lexer;

use self::state::State;

pub type Error = RuinousError<Lexer, State>;

mod error;
mod state;

pub struct Parser<R> {
    parser: RuinousParser<R>,
}

impl Parser<BufReader<File>> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let parser = RuinousParser::from_file(path)?;
        Ok(Self { parser })
    }
}

impl<'a> Parser<Cursor<&'a str>> {
    pub fn from_str(input: &'a str) -> Self {
        let parser = RuinousParser::from_str(input);
        Self { parser }
    }
}

impl<R: BufRead> Parser<R> {
    pub fn parse(self) -> Result<ast::Block, Error> {
        self.parser
            .parse(Lexer::new(), State::new())
    }
}

#[cfg(test)]
mod test {
    use std::collections::hash_map::Entry;

    use catastrophic_ast::ast::{Block, Builtin, Command, InstrValue, Instruction, Symbol, SymbolValue};
    use catastrophic_core::{
        defines::ValueType,
        span::{Location, Span},
    };

    use super::*;

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
        let parser = Parser::from_str(input);
        let result = parser.parse().unwrap();

        assert_eq!(&result, expected);
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
}
