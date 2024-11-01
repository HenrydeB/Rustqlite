extern crate text_tables;

use std::str;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use colored::*;

use crate::interpreter::stmt::{Stmt};
use crate::interpreter::token::{Literal};
use crate::vm::table::{Table, Row, Column};

pub struct VirtualMachine {
    command: Stmt,
}

impl VirtualMachine {
    pub fn new(statement: Stmt) -> Self{
        Self{
           command: statement,
        }
    }

    pub fn run(&mut self) -> Result<ColoredString, ColoredString>{ //at some point change to Table, &str
        let result = match &self.command {
            Stmt::Select{table_name, target_columns, where_conditions} => 
                self.select_table(table_name, target_columns, where_conditions),
            Stmt::Create{table_name, columns_and_data} => 
                self.create_table(table_name, columns_and_data),
            Stmt::Insert{table_name, target_columns, target_values} => 
                self.insert_into_table(table_name, target_columns, target_values),
            Stmt::Drop{table_name} => 
                self.drop_table(table_name),
            Stmt::Delete{table_name, lhs, rhs} => 
                self.delete_from_table(table_name, lhs, rhs),
            Stmt::Update{table_name, where_col, where_val, target_columns, target_values} => 
                self.update_table(table_name, where_col, where_val, target_columns, target_values),
        };

        result
    }


