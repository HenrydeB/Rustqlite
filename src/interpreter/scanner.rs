use crate::interpreter::token::{TokenType, Token, Literal};

#[derive(Debug)]
pub struct Scanner<'s>{
    cmd: &'s str,
    position: usize,
}

impl<'s> Scanner<'s>{
  pub fn new(statement: &'s str) -> Self{
        Scanner{
            cmd: statement,
            position: 0
        }
   }

    fn advance(&mut self) -> Option<char>{
        if self.position < self.cmd.len() {
            let self_slice = &self.cmd[self.position..];
            
            match self_slice.chars().next() {
                Some(char) => {
                    self.position += 1;
                    return Some(char)
                }
                None => return Some('\0') //temp
            }
        }
        None
    }

    /// peeking ahead will help us identify certain
    /// tokens
    /// Looks at next char without moving position forward

    fn peek(&self) -> Option<char>{
        if self.position == self.cmd.len(){
           return Some('\0')
        }
       self.cmd[self.position..].chars().next() 
    }

     fn is_alphanumeric(&self, key: &str) -> bool {
        key.trim().chars().filter(|w| !w.is_whitespace()).all(|c| c.is_alphanumeric())
    }

     fn is_numeric(&self, key: &str) -> bool{
        key.chars().all(|c| c.is_numeric())
    }
    
    /// This function handles scanning the input string itself,
    /// identifying keys to be turned into tokens for the parser to 
    /// verify in the following step

     pub fn scan(&mut self) -> Result<Vec<Token>, &str>{
        let mut tokens: Vec<Token> = Vec::new();
        
        loop{
            let curr = match self.peek(){
                Some(val) => {
                    if val.is_whitespace() {
                        match self.advance(){
                            Some('\0') | None => break,
                            _ => continue,        
                        }
                    }
                    val
                },
                None => return Err("invalid sequence"),
            };
            
            //for now only single line
            if curr == '\0'{
                break;
            }

            if curr == ';'{
                let token_type = TokenType::SemiColon;
                let new_token = Token::new(token_type, String::from(curr), None);
                tokens.push(new_token);
                break;
            }

            if curr.is_alphabetic() || curr.is_numeric() || curr == '\'' {
                //handle alphanumeric
                let new_token = match self.scan_alphanumeric_sequence(){
                    Ok(token) => token,
                    Err(_) => return Err("invalid sequence, expected alphanumeric sequence"),
                };
                
                tokens.push(new_token);

                let check_curr = self.peek();
                if check_curr.is_some() && (check_curr.unwrap() == ',' 
                                            || check_curr.unwrap() == ')'
                                            || check_curr.unwrap() == ';'){
                    // if we have a comma after an alphanumeric, we want to
                    // process it separately
                    continue;
                } else if check_curr.is_some() && check_curr.unwrap() == '\0'{
                    break;
                }
            }else {
                let input = String::from(curr); 

                let token_type = match self.get_tokentype(&input, false){
                        Some(t_type) => t_type,
                        None => return Err("invalid token"),
                };
                let literal_type = self.get_literal_type(&input, false);
                
                let new_token = Token::new(token_type, input, literal_type);
                tokens.push(new_token); 
            }
            
            match self.advance() {
                    Some('\0') | None => break,
                    _ => {},
                }
        }
        Ok(tokens)
    }

    fn scan_alphanumeric_sequence(&mut self) -> Result<Token, &str>{
        let mut coll = String::new();
        let mut open_string = false;
        let mut is_string = false;
        loop{
            let curr = self.peek();

            match curr{
                Some(curr_char) if curr_char.is_whitespace() && open_string == false || 
                    curr_char == '\0' ||
                    curr_char == ',' || 
                    curr_char == '(' ||
                    curr_char == ')' ||
                    curr_char == ';' => {
                    break
                },
                Some(curr_char) if curr_char == '\'' => {
                    if  open_string == false {
                        open_string = true;
                        is_string = true;
                        self.advance();
                        continue;
                    } else {
                        open_string = false;
                        break;
                    }
                },
                Some(curr_char) => coll.push(curr_char),
                None => return Err("invalid character detected"),
            }
            match self.advance(){
                Some('\0') | None => break,
                _ => { },
            }
        }
       
        if open_string == true {
            let literal_type = self.get_literal_type(&coll, is_string); 
            return Ok(Token::new(TokenType::String, coll, literal_type));
        }

        let token_type = match self.get_tokentype(&coll, is_string){
            Some(t_type) => t_type,
            None => return Err("invalid token"),
        };

        let literal_type = self.get_literal_type(&coll, is_string);
        
        let new_token = Token::new(token_type, coll, literal_type);
        Ok(new_token)
    }
    
     /// Pretty self explanatory, we pass in a constructed
     /// part of the SQL command so we can identify the type
     fn get_tokentype(&self, keyword: &str, is_string: bool) -> Option<TokenType>{
        match keyword{
            "select" => Some(TokenType::Select),
            "*" => Some(TokenType::All),
            "from" => Some(TokenType::From),
            "where" => Some(TokenType::Where),
            "create" => Some(TokenType::Create),
            "table" => Some(TokenType::Table),
            "insert" => Some(TokenType::Insert),
            "into" => Some(TokenType::Into),
            "update" => Some(TokenType::Update),
            "values" => Some(TokenType::Values),
            "set" => Some(TokenType::Set),
            "delete" => Some(TokenType::Delete),
            "drop" => Some(TokenType::Drop),
            "int" => Some(TokenType::Int),
            "varchar" => Some(TokenType::VarChar),
            "bool" => Some(TokenType::Bool),
            "=" => Some(TokenType::Equal),
            "equals" => Some(TokenType::Equal),
            "(" => Some(TokenType::LeftParen),
            ")" => Some(TokenType::RightParen),
            "\'" => Some(TokenType::String), //this needs to handle contents
            "and" => Some(TokenType::And),
            "false" => Some(TokenType::False),
            "true" => Some(TokenType::True),
            "," => Some(TokenType::Comma),
            ";" => Some(TokenType::SemiColon),
            "\0" => Some(TokenType::EOF), //error
            _ => {
               if self.is_numeric(keyword) && !is_string {
                    Some(TokenType::Number) //will need to expand for float
               } else if is_string { 
                    Some(TokenType::String) 
               } else if self.is_alphanumeric(keyword){
                    Some(TokenType::Identifier) 
               } else {
                    None
               }
            },
        }
    }

    fn get_literal_type(&self, literal: &str, is_string: bool) -> Option<Literal>{
        
        if self.is_numeric(literal) && !is_string{
            Some(Literal::Number(literal.parse::<i64>().unwrap()))
        } else if self.is_alphanumeric(literal) && is_string {
            Some(Literal::String(String::from(literal)))
        } else if literal == "true" && !is_string{
            Some(Literal::Boolean(true))
        } else if literal == "false" && !is_string { 
            Some(Literal::Boolean(false))
        } else if literal == "null" {
            Some(Literal::Null)
        } else {
            None
        }
    } 
}


