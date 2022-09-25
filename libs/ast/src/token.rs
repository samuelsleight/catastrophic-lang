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

    Ampersand,
    Tilde,

    Colon,
    Question,

    LCurly,
    RCurly,

    Unexpected(char),
}
