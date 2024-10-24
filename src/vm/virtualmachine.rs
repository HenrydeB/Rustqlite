#![allow(unused_imports)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};


use crate::interpreter::stmt::{Stmt};
use crate::interpreter::token::{Literal};
use crate::vm::table::{Table, Row, Column};

pub struct VirtualMachine {
    command: Stmt,
}

//need to find a way to persist file connection as soon as we connect
//or is that best practice? Is it a big deal?
impl VirtualMachine {
    pub fn new(statement: Stmt) -> Self{
        Self{
           command: statement,
        }
    }

    // I think we can just return the same table struct for printing?
    // if not that then we can just return two vectors of column names 
    // and rows. Or even a Vec<Vec<String>> sort of situation where 
    // we just stuff a bunch of vectors with whatever data and print it 
    // that way
    pub fn run(&mut self) -> Result<(), String>{ //at some point change to Table, &str
       //identify which command and run with it 
        match &self.command {
            Stmt::Select{table_name, target_columns, where_conditions} => 
                self.select_table(table_name, target_columns, where_conditions),
            Stmt::Insert{..} => Ok({}),
            Stmt::Create{..} => Ok({}),
            Stmt::Drop{..} => Ok({}),
            Stmt::Delete{..} => Ok({}),
            Stmt::Update{..} => Ok({}),
        }
    }


    fn select_table(&self, 
                    table_name: &str, 
                    _target_columns: &Vec<String>,
                    _where_conditions: &Option<(Vec<String>, Vec<Literal>)>
                    ) -> Result<(), String>{
    
        let _target_table: Table = match self.read_file(&table_name){
            Ok(table) => table,
            Err(err) => return Err(err),
        };

        //from the table, filter columns list by target columns
        //based on that list, we can filter by the where conditions
        //
        //if a column from the where conditions isn't in our set
        //return an error
        //
        //otherwise, begin collecting sets of data, maybe a vec of rows
        //and then we have a vec of strings for the column names for us to keep
        //an eye on
        //
        //consider extracting the where logic into it's own function
        
        Ok(())
    }

    /*

    fn insert_into_table() -> Result<&str, &str>{

    }

    
    fn create_table() -> Result<&str, &str>{

    }
    
    fn drop_table() -> Result<&str, &str>{

    }


    fn delete_from_table() -> Result<&str, &str>{

    }

    fn update_table() -> Result<&str, &str>{
        
    }*/

    

    //for now each table will have it's own file
    //then we will figure out how to do the actual data model
    fn read_file(&self, tablename: &str) -> Result<Table, String> { 
       let file_path = format!("data/{}.rdb", tablename);

       let mut file = File::open(&file_path).map_err(|err| err.to_string())?; //each table gets their own file for now, how can we structure
                                    //this?
       let mut buff = Vec::new();
       file.read_to_end(&mut buff).map_err(|err| err.to_string())?;
       let memory_table: Table = bincode::deserialize(&buff).unwrap();
       Ok(memory_table)
    }

    //simply writes it back
    #[allow(dead_code)]
    fn write_file(in_table: &Table, is_create: bool) -> Result<(), String>{
        let file_path = format!("data/{}.rdb", &in_table.name);
        
        if is_create && std::path::Path::new(&file_path).exists(){
            return Err("cannot create a table with an existing table name".to_string());
        }

        if !is_create && !std::path::Path::new(&file_path).exists(){
            return Err("cannot perform operation, table does not exist".to_string());
        }

        let encode: Vec<u8> = bincode::serialize(in_table).unwrap();
        let mut file = File::create(file_path).map_err(|err| err.to_string())?; //same deal with file path here
        file.write_all(&encode).map_err(|err| err.to_string())?; //add error handling
        Ok(())
    }
    
}
