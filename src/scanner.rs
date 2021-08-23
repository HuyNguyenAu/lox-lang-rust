use std::collections::HashMap;

use crate::Lox;

pub mod token;

use token::{ Token, TokenType, Literal };

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    end: usize,
    line: u32,
    keywords: HashMap<String, TokenType>
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut reserved: HashMap<String, TokenType> = HashMap::new();
        reserved.insert(String::from("and"),TokenType::And);
        reserved.insert(String::from("class"),TokenType::Class);
        reserved.insert(String::from("else"),TokenType::Else);
        reserved.insert(String::from("false"),TokenType::False);
        reserved.insert(String::from("for"),TokenType::For);
        reserved.insert(String::from("fun"),TokenType::Fun);
        reserved.insert(String::from("if"),TokenType::If);
        reserved.insert(String::from("nil"),TokenType::Nil);
        reserved.insert(String::from("or"),TokenType::Or);
        reserved.insert(String::from("print"),TokenType::Print);
        reserved.insert(String::from("return"),TokenType::Return);
        reserved.insert(String::from("super"),TokenType::Super);
        reserved.insert(String::from("this"),TokenType::This);
        reserved.insert(String::from("true"),TokenType::True);
        reserved.insert(String::from("var"),TokenType::Var);
        reserved.insert(String::from("while"),TokenType::While);

        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            end: 0,
            line: 1,
            keywords: reserved
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
                } else if self.match_token('*') {
                    // A multi comment goes until '*/'.
                    while !self.is_at_end() {
                        if self.peek() == '*' && self.peek_next() == '/' {
                            // Consume '*/'.
                            self.advance();
                            self.advance();
                            break;
                        }

                        if self.match_token('\n') {
                            self.line += 1;
                        }

                        self.advance();
                    }
                } 
                else {
                    self.add_token(TokenType::Slash);
                }
            },
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    Lox::error(self.line, String::from("Unexpected character."));
                }
            }
        }
    }

    fn identifier(&mut self) {
        // Avoids borrowing immutable and mutable clashes.
        let c = self.peek();

        while self.is_alpha_numeric(c) {
            self.advance();
        }

        let source = self.source.clone();

        if let Some(text) = source.get(self.start..self.current) {
            let keywords = self.keywords.clone();
            let mut token_type = TokenType::Identifier;

            if let Some(value) = keywords.get_key_value(text) {
                token_type = value.1.clone();
            }

            self.add_token(token_type);
        } else {
            panic!("Failed to get identifier!");
        }
    }

    fn number(&mut self) {
        // Avoids borrowing immutable and mutable clashes.
        let c = self.peek();

        while self.is_digit(c) {
            self.advance();
        }

        /* Avoids borrowing immutable and mutable clashes.
        Hoist var c to avoid creating seperate vars. */
        let c = self.peek_next();

        // Look for a fractional part.
        if self.peek() == '.' && self.is_digit(c) {
            // Consume the ".".
            self.advance();

            /* Avoids borrowing immutable and mutable clashes.
            Hoist var c to avoid creating seperate vars. */
            let c = self.peek();

            while self.is_digit(c) {
                self.advance();
            }
        }

        let source = self.source.clone();

        // Trim the surrounding quotes.
        if let Some(value) = source.get(self.start..self.current) {
            if let Ok(number) = value.parse() {
                self.add_token_complete(TokenType::Number, Literal:: Number(number));
            } else {
                panic!("Failed to convert to number!")
            }
        } else {
            panic!("Failed to get substring from source!");
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            } else {
                self.advance();
            }
        }

        if self.is_at_end() {
            Lox::error(self.line, String::from("Unterminated string."));
            return;
        }

        // The closing ".
        self.advance();

        /* Need to create copies of required vars to avoid mutable_borrow_reservation_conflict.
        This avoids borrowing self as immutable at source.get and self.add_token_complete in
        two places. */
        let source = self.source.clone();
        let start = self.start + 1;
        let end = self.current - 1;

        // Trim the surrounding quotes.
        if let Some(value) = source.get(start..end){
            self.add_token_complete(TokenType::String, Literal::String(value.to_string()));
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

    fn peek_next(&mut self) -> char {
        if (self.current + 1) >= self.source.len() {
            return '\0';
        }

        if let Some(c) = self.source.chars().nth(self.current + 1) {
            return c;
        }

        panic!("Failed to peek next char in source!");
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        if let Some(c) = self.source.chars().nth(self.current) {
            self.current += 1;
            return c;
        }

        panic!("Failed to advance!")
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