mod common;
use common::assertions::{assert_equal, assert_false, assert_true, AssertionResult};
use kern_parser::ast::{Definition, Program};
use kern_parser::parser::{ParseError, Parser};

#[test]
fn test_empty_input() {
    let input = "";
    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    match result {
        Ok(program) => assert_eq!(program.definitions.len(), 0),
        Err(_) => panic!("Expected empty program, got error"),
    }
}

#[test]
fn test_whitespace_only_input() {
    let input = "   \n\t  \n   ";
    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    match result {
        Ok(program) => assert_eq!(program.definitions.len(), 0),
        Err(_) => panic!("Expected empty program, got error"),
    }
}

#[test]
fn test_long_identifiers() {
    let long_name = "a".repeat(100); // Reasonably long
    let input = format!("entity {} {{ field }}", long_name);

    let mut parser = Parser::new(&input);
    let _ = parser.parse_program();
    // Verify it doesn't panic
}

#[test]
fn test_extremely_large_numbers() {
    let input = "rule Test: if x > 999999999999999 then halt";
    let mut parser = Parser::new(input);
    let _ = parser.parse_program();
    // Should handle or report error gracefully
}
