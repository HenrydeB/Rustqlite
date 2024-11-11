extern crate text_tables;

use std::str;
use std::collections::{HashMap, BTreeMap};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use colored::*;

use crate::interpreter::stmt::{Stmt};
use crate::interpreter::token::{Literal};
use crate::vm::table::{Table, Row, Column};
    
#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[derive(Debug, Clone)]
pub struct Database {
    pub tables: BTreeMap<String, Table>,
}

pub struct VirtualMachine {
    command: Stmt,
}

impl VirtualMachine {
    pub fn new(statement: Stmt) -> Self{
        Self{
           command: statement,
        }
    }

    pub fn run(&mut self) -> Result<String, String>{ //at some point change to Table, &str
        let result = match &self.command {
            Stmt::Select{table_name, target_columns, where_conditions} => 
                VirtualMachine::select_table(table_name, target_columns, where_conditions),
            Stmt::Create{table_name, columns_and_data} => 
                VirtualMachine::create_table(table_name, columns_and_data),
            Stmt::Insert{table_name, target_columns, target_values} => 
                VirtualMachine::insert_into_table(table_name, target_columns, target_values),
            Stmt::Drop{table_name} => 
                VirtualMachine::drop_table(table_name),
            Stmt::Delete{table_name, lhs, rhs} => 
                VirtualMachine::delete_from_table(table_name, lhs, rhs),
            Stmt::Update{table_name, where_col, where_val, target_columns, target_values} => 
                VirtualMachine::update_table(table_name, where_col, where_val, target_columns, target_values),
        };
        result
    }


