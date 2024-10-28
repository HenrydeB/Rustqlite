use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::interpreter::token::{Literal};

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Column{ // we can likely change the internal datatypes with lifetimes
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
}

#[derive(Clone)]
#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Row {
                // col, content -> so when we filter we can look up by col
   pub values: HashMap<String,Literal>, //we can literally just copy over this from the Stmt
}

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


#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Table {
   pub name: String,
   pub columns: Vec<Column>,
   pub rows: BTreeMap<i32, Row>, //isize?
}

impl Table {
    pub fn new(name: String, columns:Vec<Column>) -> Self{
        Table {
            name,
            columns,
            rows: BTreeMap::new(),
        }
    }
}
