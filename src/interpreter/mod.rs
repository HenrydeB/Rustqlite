mod scanner;
mod parser;
pub mod token;
pub mod stmt;

use crate::interpreter::stmt::{Stmt};
use scanner::Scanner;
use parser::Parser;

pub fn interpret(cmd: &str) -> Result<Stmt, String> {
    let mut scanner: Scanner = Scanner::new(cmd);
    let tokens = scanner.scan();

    let mut parser: Parser = Parser::new(&tokens);
    let statement = parser.parse();
   
    match statement {
        Ok(stmt) => Ok(stmt),
        Err(err) => {
           let error = err.to_string();
           Err(error)
        },
    }
}
