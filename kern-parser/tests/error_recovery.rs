//! Parser error recovery tests
//! These tests verify that the parser can handle malformed input gracefully
//! and continue parsing subsequent valid definitions

use kern_parser::{Definition, Parser};

#[test]
fn test_entity_definition_recovery() {
    let input = r#"
        entity IncompleteEntity { id
        entity CompleteEntity { field1 field2 }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    // The parser should still be able to parse the complete entity after the malformed one
    assert!(result.is_ok());
    let program = result.unwrap();

    // Should have recovered and parsed the complete entity
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Entity(entity) = &program.definitions[0] {
        assert_eq!(entity.name, "CompleteEntity");
        assert_eq!(entity.fields.len(), 2);
    } else {
        panic!("Expected entity definition after recovery");
    }
}

#[test]
fn test_rule_definition_recovery() {
    let input = r#"
        rule IncompleteRule: if condition
        rule CompleteRule: if value > 0 then action()
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    // The parser should still be able to parse the complete rule after the malformed one
    assert!(result.is_ok());
    let program = result.unwrap();

    // Should have recovered and parsed the complete rule
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Rule(rule) = &program.definitions[0] {
        assert_eq!(rule.name, "CompleteRule");
    } else {
        panic!("Expected rule definition after recovery");
    }
}

#[test]
fn test_mixed_recovery() {
    let input = r#"
        entity IncompleteEntity { id
        rule ValidRule: if condition then action()
        flow IncompleteFlow {
        flow ValidFlow { action() }
        constraint ValidConstraint: value > 0
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    // The parser should recover from multiple errors and parse valid definitions
    assert!(result.is_ok());
    let program = result.unwrap();

    // Should have recovered and parsed the valid definitions
    assert_eq!(program.definitions.len(), 2);

    // Check that we have one rule and one flow
    let mut rule_count = 0;
    let mut flow_count = 0;
    let mut constraint_count = 0;

    for def in &program.definitions {
        match def {
            Definition::Rule(_) => rule_count += 1,
            Definition::Flow(_) => flow_count += 1,
            Definition::Constraint(_) => constraint_count += 1,
            Definition::Entity(_) => {} // We expect no entities since the first was malformed
        }
    }

    assert_eq!(rule_count, 1);
    assert_eq!(flow_count, 1);
    assert_eq!(constraint_count, 1);
}

#[test]
fn test_error_recovery_with_valid_definitions_after_error() {
    let input = r#"
        entity TestEntity { field1 field2 }
        rule IncompleteRule: if condition  // Missing 'then' part
        rule AnotherValidRule: if value > 0 then action()
        flow ValidFlow { action() }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    // Should parse the first entity and the valid rule after the error
    assert!(result.is_ok());
    let program = result.unwrap();

    // Should have recovered and parsed the valid definitions
    assert_eq!(program.definitions.len(), 3);

    // Check the types of definitions
    if let Definition::Entity(entity) = &program.definitions[0] {
        assert_eq!(entity.name, "TestEntity");
    } else {
        panic!("Expected first definition to be an entity");
    }

    if let Definition::Rule(rule) = &program.definitions[1] {
        assert_eq!(rule.name, "AnotherValidRule");
    } else {
        panic!("Expected second definition to be a rule");
    }

    if let Definition::Flow(flow) = &program.definitions[2] {
        assert_eq!(flow.name, "ValidFlow");
    } else {
        panic!("Expected third definition to be a flow");
    }
}

#[test]
fn test_recovery_disabled_behavior() {
    let input = r#"
        entity IncompleteEntity { id
        entity CompleteEntity { field1 field2 }
    "#;

    let mut parser = Parser::new(input);
    // Disable recovery to test the difference
    parser.recovery_enabled = false;
    let result = parser.parse_program();

    // With recovery disabled, the parser should fail completely
    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert!(!errors.is_empty());
}
