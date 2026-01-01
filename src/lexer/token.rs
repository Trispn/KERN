use crate::shared::source_location::SourceLocation;
pub use crate::lexer::token_kind::TokenKind;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub location: SourceLocation,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, location: SourceLocation) -> Self {
        Self { kind, lexeme, location }
    }
}