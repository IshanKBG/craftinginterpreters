use std::collections::HashMap;
use std::fmt::{self};
//reimplment using iterators
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Lambda,

    Eof,
}
#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}
#[derive(Clone)]
pub struct Token {
    pub tty: TokenType,
    pub lexeme: Vec<u8>,
    pub literal: Option<Literal>,
    pub line: usize,
    pub col: i64,
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {{ ty: {:?}, lexeme: \"{}\", literal: {:?}, line: {:?}, col: {:?}}}",
            self.tty,
            String::from_utf8(self.lexeme.clone()).unwrap(),
            self.literal,
            self.line,
            self.col
        )
    }
}
#[derive(Debug)]
pub struct ScannerError {
    pub what: String,
    pub line: usize,
    pub col: i64,
}

pub struct Scanner {
    pub source: Vec<u8>,
    pub tokens: Vec<Token>,
    pub err: Option<ScannerError>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub col: i64,
    pub keywords: HashMap<String, TokenType>,
}
impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            source: Vec::new(),
            tokens: Vec::new(),
            current: 0,
            start: 0,
            err: None,
            line: 1,
            col: -1,
            keywords: vec![
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
                ("lambda", TokenType::Lambda),
            ]
            .into_iter()
            .map(|(k, v)| (String::from(k), v))
            .collect(),
        }
    }
    pub fn scan_tokens(&mut self, input: String) {
        self.source = input.into_bytes();
        while !self.err.is_some() && !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        match self.err {
            Some(_) => {}
            None => self.tokens.push(Token {
                tty: TokenType::Eof,
                lexeme: Vec::new(),
                literal: None,
                line: self.line,
                col: self.col,
            }),
        }
    }
    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.col += 1;
        char::from(self.source[self.current - 1])
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            '=' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }
            '<' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }
            '>' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
                self.col = 0
            }

            '"' => self.string(),
            _ => {
                if c.is_ascii_digit() {
                    self.number()
                } else if c.is_alphabetic() {
                    self.identifier()
                } else {
                    self.err = Some(ScannerError {
                        what: format!("Unexpected character at {}", c),
                        line: self.line,
                        col: self.col,
                    })
                }
            }
        }
    }
    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let val: f64 = String::from_utf8(self.source[self.start..self.current].to_vec())
            .unwrap()
            .parse()
            .unwrap();

        self.add_token_literal(TokenType::Number, Some(Literal::Number(val)))
    }
    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let literal_val =
            String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap();

        let token_type = match self.keywords.get(&literal_val) {
            Some(kw_token_type) => *kw_token_type,
            None => TokenType::Identifier,
        };

        match token_type {
            TokenType::Identifier => self.add_token_literal(
                TokenType::Identifier,
                Some(Literal::Identifier(literal_val)),
            ), // book doesn't do this. why not?}
            _ => self.add_token(token_type),
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
            self.err = Some(ScannerError {
                what: "Unterminated string.".to_string(),
                line: self.line,
                col: self.col,
            })
        }
        self.advance();
        self.add_token_literal(
            TokenType::String,
            Some(Literal::Str(
                String::from_utf8(self.source[self.start + 1..self.current - 1].to_vec()).unwrap(),
            )),
        )
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            char::from(self.source[self.current + 1])
        }
    }
    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            char::from(self.source[self.current])
        }
    }
    fn matches(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if char::from(self.source[self.current]) != c {
            return false;
        }
        self.current += 1;
        self.col += 1;
        true
    }
    fn add_token(&mut self, tty: TokenType) {
        return self.add_token_literal(tty, None);
    }
    fn add_token_literal(&mut self, tty: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_vec();
        self.tokens.push(Token {
            tty,
            lexeme: text,
            literal,
            line: self.line,
            col: self.col,
        });
    }
}
