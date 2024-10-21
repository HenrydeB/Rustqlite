use crate::interpreter::token::{Literal};

//if all we may have to 
//figure something out
//
//commenting out lifetime in case we want to swap out the Strings for &str
#[derive(Debug)]
pub enum Stmt/*<'s>*/{
   Select{
        table_name: String,
        target_columns: Vec<String>,
        where_conditions: Option<(Vec<String>, Vec<Literal>)>,
   },
   Insert{
        table_name: String,
        target_columns: Vec<String>,
        target_values: Vec<Literal>, 
   },
   Create{
        table_name: String,
        columns_and_data: Vec<(String, String)>,
    },
   Drop{
        table_name: String,
   },
   Delete{ 
        table_name: String,
        lhs: Vec<String>,
        rhs: Vec<Literal>,
   },
   Update{
        table_name: String,
        where_col: Vec<String>, 
        where_val: Vec<Literal>, 
        target_columns: Vec<String>,
        target_values: Vec<Literal>, //explore different data structure
   },
}
