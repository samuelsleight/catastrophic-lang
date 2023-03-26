use std::collections::hash_map::Entry;

use catastrophic_ast::{
    ast::{self, Command},
    token::Token,
};
use catastrophic_core::{defines::ValueType, span::Span};
use ruinous::parser::{state::State as ParserState, ParseErrors};

use super::{
    ast::{Builtin, InstrValue, Instruction, SymbolValue},
    error::ParseError,
    output::ParseOutput,
};

#[derive(Debug, Copy, Clone)]
enum BlockTermination {
    Eof,
    Curly(Span<()>),
}

#[derive(Debug, Clone)]
enum StackItem {
    OpenBlock,
    Command(Command),
    Ident(String),
    Number(ValueType),
    Builtin(Builtin),
    Label(String),
    Arg(String),
    Comment(String),
    Block(ast::Block),
}

pub struct State {
    stack: Vec<Span<StackItem>>,
    blocks: Vec<ast::Block>,
    errors: Vec<ParseError>,

    permissive: bool,
}

impl State {
    pub fn new(permissive: bool) -> Self {
        Self {
            stack: Vec::new(),
            blocks: vec![ast::Block::no_args()],
            errors: Vec::new(),

            permissive,
        }
    }

    fn process_ident(&mut self, ident: String, span: Span<()>) {
        self.stack
            .push(span.swap(StackItem::Ident(ident)));
    }

    fn process_string(&mut self, string: String, span: Span<()>) {
        for char in string.chars().rev() {
            self.stack
                .push(span.swap(StackItem::Number(char as ValueType)))
        }
    }

    fn process_command(&mut self, command: Command, span: Span<()>) {
        self.stack
            .push(span.swap(StackItem::Command(command)));
    }

    fn process_comment(&mut self, comment: String, span: Span<()>) {
        self.stack
            .push(span.swap(StackItem::Comment(comment)))
    }

