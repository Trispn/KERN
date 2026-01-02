use crate::token::{LexerError, LexerErrorType, Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
    line: usize,
    column: usize,
    errors: Vec<LexerError>,
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
            errors: Vec::new(),
        };
        lexer.read_char();
        lexer
    }

    pub fn get_errors(&self) -> &Vec<LexerError> {
        &self.errors
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
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

    #[allow(dead_code)]
    fn skip_string(&mut self) {
        // According to KERN spec, strings are not first-class citizens
        // So we treat quotes as illegal characters
        let quote_char = self.ch;
        let start_line = self.line;
        let start_column = self.column;
        let start_position = self.position;

        self.read_char(); // consume the opening quote

        // Continue until we find the closing quote or reach end of input
        while self.ch != '\0' && self.ch != quote_char {
            if self.ch == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
            self.read_char();
        }

        // If we found a closing quote, consume it
        if self.ch == quote_char {
            self.read_char();
        } else {
            // Unterminated string error
            self.errors.push(LexerError::new(
                LexerErrorType::UnterminatedString,
                format!(
                    "Unterminated string starting at line {}, column {}",
                    start_line, start_column
                ),
                start_line,
                start_column,
                start_position,
            ));
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
        let start_line = self.line;
        let start_column = self.column - 1; // Adjust for the first digit
        let start_position = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        let number_str: String = self.input[start_pos..self.position].iter().collect();
        match number_str.parse::<i64>() {
            Ok(num) => num,
            Err(_) => {
                self.errors.push(LexerError::new(
                    LexerErrorType::InvalidNumber,
                    format!("Invalid number: {}", number_str),
                    start_line,
                    start_column,
                    start_position,
                ));
                0 // Return default value in case of error
            }
        }
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
                    self.read_char(); // consume second '='
                    Token::from_str(
                        TokenType::Equal,
                        "==",
                        self.line,
                        self.column - 1,
                        self.position - 1,
                    )
                } else {
                    Token::from_str(
                        TokenType::Assignment,
                        "=",
                        self.line,
                        self.column,
                        self.position,
                    )
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char(); // consume '='
                    Token::from_str(
                        TokenType::NotEqual,
                        "!=",
                        self.line,
                        self.column - 1,
                        self.position - 1,
                    )
                } else {
                    // Error case: '!' not followed by '=' - this is an illegal character
                    let current_ch = self.ch;
                    let line = self.line;
                    let column = self.column;
                    let position = self.position;
                    self.read_char();

                    self.errors.push(LexerError::new(
                        LexerErrorType::InvalidCharacter(current_ch),
                        format!("Invalid character: '{}' (did you mean '!=')?", current_ch),
                        line,
                        column,
                        position,
                    ));

                    Token::new(
                        TokenType::Illegal(current_ch),
                        Some(current_ch.to_string()),
                        line,
                        column,
                        position,
                    )
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char(); // consume '='
                    Token::from_str(
                        TokenType::GreaterEqual,
                        ">=",
                        self.line,
                        self.column - 1,
                        self.position - 1,
                    )
                } else {
                    Token::from_str(
                        TokenType::Greater,
                        ">",
                        self.line,
                        self.column,
                        self.position,
                    )
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char(); // consume '='
                    Token::from_str(
                        TokenType::LessEqual,
                        "<=",
                        self.line,
                        self.column - 1,
                        self.position - 1,
                    )
                } else {
                    Token::from_str(TokenType::Less, "<", self.line, self.column, self.position)
                }
            }
            '{' => Token::from_str(
                TokenType::LeftBrace,
                "{",
                self.line,
                self.column,
                self.position,
            ),
            '}' => Token::from_str(
                TokenType::RightBrace,
                "}",
                self.line,
                self.column,
                self.position,
            ),
            '(' => Token::from_str(
                TokenType::LeftParen,
                "(",
                self.line,
                self.column,
                self.position,
            ),
            ')' => Token::from_str(
                TokenType::RightParen,
                ")",
                self.line,
                self.column,
                self.position,
            ),
            ',' => Token::from_str(TokenType::Comma, ",", self.line, self.column, self.position),
            '.' => Token::from_str(TokenType::Dot, ".", self.line, self.column, self.position),
            ':' => Token::from_str(TokenType::Colon, ":", self.line, self.column, self.position),
            '\0' => Token::simple(TokenType::Eof, self.line, self.column, self.position),
            _ if is_letter(self.ch) => {
                let _start_pos = self.position;
                let start_line = self.line;
                let start_column = self.column;
                let start_position = self.position;

                let ident = self.read_identifier();
                let token_type = self.lookup_identifier(&ident);
                let token_value = Some(ident.clone());

                Token::new(
                    token_type,
                    token_value,
                    start_line,
                    start_column,
                    start_position,
                )
            }
            _ if self.ch.is_ascii_digit() => {
                let start_pos = self.position;
                let start_line = self.line;
                let start_column = self.column;
                let start_position = self.position;

                let value = self.read_number();
                let number_str = self.input[start_pos..self.position]
                    .iter()
                    .collect::<String>();

                Token::new(
                    TokenType::Number(value),
                    Some(number_str.clone()),
                    start_line,
                    start_column,
                    start_position,
                )
            }
            '"' | '\'' => {
                // According to KERN spec, strings are not first-class citizens
                // So we treat quotes as illegal characters
                let current_ch = self.ch;
                let line = self.line;
                let column = self.column;
                let position = self.position;

                self.read_char(); // consume the quote

                self.errors.push(LexerError::new(
                    LexerErrorType::InvalidCharacter(current_ch),
                    format!("Strings are not supported in KERN: '{}'", current_ch),
                    line,
                    column,
                    position,
                ));

                Token::new(
                    TokenType::Illegal(current_ch),
                    Some(current_ch.to_string()),
                    line,
                    column,
                    position,
                )
            }
            _ => {
                // Handle unrecognized characters
                let current_ch = self.ch;
                let line = self.line;
                let column = self.column;
                let position = self.position;

                self.read_char();

                self.errors.push(LexerError::new(
                    LexerErrorType::InvalidCharacter(current_ch),
                    format!("Unrecognized character: '{}'", current_ch),
                    line,
                    column,
                    position,
                ));

                Token::new(
                    TokenType::Illegal(current_ch),
                    Some(current_ch.to_string()),
                    line,
                    column,
                    position,
                )
            }
        };

        self.read_char();
        token
    }

    // Add a method to peek at the next token without consuming it
    pub fn peek_next_token(&mut self) -> Token {
        // Save current state
        let saved_position = self.position;
        let saved_read_position = self.read_position;
        let saved_ch = self.ch;
        let saved_line = self.line;
        let saved_column = self.column;

        // Get the next token
        let token = self.next_token();

        // Restore state
        self.position = saved_position;
        self.read_position = saved_read_position;
        self.ch = saved_ch;
        self.line = saved_line;
        self.column = saved_column;

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

    pub fn tokenize_with_errors(&mut self) -> (Vec<Token>, Vec<LexerError>) {
        let tokens = self.tokenize_all();
        let errors = self.errors.clone();
        (tokens, errors)
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
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
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier("Farmer".to_string())
        );
        assert_eq!(tokens[2].token_type, TokenType::LeftBrace);
        assert_eq!(
            tokens[3].token_type,
            TokenType::Identifier("id".to_string())
        );
        assert_eq!(
            tokens[4].token_type,
            TokenType::Identifier("location".to_string())
        );
        assert_eq!(
            tokens[5].token_type,
            TokenType::Identifier("produce".to_string())
        );
        assert_eq!(tokens[6].token_type, TokenType::RightBrace);
        assert_eq!(tokens[7].token_type, TokenType::Eof);
    }

    #[test]
    fn test_rule_tokenization() {
        let input = "rule Name: if condition then action";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_all();

        assert_eq!(tokens[0].token_type, TokenType::Rule);
        assert_eq!(
            tokens[1].token_type,
            TokenType::Identifier("Name".to_string())
        );
        assert_eq!(tokens[2].token_type, TokenType::Colon);
        assert_eq!(tokens[3].token_type, TokenType::If);
        assert_eq!(
            tokens[4].token_type,
            TokenType::Identifier("condition".to_string())
        );
        assert_eq!(tokens[5].token_type, TokenType::Then);
        assert_eq!(
            tokens[6].token_type,
            TokenType::Identifier("action".to_string())
        );
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

        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier("num".to_string())
        );
        assert_eq!(tokens[1].token_type, TokenType::Number(42));
        assert_eq!(tokens[2].token_type, TokenType::Eof);
    }
}
