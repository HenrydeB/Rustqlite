mod virtualmachine;
pub mod table;

use crate::interpreter::stmt::{Stmt};

use virtualmachine::VirtualMachine;

pub fn process(stmt: Stmt){

    let mut vm = VirtualMachine::new(stmt);
    let _ = vm.run();
}
