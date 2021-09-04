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
        let literal : String  = match self.literal.clone() {
            Literal::Bool(value) => format!("{}", value),
            Literal::Number(value) => {
                let mut x = value.to_string();

                if value.fract() == 0.0 {
                    x = format!("{}.0", value);
                }
                
                x
            },
            Literal::String(value) => value,
            Literal::Nil => String::from("null"),
        };

        format!(
            "{:?} {} {}", 
            self.token_type,
            self.lexeme,
            literal,
        )
    }
}