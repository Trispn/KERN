mod common;
use common::assertions::{assert_equal, assert_true, AssertionResult};
use kern_parser::ast::{ConstraintDef, Definition, EntityDef, FlowDef, Program, RuleDef};
use kern_parser::parser::Parser;

fn run_parser_test(input: &str) -> Result<Program, String> {
    let mut parser = Parser::new(input);
    match parser.parse_program() {
        Ok(program) => Ok(program),
        Err(errors) => Err(format!("Parser failed: {:?}", errors)),
    }
}

#[test]
fn test_entity_parsing() {
    let input = "entity User { name age active }";

    match run_parser_test(input) {
        Ok(program) => {
            assert_eq!(program.definitions.len(), 1);

            match &program.definitions[0] {
                Definition::Entity(entity) => {
                    assert_eq!(entity.name, "User");
                    assert_eq!(entity.fields.len(), 3);
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
            assert_eq!(program.definitions.len(), 1);

            match &program.definitions[0] {
                Definition::Rule(rule) => {
                    assert_eq!(rule.name, "ValidateAge");
                }
                _ => panic!("Expected rule declaration"),
            }
        }
        Err(e) => panic!("Parser failed: {}", e),
    }
}

#[test]
fn test_flow_parsing() {
    // Fixed: actions in flow should be comma-separated
    let input = "flow ProcessData { load_data(), transform_data() }";

    match run_parser_test(input) {
        Ok(program) => {
            assert_eq!(program.definitions.len(), 1);

            match &program.definitions[0] {
                Definition::Flow(flow) => {
                    assert_eq!(flow.name, "ProcessData");
                }
                _ => panic!("Expected flow declaration"),
            }
        }
        Err(e) => panic!("Parser failed: {}", e),
    }
}

#[test]
fn test_constraint_parsing() {
    let input = "constraint PositiveAge: user.age > 0";

    match run_parser_test(input) {
        Ok(program) => {
            assert_eq!(program.definitions.len(), 1);

            match &program.definitions[0] {
                Definition::Constraint(constraint) => {
                    assert_eq!(constraint.name, "PositiveAge");
                }
                _ => panic!("Expected constraint declaration"),
            }
        }
        Err(e) => panic!("Parser failed: {}", e),
    }
}

#[test]
fn test_simple_entity() {
    let input = "entity Farmer { id }";
    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);
}
