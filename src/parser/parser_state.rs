use crate::lexer::token::Token;
use crate::shared::diagnostics::{Diagnostics, ErrorCode};

#[derive(Debug, Clone)]
pub struct ParserState {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub diagnostics: Diagnostics,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl ParserState {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            diagnostics: Diagnostics::new(),
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn previous_token(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous_token()
    }

    pub fn is_at_end(&self) -> bool {
        self.current_token().kind == crate::lexer::token_kind::TokenKind::Eof
    }

    pub fn match_token(&mut self, kinds: &[crate::lexer::token_kind::TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn check(&self, kind: &crate::lexer::token_kind::TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.current_token().kind == *kind
    }

    pub fn consume(&mut self, kind: crate::lexer::token_kind::TokenKind, error_code: ErrorCode, message: &str) -> Option<Token> {
        if self.check(&kind) {
            return Some(self.advance().clone());
        }

        self.diagnostics.add_error(
            error_code,
            message.to_string(),
            self.current_token().location.clone(),
            "unknown".to_string(),
        );
        self.had_error = true;
        None
    }

    pub fn synchronize(&mut self) {
        self.panic_mode = false;
        self.had_error = true;

        // Skip tokens until we find a synchronization point
        while !self.is_at_end() {
            // Synchronization tokens - places where we can safely resume parsing
            match self.current_token().kind {
                crate::lexer::token_kind::TokenKind::Semicolon => {
                    // Consume the semicolon and return
                    self.advance();
                    return;
                }
                crate::lexer::token_kind::TokenKind::Entity |
                crate::lexer::token_kind::TokenKind::Rule |
                crate::lexer::token_kind::TokenKind::Flow |
                crate::lexer::token_kind::TokenKind::Constraint => {
                    // Found the start of a new declaration, we can resume parsing
                    return;
                }
                _ => {
                    // Skip this token and check the next one
                    self.advance();
                }
            }
        }
    }

    pub fn error_at_current(&mut self, error_code: ErrorCode, message: String) {
        self.diagnostics.add_error(
            error_code,
            message,
            self.current_token().location.clone(),
            "unknown".to_string(),
        );
        self.had_error = true;
    }
}