    fn process_number(&mut self, value: ValueType, span: Span<()>) {
        match self.stack.pop() {
            Some(stack_item) => {
                let item_span = stack_item.swap(());
                match stack_item.data {
                    StackItem::Label(ident) => {
                        self.push_symbol(item_span.swap(ident), span.swap(SymbolValue::Number(value)));
                    }
                    other => {
                        self.stack.push(item_span.swap(other));
                        self.stack
                            .push(span.swap(StackItem::Number(value)));
                    }
                }
            }
            None => self
                .stack
                .push(span.swap(StackItem::Number(value))),
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
                        self.stack
                            .push(span.swap(StackItem::Builtin(builtin)));
                    }
                }
            }
            None => self
                .stack
                .push(span.swap(StackItem::Builtin(builtin))),
        }
    }

    fn process_arrow(&mut self, span: Span<()>) {
        match self.stack.pop() {
            Some(stack_item) => {
                let item_span = stack_item.swap(());
                match stack_item.data {
                    StackItem::Ident(ident) => self
                        .stack
                        .push(item_span.swap(StackItem::Arg(ident))),
                    _ => self
                        .errors
                        .push(ParseError::ArrowWithoutArg(span)),
                }
            }
            None => self
                .errors
                .push(ParseError::ArrowWithoutArg(span)),
        }
    }

    fn process_colon(&mut self, span: Span<()>) {
        match self.stack.pop() {
            Some(stack_item) => {
                let item_span = stack_item.swap(());
                match stack_item.data {
                    StackItem::Ident(ident) => self
                        .stack
                        .push(item_span.swap(StackItem::Label(ident))),
                    _ => self
                        .errors
                        .push(ParseError::LabelWithoutName(span)),
                }
            }
            None => self
                .errors
                .push(ParseError::LabelWithoutName(span)),
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

        self.blocks
            .push(ast::Block::with_args(args));
        self.stack
            .push(span.swap(StackItem::OpenBlock));
    }

    fn process_close_block(&mut self, span: Span<()>) {
        let (block, termination) = self.terminate_block();

        let start_span = match termination {
            BlockTermination::Curly(span) => span,
            BlockTermination::Eof => {
                self.blocks.push(block);
                return self
                    .errors
                    .push(ParseError::BlockClosedWithoutOpening(span));
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
                        self.stack
                            .push(span.swap(StackItem::Block(block)));
                    }
                }
            }
            None => self
                .stack
                .push(span.swap(StackItem::Block(block))),
        }
    }

    fn push_symbol(&mut self, name: Span<String>, value: Span<SymbolValue>) {
        let name_span = name.swap(());

        if let Err(e) = match self
            .blocks
            .last_mut()
            .unwrap()
            .with_symbol(name.data)
        {
            Entry::Occupied(entry) => Err(ParseError::DuplicateSymbolError {
                first: entry.get().name_span,
                duplicate: name_span,
            }),

            Entry::Vacant(entry) => {
                entry.insert(ast::Symbol::new(name_span, value));
                Ok(())
            }
        } {
            self.errors.push(e);
        }
    }

    fn terminate_block(&mut self) -> (ast::Block, BlockTermination) {
        let mut block = self.blocks.pop().unwrap();

        while let Some(stack_item) = self.stack.pop() {
            let item_span = stack_item.swap(());
            match stack_item.data {
                StackItem::OpenBlock => return (block, BlockTermination::Curly(item_span)),
                StackItem::Command(command) => block.push_instruction(item_span.swap(Instruction::Command(command))),
                StackItem::Ident(ident) => block.push_instruction(item_span.swap(Instruction::Push(InstrValue::Ident(ident)))),
                StackItem::Number(value) => block.push_instruction(item_span.swap(Instruction::Push(InstrValue::Number(value)))),
                StackItem::Builtin(builtin) => block.push_instruction(item_span.swap(Instruction::Push(InstrValue::Builtin(builtin)))),
                StackItem::Block(value) => block.push_instruction(item_span.swap(Instruction::Push(InstrValue::Block(value)))),
                StackItem::Comment(comment) => block
                    .comments
                    .push(item_span.swap(comment)),
                StackItem::Label(_) => self
                    .errors
                    .push(ParseError::LabelWithoutValue(item_span)),
                StackItem::Arg(_) => self
                    .errors
                    .push(ParseError::ArrowWithoutBlock(item_span)),
            }
        }

        (block, BlockTermination::Eof)
    }

    pub fn process(&mut self, token: Span<Token>) {
        let span = token.swap(());

        match token.data {
            Token::Ident(ident) => self.process_ident(ident, span),
            Token::String(string) => self.process_string(string, span),
            Token::Integer(value) => self.process_number(value, span),
            Token::Arrow => self.process_arrow(span),
            Token::Parens => self.process_command(Command::Call, span),
            Token::Plus => self.process_builtin(Builtin::Plus, span),
            Token::Minus => self.process_builtin(Builtin::Minus, span),
            Token::Multiply => self.process_builtin(Builtin::Multiply, span),
            Token::Divide => self.process_builtin(Builtin::Divide, span),
            Token::Equals => self.process_builtin(Builtin::Equals, span),
            Token::GreaterThan => self.process_builtin(Builtin::GreaterThan, span),
            Token::LessThan => self.process_builtin(Builtin::LessThan, span),
            Token::Period => self.process_command(Command::OutputNumber, span),
            Token::Comma => self.process_command(Command::OutputChar, span),
            Token::Ampersand => self.process_command(Command::InputNumber, span),
            Token::Tilde => self.process_command(Command::InputChar, span),
            Token::Colon => self.process_colon(span),
            Token::Question => self.process_builtin(Builtin::IfThenElse, span),
            Token::LCurly => self.process_open_block(span),
            Token::RCurly => self.process_close_block(span),
            Token::Comment(comment) => self.process_comment(comment, span),
            Token::Unexpected(c) => self
                .errors
                .push(ParseError::UnexpectedChar(span.swap(c))),
        }
    }

    pub fn finish(mut self) -> Result<ParseOutput, ParseErrors<ParseError>> {
        let (block, termination) = self.terminate_block();

        if let BlockTermination::Curly(span) = termination {
            self.errors
                .push(ParseError::BlockWithoutClosing(span));
        }

        let output = ParseOutput {
            ast: block,
            errors: self.errors,
        };

        if self.permissive || output.errors.is_empty() {
            Ok(output)
        } else {
            Err(output.errors.into())
        }
    }
}

impl ParserState<Token> for State {
    type Ast = ParseOutput;
    type Error = ParseError;

    fn process(&mut self, token: Span<Token>) {
        self.process(token)
    }

    fn finish(self) -> Result<Self::Ast, ParseErrors<Self::Error>> {
        self.finish()
    }
}
