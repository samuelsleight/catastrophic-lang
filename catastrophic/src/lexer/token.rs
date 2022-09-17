#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Integer(u64),

    Arrow,

    Plus,
    Minus,

    Equals,
    GreaterThan,

    Colon,
    Question,

    LParen,
    RParen,
    LCurly,
    RCurly,

    Unexpected(char),
}
