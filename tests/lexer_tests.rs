#[cfg(test)]
mod lexer_tests {
    use kern_lexer::lexer::Lexer;
    use kern_lexer::token::Token;
    use kern_lexer::token_kind::TokenKind;
    use crate::assertions::{assert_equal, assert_true, AssertionResult};

    fn run_lexer_test(input: &str, expected_tokens: Vec<TokenKind>) -> AssertionResult {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        // Filter out whitespace tokens for comparison
        let actual_tokens: Vec<TokenKind> = tokens
            .into_iter()
            .filter(|token| token.kind != TokenKind::Whitespace)
            .map(|token| token.kind)
            .collect();
        
        assert_equal(actual_tokens, expected_tokens, "Token sequence mismatch")
    }

    #[test]
    fn test_entity_declaration() {
        let input = "entity User { name age }";
        let expected = vec![
            TokenKind::Entity,
            TokenKind::Identifier,
            TokenKind::LeftBrace,
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::RightBrace,
            TokenKind::Eof,
        ];
        
        let result = run_lexer_test(input, expected);
        assert!(result.success, "{}", result.message.unwrap_or_default());
    }

    #[test]
    fn test_rule_declaration() {
        let input = "rule ValidateUser: if user.valid then approve(user)";
        let expected = vec![
            TokenKind::Rule,
            TokenKind::Identifier,
            TokenKind::Colon,
            TokenKind::If,
            TokenKind::Identifier,
            TokenKind::Dot,
            TokenKind::Identifier,
            TokenKind::Then,
            TokenKind::Identifier,
            TokenKind::LeftParen,
            TokenKind::Identifier,
            TokenKind::RightParen,
            TokenKind::Eof,
        ];
        
        let result = run_lexer_test(input, expected);
        assert!(result.success, "{}", result.message.unwrap_or_default());
    }

    #[test]
    fn test_flow_declaration() {
        let input = "flow ProcessData { step1: load(), step2: transform() }";
        let expected = vec![
            TokenKind::Flow,
            TokenKind::Identifier,
            TokenKind::LeftBrace,
            TokenKind::Identifier,
            TokenKind::Colon,
            TokenKind::Identifier,
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::Comma,
            TokenKind::Identifier,
            TokenKind::Colon,
            TokenKind::Identifier,
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::RightBrace,
            TokenKind::Eof,
        ];
        
        let result = run_lexer_test(input, expected);
        assert!(result.success, "{}", result.message.unwrap_or_default());
    }

    #[test]
    fn test_constraint_declaration() {
        let input = "constraint ValidEmail: user.email.contains(\"@\")";
        let expected = vec![
            TokenKind::Constraint,
            TokenKind::Identifier,
            TokenKind::Colon,
            TokenKind::Identifier,
            TokenKind::Dot,
            TokenKind::Identifier,
            TokenKind::Dot,
            TokenKind::Identifier,
            TokenKind::LeftParen,
            TokenKind::String,
            TokenKind::RightParen,
            TokenKind::Eof,
        ];
        
        let result = run_lexer_test(input, expected);
        assert!(result.success, "{}", result.message.unwrap_or_default());
    }

    #[test]
    fn test_numbers() {
        let input = "num1 = 42 num2 = -17";
        let expected = vec![
            TokenKind::Identifier,
            TokenKind::Equals,
            TokenKind::Number,
            TokenKind::Identifier,
            TokenKind::Equals,
            TokenKind::Number,
            TokenKind::Eof,
        ];
        
        let result = run_lexer_test(input, expected);
        assert!(result.success, "{}", result.message.unwrap_or_default());
    }

    #[test]
    fn test_strings() {
        let input = r#"str1 = "hello" str2 = "world""#;
        let expected = vec![
            TokenKind::Identifier,
            TokenKind::Equals,
            TokenKind::String,
            TokenKind::Identifier,
            TokenKind::Equals,
            TokenKind::String,
            TokenKind::Eof,
        ];
        
        let result = run_lexer_test(input, expected);
        assert!(result.success, "{}", result.message.unwrap_or_default());
    }

    #[test]
    fn test_booleans() {
        let input = "flag1 = true flag2 = false";
        let expected = vec![
            TokenKind::Identifier,
            TokenKind::Equals,
            TokenKind::True,
            TokenKind::Identifier,
            TokenKind::Equals,
            TokenKind::False,
            TokenKind::Eof,
        ];
        
        let result = run_lexer_test(input, expected);
        assert!(result.success, "{}", result.message.unwrap_or_default());
    }

    #[test]
    fn test_symbols() {
        let input = "sym1 sym2 sym3";
        let expected = vec![
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::Eof,
        ];
        
        let result = run_lexer_test(input, expected);
        assert!(result.success, "{}", result.message.unwrap_or_default());
    }

    #[test]
    fn test_operators() {
        let input = "a == b and c != d or e > f and g < h";
        let expected = vec![
            TokenKind::Identifier,
            TokenKind::EqualsEquals,
            TokenKind::Identifier,
            TokenKind::And,
            TokenKind::Identifier,
            TokenKind::NotEquals,
            TokenKind::Identifier,
            TokenKind::Or,
            TokenKind::Identifier,
            TokenKind::GreaterThan,
            TokenKind::Identifier,
            TokenKind::And,
            TokenKind::Identifier,
            TokenKind::LessThan,
            TokenKind::Identifier,
            TokenKind::Eof,
        ];
        
        let result = run_lexer_test(input, expected);
        assert!(result.success, "{}", result.message.unwrap_or_default());
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let expected = vec![TokenKind::Eof];
        
        let result = run_lexer_test(input, expected);
        assert!(result.success, "{}", result.message.unwrap_or_default());
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "  entity   User  {  name  }  ";
        let expected = vec![
            TokenKind::Entity,
            TokenKind::Identifier,
            TokenKind::LeftBrace,
            TokenKind::Identifier,
            TokenKind::RightBrace,
            TokenKind::Eof,
        ];
        
        let result = run_lexer_test(input, expected);
        assert!(result.success, "{}", result.message.unwrap_or_default());
    }

    #[test]
    fn test_comments() {
        let input = "// This is a comment\nentity User { name } // Another comment";
        let expected = vec![
            TokenKind::Entity,
            TokenKind::Identifier,
            TokenKind::LeftBrace,
            TokenKind::Identifier,
            TokenKind::RightBrace,
            TokenKind::Eof,
        ];
        
        let result = run_lexer_test(input, expected);
        assert!(result.success, "{}", result.message.unwrap_or_default());
    }
}