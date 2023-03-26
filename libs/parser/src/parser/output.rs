use catastrophic_ast::ast;

use super::error::ParseError;

pub struct ParseOutput {
    pub ast: ast::Block,
    pub errors: Vec<ParseError>,
}