    fn select_table( 
                    table_name: &str, 
                    target_columns: &Vec<String>,
                    where_conditions: &Option<(Vec<String>, Vec<Literal>)>
                    ) -> Result<String, String>{
   
        let target_table: Table =  VirtualMachine::read_file(&table_name)?;
        
        let table_name = &target_table.name;

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

        let (where_col, where_vals): (Vec<String>, Vec<Literal>) = match where_conditions {
            Some((wc, wv)) => (wc.to_vec(), wv.to_vec()),
            None => (Vec::new(), Vec::new()),
        };

        VirtualMachine::validate_schema(&where_col, 
                                        &where_vals, 
                                        &target_table.schema)?;

        table_data.push(cols.clone());

        if where_col.len() > 0 && where_vals.len() > 0{

            let ids =  VirtualMachine::collect_target_ids(&target_table.rows, 
                                                    &where_col,
                                                    &where_vals)?; 
            for id in &ids{
                let mut row_data: Vec<String> = Vec::new();

                for column in &cols{
                   if column == "id" {
                       row_data.push(id.to_string());
                   }
                   let row = match target_table.rows.get(id){
                    Some(r) => r,
                    _ => return Err(String::from("invalid target row")),
                   }; 

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
        } else {
        //just push through the rows no filter
            for (id, row) in target_table.rows.into_iter(){
               let mut row_data: Vec<String> = Vec::new();
                
               for column in &cols{

                   if column == "id" {
                       row_data.push(id.to_string());
                   }

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
        }
        // => pipe the table into a formatting printer function
        let mut out = Vec::new();
        text_tables::render(&mut out, table_data).unwrap();
        println!("\n--{}--", table_name.to_uppercase());
        println!("{}", str::from_utf8(&out).unwrap());
        Ok(String::from(""))
    }

    fn create_table(name: &str,
                    data: &Vec<(String, String)>) -> Result<String, String>{

        let mut columns: Vec<Column> = Vec::new();
        let mut schema: HashMap<String,String> = HashMap::new();
        let mut idx = 0;

        for (col_name, datatype) in data{
            if idx == 0 && col_name != "id"{
                //first column should be ID, otherwise we make it ID
                //other ID fields should be explicitly the id for a FK relationship
                columns.push(Column::new(String::from("id"), String::from("int")));
                schema.insert(String::from("id"), String::from("int"));
            }

            if schema.contains_key(col_name) && col_name != "id"{
                return Err(String::from("cannot have duplicate column names"));
            } else if schema.contains_key(col_name) && col_name == "id"{
                continue;
            } 
            columns.push(Column::new(String::from(col_name), String::from(datatype)));
            schema.insert(String::from(col_name), String::from(datatype));
            idx +=1;
        }

        let table = Table::new(name.to_string(), columns, schema);
        VirtualMachine::write_file(table)?;

        //if written, we print to user success, then return Ok
        Ok(String::from("Table created successfully"))
    }

    fn insert_into_table(name: &str, 
                         columns: &Vec<String>, 
                         values: &Vec<Literal>) -> Result<String, String>{

        let mut target_table: Table = VirtualMachine::read_file(&name)?;

        let id: i64 = if columns.len() <= 0 || (columns.len() > 0 && !columns.contains(&"id".to_string())){

            let has_potential_id = match values.get(0){
                Some(val) => val,
                _ => &Literal::None,
            };

            match has_potential_id {
                Literal::Number(val) => *val,
                _ => (target_table.rows.len() + 1) as i64, 
            }

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

            *id 
        };

        let row_vals = values.iter()
                         .filter(|val| **val != Literal::Number(id))
                         .map(|lit| lit.clone())
                         .collect();

        let col_names = target_table.columns.iter()
                                .filter(|id| id.name != "id")
                                .map(|s| s.name.clone())
                                .collect();

        if columns.len() > 0 {
            VirtualMachine::validate_schema(&columns, &values, &target_table.schema)?;
        } else {
            VirtualMachine::validate_schema(&col_names, &row_vals, &target_table.schema)?;
        }  

        let row = if columns.len() <= 0 {
            Row::new(col_names, row_vals)
        } else {
            let mut filled_rows: Vec<Literal> = Vec::new();

            for col in &target_table.columns{
               
                if &col.name == "id"{
                    continue;
               }

               if columns.contains(&col.name){
                    let idx = match columns.iter().position(|x| x == &col.name){
                        Some(i) => i,
                        None => return Err(String::from("Invalid column sequence")),
                    }; 
                    filled_rows.push(values[idx].clone());
               } else {
                   //adds default value in place of empty space
                    let filler_val = match &*col.datatype {
                        "varchar" => Literal::String(String::from("")),
                        "int" => Literal::Number(0),
                        "bool" => Literal::Boolean(false),
                        _ => Literal::String(String::from("")),
                    }; 
                    filled_rows.push(filler_val);
               } 
            }
            Row::new(col_names, filled_rows)
        };
        target_table.rows.insert(id, row);

        VirtualMachine::write_file(target_table)?;

        Ok(String::from("Command committed successfully"))
    }

   ///if we do end up going with the single page format, this will likely look
   ///more like the delete_from_table() format to follow
   ///
    fn drop_table(name: &String) -> Result<String, String>{
     
        let get_file = OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .create(true)
                                    .open("data/database.rdb");

        let mut file = get_file.map_err(|err| err.to_string())?;

        let mut buff = Vec::new();
        file.read_to_end(&mut buff).map_err(|err| err.to_string())?;
        
        let mut memory_db: Database = match bincode::deserialize(&buff){
            Ok(exists) => exists,
            Err(_) => {
                println!("{}", "No database found.. creating new DB instance".yellow());
                Database{
                    tables : BTreeMap::new(),
                }
            },
        };

        let res = match memory_db.tables.remove(name){
            Some(_) => Ok(String::from("Table dropped successfully")),
            None => Err(String::from("Unable to remove table")),
        };

       
        if !res.is_err() {
            let encode: Vec<u8> = bincode::serialize(&memory_db).unwrap();
            let mut file =  File::create("data/database.rdb")
                .map_err(|err| err.to_string())?;
            file.write_all(&encode).map_err(|err| err.to_string())?; 
        }
       res 
    }


    fn delete_from_table(name: &String,
                         columns: &Vec<String>,
                         values: &Vec<Literal>) -> Result<String, String>{
 
        let mut target_table: Table = VirtualMachine::read_file(&name)?;

        VirtualMachine::validate_schema(columns, values, &target_table.schema)?;

        let ids: Vec<i64> = VirtualMachine::collect_target_ids(&target_table.rows, 
                                                          columns, 
                                                          values)?;
 
        let mut success: bool = false;
        for id in ids{
            match target_table.rows.remove(&id){
                Some(_) => success = true,
                None => {},
            }
       }

        match VirtualMachine::write_file(target_table){
            Ok(_) => {
                if success != true{
                    return Err(String::from("Unable to remove row(s) from table"));
                }
            },
            Err(err) => return Err(err),
        }
       Ok(String::from("Row(s) have been successfully removed from table")) 
    }


    fn update_table( name: &String,
                    where_cols: &Vec<String>,
                    where_vals: &Vec<Literal>, 
                    target_cols: &Vec<String>,
                    target_vals: &Vec<Literal>) -> Result<String, String>{
      

        let mut target_table: Table = VirtualMachine::read_file(name)?;

        VirtualMachine::validate_schema(where_cols, where_vals, &target_table.schema)?;

        let ids = VirtualMachine::collect_target_ids(&target_table.rows, where_cols, where_vals)?;

        // now that we have the IDs, we can get the rows, then replace the 
        // target columns with the target values
        
        for id in &ids{
            let row = match target_table.rows.get(id){
                Some(inner_row) => inner_row,
                None => return Err(String::from("Invalid row in table")),
            };

            VirtualMachine::validate_schema(&target_cols, &target_vals, &target_table.schema)?;

            let mut row_replacement = row.clone();
            
            for (col, val) in target_cols.into_iter().zip(target_vals){
                row_replacement.values.insert(col.to_string(), val.clone());       
            }
 
            target_table.rows.insert(*id, row_replacement);
        }

        VirtualMachine::write_file(target_table)?;
        Ok(String::from("Row(s) have been successfully updated"))
    }
    
    fn validate_schema(col_names: &Vec<String>,
                       values: &Vec<Literal>,
                       schema: &HashMap<String, String>) -> Result<(), String> {

        for (name, val) in col_names.iter().zip(values){
            let col_type = match schema.get(&String::from(name)){
                Some(inner_type) => inner_type, 
                None => {
                    continue;
                },
            };
            
            match val {
                Literal::String(_) if col_type != "varchar" =>
                     return Err("Invalid column-type combination for varchar".to_string()),
                Literal::Number(_) if col_type != "int" => 
                     return Err("Invalid column-type combination for number".to_string()),
                Literal::Boolean(_) if col_type != "bool" => 
                     return Err("Invalid column-type combination for bool".to_string()),
                _ => {},
            }
        }

        Ok(())
    }


    fn collect_target_ids(rows: &BTreeMap<i64, Row>, 
                          columns: &Vec<String>, 
                          values: &Vec<Literal>) -> Result<Vec<i64>, String>{
        let mut ids: Vec<i64> = Vec::new();
        for (id, row) in rows{

            for (col, val) in columns.into_iter().zip(values) {
                if col == "id" {
                    let inner_id = match val{
                        Literal::Number(inner_id) => inner_id,
                        _ => return Err(String::from("Invalid input for ID value")),
                    };

                    if *inner_id == *id as i64{
                        ids.push(*id);
                    } 

                    continue;
                }

                match row.values.get(col) {
                    Some(Literal::String(row_val))  => {

                        let inner_val = match val {
                            Literal::String(inner) => inner,
                            _ => continue, 
                        };

                        if *row_val == *inner_val{
                            ids.push(*id);
                        } 
                    },
                    Some(Literal::Number(row_val)) => {
                        
                        let inner_val = match val {
                            Literal::Number(inner) => inner,
                            _ => continue, 
                        };
                        if *row_val == *inner_val{
                            ids.push(*id);
                        }
                    }, 
                    Some(Literal::Boolean(row_val)) => {
                        //find a way to convert
                        let inner_val = match val {
                            Literal::Boolean(inner) => inner,
                            _ => continue, 
                        };
                        if *row_val == *inner_val{
                            ids.push(*id);
                        } 
                    },
                    Some(Literal::Null) => {
                       if *val == Literal::Null{
                            ids.push(*id);
                       } 
                    },
                    Some(Literal::None) => continue,
                    _ => continue,
                }
            }            
        }
        Ok(ids)
    }
    

    //for now each table will have it's own file
    //then we will figure out how to do the actual data model
    fn read_file(tablename: &str) -> Result<Table, String> { 
       
        let get_file = OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .create(true)
                                    .open("data/database.rdb");

        let mut file = get_file.map_err(|err| err.to_string())?;
        
        let mut buff = Vec::new();
        file.read_to_end(&mut buff).map_err(|err| err.to_string())?;        
        let memory_db: Database = match bincode::deserialize(&buff){
            Ok(exists) => exists,
            Err(_) => {
                println!("{}", "No database found.. creating new DB instance".yellow());
                Database{
                    tables : BTreeMap::new(),
                }
            },
        };

        match memory_db.tables.get(tablename){
            Some(table) => Ok(table.clone()),
            None => return Err(String::from("Target table not found")),
        }
    }

    //simply writes it back
    fn write_file(in_table: Table) -> Result<(), String>{
        let get_file = OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .create(true)
                                    .open("data/database.rdb");

        let mut file = get_file.map_err(|err| err.to_string())?;

        let mut buff = Vec::new();
        file.read_to_end(&mut buff).map_err(|err| err.to_string())?;

        let mut memory_db: Database = match bincode::deserialize(&buff){
            Ok(exists) => exists,
            Err(_) => {
                println!("{}", "No database found.. creating new DB instance".yellow());
                Database{
                    tables : BTreeMap::new(),
                }
            },
        };

        memory_db.tables.insert(in_table.name.clone(), in_table);

        let encode: Vec<u8> = bincode::serialize(&memory_db).unwrap();
        let mut file = File::create("data/database.rdb").map_err(|err| err.to_string())?; 

        file.write_all(&encode).map_err(|err| err.to_string())?;
        
        Ok(())
    }
}
