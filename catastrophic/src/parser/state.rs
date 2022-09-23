use crate::{lexer::Token, span::Span};

use super::{
    ast::{Builtin, InstrValue, Instruction, SymbolValue},
    error::{ParseError, ParseErrors},
    Block,
};

#[derive(Debug, Copy, Clone)]
enum BlockTermination {
    Eof,
    Curly(Span<()>),
}

#[derive(Debug, Clone)]
enum StackItem {
    OpenBlock,
    Call,
    Ident(String),
    Number(u64),
    Builtin(Builtin),
    Label(String),
    Arg(String),
    Block(Block),
}

pub struct State {
    stack: Vec<Span<StackItem>>,
    blocks: Vec<Block>,
    errors: Vec<ParseError>,
}

impl State {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            blocks: vec![Block::no_args()],
            errors: Vec::new(),
        }
    }

    fn process_ident(&mut self, ident: String, span: Span<()>) {
        self.stack.push(span.swap(StackItem::Ident(ident)));
    }

    fn process_parens(&mut self, span: Span<()>) {
        self.stack.push(span.swap(StackItem::Call));
    }

    fn process_number(&mut self, value: u64, span: Span<()>) {
        match self.stack.pop() {
            Some(stack_item) => {
                let item_span = stack_item.swap(());
                match stack_item.data {
                    StackItem::Label(ident) => {
                        self.push_symbol(item_span.swap(ident), span.swap(SymbolValue::Number(value)));
                    }
                    other => {
                        self.stack.push(item_span.swap(other));
                        self.stack.push(span.swap(StackItem::Number(value)));
                    }
                }
            }
            None => self.stack.push(span.swap(StackItem::Number(value))),
        }
    }

    fn process_builtin(&mut self, builtin: Builtin, span: Span<()>) {
        match self.stack.pop() {
            Some(stack_item) => {
                let item_span = stack_item.swap(());
                match stack_item.data {
                    StackItem::Label(ident) => self.push_symbol(item_span.swap(ident), span.swap(SymbolValue::Builtin(builtin))),
                    other => {
                        self.stack.push(item_span.swap(other));
                        self.stack.push(span.swap(StackItem::Builtin(builtin)));
                    }
                }
            }
            None => self.stack.push(span.swap(StackItem::Builtin(builtin))),
        }
    }

    fn process_arrow(&mut self, span: Span<()>) {
        match self.stack.pop() {
            Some(stack_item) => {
                let item_span = stack_item.swap(());
                match stack_item.data {
                    StackItem::Ident(ident) => self.stack.push(item_span.swap(StackItem::Arg(ident))),
                    _ => self.errors.push(ParseError::ArrowWithoutArg(span)),
                }
            }
            None => self.errors.push(ParseError::ArrowWithoutArg(span)),
        }
    }

    fn process_colon(&mut self, span: Span<()>) {
        match self.stack.pop() {
            Some(stack_item) => {
                let item_span = stack_item.swap(());
                match stack_item.data {
                    StackItem::Ident(ident) => self.stack.push(item_span.swap(StackItem::Label(ident))),
                    _ => self.errors.push(ParseError::LabelWithoutName(span)),
                }
            }
            None => self.errors.push(ParseError::LabelWithoutName(span)),
        }
    }

    fn process_open_block(&mut self, span: Span<()>) {
        let mut args = Vec::new();

        while let Some(stack_item) = self.stack.pop() {
            let item_span = stack_item.swap(());
            match stack_item.data {
                StackItem::Arg(arg) => args.push(item_span.swap(arg)),
                other => {
                    self.stack.push(item_span.swap(other));
                    break;
                }
            }
        }

        self.blocks.push(Block::with_args(args));
        self.stack.push(span.swap(StackItem::OpenBlock));
    }

    fn process_close_block(&mut self, span: Span<()>) {
        let (block, termination) = self.terminate_block();

        let start_span = match termination {
            BlockTermination::Curly(span) => span,
            BlockTermination::Eof => {
                self.blocks.push(block);
                return self.errors.push(ParseError::BlockClosedWithoutOpening(span));
            }
        };

        let span = Span::new(start_span.start, span.end, ());

        match self.stack.pop() {
            Some(stack_item) => {
                let item_span = stack_item.swap(());
                match stack_item.data {
                    StackItem::Label(ident) => self.push_symbol(item_span.swap(ident), span.swap(SymbolValue::Block(block))),
                    other => {
                        self.stack.push(item_span.swap(other));
                        self.stack.push(span.swap(StackItem::Block(block)));
                    }
                }
            }
            None => self.stack.push(span.swap(StackItem::Block(block))),
        }
    }

    fn push_symbol(&mut self, name: Span<String>, value: Span<SymbolValue>) {
        if let Err(e) = self.blocks.last_mut().unwrap().add_symbol(name, value) {
            self.errors.push(e);
        }
    }

    fn terminate_block(&mut self) -> (Block, BlockTermination) {
        let mut block = self.blocks.pop().unwrap();

        while let Some(stack_item) = self.stack.pop() {
            let item_span = stack_item.swap(());
            match stack_item.data {
                StackItem::OpenBlock => return (block, BlockTermination::Curly(item_span)),
                StackItem::Call => block.push_instruction(item_span.swap(Instruction::Call)),
                StackItem::Ident(ident) => block.push_instruction(item_span.swap(Instruction::Push(InstrValue::Ident(ident)))),
                StackItem::Number(value) => block.push_instruction(item_span.swap(Instruction::Push(InstrValue::Number(value)))),
                StackItem::Builtin(builtin) => block.push_instruction(item_span.swap(Instruction::Push(InstrValue::Builtin(builtin)))),
                StackItem::Block(value) => block.push_instruction(item_span.swap(Instruction::Push(InstrValue::Block(value)))),
                StackItem::Label(_) => self.errors.push(ParseError::LabelWithoutValue(item_span)),
                StackItem::Arg(_) => self.errors.push(ParseError::ArrowWithoutBlock(item_span)),
            }
        }

        (block, BlockTermination::Eof)
    }

    pub fn process(&mut self, token: Span<Token>) {
        let span = token.swap(());

        match token.data {
            Token::Ident(ident) => self.process_ident(ident, span),
            Token::Integer(value) => self.process_number(value, span),
            Token::Arrow => self.process_arrow(span),
            Token::Parens => self.process_parens(span),
            Token::Plus => self.process_builtin(Builtin::Plus, span),
            Token::Minus => self.process_builtin(Builtin::Minus, span),
            Token::Equals => self.process_builtin(Builtin::Equals, span),
            Token::GreaterThan => todo!("Unsupported >"),
            Token::Colon => self.process_colon(span),
            Token::Question => self.process_builtin(Builtin::IfThenElse, span),
            Token::LParen => todo!("Unsupported ("),
            Token::RParen => todo!("Unsupported )"),
            Token::LCurly => self.process_open_block(span),
            Token::RCurly => self.process_close_block(span),
            Token::Unexpected(c) => todo!("Error! Unexpected character {}", c),
        }
    }

    pub fn finish(mut self) -> Result<Block, ParseErrors> {
        let (block, termination) = self.terminate_block();

        if let BlockTermination::Curly(span) = termination {
            self.errors.push(ParseError::BlockWithoutClosing(span));
        }

        if self.errors.is_empty() {
            Ok(block)
        } else {
            Err(self.errors.into())
        }
    }
}