mod scanner;
pub mod token;

use scanner::Scanner;

pub fn interpret(cmd: &str){
    let mut scanner: Scanner = Scanner::new(cmd);
    scanner.scan();
}
