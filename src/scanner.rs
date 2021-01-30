use crate::tokens::TokenType::*;
use crate::tokens::{LiteralValue, Token, TokenType};

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), AND);
        keywords.insert("class".to_string(), CLASS);
        keywords.insert("else".to_string(), ELSE);
        keywords.insert("false".to_string(), FALSE);
        keywords.insert("for".to_string(), FOR);
        keywords.insert("fun".to_string(), FUN);
        keywords.insert("if".to_string(), IF);
        keywords.insert("nil".to_string(), NIL);
        keywords.insert("or".to_string(), OR);
        keywords.insert("print".to_string(), PRINT);
        keywords.insert("return".to_string(), RETURN);
        keywords.insert("super".to_string(), SUPER);
        keywords.insert("this".to_string(), THIS);
        keywords.insert("true".to_string(), TRUE);
        keywords.insert("var".to_string(), VAR);
        keywords.insert("while".to_string(), WHILE);
        keywords
    };
}

static DIGITS: std::ops::RangeInclusive<char> = '0'..='9';

static ALPHA_UPPER: std::ops::RangeInclusive<char> = 'A'..='Z';
static ALPHA_LOWER: std::ops::RangeInclusive<char> = 'a'..='z';

struct InnerScanner<'a> {
    src: &'a String,
    start: usize,
    current: usize,
    line: u32,
    tokens: Vec<Token>,
}

pub struct Scanner<'a> {
    //exploring interior mutability so users don't need to bind the scanner as mut
    inner: RefCell<InnerScanner<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a String) -> Self {
        Scanner {
            inner: RefCell::new(InnerScanner::new(src)),
        }
    }

    pub fn scan_tokens(&self) -> impl Deref<Target = Vec<Token>> + '_ {
        self.inner.borrow_mut().scan_tokens();

        std::cell::Ref::map(self.inner.borrow(), |d| &d.tokens)
    }
}