    fn select_table(&self, 
                    table_name: &str, 
                    target_columns: &Vec<String>,
                    where_conditions: &Option<(Vec<String>, Vec<Literal>)>
                    ) -> Result<ColoredString, ColoredString>{
   
        //this should be fine because we are only altering it in memory
        let target_table: Table = match self.read_file(&table_name){
            Ok(table) => table,
            Err(err) => return Err(err.red()),
        };
        
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

        table_data.push(cols.clone());

        if where_col.len() > 0 && where_vals.len() > 0{

            let ids = match self.collect_target_ids(&target_table.rows, 
                                                    &where_col,
                                                    &where_vals){
                Ok(id) => id,
                Err(err) => return Err(err),
            };

            for id in &ids{
                let mut row_data: Vec<String> = Vec::new();

                for column in &cols{
                   if column == "id" {
                       row_data.push(id.to_string());
                   }
                   let row = match target_table.rows.get(id){
                    Some(r) => r,
                    _ => return Err("invalid target row".red()),
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
            for (id, row) in &mut target_table.rows.clone().into_iter(){
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
        Ok("".green())
    }

    fn create_table(
                    &self,
                    name: &str,
                    data: &Vec<(String, String)>) -> Result<ColoredString, ColoredString>{

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
                return Err("cannot have duplicate column names".red());
            } else if schema.contains_key(col_name) && col_name == "id"{
                continue;
            } 
            columns.push(Column::new(String::from(col_name), String::from(datatype)));
            schema.insert(String::from(col_name), String::from(datatype));
            idx +=1;
        }

        let table = Table::new(name.to_string(), columns, schema);
        match self.write_file(&table, true){
            Ok(_) => {},
            Err(err) => return Err(err.red()),
        }

        //if written, we print to user success, then return Ok
        Ok("Table created successfully".green())
    }

    fn insert_into_table(&self,
                         name: &str, 
                         columns: &Vec<String>, 
                         values: &Vec<Literal>) -> Result<ColoredString, ColoredString>{

        let mut target_table: Table = match self.read_file(&name){
            Ok(table) => table,
            Err(err) => return Err(err),
        };



        let id = if columns.len() <= 0 || (columns.len() > 0 && !columns.contains(&"id".to_string())){

            let has_potential_id = match values.get(0){
                Some(val) => val,
                _ => &Literal::None,
            };

            match has_potential_id {
                Literal::Number(val) => *val as i32,
                _ => (target_table.rows.len() + 1) as i32, 
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

            *id as i32 
        };

        //probably should add type checking

        let row_vals = values.iter()
                             .filter(|val| **val != Literal::Number(id as i64))
                             .map(|lit| lit.clone())
                             .collect();

        let row = if columns.len() <= 0 {
            let col_names = target_table.columns.iter()
                                    .filter(|id| id.name != "id")
                                    .map(|s| s.name.clone())
                                    .collect();

            
            self.validate_schema(&col_names, &row_vals, &target_table.schema)?;
            

            Row::new(col_names, row_vals)
        } else {
            self.validate_schema(&columns, &row_vals, &target_table.schema)?;
            Row::new(columns.to_vec(), row_vals)
        };

        target_table.rows.insert(id as i64, row);
         
        match self.write_file(&target_table, false){
            Ok(_) => {},
            Err(err) => return Err(err.red()),
        }

        Ok("Command committed successfully".green())
    }

   ///if we do end up going with the single page format, this will likely look
   ///more like the delete_from_table() format to follow
   ///
    fn drop_table(&self, name: &String) -> Result<ColoredString, ColoredString>{
        let table_name = format!("./data/{}.rdb", name);
       match fs::remove_file(table_name) {
           Ok(_) => {
               Ok("Command committed successfully".green())
           },
           Err(err) =>{ 
               println!("{}", err);
               Err("Failed to remove file".red())
           },
       }
    }


    fn delete_from_table(&self, 
                         name: &String,
                         columns: &Vec<String>,
                         values: &Vec<Literal>) -> Result<ColoredString, ColoredString>{
 
        let mut target_table: Table = match self.read_file(&name){
            Ok(table) => table,
            Err(err) => return Err(err.red()),
        };
        let ids: Vec<i64> = match self.collect_target_ids(&target_table.rows, columns, values){
                Ok(ids) => ids,
                Err(err) => return Err(err.red()),
        };
        
        let mut success: bool = false;
        for id in ids{
            match target_table.rows.remove(&id){
                Some(_) => success = true,
                None => success = false,
            }
       }

        match self.write_file(&target_table, false){
            Ok(_) => {
                if success != true{
                    return Err("Unable to remove row(s) from table".red());
                }
            },
            Err(err) => return Err(err.red()),
        }
       Ok("Row(s) have been successfully removed from table".green()) 
    }


    fn update_table(&self,
                    name: &String,
                    where_cols: &Vec<String>,
                    where_vals: &Vec<Literal>, 
                    target_cols: &Vec<String>,
                    target_vals: &Vec<Literal>) -> Result<ColoredString, ColoredString>{
      

        let mut target_table: Table = match self.read_file(name){
            Ok(table) => table,
            Err(err) => return Err(err),
        };

        let ids = match self.collect_target_ids(&target_table.rows, where_cols, where_vals){
            Ok(id) => id,
            Err(err) => return Err(err),
        };

        // now that we have the IDs, we can get the rows, then replace the 
        // target columns with the target values
        
        for id in &ids{
            let row = match target_table.rows.get(id){
                Some(inner_row) => inner_row,
                None => return Err("Invalid row in table".red()),
            };

            self.validate_schema(&target_cols, &target_vals, &target_table.schema)?;

            let mut row_replacement = row.clone();

            
            for (col, val) in target_cols.into_iter().zip(target_vals){
                row_replacement.values.insert(col.to_string(), val.clone());       
            }

            
            target_table.rows.insert(*id, row_replacement);
        }

        match self.write_file(&target_table, false){
            Ok(_) => {},
            Err(err) => return Err(err.red()),
        } 

        Ok("Rows have been successfully been updated".green())
    }
    
    fn validate_schema(&self,
                       col_names: &Vec<String>,
                       values: &Vec<Literal>,
                       schema: &HashMap<String, String>) -> Result<(), ColoredString> {

        for (name, val) in col_names.iter().zip(values){
           
            let col_type = match schema.get(&String::from(name)){
                Some(inner_type) => inner_type, 
                None => {
                    continue;
                },
            };
            
            match val {
                Literal::String(_) if col_type != "varchar" =>
                     return Err("Invalid column-type combination".red()),
                Literal::Number(_) if col_type != "int" => 
                     return Err("Invalid column-type combination".red()),
                Literal::Boolean(_) if col_type != "bit" => 
                     return Err("Invalid column-type combination".red()),
                _ => {},
            }
        }

        Ok(())
    }


    fn collect_target_ids(&self,
                          rows: &BTreeMap<i64, Row>, 
                          columns: &Vec<String>, 
                          values: &Vec<Literal>) -> Result<Vec<i64>, ColoredString>{
        let mut ids: Vec<i64> = Vec::new();
        for (id, row) in rows{

            for (col, val) in columns.into_iter().zip(values) {
                if col == "id" {
                    let inner_id = match val{
                        Literal::Number(inner_id) => inner_id,
                        _ => return Err("Invalid input for ID value".red()),
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
    fn read_file(&self, tablename: &str) -> Result<Table, ColoredString> { 
       let file_path = format!("data/{}.rdb", tablename);

       let mut file = File::open(&file_path).map_err(|err| err.to_string().red())?; //each table gets their own file for now, how can we structure
                                    //this?
       let mut buff = Vec::new();
       file.read_to_end(&mut buff).map_err(|err| err.to_string().red())?;
       let memory_table: Table = bincode::deserialize(&buff).unwrap();
       Ok(memory_table)
    }

    //simply writes it back
    #[allow(dead_code)]
    fn write_file(&self, in_table: &Table, is_create: bool) -> Result<(), ColoredString>{
        let file_path = format!("data/{}.rdb", &in_table.name);
        
        if is_create && std::path::Path::new(&file_path).exists(){
            return Err("cannot create a table with an existing table name".red());
        }

        if !is_create && !std::path::Path::new(&file_path).exists(){
            return Err("cannot perform operation, table does not exist".red());
        }

        let encode: Vec<u8> = bincode::serialize(in_table).unwrap();
        let mut file = File::create(file_path).map_err(|err| err.to_string().red())?; //same deal with file path here
        file.write_all(&encode).map_err(|err| err.to_string().red())?; //add error handling
        Ok(())
    }
}
