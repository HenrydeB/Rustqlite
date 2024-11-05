mod scanner;
mod parser;
pub mod token;
pub mod stmt;

use colored::*;
use crate::interpreter::stmt::{Stmt};
use scanner::Scanner;
use parser::Parser;

pub fn interpret(cmd: &str) -> Result<Stmt, ColoredString> {
    let mut scanner: Scanner = Scanner::new(cmd);
    let tokens = match scanner.scan(){
        Ok(tks) => tks,
        Err(err) => return Err(err.red()),
    };

    let mut parser: Parser = Parser::new(&tokens);
    let statement = parser.parse();
   
    match statement {
        Ok(stmt) => Ok(stmt),
        Err(err) => {
           Err(err.red())
        },
    }
}
