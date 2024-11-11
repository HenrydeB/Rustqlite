mod virtualmachine;
pub mod table;
use colored::*;
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Read;
use crate::vm::virtualmachine::Database;
use crate::interpreter::stmt::{Stmt};

use virtualmachine::VirtualMachine;

pub fn process(stmt: Stmt) -> Result<ColoredString, ColoredString>{

    let mut vm = VirtualMachine::new(stmt);
    match vm.run(){
        Ok(msg) => Ok(msg.green()),
        Err(err) => Err(err.red()),
    }
}

pub fn print_schema() -> Result<(), ColoredString>{ 
    let get_file = OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .open("data/database.rdb");

    let mut file = match get_file{
        Ok(db) => db,
        Err(err) => return Err(err.to_string().red()),
    };

    let mut buff = Vec::new();
    file.read_to_end(&mut buff).map_err(|err| err.to_string().red())?;
    let memory_db: Database = match bincode::deserialize(&buff){
        Ok(exists) => exists,
        Err(_) => {
            println!("{}", "No database found.. creating new DB instance".yellow());
            Database{
                tables : BTreeMap::new(),
            }
        },
    };

    let table_names: Vec<&String> = memory_db.tables.keys().collect();

    for table in table_names{
        let name  = table;
        println!("{}", name.blue());
    }

    Ok(())

}

