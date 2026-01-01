// Simple test to verify the lexer functionality without using cargo
// This can be used to manually test the lexer implementation

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Entity,
    Rule,
    Flow,
    Constraint,
    If,
    Then,
    Else,
    Loop,
    Break,
    Halt,
    And,
    Or,

    // Identifiers and literals
    Identifier(String),
    Number(i64),

    // Symbols
    Colon,
    Comma,
    Dot,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Assignment,

    // Error
    Illegal(char),

    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub position: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize, position: usize) -> Self {
        Token {
            token_type,
            line,
            column,
            position,
        }
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 0,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        
        self.position = self.read_position;
        self.read_position += 1;
        self.column += 1;
        
        if self.ch == '\n' {
            self.line += 1;
            self.column = 0;
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            if self.ch == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        while is_letter(self.ch) || self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[start_pos..self.position].iter().collect()
    }

    fn read_number(&mut self) -> i64 {
        let start_pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[start_pos..self.position].iter().collect::<String>().parse().unwrap()
    }

    fn lookup_identifier(&self, identifier: &str) -> TokenType {
        match identifier {
            "entity" => TokenType::Entity,
            "rule" => TokenType::Rule,
            "flow" => TokenType::Flow,
            "constraint" => TokenType::Constraint,
            "if" => TokenType::If,
            "then" => TokenType::Then,
            "else" => TokenType::Else,
            "loop" => TokenType::Loop,
            "break" => TokenType::Break,
            "halt" => TokenType::Halt,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            _ => TokenType::Identifier(identifier.to_string()),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char(); // consume '='
                    Token::new(TokenType::Equal, self.line, self.column - 1, self.position - 1)
                } else {
                    Token::new(TokenType::Assignment, self.line, self.column, self.position)
                }
            },
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char(); // consume '='
                    Token::new(TokenType::NotEqual, self.line, self.column - 1, self.position - 1)
                } else {
                    // Error case: '!' not followed by '=' - this is an illegal character
                    let current_ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::Illegal(current_ch), self.line, self.column, self.position)
                }
            },
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char(); // consume '='
                    Token::new(TokenType::GreaterEqual, self.line, self.column - 1, self.position - 1)
                } else {
                    Token::new(TokenType::Greater, self.line, self.column, self.position)
                }
            },
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char(); // consume '='
                    Token::new(TokenType::LessEqual, self.line, self.column - 1, self.position - 1)
                } else {
                    Token::new(TokenType::Less, self.line, self.column, self.position)
                }
            },
            '{' => Token::new(TokenType::LeftBrace, self.line, self.column, self.position),
            '}' => Token::new(TokenType::RightBrace, self.line, self.column, self.position),
            '(' => Token::new(TokenType::LeftParen, self.line, self.column, self.position),
            ')' => Token::new(TokenType::RightParen, self.line, self.column, self.position),
            ',' => Token::new(TokenType::Comma, self.line, self.column, self.position),
            '.' => Token::new(TokenType::Dot, self.line, self.column, self.position),
            ':' => Token::new(TokenType::Colon, self.line, self.column, self.position),
            '\0' => Token::new(TokenType::Eof, self.line, self.column, self.position),
            _ if is_letter(self.ch) => {
                let ident = self.read_identifier();
                return Token::new(self.lookup_identifier(&ident), self.line - if self.ch == '\n' { 1 } else { 0 }, 
                                self.column - ident.len(), self.position - ident.len());
            },
            _ if self.ch.is_ascii_digit() => {
                let value = self.read_number();
                return Token::new(TokenType::Number(value), self.line, 
                                self.column - (value.to_string().len()), self.position - (value.to_string().len()));
            },
            _ => {
                // Handle unrecognized characters
                let current_ch = self.ch;
                self.read_char();
                Token::new(TokenType::Illegal(current_ch), self.line, self.column, self.position)
            }
        };

        self.read_char();
        token
    }

    pub fn tokenize_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token();
            tokens.push(token.clone());
            
            if matches!(token.token_type, TokenType::Eof) {
                break;
            }
        }
        
        tokens
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn main() {
    let input = r#"
entity Farmer {
    id
    location
    produce
}

rule CheckLocation:
    if farmer.location == "valid"
    then approve_farmer(farmer)
"#;

    let mut lexer = Lexer::new(input);
    
    println!("Tokenizing input:\n{}", input);
    println!("\nTokens:");
    
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        
        if matches!(token.token_type, TokenType::Eof) {
            break;
        }
    }
    
    // Test specific cases
    println!("\n--- Testing specific cases ---");
    
    // Test identifier and keyword recognition
    let mut lexer2 = Lexer::new("entity rule if then");
    let tokens2 = lexer2.tokenize_all();
    println!("Keywords test: {:?}", tokens2);
    
    // Test operators
    let mut lexer3 = Lexer::new("== != > < >= <=");
    let tokens3 = lexer3.tokenize_all();
    println!("Operators test: {:?}", tokens3);
    
    // Test numbers
    let mut lexer4 = Lexer::new("42 -123");
    let tokens4 = lexer4.tokenize_all();
    println!("Numbers test: {:?}", tokens4);
    
    // Test error case
    let mut lexer5 = Lexer::new("entity test ! invalid");
    let tokens5 = lexer5.tokenize_all();
    println!("Error handling test: {:?}", tokens5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_tokenization() {
        let input = "entity Farmer { id location produce }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_all();

        assert_eq!(tokens[0].token_type, TokenType::Entity);
        assert_eq!(tokens[1].token_type, TokenType::Identifier("Farmer".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[3].token_type, TokenType::Identifier("id".to_string()));
        assert_eq!(tokens[4].token_type, TokenType::Identifier("location".to_string()));
        assert_eq!(tokens[5].token_type, TokenType::Identifier("produce".to_string()));
        assert_eq!(tokens[6].token_type, TokenType::RightBrace);
        assert_eq!(tokens[7].token_type, TokenType::Eof);
    }

    #[test]
    fn test_rule_tokenization() {
        let input = "rule Name: if condition then action";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_all();

        assert_eq!(tokens[0].token_type, TokenType::Rule);
        assert_eq!(tokens[1].token_type, TokenType::Identifier("Name".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::Colon);
        assert_eq!(tokens[3].token_type, TokenType::If);
        assert_eq!(tokens[4].token_type, TokenType::Identifier("condition".to_string()));
        assert_eq!(tokens[5].token_type, TokenType::Then);
        assert_eq!(tokens[6].token_type, TokenType::Identifier("action".to_string()));
    }

    #[test]
    fn test_operators() {
        let input = "== != > < >= <=";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_all();

        assert_eq!(tokens[0].token_type, TokenType::Equal);
        assert_eq!(tokens[1].token_type, TokenType::NotEqual);
        assert_eq!(tokens[2].token_type, TokenType::Greater);
        assert_eq!(tokens[3].token_type, TokenType::Less);
        assert_eq!(tokens[4].token_type, TokenType::GreaterEqual);
        assert_eq!(tokens[5].token_type, TokenType::LessEqual);
        assert_eq!(tokens[6].token_type, TokenType::Eof);
    }

    #[test]
    fn test_numbers() {
        let input = "num 42";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_all();

        assert_eq!(tokens[0].token_type, TokenType::Identifier("num".to_string()));
        assert_eq!(tokens[1].token_type, TokenType::Number(42));
        assert_eq!(tokens[2].token_type, TokenType::Eof);
    }
}