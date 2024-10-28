#![allow(unused_imports)]

extern crate text_tables;

use std::str;
use std::io;
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
    pub fn run(&mut self) -> Result<(), String>{ //at some point change to Table, &str
       //identify which command and run with it 
        match &self.command {
            Stmt::Select{table_name, target_columns, where_conditions} => 
                self.select_table(table_name, target_columns, where_conditions),
            Stmt::Create{table_name, columns_and_data} => 
                self.create_table(table_name, columns_and_data),
            Stmt::Insert{table_name, target_columns, target_values} => 
                self.insert_into_table(table_name, target_columns, target_values),
            _ => todo!()
            //
            //Stmt::Drop{..} => Ok({}),
            //Stmt::Delete{..} => Ok({}),
            //Stmt::Update{..} => Ok({}),
        }
    }


    //need to verify the literals are comparing themselve correctly
    fn select_table(&self, 
                    table_name: &str, 
                    target_columns: &Vec<String>,
                    _where_conditions: &Option<(Vec<String>, Vec<Literal>)>
                    ) -> Result<(), String>{
   
        //this should be fine because we are only altering it in memory
        let target_table: Table = match self.read_file(&table_name){
            Ok(table) => table,
            Err(err) => return Err(err),
        };
        
        let table_name = &target_table.name;
        //need to handle if the table does not exist

        let is_all = match target_columns.get(0){
           Some(val) if val == "*" => true,
           _ => false,
        };

        let mut table_data: Vec<Vec<String>> = Vec::new();
        let cols = if is_all{
            target_table.columns.iter()
                                    .map(|s| s.name.clone())
                                    .collect()
        } else {
           target_columns.to_vec()
        };
       
        
        //just push through the rows no filter
            table_data.push(cols.clone());
            for (id, row) in &mut target_table.rows.clone().into_iter(){
               let mut row_data: Vec<String> = Vec::new();
               row_data.push(id.to_string());
                
               for column in &cols{ 
                   let column_data = row.values.get(column);
                   match column_data {
                    Some(data) => {
                        let data_val = match data {
                            Literal::Number(val) => val.to_string(),
                            Literal::String(val) => String::from(val),
                            Literal::Boolean(val) => val.to_string(),
                            Literal::Null => String::from("NULL"),
                            _ => String::from(""), // throws err
                        };
                        row_data.push(data_val);
                    },
                    None => {}, //throws err
                   }
               }
               table_data.push(row_data);
            }
        
        // => pipe the table into a formatting printer function
        let mut out = Vec::new();
        text_tables::render(&mut out, table_data).unwrap();
        println!("\n--{}--", table_name.to_uppercase());
        println!("{}", str::from_utf8(&out).unwrap());
        Ok(())
    }

    fn create_table(
                    &self,
                    name: &str,
                    data: &Vec<(String, String)>) -> Result<(), String>{

        let mut columns: Vec<Column> = Vec::new();

        for (col_name, datatype) in data{
            columns.push(Column::new(col_name.to_string(), datatype.to_string()));
        }

        let table = Table::new(name.to_string(), columns);
        match Self::write_file(&table, true){
            Ok(_) => println!("Table created successfully"),
            Err(err) => println!("{}", err),
        }

        //if written, we print to user success, then return Ok
        Ok(())
    }

    fn insert_into_table(&self,
                         name: &str, 
                         columns: &Vec<String>, 
                         values: &Vec<Literal>) -> Result<(), String>{

        let mut target_table: Table = match self.read_file(&name){
            Ok(table) => table,
            Err(err) => return Err(err),
        };

        let id = if columns.len() <= 0 || !columns.contains(&"id".to_string()){
            (target_table.rows.len() + 1) as i32 
        } else {
            //need to both extract the ID from the values list based on 
            //the index of the column name id, AND filter them out of the 
            //column list

            let id_location = match columns.iter().position(|name| name == "id"){
                Some(loc) => loc,
                None => 0,
            };
            
            let id_exists = match values.get(id_location){
                Some(val) => val,
                None => &Literal::Null,
            };
            
            let id = match id_exists {
                Literal::Number(val) => val,
                _ => &(0 as i64),
            };

            *id as i32 
        };

        let row_vals = values.iter()
                             .filter(|val| **val != Literal::Number(id as i64))
                             .map(|lit| lit.clone())
                             .collect();

        let row = if columns.len() <= 0 {
            let col_names = target_table.columns.iter()
                                    .filter(|id| id.name != "id")
                                    .map(|s| s.name.clone())
                                    .collect();

            Row::new(col_names, row_vals)
        } else {
            Row::new(columns.to_vec(), row_vals)
        };

        target_table.rows.insert(id, row);
         
        match Self::write_file(&target_table, false){
            Ok(_) => println!("Command committed successfully"),
            Err(err) => println!("{}", err),
        }

        Ok(())
    }

    
/*    
    fn drop_table() -> Result<(), &str>{

    }


    fn delete_from_table() -> Result<&str, &str>{

    }

    fn update_table() -> Result<&str, &str>{
        
    }*/
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

pub struct Column{ // we can likely change the internal datatypes with lifetimes
    pub name: String,
    pub datatype: String,
}
*/
/*
   fn print_table_format(table: &Table){
       println!("");
       println!("TABLE : {}", table.name.to_uppercase());

       let mut col_row = String::from("| ");
       let mut divider = String::from("--");
       for col in &table.columns {
           col_row.push_str(&format!("{} | ", col.name));
            
           divider.push_str("---");
       }

       println!("{}", &divider);
       println!("{}", &col_row);
       println!("{}", &divider);
/*
       for (_, row) in &table.rows{
           for (pkey, data) in row.into_iter(){

           }
       } */
       //extract row -> extract individual column by name -> print??
       

   } 
*/
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
