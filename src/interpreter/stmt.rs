use crate::interpreter::token::{Literal};

#[derive(Debug, Clone)]
pub enum Stmt{
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
        target_values: Vec<Literal>, 
   },
}
