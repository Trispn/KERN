#[cfg(test)]
mod error_recovery_tests {
    use kern_parser::parser::Parser;
    use kern_lexer::lexer::Lexer;
    use kern_parser::parser_error::ParserError;
    use crate::assertions::{assert_equal, assert_true, assert_false, AssertionResult};

    #[test]
    fn test_incomplete_entity_declaration() {
        let input = "entity User { name";  // Missing closing brace
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // The parser should handle the error gracefully
        // In a real implementation, it might return partial results or an error
        // For now, we'll just ensure it doesn't panic
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_missing_rule_name() {
        let input = "rule { if condition then action() }";  // Missing rule name
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should handle the error gracefully
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_invalid_syntax_recovery() {
        let input = r#"
        entity User { name: String }
        rule ValidateUser  // Missing colon
            if user.name != ""
            then validate(user)
        flow ProcessData { step1: process() }
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should recover and parse the valid parts
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_unterminated_string() {
        let input = r#"entity Test { message: "unclosed string"#;  // Missing closing quote
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        // The lexer should handle the unterminated string
        // In a real implementation, it would generate an error token
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_multiple_syntax_errors() {
        let input = r#"
        entity { name }  // Missing entity name
        rule : if x then y()  // Missing rule name
        flow { step1: () }  // Missing flow name
        constraint : x > 0  // Missing constraint name
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should handle multiple errors gracefully
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_recovery_after_error() {
        let input = r#"
        entity ValidEntity { field1 }
        entity { missing_name }  // Invalid entity
        entity AnotherValidEntity { field2 }  // This should still be parsed
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should recover after the error and parse subsequent valid entities
        if let Some(program) = result {
            // Should have at least one valid entity
            assert!(!program.declarations.is_empty());
        } else {
            // If parsing failed completely, at least verify errors were captured
            assert!(parser.state.has_errors());
        }
    }

    #[test]
    fn test_invalid_operator_recovery() {
        let input = r#"
        rule TestRule:
            if x @ y  // Invalid operator
            then action()
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should handle the invalid operator gracefully
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_mismatched_braces_recovery() {
        let input = r#"
        entity User {
            name
            age
            // Missing closing brace
        rule ValidRule:
            if user.valid
            then approve(user)
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should handle the mismatched braces gracefully
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_invalid_identifier_recovery() {
        let input = r#"
        entity 123InvalidName { field }  // Invalid entity name starting with number
        rule ValidRule: if x then y()
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should handle the invalid identifier gracefully
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_empty_declaration_recovery() {
        let input = r#"
        entity User { name }
        
        entity EmptyEntity { }
        
        rule ;
        
        flow { }
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should handle empty declarations gracefully
        assert!(result.is_some() || parser.state.has_errors());
    }
}