
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub enum Literal{
    Number(i64),
    String(String),
    Boolean(bool),
    Null, //this one may be difficult to implement
    None
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
