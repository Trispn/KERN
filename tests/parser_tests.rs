mod common;
use common::assertions::{assert_equal, assert_true, AssertionResult};
use kern_lexer::lexer::Lexer;
use kern_parser::ast_nodes::{Constraint, Declaration, Entity, Flow, Program, Rule};
use kern_parser::parser::Parser;

fn run_parser_test(input: &str) -> Result<Program, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    match parser.parse_program() {
        Some(program) => Ok(program),
        None => Err("Parser returned None".to_string()),
    }
}

#[test]
fn test_entity_parsing() {
    let input = "entity User { name age active }";

    match run_parser_test(input) {
        Ok(program) => {
            assert_eq!(program.declarations.len(), 1);

            match &program.declarations[0] {
                Declaration::Entity(entity) => {
                    assert_eq!(entity.name, "User");
                    assert_eq!(entity.fields.len(), 3);
                    assert_eq!(entity.fields[0], "name");
                    assert_eq!(entity.fields[1], "age");
                    assert_eq!(entity.fields[2], "active");
                }
                _ => panic!("Expected entity declaration"),
            }
        }
        Err(e) => panic!("Parser failed: {}", e),
    }
}

#[test]
fn test_rule_parsing() {
    let input = "rule ValidateAge: if user.age >= 18 then set_adult(user)";

    match run_parser_test(input) {
        Ok(program) => {
            assert_eq!(program.declarations.len(), 1);

            match &program.declarations[0] {
                Declaration::Rule(rule) => {
                    assert_eq!(rule.name, "ValidateAge");
                    // Check that the rule has a condition and action
                    assert!(rule.condition.is_some());
                    assert!(!rule.actions.is_empty());
                }
                _ => panic!("Expected rule declaration"),
            }
        }
        Err(e) => panic!("Parser failed: {}", e),
    }
}

#[test]
fn test_flow_parsing() {
    let input = "flow ProcessData { step1: load_data(), step2: transform_data() }";

    match run_parser_test(input) {
        Ok(program) => {
            assert_eq!(program.declarations.len(), 1);

            match &program.declarations[0] {
                Declaration::Flow(flow) => {
                    assert_eq!(flow.name, "ProcessData");
                    assert_eq!(flow.steps.len(), 2);
                }
                _ => panic!("Expected flow declaration"),
            }
        }
        Err(e) => panic!("Parser failed: {}", e),
    }
}

#[test]
fn test_constraint_parsing() {
    let input = "constraint ValidEmail: user.email.contains(\"@\")";

    match run_parser_test(input) {
        Ok(program) => {
            assert_eq!(program.declarations.len(), 1);

            match &program.declarations[0] {
                Declaration::Constraint(constraint) => {
                    assert_eq!(constraint.name, "ValidEmail");
                    // Check that the constraint has a condition
                    assert!(constraint.condition.is_some());
                }
                _ => panic!("Expected constraint declaration"),
            }
        }
        Err(e) => panic!("Parser failed: {}", e),
    }
}

#[test]
fn test_multiple_declarations() {
    let input = r#"
    entity User { name email }
    rule ValidateUser: if user.name != "" then validate_email(user)
    flow ProcessUsers { step1: load_users() }
    constraint UniqueEmail: user.email != other.email
    "#;

    match run_parser_test(input) {
        Ok(program) => {
            assert_eq!(program.declarations.len(), 4);

            // Check that all declaration types are present
            let mut entity_count = 0;
            let mut rule_count = 0;
            let mut flow_count = 0;
            let mut constraint_count = 0;

            for decl in &program.declarations {
                match decl {
                    Declaration::Entity(_) => entity_count += 1,
                    Declaration::Rule(_) => rule_count += 1,
                    Declaration::Flow(_) => flow_count += 1,
                    Declaration::Constraint(_) => constraint_count += 1,
                }
            }

            assert_eq!(entity_count, 1);
            assert_eq!(rule_count, 1);
            assert_eq!(flow_count, 1);
            assert_eq!(constraint_count, 1);
        }
        Err(e) => panic!("Parser failed: {}", e),
    }
}

#[test]
fn test_nested_entity_fields() {
    let input = r#"
    entity Organization {
        name
        users
        location
    }
    "#;

    match run_parser_test(input) {
        Ok(program) => {
            assert_eq!(program.declarations.len(), 1);

            match &program.declarations[0] {
                Declaration::Entity(entity) => {
                    assert_eq!(entity.name, "Organization");
                    assert_eq!(entity.fields.len(), 3);
                    assert_eq!(entity.fields[0], "name");
                    assert_eq!(entity.fields[1], "users");
                    assert_eq!(entity.fields[2], "location");
                }
                _ => panic!("Expected entity declaration"),
            }
        }
        Err(e) => panic!("Parser failed: {}", e),
    }
}

#[test]
fn test_complex_rule_condition() {
    let input = r#"
    rule ComplexRule:
        if user.age >= 18 and user.verified == true and user.email.contains("@")
        then approve_user(user)
    "#;

    match run_parser_test(input) {
        Ok(program) => {
            assert_eq!(program.declarations.len(), 1);

            match &program.declarations[0] {
                Declaration::Rule(rule) => {
                    assert_eq!(rule.name, "ComplexRule");
                    assert!(rule.condition.is_some());
                    assert!(!rule.actions.is_empty());
                }
                _ => panic!("Expected rule declaration"),
            }
        }
        Err(e) => panic!("Parser failed: {}", e),
    }
}

#[test]
fn test_empty_entity() {
    let input = "entity Empty { }";

    match run_parser_test(input) {
        Ok(program) => {
            assert_eq!(program.declarations.len(), 1);

            match &program.declarations[0] {
                Declaration::Entity(entity) => {
                    assert_eq!(entity.name, "Empty");
                    assert_eq!(entity.fields.len(), 0);
                }
                _ => panic!("Expected entity declaration"),
            }
        }
        Err(e) => panic!("Parser failed: {}", e),
    }
}

#[test]
fn test_multiline_flow() {
    let input = r#"
    flow MultiStepFlow {
        step1: load_data(),
        step2: process_data(),
        step3: save_data()
    }
    "#;

    match run_parser_test(input) {
        Ok(program) => {
            assert_eq!(program.declarations.len(), 1);

            match &program.declarations[0] {
                Declaration::Flow(flow) => {
                    assert_eq!(flow.name, "MultiStepFlow");
                    assert_eq!(flow.steps.len(), 3);
                }
                _ => panic!("Expected flow declaration"),
            }
        }
        Err(e) => panic!("Parser failed: {}", e),
    }
}

#[test]
fn test_simple_constraint() {
    let input = "constraint PositiveValue: value > 0";

    match run_parser_test(input) {
        Ok(program) => {
            assert_eq!(program.declarations.len(), 1);

            match &program.declarations[0] {
                Declaration::Constraint(constraint) => {
                    assert_eq!(constraint.name, "PositiveValue");
                    assert!(constraint.condition.is_some());
                }
                _ => panic!("Expected constraint declaration"),
            }
        }
        Err(e) => panic!("Parser failed: {}", e),
    }
}
