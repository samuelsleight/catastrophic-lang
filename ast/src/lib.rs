#[derive(Clone, Debug)]
pub enum Func {
    Body(Vec<Expr>),
    Func(Box<ParameterisedFunc>),
}

#[derive(Clone, Debug)]
pub struct ParameterisedFunc {
    pub param: String,
    pub result: Func,
}

#[derive(Clone, Debug)]
pub enum Op {
    Plus,
    Minus,
    Times,
    DividedBy,
    Equals,
    IfElse
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Ident(String),
    Func(ParameterisedFunc),
    Op(Op),
}

#[derive(Clone, Debug)]
pub struct NamedValue {
    pub name: String,
    pub value: Value,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Value(Value),
    NamedValue(NamedValue),
    Parens,
    Comment(String),
}
