use std::fmt::{Display, Formatter};

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

pub enum LiteralValue {
    NoVal,
    Num(i64),
    NumFloat(f64),
    Str(String),
}

pub struct Token {
    pub tt: TokenType,
    pub lexeme: String,
    line: u32,
    pub literal: LiteralValue,
}

impl Token {
    pub fn new(tt: TokenType, lexeme: String, line: u32, literal: LiteralValue) -> Self {
        Token {
            tt,
            lexeme,
            line,
            literal,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} ", self.tt, self.lexeme)
    }
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::NoVal => write!(f, "nil"),
            LiteralValue::Num(n) => write!(f, "{}", n),
            LiteralValue::NumFloat(fl) => write!(f, "{}", fl),
            LiteralValue::Str(s) => write!(f, "{}", s),
        }
    }
}
