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
//
//it also might be better to just print from here and have a formatting module
//that handles this
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
    pub fn run(&mut self) -> Result<Table, String>{ //at some point change to Table, &str
       //identify which command and run with it 
        match &self.command {
            Stmt::Select{table_name, target_columns, where_conditions} => 
                self.select_table(table_name, target_columns, where_conditions),
            _ => todo!()
            //Stmt::Insert{..} => Ok({}),
            //Stmt::Create{..} => Ok({}),
            //Stmt::Drop{..} => Ok({}),
            //Stmt::Delete{..} => Ok({}),
            //Stmt::Update{..} => Ok({}),
        }
    }


/*
#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Table {
   pub name: String,
   pub columns: Vec<Column>,
   pub rows: BTreeMap<i32, Row>, //isize?
}

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Row {
                // col, content -> so when we filter we can look up by col
    values: HashMap<String,Literal>, //we can literally just copy over this from the Stmt
}
*/
    //need to verify the literals are comparing themselve correctly
    fn select_table(&self, 
                    table_name: &str, 
                    target_columns: &Vec<String>,
                    where_conditions: &Option<(Vec<String>, Vec<Literal>)>
                    ) -> Result<Table, String>{
   
        //this should be fine because we are only altering it in memory
        let mut target_table: Table = match self.read_file(&table_name){
            Ok(table) => table,
            Err(err) => return Err(err),
        };

        let is_all = match target_columns.get(0){
           Some(val) => val == "*",
           _ => false,
        };

        for (id, row) in &mut target_table.rows {
            //need to extract the data structure first
            if !is_all{
                row.values.retain(|key, _| target_columns.contains(key));
            }               

            if let Some((columns, values)) = where_conditions{
               for (col, val) in columns.iter().zip(values.iter()){
                    if col == "id" {
                        match val {
                            Literal::Number(val) => {
                                if *val != *id as i64{
                                    continue;
                                }
                            },
                            _ => {},
                        };
                    }
                    row.values.retain(|key, stored_val|{
                        if key == col{
                            if stored_val == val{
                                return true;
                            }
                        }
                        return false;
                    });
               } 
            }
        }

        //if target columns is just '*', then we use all the columns in each
        //row. Otherwise we can loop through the target columns and 'get' each
        //idk would it be better to just stringify them at this point?
        //
        //if a column from the where conditions isn't in our set
        //return an error
        //
        //otherwise, begin collecting sets of data, maybe a vec of rows
        //and then we have a vec of strings for the column names for us to keep
        //an eye on
        //
        //consider extracting the where logic into it's own function
        
        Ok(target_table)
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
