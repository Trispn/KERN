mod common;
use common::assertions::{assert_equal, assert_true, AssertionResult};
use kern_parser::ast::{
    Action, Comparator, Condition, ConstraintDef, Definition, EntityDef, Expression, FieldDef,
    FlowDef, Program, RuleDef, Term,
};

#[test]
fn test_entity_node_creation() {
    let entity = EntityDef {
        name: "User".to_string(),
        fields: vec![
            FieldDef {
                name: "name".to_string(),
            },
            FieldDef {
                name: "age".to_string(),
            },
            FieldDef {
                name: "email".to_string(),
            },
        ],
    };

    assert_eq!(entity.name, "User");
    assert_eq!(entity.fields.len(), 3);
    assert_eq!(entity.fields[0].name, "name");
    assert_eq!(entity.fields[1].name, "age");
    assert_eq!(entity.fields[2].name, "email");
}

#[test]
fn test_rule_node_creation() {
    // Create a simple rule with a condition and action
    let rule = RuleDef {
        name: "ValidateUser".to_string(),
        condition: Condition::Expression(Expression::Comparison {
            left: Box::new(Term::QualifiedRef("user".to_string(), "age".to_string())),
            op: Comparator::GreaterEqual,
            right: Box::new(Term::Number(18)),
        }),
        actions: vec![],
    };

    assert_eq!(rule.name, "ValidateUser");
}

#[test]
fn test_flow_node_creation() {
    let flow = FlowDef {
        name: "ProcessData".to_string(),
        actions: vec![],
    };

    assert_eq!(flow.name, "ProcessData");
}

#[test]
fn test_constraint_node_creation() {
    let constraint = ConstraintDef {
        name: "ValidEmail".to_string(),
        condition: Condition::Expression(Expression::Comparison {
            left: Box::new(Term::QualifiedRef("user".to_string(), "email".to_string())),
            op: Comparator::NotEqual,
            right: Box::new(Term::Number(0)), // Placeholder
        }),
    };

    assert_eq!(constraint.name, "ValidEmail");
}

#[test]
fn test_program_node_creation() {
    let entity = EntityDef {
        name: "User".to_string(),
        fields: vec![FieldDef {
            name: "name".to_string(),
        }],
    };

    let rule = RuleDef {
        name: "ValidateUser".to_string(),
        condition: Condition::Expression(Expression::Comparison {
            left: Box::new(Term::Number(1)),
            op: Comparator::Equal,
            right: Box::new(Term::Number(1)),
        }),
        actions: vec![],
    };

    let program = Program {
        definitions: vec![Definition::Entity(entity), Definition::Rule(rule)],
    };

    assert_eq!(program.definitions.len(), 2);
}

#[test]
fn test_literal_ast_nodes() {
    let number_term = Term::Number(42);

    match number_term {
        Term::Number(n) => assert_eq!(n, 42),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_identifier_ast_node() {
    let id_term = Term::Identifier("user".to_string());
    let qualified_term = Term::QualifiedRef("user".to_string(), "name".to_string());

    match id_term {
        Term::Identifier(name) => assert_eq!(name, "user"),
        _ => panic!("Expected identifier"),
    }

    match qualified_term {
        Term::QualifiedRef(entity, field) => {
            assert_eq!(entity, "user");
            assert_eq!(field, "name");
        }
        _ => panic!("Expected qualified ref"),
    }
}
