use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::interpreter::token::{Literal};

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Column{
    name: String,
    datatype: Literal,
}

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Row {
                // col, content -> so when we filter we can look up by col
    values: HashMap<String,Literal>, //we can literally just copy over this from the Stmt
}


#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Table {
   pub name: String,
   pub columns: Vec<Column>,
   pub rows: BTreeMap<i32, Row>, //isize?
}
