use crate::shared::source_location::SourceLocation;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Entity,
    Rule,
    Flow,
    Constraint,
    If,
    Then,
    Else,
    True,
    False,
    Sym,
    Num,
    Bool,
    Vec,
    Ref,
    Ctx,
    
    // Literals
    Identifier,
    StringLiteral,
    NumberLiteral,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Ampersand,
    AmpersandAmpersand,
    Pipe,
    PipePipe,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Arrow,
    
    // Special
    Eof,
    Error,
}

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