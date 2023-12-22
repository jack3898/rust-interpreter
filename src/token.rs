use crate::{literal_type::Lit, token_type::Tok};

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: Tok,
    /**
     * The lexeme is the actual text that was matched for this token.
     */
    pub lexeme: String,
    /**
     * The literal is the memory value of the token.
     */
    pub literal: Option<Lit>,
    pub line: usize,
}

impl Token {
    // pub fn new(
    //     token_type: TokenType,
    //     lexeme: String,
    //     literal: Option<LiteralType>,
    //     line: usize,
    // ) -> Self {
    //     Self {
    //         token_type,
    //         lexeme,
    //         literal,
    //         line,
    //     }
    // }

    // pub fn to_string(&self) -> String {
    //     format!(
    //         "{:?} {} {}",
    //         self.token_type,
    //         self.lexeme,
    //         self.literal.as_ref().unwrap().to_string()
    //     )
    // }
}
