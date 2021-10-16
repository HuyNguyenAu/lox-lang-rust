use std::{collections::HashMap};

use crate::Lox;

mod scanner;
mod expression;

use scanner::{Token, TokenType};
use expression::{Expression, Binary};

pub struct Parser {
    tokens: Vec<Token>,
    current: u32,
} 

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0, // Offset of the current token being parsed.
        }
    }

    // Rule: equality -> comparison ( ( "!=" | "==" ) comparison )* ;.
    fn equality(&mut self) -> Expression {
        let mut expression: Expression = self.comparison();

        while self.match_tokens(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expression = Binary.new(expression, operator, right);
        }

        expression
    }

    // Rule: comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;.
    fn comparison(&mut self) -> Expression {
        let mut expression: Expression = self.term();

        while self.match_tokens(vec![TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.term();
            expression = Binary.new(expression, operator, right);
        }

        expression
    }

    // Rule: term -> factor ( ( "-" | "+" ) factor )* ;.
    fn term(&mut self) -> Expression {
        let mut expression: Expression = self.factor();

        while self.match_tokens(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expression = Binary.new(expression, operator, right);
        }

        expression
    }

    // Rule: factor -> unary ( ( "/" | "*" ) unary )* ;.
    fn factor(&mut self) -> Expression {
        let mut expression: Expression = self.unary();

        while self.match_tokens(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expression = Binary.new(expression, operator, right);
        }

        expression
    }

    // Rule: unary -> ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Expression {
        if self.match_tokens(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Unary.new(operator, right);
        }

        return self.primary();
    }

    // Rule: primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Expression {
        if self.match_tokens(vec![TokenType::FALSE]) {
            return Literal.new(false);
        }
        if self.match_tokens(vec![TokenType::TRUE]) {
            return Literal.new(true);
        }
        if self.match_tokens(vec![TokenType::NIL]) {
            return Literal.new(null);
        }

        if self.match_tokens(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Literal.new(self.previous().literal);
        }

        if (self.match_tokens(vec![TokenType::LEFT_PAREN])) {
            let expression = self.expression();
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            return Expression.Grouping(expression);
        }
    }


    /* Check if the current token has any of the given types.
    If so, return true and consume the token, otherwise return false. */
    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Token {
        if self.check(token_type) {
            return self.advance();
        }

    }

    // Return true if the current token is of the given type.
    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    // Consume the current token and return it.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    // Check if we've run out of tokens to parse.
    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    // Return the current token we have yet to consume.
    fn peek(&mut self) -> Token {
        self.tokens[self.current]
    }

    // Return the most recent token we have consumed.
    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1]
    }
}
