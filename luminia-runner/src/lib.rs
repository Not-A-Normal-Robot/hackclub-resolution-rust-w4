pub mod ast;
pub mod errors;
pub mod interpreter;

use lalrpop_util::lalrpop_mod;

use crate::{errors::ParseAndRunError, interpreter::Interpreter, parser::ProgramParser};

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    pub parser, "/luminia.rs"
);

pub fn parse_and_run(source: &'_ str) -> Result<(), ParseAndRunError<'_>> {
    let parser = ProgramParser::new();
    let program = parser.parse(source)?;

    let mut interp = Interpreter::new();
    Ok(interp.run(program)?)
}
