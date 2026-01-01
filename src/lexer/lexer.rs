use crate::lexer::token::{Token};
use crate::shared::source_location::SourceLocation;
use crate::lexer::keywords::is_keyword;
use crate::lexer::token_kind::TokenKind;

pub struct Lexer {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            if let Some(token) = self.scan_token() {
                tokens.push(token);
            }
        }

        tokens.push(Token::new(
            TokenKind::Eof,
            "".to_string(),
            self.get_location(),
        ));

        tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance();
        match c {
            '(' => Some(self.make_token(TokenKind::LeftParen)),
            ')' => Some(self.make_token(TokenKind::RightParen)),
            '{' => Some(self.make_token(TokenKind::LeftBrace)),
            '}' => Some(self.make_token(TokenKind::RightBrace)),
            '[' => Some(self.make_token(TokenKind::LeftBracket)),
            ']' => Some(self.make_token(TokenKind::RightBracket)),
            ';' => Some(self.make_token(TokenKind::Semicolon)),
            ',' => Some(self.make_token(TokenKind::Comma)),
            '.' => Some(self.make_token(TokenKind::Dot)),
            '-' => {
                if self.match_char('>') {
                    Some(self.make_token(TokenKind::Arrow))
                } else {
                    Some(self.make_token(TokenKind::Minus))
                }
            }
            '+' => Some(self.make_token(TokenKind::Plus)),
            '*' => Some(self.make_token(TokenKind::Star)),
            '/' => {
                if self.match_char('/') {
                    // Single-line comment
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    None // Don't return a token for comments
                } else {
                    Some(self.make_token(TokenKind::Slash))
                }
            }
            '!' => {
                if self.match_char('=') {
                    Some(self.make_token(TokenKind::BangEqual))
                } else {
                    Some(self.make_token(TokenKind::Bang))
                }
            }
            '=' => {
                if self.match_char('=') {
                    Some(self.make_token(TokenKind::EqualEqual))
                } else {
                    Some(self.make_token(TokenKind::Equal))
                }
            }
            '<' => {
                if self.match_char('=') {
                    Some(self.make_token(TokenKind::LessEqual))
                } else {
                    Some(self.make_token(TokenKind::Less))
                }
            }
            '>' => {
                if self.match_char('=') {
                    Some(self.make_token(TokenKind::GreaterEqual))
                } else {
                    Some(self.make_token(TokenKind::Greater))
                }
            }
            '&' => {
                if self.match_char('&') {
                    Some(self.make_token(TokenKind::AmpersandAmpersand))
                } else {
                    Some(self.make_token(TokenKind::Ampersand))
                }
            }
            '|' => {
                if self.match_char('|') {
                    Some(self.make_token(TokenKind::PipePipe))
                } else {
                    Some(self.make_token(TokenKind::Pipe))
                }
            }
            '%' => Some(self.make_token(TokenKind::Percent)),
            ':' => Some(self.make_token(TokenKind::Colon)),
            ' ' | '\r' | '\t' => None, // Skip whitespace
            '\n' => {
                self.line += 1;
                self.column = 1;
                None
            }
            '"' => Some(self.string()),
            '0'..='9' => Some(self.number()),
            'a'..='z' | 'A'..='Z' | '_' => Some(self.identifier()),
            _ => Some(Token::new(
                TokenKind::Error,
                format!("Unexpected character: {}", c),
                self.get_location(),
            )),
        }
    }

    fn string(&mut self) -> Token {
        let start_location = self.get_location();
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Token::new(
                TokenKind::Error,
                "Unterminated string".to_string(),
                start_location,
            );
        }

        // Consume the closing "
        self.advance();

        let value = self.source[self.start + 1..self.current - 1].iter().collect();
        Token::new(TokenKind::StringLiteral, value, start_location)
    }

    fn number(&mut self) -> Token {
        let start_location = self.get_location();

        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        Token::new(TokenKind::NumberLiteral, value, start_location)
    }

    fn identifier(&mut self) -> Token {
        let start_location = self.get_location();

        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        
        match is_keyword(&text) {
            Some(keyword_kind) => Token::new(keyword_kind, text, start_location),
            None => Token::new(TokenKind::Identifier, text, start_location),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        self.column += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, kind: TokenKind) -> Token {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        Token::new(kind, lexeme, self.get_location())
    }

    fn get_location(&self) -> SourceLocation {
        SourceLocation::new(self.line, self.column, self.current)
    }
}