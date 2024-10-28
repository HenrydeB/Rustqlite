
#[derive(Debug, Clone)]
pub enum TokenType {
    // keywords 
    Select, All, From, Where, Create, TableName, 
    Insert, Into, Update, Delete, Drop, Set, Values,  
    
    //datatypes
    Int, VarChar, Bit,

    //punctuation
    LeftParen, RightParen, Comma, SemiColon, Asterisk, Quote,

    //ops
    Equal, Greater, Less, NotEqual, And, Or, Not,

    //Literals
    Number, String, True, False, 

    //Identifier
    Identifier, Column, Table,

    EOF
}

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[derive(Debug, Clone, PartialEq)]
pub enum Literal{
    Number(i64),
    String(String),
    Boolean(bool),
    Null, //this one may be difficult to implement
    None
}

impl PartialEq<i64> for Literal{
    fn eq(&self, other: &i64) -> bool{
        match self{
            Literal::Number(val) => val == other,
            _ => false,
        }
    }
}



#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
}

impl Token{
    pub fn new(token_type: TokenType, lexeme: String, literal:Option<Literal>) -> Self{
        Token{
            token_type,
            lexeme,
            literal, //see if you can pass none if there are no literals
        }
    }
}
