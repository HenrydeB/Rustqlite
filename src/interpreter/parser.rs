use crate::interpreter::token::{TokenType, Token, Literal};
use crate::interpreter::stmt::{Stmt};
use std::cell::Cell;

#[derive(Debug)]
pub struct Parser<'p>{
    tokens: &'p [Token],
    position: Cell<usize>,
}

impl<'p> Parser<'p>{
  pub fn new(tokens: &'p [Token]) -> Self{
        Parser{
            tokens: tokens,
            position: Cell::new(0),
        }
   }

  fn peek(&self) -> Option<&'p Token>{
    self.tokens.get(self.position.get())
  }

  fn advance(&self) {
     let curr_idx = self.position.get();
     if curr_idx < self.tokens.len(){
        self.position.set(curr_idx + 1);
     }
  }

  fn peek_next(&self) ->Option<&'p Token> {
        self.tokens.get(self.position.get() + 1)
  }

  //we can expect to return one type of statement in this project
 pub fn parse(&mut self) -> Result<Stmt, &str>{
        let stmt_token = self.peek();
        
        match stmt_token {
            Some(stmt) => {
                match stmt.token_type{
                    TokenType::Select => self.select_stmt(),
                    TokenType::Create => self.create_stmt(),
                    TokenType::Drop =>   self.drop_stmt(),
                    TokenType::Delete => self.delete_stmt(),
                    TokenType::Insert => self.insert_stmt(),
                    TokenType::Update => self.update_stmt(),
                    _ => Err("invalid syntax, invalid start to SQL statement")
                }
            },
            None => Err("invalid syntax, invalid start to SQL statement") 
        }
    }
    
  fn select_stmt(&self) -> Result<Stmt, &str>{
     let mut columns_set: Vec<String> = Vec::new();
     self.advance();
     let first_col = self.peek(); 
    
     match first_col {
        Some(col) => {
             match col.token_type {
               TokenType::All => {
                   columns_set.push(String::from("*"));
                   self.advance();
               },
               TokenType::Identifier => { 
                   if self.parse_comma_list(&mut columns_set).is_err() {
                        return Err("Invalid syntax, check selected columns");
                    } 
               },
               _ =>  return Err("Invalid syntax, expected 'all' type or column name identifier"),
             }
        },
        None => return Err("Invalid syntax, check statement"),
     }
     
     let from_token = self.peek();
   
     match from_token {
        Some(token) => {
             match token.token_type {
                TokenType::From => self.advance(),
                _ => return Err("invalid syntax, expected 'from'"),
             };
        },
        None => return Err("Invalid syntax, check statement"),
     }
    
    let table_name: &str = self.get_table_name()?;
    
    let mut has_conditions = false;

    if let Some(token) = self.peek_next(){
       match token.token_type {
            TokenType::Where => {
                self.advance();
                self.advance();
                has_conditions = true;
            },
            _ => {},
       }
    }

    if has_conditions {
        let (target_cols, has_vals) = self.parse_equality_list()?;

        match self.expect_terminator() {
            Ok(_end) => {
                 return Ok(Stmt::Select{
                            table_name: String::from(table_name), 
                            target_columns: columns_set,
                            where_conditions: Some((target_cols, has_vals)),
                            });
            
            },
            Err(err) => return Err(err),
        }
    }
    match self.expect_terminator() {
        Ok(_end) =>  Ok(Stmt::Select{
                        table_name: String::from(table_name), 
                        target_columns: columns_set,
                        where_conditions: None,
                        }), 
        Err(err) => Err(err),
    }
  }

  fn drop_stmt(&self) -> Result<Stmt, &str>{
      self.advance();
              
      if let Some(token) = self.peek(){
          match token.token_type {
            TokenType::Table => self.advance(),
            _ => return Err("Invalid syntax, expected keyword 'table'"),
          }
      }

    let table_name = self.get_table_name()?;
 
    match self.expect_terminator() {
        Ok(_end) => Ok(Stmt::Drop{table_name: String::from(table_name)}),
        Err(err) => Err(err),
    }
  }

  fn insert_stmt(&self) -> Result<Stmt, &str>{
    self.advance(); 
    match self.peek() {
        Some(action) => {
            match action.token_type {
                TokenType::Into => self.advance(),
                _ => return Err("invalid syntax, expect 'into'"),
            };
        },
        None => return Err("Invalid syntax, check statement"),
    }

    let table_name = self.get_table_name()?; 
    
    self.advance();

    let mut is_values = false;
    let mut col_list = Vec::new();
    let mut val_list = Vec::new();

    //Get collected set of columns and/or values

    if let Some(next) = self.peek(){
       match next.token_type {
            TokenType::Values => is_values = true,
            _ => {},
       }
    }

    if is_values {
       self.advance();
       self.parse_comma_list_literal(&mut val_list)?;
       return Ok(Stmt::Insert{
            table_name: table_name.to_string(), 
            target_columns: col_list, 
            target_values: val_list
        });
    }

    if let Some(next) = self.peek(){
        match next.token_type {
            TokenType::LeftParen => self.parse_comma_list(&mut col_list)?,  
            _ => return Err("Invalid syntax, expected target column list"),
        }
    }

   self.advance();

   if let Some(vals) = self.peek(){
        match vals.token_type {
            TokenType::Values => self.advance(),
            _ => return Err("invalid syntax, expected 'values' keyword"),
        }
   }


    if let Some(next) = self.peek(){
        match next.token_type {
            TokenType::LeftParen => self.parse_comma_list_literal(&mut val_list)?, 
            _ => return Err("invalid syntax, expected value list"),
        }
    }

    match self.expect_terminator() {
        Ok(true) => Ok(Stmt::Insert{
            table_name: table_name.to_string(), 
            target_columns: col_list, 
            target_values: val_list
        }),
        Ok(false) => Err("invalid end to statement"),
        Err(err) => Err(err),
    }
  }


  fn update_stmt(&self) -> Result<Stmt, &str>{
    self.advance();
    let table_name = self.get_table_name()?;
    
    self.advance();

    match self.peek() {
        Some(set) => {
            match set.token_type{
               TokenType::Set => self.advance(),
               _ => return Err("Invalid syntax, expected 'set'"),
            };
        },
        None => return Err("Invalid syntax, check statement"),
    }

    let (target_cols, new_vals) = self.parse_equality_list()?;
  
    match self.peek() {
        Some(token) => {
            match token.token_type {
                TokenType::Where => self.advance(),
                _ => return Err("Invalid syntax, expected 'where'"),
            };
        },
        None => return Err("Invalid syntax, check statement"),
    }

    let (lhs, rhs) = self.parse_equality_list()?;

    match self.expect_terminator() {
        Ok(true) => {
            Ok(Stmt::Update{
                        table_name: table_name.to_string(), 
                        where_col: lhs, 
                        where_val: rhs, 
                        target_columns: target_cols, 
                        target_values: new_vals
                    })
        },
        Ok(false) => return Err("expected end to statement"),
        Err(err) => Err(err),
    }
  }

  fn create_stmt(&mut self) -> Result<Stmt, &str>{
    self.advance();

    if let Some(table_token) = self.peek(){
        match table_token.token_type {
            TokenType::Table => self.advance(),
            _ => return Err("Invalid syntax, incomplete Create Table statement"),
        }
    }

    let table_name = self.get_table_name()?; 
    
    self.advance(); 

    match self.peek() {
        Some(token) => {
            match token.token_type {
                TokenType::LeftParen => {
                    self.advance();
                    
                    let def = self.parse_create_list()?;
                    
                    match self.expect_terminator() {
                        Ok(true) => {
                            return Ok(Stmt::Create{
                                      table_name: table_name.to_string(), 
                                      columns_and_data: def
                                  }); 
                        }, 
                        Ok(false) => return Err("expected end to statement"),
                        Err(err) => return Err(err), 
                    }
                },
                _ => Err("Invalid syntax, expected parenthesis-bound list"),
                }
            },
            None => return Err("invalid syntax, missing token for valid statement"),
        } 
    }
    

  fn delete_stmt(&mut self) -> Result<Stmt, &str>{
    self.advance();
    match self.peek(){
        Some(token) => {
            match token.token_type{
                TokenType::From => self.advance(),
                _ => return Err("invalid syntax, expected 'from'"),
            }
        },
        None => return Err("invalid syntax, check statement"),
    }
   
    let name = self.get_table_name()?;
    
    self.advance();

    match self.peek() {
        Some(token) => {
            match token.token_type{ 
                TokenType::Where => {}, 
                _ => return Err("invalid syntax, expected 'where'"),
            }
        },
        None => return Err("invalid syntax, check statement"),
    }
    self.advance();

    let (lhs, rhs) = self.parse_equality_list()?;
         
    Ok(Stmt::Delete{
        table_name: name.to_string(), 
        lhs, 
        rhs,
    })   
  }

  fn get_table_name(&self)  -> Result<&str, &str>{
    let table_token = self.peek();
    match table_token {
        Some(token) => {
            match token.token_type {
                TokenType::Identifier => Ok(&token.lexeme),
                _ =>  Err("Invalid syntax, expected identifier")
            }
        },
        None => return Err("Invalid syntax, check statement format"),
    }
  }


  fn expect_terminator(&self) -> Result<bool, &str> {
    let terminator_token = self.peek_next();
    match terminator_token {
        Some(token) => {
            match token.token_type{
                TokenType::SemiColon => Ok(true),
                TokenType::Where => Ok(false),
                _ => Err("Invalid syntax, expected line terminator")
            } 
        },
        None => return Err("Invalid syntax, expected terminator token"),
    }
  }

  fn parse_create_list(&self) -> Result<Vec<(String, String)>, &str> {
        let mut cols_data = Vec::new();

        loop {
           let lhs = self.peek();
           let mut col_name = String::new();
 
           match lhs {
                Some(token) => {
                   match token.token_type {
                    TokenType::Where | TokenType::RightParen => break,
                    TokenType::Comma  => {
                        self.advance();
                        continue;
                    },
                    TokenType::Identifier => {
                        col_name.push_str(&token.lexeme); 
                    },
                    TokenType::And => continue, //this may need to shift elsewhere
                    _ => return Err("invalid syntax, expected 'identifier'"), 
                   } 
                },
                None => return Err("Invalid Syntax, expected valid token"),
           };
           self.advance();
           let rhs = self.peek();
           let mut datatype = String::new(); 
           match rhs {
            Some(token) => {
               match token.token_type{
                   TokenType::Int | TokenType::VarChar | TokenType::Bool => {
                       datatype.push_str(&token.lexeme);
                       self.advance();
                   },
                   _ => return Err("invalid syntax, column requires datatype"), 
               }
            },
            None => return Err("invalid syntax, expected valid token")
           };
           cols_data.push((col_name, datatype));
        } 
       Ok(cols_data)  
  }

  /// Update statements and Where conditions have a lhs and a rhs
  /// as we parse through their list. This function handles this kind
  /// of parsing
  
  fn parse_equality_list(&self) -> Result<(Vec<String>, Vec<Literal>), &str>{ 
    let mut cols = Vec::new();
    let mut vals = Vec::new();

    loop{
       let lhs = self.peek();
       
       match lhs {
            Some(token) => {
               match token.token_type {
                TokenType::Where | TokenType::SemiColon => break,
                TokenType::Identifier => {
                    cols.push(token.lexeme.clone()); 
                },
                TokenType::And | TokenType::Comma => {
                    self.advance();
                    continue;
                }, 
                _ => return Err("invalid syntax, expected 'identifier'"), 
               } 
            },
            None => return Err("Invalid Syntax, expected valid token"),
       };
       self.advance();
       let expect_equal = self.peek();
    
       match expect_equal {
        Some(token)  => {
            match token.token_type {
                TokenType::Equal => self.advance(),
                _ => return Err("invalid syntax, expected '='"), 
           }; 
        },
        None => return Err("invalid syntax, expected token"),
       };

       let rhs = self.peek();
       
       match rhs {
        Some(token) => {
           match token.token_type{
            TokenType::Number | TokenType::True | TokenType::False | TokenType::String => {
                    if let Some(literal) = &token.literal { 
                        vals.push(literal.clone());  
                    } else {
                        return Err("invalid syntax, expected literal");
                    }
                },
            _ => return Err("invalid syntax, expected literal"),
           }
        },
        None => return Err("invalid syntax, expected valid token")
       };

        if let Some(table_token) = self.peek_next(){
            match table_token.token_type {
                TokenType::SemiColon => break,
                _ => self.advance(),
            }
        } 
    } 
       Ok((cols, vals))
  }

    fn parse_comma_list_literal(&self, target_vals: &mut Vec<Literal>) -> Result<(), &str> {
        loop {
            let current_token = self.peek();

            match current_token {
                Some(token) => {
                    match token.token_type {
                        TokenType::From | TokenType::RightParen => break, 
                        TokenType::Number | TokenType::String | TokenType::True | TokenType::False => {
                            if let Some(literal) = &token.literal{
                                target_vals.push(literal.clone()); 
                            }
                        },
                        TokenType::Comma | TokenType::LeftParen => {
                            self.advance();
                            continue;
                        },
                        _ => return Err("Invalid syntax, expected literal"),
                    }
                }
                None => return Err("Invalid syntax, expected valid token"),
            }
            self.advance();
        };
        Ok(())
      }

    fn parse_comma_list(&self, target_columns: &mut Vec<String>) -> Result<(), &str> {
        loop {
            let current_token = self.peek();

            match current_token {
                Some(token) => {
                    match token.token_type {
                        TokenType::From | TokenType::RightParen => break, 
                        TokenType::Identifier => {
                            target_columns.push(token.lexeme.clone()); 
                        },
                        TokenType::Comma | TokenType::LeftParen => {
                            self.advance();
                            continue;
                        },
                        _ => return Err("Invalid syntax, expected column name"),
                    }
                }
                None => return Err("Invalid syntax, expected valid token"),
            }
            self.advance();
        };
        Ok(())
      }
}
