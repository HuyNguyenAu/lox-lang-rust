use crate::Lox;

pub mod token;

use token::{ Token, TokenType, Literal };

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    end: usize,
    line: u32
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            end: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(
            Token::new(
                TokenType::EOF, 
                String::new(), 
                Literal::Nil, 
                self.line
                )
            );

        self.tokens.clone()
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token_type: TokenType = match self.match_token('=') {
                    true => TokenType::BangEqual,
                    false => TokenType::Bang,
                };

                self.add_token(token_type);
            },
            '=' => {
                let token_type: TokenType = match self.match_token('=') {
                    true => TokenType::EqualEqual,
                    false => TokenType::Equal,
                };

                self.add_token(token_type);
            },
            '<' => {
                let token_type: TokenType = match self.match_token('=') {
                    true => TokenType::LessEqual,
                    false => TokenType::Less,
                };

                self.add_token(token_type);
            },
            '>' => {
                let token_type: TokenType = match self.match_token('=') {
                    true => TokenType::GreaterEqual,
                    false => TokenType::Greater,
                };

                self.add_token(token_type);
            },
            '/' => {
                if self.match_token('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            _ => Lox::error(self.line, String::from("Unexpected character."))
        }
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(c) = self.source.chars().nth(self.current) {
            if c != expected {
                return false;
            }
        }

        self.current += 1;

        return true;
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        if let Some(c) = self.source.chars().nth(self.current) {
            return c;
        }

        panic!("")
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        if let Some(c) = self.source.chars().nth(self.current) {
            self.current += 1;
            return c;
        }

        panic!("Failed to advance.")
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_complete(token_type, Literal::Nil);
    }

    fn add_token_complete(&mut self, token_type: TokenType, literal: Literal) {
        if let Some(text) = self.source.get(self.start..self.current) {
            self.tokens.push(
                Token::new(token_type, text.to_string(), literal, self.line)
            )
        }
    }
}