mod virtualmachine;
pub mod table;
use colored::*;

use crate::interpreter::stmt::{Stmt};

use virtualmachine::VirtualMachine;

pub fn process(stmt: Stmt) -> Result<ColoredString, ColoredString>{

    let mut vm = VirtualMachine::new(stmt);
    vm.run()
}
