use crate::interpreter::token::{TokenType, Token, Literal};


//may have to change the way we do the scanning, I don't think the inserts
//would work properly
//
//maybe if we detect an insert, we still create the tokens but we have two
//vectors, one for column name one for value
//
//will likely have to change the token syntax to fit what we need for 
//our inserts and such
//so we have a vector that contains our tokens
//then as we collect the values we can insert the corresponding value
//at the same index as said token we had been defining
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

   /// the following are Scanner struct
   /// helper methods
   ///
   /// fn advance() moves the position of the scanner
   /// to the next position of the input

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
        if key == "true" || key == "false"{
            return false;
        }
        key.chars().all(|c| c.is_alphanumeric())
    }

     fn is_numeric(&self, key: &str) -> bool{
        key.chars().all(|c| c.is_numeric())
    }
    
    /// This function handles scanning the input string itself,
    /// identifying keys to be turned into tokens for the parser to 
    /// verify in the following step

     pub fn scan(&mut self) -> Vec<Token>{
        let mut tokens: Vec<Token> = Vec::new();
        
        loop{
            let curr = self.peek();

            if curr.expect("invalid sequence").is_whitespace() { 
                match self.advance(){
                    Some('\0') | None => break,
                    _ => continue,
                }
            }
            
            //for now only single line
            if curr.unwrap() == '\0'{
                break;
            }

            if curr.unwrap() == '\''{
                println!("here");
            }

            if curr.unwrap() == ';'{
                let token_type = TokenType::SemiColon;
                let input = String::from(curr.unwrap());
                let new_token = Token::new(token_type, input, None);
                tokens.push(new_token);
                break;
            }

            if curr.expect("invalid sequence").is_alphabetic() || 
                curr.expect("invalid sequence").is_numeric() || 
                curr.unwrap() == '\'' {
                //handle alphanumeric
                let new_token = self.scan_alphanumeric_sequence();
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
                let input = String::from(curr.expect("invalid sequence"));

                let token_type = self.get_tokentype(&input)
                                        .expect("invalid token");
                let literal_type = self.get_literal_type(&input);
                
                let new_token = Token::new(token_type, input, literal_type);
                tokens.push(new_token); 
            }
            
            let move_next = self.advance();

            match move_next {
                    Some('\0') | None => break,
                    _ => {},
                }
        }
        tokens
    }

    fn scan_alphanumeric_sequence(&mut self) -> Token{
        let mut coll = String::new();
        let mut open_string = false;
        loop{
            let curr = self.peek();

            match curr{
                Some(curr_char) if curr_char.is_whitespace() || 
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
                        self.advance();
                        continue;
                    } else {
                        break;
                    }
                },
                Some(curr_char) => coll.push(curr_char),
                None => println!("invalid character detected"),
            }
            match self.advance(){
                Some('\0') | None => break,
                _ => { },
            }
        }
       
        if open_string == true {
            let literal_type = self.get_literal_type(&coll); 
            return Token::new(TokenType::String, coll, literal_type);
        }

        let token_type = self.get_tokentype(&coll).expect("invalid token");
        let literal_type = self.get_literal_type(&coll);
        
        let new_token = Token::new(token_type, coll, literal_type);
        new_token
    }
    
     /// Pretty self explanatory, we pass in a constructed
     /// part of the SQL command so we can identify the type
     fn get_tokentype(&self, keyword: &str) -> Option<TokenType>{
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
            "bit" => Some(TokenType::Bit),
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
               if self.is_numeric(keyword) {
                    Some(TokenType::Number) //will need to expand for float
               } else if keyword.contains('\'') { 
                    Some(TokenType::String) 
               } else if self.is_alphanumeric(keyword){
                    Some(TokenType::Identifier) 
               } else {
                    None
               }
            },
        }
    }

    fn get_literal_type(&self, literal: &str) -> Option<Literal>{
        
        if self.is_numeric(literal) {
            Some(Literal::Number(literal.parse::<i64>().unwrap()))
        } else if self.is_alphanumeric(literal) {
            Some(Literal::String(String::from(literal)))
        } else if literal == "true" {
            Some(Literal::Boolean(true))
        } else if literal == "false" { 
            Some(Literal::Boolean(false))
        } else if literal == "null" {
            Some(Literal::Null)
        } else {
            None
        }

    } 
}


