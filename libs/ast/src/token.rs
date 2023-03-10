use catastrophic_core::defines::ValueType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    String(String),
    Integer(ValueType),

    Arrow,
    Parens,

    Plus,
    Minus,
    Multiply,
    Divide,

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

    Comment(String),
    Unexpected(char),
}
