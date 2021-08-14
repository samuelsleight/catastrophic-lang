extern crate catastrophic_ast as ast;

use std::iter::IntoIterator;

pub use crate::error::Error;

mod error;
pub mod instr;

fn instr_value(value: ast::Value) -> Result<instr::InstrValue, Error> {
    Ok(match value {
        ast::Value::Int(int) => instr::InstrValue::Value(instr::Value::Int(int)),
        ast::Value::Ident(ident) => instr::InstrValue::Ident(ident),
        ast::Value::Func(func) => instr::InstrValue::Value(instr::Value::Function(analyse_function(func)?)),
        ast::Value::Op(op) => instr::InstrValue::Value(instr::Value::Op(op.into())),
    })
}

fn env_value(name: &str, value: ast::Value) -> Result<instr::EnvValue, Error> {
    Ok(match value {
        ast::Value::Int(int) => instr::EnvValue::Value(instr::Value::Int(int)),
        ast::Value::Ident(ident) => return Err(Error::NamedNamedValue(name.to_owned(), ident)),
        ast::Value::Func(func) => instr::EnvValue::Value(instr::Value::Function(analyse_function(func)?)),
        ast::Value::Op(op) => instr::EnvValue::Value(instr::Value::Op(op.into())),
    })
}

fn analyse_function(parameterised_function: ast::ParameterisedFunc) -> Result<instr::Function, Error> {
    recurse_parameters(instr::Function::new(), parameterised_function, 0)
}

fn recurse_parameters(mut function: instr::Function, parameterised_function: ast::ParameterisedFunc, index: usize) -> Result<instr::Function, Error> {
    if function.env.insert(parameterised_function.param.clone(), instr::EnvValue::Arg(index)).is_some() {
        return Err(Error::DuplicateNamedValue(parameterised_function.param.clone()));
    }

    match parameterised_function.result {
        ast::Func::Body(body) => analyse_body(body, function),
        ast::Func::Func(parameterised_function) => recurse_parameters(function, *parameterised_function, index + 1),
    }
}

fn analyse_body<Input>(input: Input, mut function: instr::Function) -> Result<instr::Function, Error>
where
    Input: IntoIterator<Item = ast::Expr>,
{
    for expr in input {
        match expr {
            ast::Expr::Value(value) => function.instrs.push(instr::Instr::Push(instr_value(value)?)),
            ast::Expr::NamedValue(value) => {
                if function.env.insert(value.name.clone(), env_value(&value.name, value.value)?).is_some() {
                    return Err(Error::DuplicateNamedValue(value.name));
                }
            }
            ast::Expr::Parens => function.instrs.push(instr::Instr::Apply),
            ast::Expr::Comment(_) => (),
        }
    }

    Ok(function)
}

pub fn analyse<Input>(input: Input) -> Result<instr::Function, Error>
where
    Input: IntoIterator<Item = ast::Expr>,
{
    analyse_body(input, instr::Function::new())
}
