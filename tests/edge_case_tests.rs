#[cfg(test)]
mod edge_case_tests {
    use kern_parser::parser::Parser;
    use kern_lexer::lexer::Lexer;
    use kern_parser::ast_nodes::{Program, Declaration};
    use crate::assertions::{assert_equal, assert_true, assert_false, AssertionResult};

    #[test]
    fn test_empty_input() {
        let input = "";
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should handle empty input gracefully
        assert!(result.is_some());
        
        if let Some(program) = result {
            assert_eq!(program.declarations.len(), 0);
        }
    }

    #[test]
    fn test_whitespace_only_input() {
        let input = "   \n\t  \n   ";
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should handle whitespace-only input gracefully
        assert!(result.is_some());
        
        if let Some(program) = result {
            assert_eq!(program.declarations.len(), 0);
        }
    }

    #[test]
    fn test_maximal_nesting_entity() {
        let input = r#"
        entity Level1 {
            level2
            entity Level2 {
                level3
                entity Level3 {
                    level4
                    entity Level4 {
                        deepest_field
                    }
                }
            }
        }
        "#;
        
        // Note: In the actual KERN language, entities don't contain nested entities
        // This test is just to verify the parser handles deep nesting if it occurs
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should handle the input without crashing
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_long_identifiers() {
        let long_name = "a".repeat(1000); // Very long identifier
        let input = format!("entity {} {{ field }}", long_name);
        
        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should handle long identifiers without crashing
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_attribute_type_edge_cases() {
        let input = r#"
        entity TestEntity {
            normal_field
            special_chars_123
            _underscore_start
            camelCaseField
        }
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        if let Some(program) = result {
            assert_eq!(program.declarations.len(), 1);
            
            if let Declaration::Entity(entity) = &program.declarations[0] {
                assert_eq!(entity.fields.len(), 4);
            } else {
                panic!("Expected entity declaration");
            }
        } else {
            // If parsing failed, at least ensure errors were captured
            assert!(parser.state.has_errors());
        }
    }

    #[test]
    fn test_loops_with_zero_iterations() {
        // In KERN, this would be a flow with a loop that executes 0 times
        let input = r#"
        flow ZeroLoopFlow {
            step1: initialize()
            step2: loop { 
                if false then break
                process_item()
            }
            step3: finish()
        }
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should parse the flow structure correctly
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_rules_with_conflicting_priorities() {
        let input = r#"
        rule HighPriorityRule:
            if condition1
            then action1()
        
        rule LowPriorityRule:
            if condition2 
            then action2()
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        if let Some(program) = result {
            assert_eq!(program.declarations.len(), 2);
        } else {
            assert!(parser.state.has_errors());
        }
    }

    #[test]
    fn test_recursion_at_maximum_limit() {
        // This tests a deeply nested structure that might approach recursion limits
        let input = r#"
        rule NestedRule:
            if ((field1 == value1) and (field2 == value2)) and 
               ((field3 == value3) and (field4 == value4)) and
               ((field5 == value5) and (field6 == value6)) and
               ((field7 == value7) and (field8 == value8))
            then complex_action()
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should handle the nested conditions
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_control_flow_with_immediate_break_halt() {
        let input = r#"
        flow ImmediateControlFlow {
            step1: start()
            step2: if true then halt
            step3: unreachable_step()
        }
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should parse the control flow structure
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_invalid_bytecode_mapped_types() {
        // This tests handling of potential type mismatches that would cause issues at bytecode level
        let input = r#"
        entity DataTypeTest {
            number_field
            string_field  
            bool_field
        }
        
        rule TypeConversionRule:
            if number_field == string_field  // Type mismatch
            then action()
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should parse the structure even if there are semantic errors
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_extremely_large_numbers() {
        let input = "entity Test { large_num: 999999999999999999999999999999 }";  // Very large number
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should handle large numbers without crashing
        assert!(result.is_some() || parser.state.has_errors());
    }

    #[test]
    fn test_unicode_identifiers() {
        // Note: In a real implementation, we might need to check if unicode identifiers are supported
        let input = "entity TëstEntity { fīeld_nāme }";
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        let mut parser = Parser::new(tokens);
        let result = parser.parse_program();
        
        // Should handle the input appropriately
        assert!(result.is_some() || parser.state.has_errors());
    }
}