mod scanner;
mod parser;
pub mod token;
pub mod stmt;

use scanner::Scanner;
use parser::Parser;

pub fn interpret(cmd: &str){
    let mut scanner: Scanner = Scanner::new(cmd);
    let tokens = scanner.scan();

    let mut parser: Parser = Parser::new(&tokens);
    let res = parser.parse();
    println!("{:?}", tokens);
    println!("{:?}", res);
}