impl<'a> InnerScanner<'a> {
    pub fn new(src: &'a String) -> Self {
        InnerScanner {
            src,
            start: 0,
            line: 1,
            current: 0,
            tokens: vec![],
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(LEFT_PAREN),
            ')' => self.add_token(RIGHT_PAREN),
            '{' => self.add_token(LEFT_BRACE),
            '}' => self.add_token(RIGHT_BRACE),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),
            '!' => {
                if self.matching('=') {
                    self.add_token(BANG_EQUAL);
                } else {
                    self.add_token(BANG);
                }
            }
            '=' => {
                if self.matching('=') {
                    self.add_token(EQUAL_EQUAL);
                } else {
                    self.add_token(EQUAL);
                }
            }
            '<' => {
                if self.matching('=') {
                    self.add_token(LESS_EQUAL);
                } else {
                    self.add_token(LESS);
                }
            }
            '>' => {
                if self.matching('=') {
                    self.add_token(GREATER_EQUAL);
                } else {
                    self.add_token(GREATER);
                }
            }
            '/' => {
                if self.matching('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH);
                }
            }
            ' ' | '\r' | '\t' => {} //Ignore whitespace
            '\n' => self.line += 1,
            '"' => self.string(),
            '0'..='9' => {
                self.number();
            }

            _ => {
                if ALPHA_LOWER.contains(&c) || ALPHA_UPPER.contains(&c) || c == '_' {
                    self.identifier();
                } else {
                    println!("Unexpected character");
                }
            }
        };
    }

    fn matching(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let cur_char = self.src.as_bytes()[self.current] as char;
        if cur_char != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.src.as_bytes()[self.current] as char
        }
    }
    fn peek_next(&self) -> char {
        if (self.current + 1) > self.src.len() {
            '\0'
        } else {
            self.src.as_bytes()[self.current + 1] as char
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            println!("Unterminated string");
        }

        //consume closing "
        self.advance();

        let s = &self.src.as_str()[(self.start + 1)..(self.current - 1)];
        self.add_token_with_value(STRING, LiteralValue::Str(String::from(s)));
    }

    fn number(&mut self) {
        while DIGITS.contains(&self.peek()) {
            self.advance();
        }

        let mut is_float = false;

        if self.peek() == '.' && DIGITS.contains(&self.peek_next()) {
            //consume the . (dot)
            self.advance();
            is_float = true;
            while DIGITS.contains(&self.peek()) {
                self.advance();
            }
        }

        let s = &self.src.as_str()[self.start..self.current];

        let literal = if is_float {
            LiteralValue::NumFloat(s.parse::<f64>().unwrap())
        } else {
            LiteralValue::Num(s.parse::<i64>().unwrap())
        };

        self.add_token_with_value(NUMBER, literal);
    }

    fn identifier(&mut self) {
        while Self::is_alpha_numeric(&self.peek()) {
            self.advance();
        }

        let txt = &self.src.as_str()[self.start..self.current];
        let r = KEYWORDS.get(txt);

        let tt = if r.is_none() {
            IDENTIFIER
        } else {
            r.unwrap().clone()
        };

        self.add_token(tt);
    }

    fn is_alpha_numeric(c: &char) -> bool {
        DIGITS.contains(c) || ALPHA_UPPER.contains(c) || ALPHA_LOWER.contains(c)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.src.as_bytes()[self.current - 1] as char
    }

    fn add_token_with_value(&mut self, tt: TokenType, literal: LiteralValue) {
        let txt = &self.src.as_str()[self.start..self.current];
        let tok = Token::new(tt, String::from(txt), self.line, literal);
        self.tokens.push(tok);
    }

    fn add_token(&mut self, tt: TokenType) {
        self.add_token_with_value(tt, LiteralValue::NoVal);
    }

    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            EOF,
            "".to_string(),
            self.line,
            LiteralValue::NoVal,
        ));

        &self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scan_string() {
        let prg = String::from("\"this is a rox string\"");
        let s = Scanner::new(&prg);
        let toks = s.scan_tokens();

        assert_eq!(toks.len(), 2);

        let Token { tt, .. } = toks.get(1).unwrap();
        assert_eq!(tt, &EOF);

        let Token {
            tt,
            lexeme,
            literal,
            ..
        } = toks.get(0).unwrap();
        assert_eq!(tt, &STRING);
        assert_eq!(lexeme, "\"this is a rox string\"");
        if let LiteralValue::Str(s) = literal {
            assert_eq!(s, "this is a rox string");
        } else {
            panic!("Expected String");
        }
    }

    #[test]
    fn scan_num() {
        let prg = String::from("992");
        let s = Scanner::new(&prg);
        let toks = s.scan_tokens();

        assert_eq!(toks.len(), 2);

        let Token { tt, .. } = toks.get(1).unwrap();
        assert_eq!(tt, &EOF);

        let Token {
            tt,
            lexeme,
            literal,
            ..
        } = toks.get(0).unwrap();
        assert_eq!(tt, &NUMBER);
        assert_eq!(lexeme, "992");

        if let LiteralValue::Num(f) = literal {
            assert_eq!(f, &992i64);
        } else {
            panic!("Expected Integer ");
        }
    }

    #[test]
    fn scan_num_float() {
        let prg = String::from("11.24");
        let s = Scanner::new(&prg);
        let toks = s.scan_tokens();

        assert_eq!(toks.len(), 2);

        let Token { tt, .. } = toks.get(1).unwrap();
        assert_eq!(tt, &EOF);

        let Token {
            tt,
            lexeme,
            literal,
            ..
        } = toks.get(0).unwrap();
        assert_eq!(tt, &NUMBER);
        assert_eq!(lexeme, "11.24");
        if let LiteralValue::NumFloat(f) = literal {
            assert_eq!(f, &11.24f64);
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn scan_identifier() {
        let prg = String::from("heh123");
        let s = Scanner::new(&prg);
        let toks = s.scan_tokens();

        assert_eq!(toks.len(), 2);

        let Token {
            tt,
            lexeme,
            literal,
            ..
        } = toks.get(0).unwrap();
        assert_eq!(tt, &IDENTIFIER);
        assert_eq!(lexeme, "heh123");
        if let LiteralValue::NoVal = literal {
        } else {
            panic!("Expected No Literal");
        }
    }

    #[test]
    fn scan_keyword() {
        let prg = String::from("class");
        let s = Scanner::new(&prg);
        let toks = s.scan_tokens();

        assert_eq!(toks.len(), 2);

        let Token { tt, lexeme, .. } = toks.get(0).unwrap();
        assert_eq!(tt, &CLASS);
        assert_eq!(lexeme, "class");
    }
}
