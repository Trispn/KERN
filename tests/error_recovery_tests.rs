mod common;
use common::assertions::{assert_equal, assert_false, assert_true, AssertionResult};
use kern_parser::parser::{ParseError, Parser};

#[test]
fn test_incomplete_entity_declaration() {
    let input = "entity User { name"; // Missing closing brace
    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_err());
    assert!(!parser.get_errors().is_empty());
}

#[test]
fn test_missing_rule_name() {
    let input = "rule : if true then halt"; // Missing rule name
    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_err());
}

#[test]
fn test_invalid_syntax_recovery() {
    let input = r#"
    entity User { name }
    rule ValidateUser  // Missing colon
        if x == 1
        then halt
    flow ProcessData { halt }
    "#;

    let mut parser = Parser::new(input);
    let _ = parser.parse_program();

    // Recovery might still result in Err if total program parsing failed
    // but parser.get_errors() should contain details
    assert!(!parser.get_errors().is_empty());
}
