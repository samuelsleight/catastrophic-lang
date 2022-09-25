#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    String(String),
    Integer(u64),

    Arrow,
    Parens,

    Plus,
    Minus,

    Equals,
    LessThan,
    GreaterThan,

    Period,
    Comma,

    Colon,
    Question,

    LParen,
    RParen,
    LCurly,
    RCurly,

    Unexpected(char),
}
