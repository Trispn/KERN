#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords from KERN grammar
    Entity,      // "entity"
    Rule,        // "rule"
    Flow,        // "flow"
    Constraint,  // "constraint"
    If,          // "if"
    Then,        // "then"
    Else,        // "else"
    Loop,        // "loop"
    Break,       // "break"
    Halt,        // "halt"
    And,         // "and"
    Or,          // "or"

    // Identifiers and literals
    Identifier(String),  // letter, { letter | digit }
    Number(i64),         // [ "-" ] , digit , { digit }

    // Symbols and operators
    Colon,               // ":"
    Comma,               // ","
    Dot,                 // "."
    LeftBrace,           // "{"
    RightBrace,          // "}"
    LeftParen,           // "("
    RightParen,          // ")"
    Equal,               // "=="
    NotEqual,            // "!="
    Greater,             // ">"
    Less,                // "<"
    GreaterEqual,        // ">="
    LessEqual,           // "<="
    Assignment,          // "=" (for assignments)

    // Logical operators (as separate tokens if needed)
    LogicalAnd,          // "and" keyword already defined
    LogicalOr,           // "or" keyword already defined

    // Special tokens
    Whitespace,          // Space, tab, etc.
    Newline,             // newline (though handled in lexer)

    // Error token
    Illegal(char),       // For invalid/unrecognized characters

    // End of file
    Eof,                 // End of input
}

// Define error types for better error reporting
#[derive(Debug, Clone, PartialEq)]
pub enum LexerErrorType {
    InvalidCharacter(char),
    UnterminatedString,
    InvalidNumber,
    UnexpectedEof,
}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub error_type: LexerErrorType,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub position: usize,
}

impl LexerError {
    pub fn new(error_type: LexerErrorType, message: String, line: usize, column: usize, position: usize) -> Self {
        LexerError {
            error_type,
            message,
            line,
            column,
            position,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,  // Optional string representation of the token value
    pub line: usize,
    pub column: usize,
    pub position: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>, line: usize, column: usize, position: usize) -> Self {
        Token {
            token_type,
            value,
            line,
            column,
            position,
        }
    }
    
    pub fn from_str(token_type: TokenType, value: &str, line: usize, column: usize, position: usize) -> Self {
        Token {
            token_type,
            value: Some(value.to_string()),
            line,
            column,
            position,
        }
    }
    
    pub fn simple(token_type: TokenType, line: usize, column: usize, position: usize) -> Self {
        Token {
            token_type,
            value: None,
            line,
            column,
            position,
        }
    }
}

// Helper functions for token classification
impl TokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(self, 
            TokenType::Entity | TokenType::Rule | TokenType::Flow | 
            TokenType::Constraint | TokenType::If | TokenType::Then | 
            TokenType::Else | TokenType::Loop | TokenType::Break | 
            TokenType::Halt | TokenType::And | TokenType::Or
        )
    }
    
    pub fn is_operator(&self) -> bool {
        matches!(self, 
            TokenType::Equal | TokenType::NotEqual | TokenType::Greater | 
            TokenType::Less | TokenType::GreaterEqual | TokenType::LessEqual |
            TokenType::Assignment
        )
    }
    
    pub fn is_delimiter(&self) -> bool {
        matches!(self, 
            TokenType::Colon | TokenType::Comma | TokenType::Dot | 
            TokenType::LeftBrace | TokenType::RightBrace | 
            TokenType::LeftParen | TokenType::RightParen
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_classification() {
        assert!(TokenType::Entity.is_keyword());
        assert!(TokenType::Rule.is_keyword());
        assert!(!TokenType::Identifier("test".to_string()).is_keyword());
    }

    #[test]
    fn test_operator_classification() {
        assert!(TokenType::Equal.is_operator());
        assert!(TokenType::NotEqual.is_operator());
        assert!(!TokenType::Entity.is_operator());
    }

    #[test]
    fn test_delimiter_classification() {
        assert!(TokenType::LeftBrace.is_delimiter());
        assert!(TokenType::Comma.is_delimiter());
        assert!(!TokenType::Entity.is_delimiter());
    }

    #[test]
    fn test_token_creation() {
        let test_str = "test".to_string();
        let token = Token::new(TokenType::Identifier(test_str.clone()), Some(test_str.clone()), 1, 1, 0);
        assert_eq!(token.token_type, TokenType::Identifier(test_str.clone()));
        assert_eq!(token.value, Some(test_str.clone()));

        let simple_token = Token::simple(TokenType::Colon, 1, 1, 0);
        assert_eq!(simple_token.token_type, TokenType::Colon);
        assert_eq!(simple_token.value, None);
    }
}