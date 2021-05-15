mod token_type;
mod literal;

pub use token_type::TokenType;
pub use literal::Literal;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: u32
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line : u32) -> Self {
        Token { token_type, lexeme, literal, line}
    }

    pub fn to_string(&mut self) -> String {
        String::from(
            format!(
                "{:?} {} {:?}", 
                self.token_type,
                self.lexeme,
                self.literal,
            )
        )
    }
}