use crate::interpreter::token::{TokenType, Token, Literal};
use crate::interpreter::stmt::{Stmt};

// TODO: 
// - now that peek and advance are option types, we need to fix how we 
// process getting the literals
// - double check error handling

#[derive(Debug)]
pub struct Parser<'p>{
    tokens: &'p [Token],
    position: usize,
}

impl<'p> Parser<'p>{
  pub fn new(tokens: &'p [Token]) -> Self{
        Parser{
            tokens: tokens,
            position: 0,
        }
   }
   //try using lifetimes for this 
  fn peek(&self) -> Option<&'p Token>{
    self.tokens.get(self.position)
  }

  fn advance(&mut self) {
    self.position += 1;
  }

  fn peek_next(&self) ->Option<&'p Token> {
        self.tokens.get(self.position + 1)
  }

  //we can expect to return one type of statement in this project
 pub fn parse(&mut self) -> Result<Stmt, &str>{
        let stmt_token = self.peek();
        
        match stmt_token {
            Some(stmt) => {
                match stmt.token_type{
                    TokenType::Select => self.select_stmt(),
                    TokenType::Create => self.create_stmt(),
                    TokenType::Drop => {
                        self.advance();
                        self.drop_stmt()
                    },
                    TokenType::Delete => self.delete_stmt(),
                    TokenType::Insert => self.insert_stmt(),
                    TokenType::Update => self.update_stmt(),
                    _ => Err("invalid syntax, invalid start to SQL statement")
                }
            },
            None => Err("invalid syntax, invalid start to SQL statement") 
        }
    }
    
  fn select_stmt(&mut self) -> Result<Stmt, &str>{
     let mut columns_set: Vec<&str> = Vec::new();
     self.advance();
     let first_col = self.peek(); 
    
     match first_col {
        Some(col) => {
             match col.token_type {
               TokenType::All => columns_set.push("*"),
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
     
     self.advance();
     
     let from_token = self.peek();
   
     match from_token {
        Some(token) => {
             match token.token_type {
                TokenType::From => {},
                _ => return Err("invalid syntax, expected 'from'"),
             };
        },
        None => return Err("Invalid syntax, check statement"),
     }
    self.advance();

    let table_result = self.get_table_name();

    let table_name = match table_result {
        Ok(name) => String::from(name),
        Err(err) => return Err(err),
    };

    let expect_end_statement = self.expect_terminator();
    match expect_end_statement {
        Ok(_end) => Ok(Stmt::Select{
            table_name: String::from(&table_name), 
            target_columns: columns_set,
        }),
        Err(err) => Err(err),
    }
  }

  fn drop_stmt(&mut self) -> Result<Stmt, &str>{
    let table = {
        let table_result = self.get_table_name();
        match table_result {
            Ok(table_name) => table_name,
            Err(err) => return Err(err),  //modify for custom err || it comes from fn
        }
    };
    
    let name = String::from(table);   
    
    match self.expect_terminator() {
        Ok(_end) => Ok(Stmt::Drop{table_name: name}),
        Err(err) => Err(err),
    }
  }

  fn insert_stmt(&self) -> Result<Stmt, &str>{
    self.advance();
    let action = self.peek();
   
    //find Into

    match action {
        Some(act) => {
            match act.token_type {
                TokenType::Insert => self.advance(),
                _ => return Err("invalid syntax, expect 'into'"),
            };
        },
        None => return Err("Invalid syntax, check statement"),
    }

    // find Table Name

    let table_name = String::new(); 
    if let Ok(name) = self.get_table_name(){
        table_name.push_str(&name);
    } else {
        return Err("Inalid syntax, expected table name");
    }

    self.advance();
    let expected_next = self.peek();
    let has_collected_vals = false;
    let col_list = Vec::new();
    let val_list = Vec::new();

    //Get collected set of columns and values

    match expected_next {
        Some(next) => {
            match next.token_type {
                TokenType::LeftParen => {
                    if let Ok(selected_set) = self.parse_parenthesis(false){    
                        col_list = selected_set.iter()
                                        .map(|(cols, _)| *cols)
                                        .collect();
                    } else {
                        return Err("Invalid syntax, check target columns");
                    }

                    let expect_val = self.peek();

                    match expect_val {
                        Some(val) => {
                            match val.token_type {
                                TokenType::Values => self.advance(),
                                _ => return Err("invalid syntax, must define values after columns"),
                            };
                        },
                        None => return Err("invalid syntax, check statement"),
                    }

                    let expect_paren = self.peek();
                    match expect_paren {
                        Some(paren) => {
                            match paren.token_type {
                                TokenType::LeftParen => {
                                    if let Ok(selected_vals) = self.parse_parenthesis(false){
                                        val_list = selected_vals.iter()
                                                               .map(|(vals, _)| *vals)
                                                               .collect();
                                    } else {
                                        return Err("Invalid syntax, expected value set");
                                    }
                                },
                                _ => return Err("invalid syntax, expected parenthesis for values"),
                            };
                        },
                        None => return Err("invalid syntax, check statement"),
                    }
                },
                TokenType::Values => {
                    let expect_paren = self.peek();
                    match expect_paren {
                        Some(paren) => {
                            match paren.token_type {
                                TokenType::LeftParen => {                                     
                                    if let Ok(selected_vals) = self.parse_parenthesis(false){
                                        val_list = selected_vals.iter()
                                                               .map(|(_, vals)| *vals)
                                                               .collect();
                                    } else {
                                        return Err("Invalid syntax, expected value set");
                                    }
                                },
                                _ => return Err("invalid syntax, expected parenthesis for values"),
                            };
                        },
                        None => return Err("invalid syntax, check statement"),
                    }
                },
                _ =>  return Err("invalid syntax, expected parenthesis after table name"),
            };
        },
        None => return Err("invalid syntax, check statement"),
    }
    
    let expect_end_statement = self.expect_terminator();
    match expect_end_statement {
        Ok(_end) => Ok(Stmt::Insert{
            table_name, 
            target_columns: col_list, 
            target_values: val_list
        }),
        Err(err) => Err(err),
    }
  }


  fn update_stmt(&mut self) -> Result<Stmt, &str>{
    self.advance();
    let mut table_name = String::new();
    
    if let Ok(name) = self.get_table_name(){
        table_name.push_str(&name);
    } else {
        return Err("Inalid syntax, expected table name");
    }

    self.advance();
    let expect_set = self.peek();
   
    match expect_set {
        Some(set) => {
            match set.token_type{
               TokenType::Set => self.advance(),
               _ => return Err("Invalid syntax, expected 'set'"),
            };
        },
        None => return Err("Invalid syntax, check statement"),
    }

    let (target_cols, new_vals) = match self.parse_equality_list() {
        Ok((lhs, rhs)) => (lhs, rhs),
        Err(_) => return Err("Invalid syntax, check target columns and values"),
    };
 
    let expect_where = self.peek();
 
    match expect_where {
        Some(token) => {
            match token.token_type {
                TokenType::Where => self.advance(),
                _ => return Err("Invalid syntax, expected 'where'"),
            };
        },
        None => return Err("Invalid syntax, check statement"),
    }

    if let Ok((lhs, rhs)) = self.parse_equality_list(){
        
        if let Ok(_end) = self.expect_terminator(){
            return Ok(Stmt::Update{
                        table_name, 
                        where_col: lhs, 
                        where_val: rhs, 
                        target_columns: target_cols, 
                        target_values: new_vals
                    });
        } else {
            return Err("Invalid syntax, invalid end to statement")
        }
    } else {
        return Err("invalid syntax, check 'where' conditions");
    } 
  }


  fn create_stmt(&mut self) -> Result<Stmt, &str>{
    self.advance();
    let mut table_name = String::new(); 
    
    if let Ok(name) = self.get_table_name(){ 
        table_name.push_str(&name);
    } else {
        return Err("Invalid syntax, check table name");
    }

    self.advance();
    
    let expect_left_paren = self.peek();

    match expect_left_paren {
        Some(token) => {
            match token.token_type {
                TokenType::LeftParen => {
                    let table_def = self.parse_parenthesis(true);
                    match table_def {
                        Ok(def) => {
                          return Ok(Stmt::Create{
                              table_name, 
                              columns_and_data: def
                          }); 
                        },
                        Err(err) => return Err(err),
                    }
                },
                _ => Err("Invalid syntax, expected parenthesis-bound list"),
            }
        },
        None => return Err("invalid syntax, check statement"),
    } 
  }

  fn delete_stmt(&mut self) -> Result<Stmt, &str>{
    self.advance();
    let expect_from = self.peek();
    match expect_from{
        Some(token) => {
            match token.token_type{
                TokenType::From => self.advance(),
                _ => return Err("invalid syntax, expected 'from'"),
            }
        },
        None => return Err("invalid syntax, check statement"),
    }
   
    let mut name = String::new();
    
    if let Ok(name_val) = self.get_table_name(){
        name.push_str(name_val);
    } else {
        return Err("Invalid syntax, check target table name");
    }

    self.advance();
    let expect_where = self.peek();

    match expect_where {
        Some(token) => {
            match token.token_type{ 
                TokenType::Where => {}, 
                _ => return Err("invalid syntax, expected 'where'"),
            }
        },
        None => return Err("invalid syntax, check statement"),
    }
    self.advance();

    let res = self.parse_equality_list();

    match res {
        Ok((lhs, rhs)) => {
            let left: Vec<String> = lhs;
            let right: Vec<Literal> = rhs;
            
            Ok(Stmt::Delete{
                table_name: name, 
                lhs: left, 
                rhs: right,
            })
        },
        Err(err) => Err(err)
    }
  }

    fn parse_parenthesis(&mut self, is_create: bool) -> Result<Vec<(String, TokenType)>, &str> {
        let mut value_set = Vec::new(); // Use mut here to modify the vector
        self.advance();

        loop {
            let check_token = self.peek(); // Get the current token
            let mut token_lex = String::new(); // Make this mutable

            match check_token{
                Some(token) => {
                    match token.token_type {
                        TokenType::Identifier => {
                            token_lex.push_str(&token.lexeme); // Append the lexeme to token_lex
                            self.advance(); // Advance after using the identifier
                        },
                        _ => return Err("invalid syntax, please check parenthesis"),
                    }
                },
                None => return Err("Invalid syntax, check statement syntax"),
            } 

            if is_create {
                // Expect data type after identifier
                let expect_datatype = self.peek();

                match expect_datatype {
                    Some(token) => {
                        match token.token_type {
                            TokenType::Int | TokenType::Bit | TokenType::VarChar => {
                                value_set.push((token_lex.clone(), token.token_type)); // Clone token_lex before pushing
                                self.advance(); // Advance after confirming data type
                            },
                            _ => return Err("Invalid syntax, expected data type for column"),
                        }
                    },
                    None => return Err("Invalid syntax, check statement syntax"),
                }
            }

            // Check for punctuation
            let expect_punctuation = self.peek();
            match expect_punctuation {
                Some(token) => { // Ensure the token exists before matching
                    match token.token_type {
                        TokenType::Comma => {
                            self.advance(); // Advance for comma
                        },
                        TokenType::RightParen => {
                            self.advance(); // Advance and exit for right parenthesis
                            break; // Exit the loop
                        },
                        _ => return Err("invalid syntax, please check parenthesis"),
                    }
                },
                None => return Err("Invalid syntax, expected valid token"),
            }
        }
        Ok(value_set) // Return the populated value_set
    }


  // Consider chaging this to return two &str instead
  fn get_table_name(&self) -> Result<&str, &str>{
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
                _ => Err("Invalid syntax, expected line terminator")
            } 
        },
        None => return Err("Invalid Syntax, check statement format"),
    }
  }

  /// Update statements and Where conditions have a lhs and a rhs
  /// as we parse through their list. This function handles this kind
  /// of parsing
  
  #[allow(dead_code)]
  fn parse_equality_list(&mut self) -> Result<(Vec<String>, Vec<Literal>), &str>{ 
    let mut cols = Vec::new();
    let mut vals = Vec::new();

    loop{
       let lhs = self.peek();
       
       match lhs {
            Some(token) => {
               match &token.token_type {
                TokenType::Where | TokenType::SemiColon => break,
                TokenType::Identifier => {
                    cols.push(token.lexeme.clone()); 
                },
                TokenType::And => continue, //this may need to shift elsewhere
                _ => return Err("invalid syntax, expected 'identifier'"), 
               } 
            },
            None => return Err("Invalid Syntax, expected valid token"),
       };
       self.advance();
       let expect_equal = self.peek();
    
       match expect_equal {
        Some(token)  => {
            match &token.token_type {
                TokenType::Equal => self.advance(),
                _ => return Err("invalid syntax, expected '='"), 
           }; 
        },
        None => return Err("invalid syntax, expected token"),
       };

       self.advance(); 
       let rhs = self.peek();
       
       match rhs {
        Some(token) => {
           match &token.literal{
            Some(val) /*| Some(Literal::String(val)) | Some(Literal::Bit(val))*/ => { 
                    vals.push(val.clone()); //doesn't matter, as long as it is a literal 
                },
            _ => return Err("invalid syntax, expected literal"),
           }
        },
        None => return Err("invalid syntax, expected valid token")
       };
    } 
       Ok((cols, vals))
  }

    fn parse_comma_list(&mut self, target_columns: &mut Vec<&'p str>) -> Result<(), &str> {
        loop {
            let current_token = self.peek();

            match current_token {
                Some(token) => {
                    match token.token_type {
                        TokenType::From => break, 
                        TokenType::Identifier => {
                            target_columns.push(token.lexeme.as_str()); 
                        }
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
