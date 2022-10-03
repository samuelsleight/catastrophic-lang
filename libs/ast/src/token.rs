use catastrophic_core::defines::ValueType;

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    String(String),
    Integer(ValueType),

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
