mod virtualmachine;
pub mod db;
use colored::*;
use std::cell::RefMut;
use crate::interpreter::stmt::{Stmt};
use crate::vm::db::{Database};

use virtualmachine::VirtualMachine;

pub fn process(stmt: Stmt, db: RefMut<'_, Database>) -> Result<ColoredString, ColoredString>{

    let mut vm = VirtualMachine::new(db);
    match vm.run(stmt){
        Ok(msg) => Ok(msg.green()),
        Err(err) => Err(err.red()),
    }
}


