pub mod lexer {
    pub mod token;
    pub mod lexer;
    pub mod keywords;
    pub mod token_kind;
}

pub mod parser {
    pub mod parser;
    pub mod parser_state;
    pub mod ast_nodes;
    pub mod entity_parser;
    pub mod rule_parser;
    pub mod flow_parser;
    pub mod constraint_parser;
    pub mod expression_parser;
    pub mod precedence;
    pub mod parser_error;
}

pub mod shared {
    pub mod source_location;
    pub mod diagnostics;
    pub mod string_interner;
    pub mod config;
}

use lexer::lexer::Lexer;
use parser::parser::Parser;
use shared::diagnostics::Diagnostics;

pub struct KernCompiler {
    pub diagnostics: Diagnostics,
}

impl KernCompiler {
    pub fn new() -> Self {
        Self {
            diagnostics: Diagnostics::new(),
        }
    }

    pub fn compile(&mut self, source: &str) -> Option<parser::ast_nodes::Program> {
        // Tokenize the source
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        // Check for lexer errors
        for token in &tokens {
            if token.kind == lexer::token::TokenKind::Error {
                self.diagnostics.add_error(
                    shared::diagnostics::ErrorCode::UnexpectedCharacter,
                    token.lexeme.clone(),
                    token.location.clone(),
                    "unknown".to_string(),
                );
            }
        }

        // Parse the tokens
        let mut parser = Parser::new(tokens);
        let program = parser.parse();

        // Combine diagnostics
        self.diagnostics.diagnostics.extend(parser.state.diagnostics.diagnostics);

        if self.diagnostics.has_errors() {
            None
        } else {
            Some(program)
        }
    }
}