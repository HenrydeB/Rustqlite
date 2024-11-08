use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::interpreter::token::{Literal};

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Column{ 
    pub name: String,
    pub datatype: String,
}

impl Column {
    pub fn new(name: String, datatype: String) -> Self{
        Column {
            name,
            datatype,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_type(&self) -> &str{
        &self.datatype
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Row {
   pub values: HashMap<String,Literal>, }

impl Row{
    pub fn new(columns: Vec<String>, values: Vec<Literal>) -> Self{
        let mut row: HashMap<String, Literal> = HashMap::new();
        
        for (column, data) in columns.into_iter().zip(values){
            row.insert(column.to_string(), data.clone());
        }
       Row {
            values: row,
       } 
    }
}


#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Table {
   pub name: String,
   pub columns: Vec<Column>,
   pub rows: BTreeMap<i64, Row>, 
   pub schema: HashMap<String, String>,
}

impl Table {
    pub fn new(name: String, columns:Vec<Column>, schema:HashMap<String, String>) -> Self{
        Table {
            name,
            columns,
            rows: BTreeMap::new(),
            schema,
        }
    }
}
