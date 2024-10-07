
pub enum TokenType {
    // keywords 
    Select, All, From, Where, Create, Table, TableName, 
    Insert, Into, Update, Delete, 
    
    //datatypes
    Int, VarChar, Bit,

    //punctuation
    LeftParen, RightParen, Comma, SemiColon, Asterisk,

    //ops
    Equal, Greater, Less, NotEqual, And, Or, Not,

    //Literals
    Number, String,

    //Identifier
    Identifier, Column, Table,

    EOF
}

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
}
