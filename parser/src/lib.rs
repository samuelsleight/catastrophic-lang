#[macro_use]
extern crate lalrpop_util;

pub extern crate catastrophic_ast as ast;

use std::{fs::File, io::Read, path::Path};

pub use crate::error::Error;

mod error;

lalrpop_mod!(grammar);

pub fn parse<P: AsRef<Path>>(path: P) -> Result<Vec<ast::Expr>, Error> {
    let contents = {
        let mut buf = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut buf)?;
        buf
    };

    Ok(grammar::AstParser::new()
        .parse(&contents)
        .map_err(|err| err.map_token(|token| token.to_string()).map_error(String::from))?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
