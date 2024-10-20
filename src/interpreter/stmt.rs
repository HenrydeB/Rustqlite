use crate::interpreter::token::{TokenType, Literal};

//if all we may have to 
//figure something out
#[derive(Debug)]
pub enum Stmt<'s>{
   Select{
        table_name: String,
        target_columns: Vec<&'s str>, 
   },
   Insert{
        table_name: String,
        target_columns: Vec<String>,
        target_values: Vec<Literal>, //explore different data structure
   },
   Create{
        table_name: String,
        columns_and_data: Vec<(String, TokenType)>,
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
