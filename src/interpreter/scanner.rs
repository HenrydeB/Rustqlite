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

     fn is_alpha(&self, key: &str) -> bool {
        if key == "true" || key == "false"{
            return false;
        }
        key.chars().all(|c| c.is_alphabetic())
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
             
            if curr.expect("invalid sequence").is_alphabetic() || 
                curr.expect("invalid sequence").is_numeric() {
                //handle alphanumeric
                tokens.push(self.scan_alphanumeric_sequence());
            }else {
                let mut input = String::from(curr.expect("invalid sequence"));

                match curr{
                    Some(curr_char) => input.push(curr_char),
                    None => continue,
                }

                let token_type = self.get_tokentype(&input)
                                        .expect("invalid token");
                let literal_type = self.get_literal_type(&input);
                
                let new_token = Token::new(token_type, input, literal_type);
                tokens.push(new_token);
            }
            self.advance();
        }
    }

    fn scan_alphanumeric_sequence(&mut self) -> Token{
        let mut coll = String::new();

        loop{
            let curr = self.peek();
            if curr.expect("invalid character").is_whitespace() {
                break;
            }

            match curr{
                Some(curr_char) => coll.push(curr_char),
                None => println!("invalid character detected"),
            }
            self.advance();
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
            "all" => Some(TokenType::All),
            "from" => Some(TokenType::From),
            "where" => Some(TokenType::Where),
            "create" => Some(TokenType::Create),
            "table" => Some(TokenType::Table),
            "insert" => Some(TokenType::Insert),
            "into" => Some(TokenType::Into),
            "update" => Some(TokenType::Update),
            "delete" => Some(TokenType::Delete),
            "int" => Some(TokenType::Int),
            "varchar" => Some(TokenType::VarChar),
            "bit" => Some(TokenType::Bit),
            "=" => Some(TokenType::Equal),
            "equals" => Some(TokenType::Equal),
            "(" => Some(TokenType::LeftParen),
            ")" => Some(TokenType::RightParen),
            "\'" => Some(TokenType::Quote),
            "and" => Some(TokenType::And),
            "false" => Some(TokenType::False),
            "true" => Some(TokenType::True),
            
            "\n" => Some(TokenType::EOF), //error
            _ => {
               if self.is_numeric(keyword) {
                    Some(TokenType::Number) //will need to expand for float
               } else if self.is_alpha(keyword){
                    Some(TokenType::String) 
               } else {
                    None
               }
            },
        }
    }

     /*
    fn check_boolean(&self, literal: &str) -> bool {
       match literal{
            "equals" | "greater" | "less" | 
                "notequal" | "and" | "or" | "not" => true,
                _ => false,
       };
      false 
    }
*/
    fn get_literal_type(&self, literal: &str) -> Option<Literal>{
        
        if self.is_alpha(literal) {
            Some(Literal::String(String::from(literal)))
        } else if self.is_numeric(literal) {
            Some(Literal::Number(literal.parse::<i64>().unwrap()))
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


/*
pub enum Literal{
    Number(i64),
    String(String),
    Boolean(bool),
    Null, //this one may be difficult to implement
    None
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
}

impl Token{
    pub fn new(token_type: TokenType, lexeme: String, literal:Literal) -> Self{
        Token{
            token_type,
            lexeme,
            literal, //see if you can pass none if there are no literals
        }
    }
*/


