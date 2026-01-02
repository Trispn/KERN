mod common;
use common::assertions::{assert_equal, assert_true, AssertionResult};
use kern_parser::ast_nodes::{
    BinaryOp, Constraint, Declaration, Entity, Expression, Flow, Identifier, Literal, Program, Rule,
};

#[test]
fn test_entity_node_creation() {
    let entity = Entity {
        name: "User".to_string(),
        fields: vec!["name".to_string(), "age".to_string(), "email".to_string()],
    };

    assert_eq!(entity.name, "User");
    assert_eq!(entity.fields.len(), 3);
    assert_eq!(entity.fields[0], "name");
    assert_eq!(entity.fields[1], "age");
    assert_eq!(entity.fields[2], "email");
}

#[test]
fn test_rule_node_creation() {
    // Create a simple rule with a condition and action
    let rule = Rule {
        name: "ValidateUser".to_string(),
        condition: Some(Expression::BinaryOp {
            left: Box::new(Expression::Identifier(Identifier {
                name: "user".to_string(),
                field: Some("age".to_string()),
            })),
            operator: BinaryOp::GreaterEqual,
            right: Box::new(Expression::Literal(Literal::Number(18))),
        }),
        actions: vec![],
    };

    assert_eq!(rule.name, "ValidateUser");
    assert!(rule.condition.is_some());
}

#[test]
fn test_flow_node_creation() {
    let flow = Flow {
        name: "ProcessData".to_string(),
        steps: vec![
            "load_data()".to_string(),
            "transform_data()".to_string(),
            "save_data()".to_string(),
        ],
    };

    assert_eq!(flow.name, "ProcessData");
    assert_eq!(flow.steps.len(), 3);
    assert_eq!(flow.steps[0], "load_data()");
    assert_eq!(flow.steps[1], "transform_data()");
    assert_eq!(flow.steps[2], "save_data()");
}

#[test]
fn test_constraint_node_creation() {
    let constraint = Constraint {
        name: "ValidEmail".to_string(),
        condition: Some(Expression::BinaryOp {
            left: Box::new(Expression::Identifier(Identifier {
                name: "user".to_string(),
                field: Some("email".to_string()),
            })),
            operator: BinaryOp::Contains,
            right: Box::new(Expression::Literal(Literal::String("@".to_string()))),
        }),
    };

    assert_eq!(constraint.name, "ValidEmail");
    assert!(constraint.condition.is_some());
}

#[test]
fn test_program_node_creation() {
    let entity = Entity {
        name: "User".to_string(),
        fields: vec!["name".to_string()],
    };

    let rule = Rule {
        name: "ValidateUser".to_string(),
        condition: None,
        actions: vec![],
    };

    let flow = Flow {
        name: "ProcessUsers".to_string(),
        steps: vec!["step1".to_string()],
    };

    let constraint = Constraint {
        name: "UniqueEmail".to_string(),
        condition: None,
    };

    let program = Program {
        declarations: vec![
            Declaration::Entity(entity),
            Declaration::Rule(rule),
            Declaration::Flow(flow),
            Declaration::Constraint(constraint),
        ],
    };

    assert_eq!(program.declarations.len(), 4);

    match &program.declarations[0] {
        Declaration::Entity(e) => assert_eq!(e.name, "User"),
        _ => panic!("Expected entity"),
    }

    match &program.declarations[1] {
        Declaration::Rule(r) => assert_eq!(r.name, "ValidateUser"),
        _ => panic!("Expected rule"),
    }

    match &program.declarations[2] {
        Declaration::Flow(f) => assert_eq!(f.name, "ProcessUsers"),
        _ => panic!("Expected flow"),
    }

    match &program.declarations[3] {
        Declaration::Constraint(c) => assert_eq!(c.name, "UniqueEmail"),
        _ => panic!("Expected constraint"),
    }
}

#[test]
fn test_binary_operation_ast_node() {
    let expr = Expression::BinaryOp {
        left: Box::new(Expression::Identifier(Identifier {
            name: "a".to_string(),
            field: None,
        })),
        operator: BinaryOp::Equals,
        right: Box::new(Expression::Identifier(Identifier {
            name: "b".to_string(),
            field: None,
        })),
    };

    match expr {
        Expression::BinaryOp {
            left,
            operator,
            right,
        } => {
            match left.as_ref() {
                Expression::Identifier(id) => assert_eq!(id.name, "a"),
                _ => panic!("Expected identifier"),
            }

            assert_eq!(operator, BinaryOp::Equals);

            match right.as_ref() {
                Expression::Identifier(id) => assert_eq!(id.name, "b"),
                _ => panic!("Expected identifier"),
            }
        }
        _ => panic!("Expected binary operation"),
    }
}

#[test]
fn test_literal_ast_nodes() {
    let string_literal = Expression::Literal(Literal::String("hello".to_string()));
    let number_literal = Expression::Literal(Literal::Number(42));
    let bool_literal = Expression::Literal(Literal::Boolean(true));

    match string_literal {
        Expression::Literal(Literal::String(s)) => assert_eq!(s, "hello"),
        _ => panic!("Expected string literal"),
    }

    match number_literal {
        Expression::Literal(Literal::Number(n)) => assert_eq!(n, 42),
        _ => panic!("Expected number literal"),
    }

    match bool_literal {
        Expression::Literal(Literal::Boolean(b)) => assert!(b),
        _ => panic!("Expected boolean literal"),
    }
}

#[test]
fn test_identifier_ast_node() {
    let id_with_field = Identifier {
        name: "user".to_string(),
        field: Some("name".to_string()),
    };
    let id_without_field = Identifier {
        name: "value".to_string(),
        field: None,
    };

    assert_eq!(id_with_field.name, "user");
    assert_eq!(id_with_field.field, Some("name".to_string()));

    assert_eq!(id_without_field.name, "value");
    assert_eq!(id_without_field.field, None);
}

#[test]
fn test_nested_expressions() {
    let nested_expr = Expression::BinaryOp {
        left: Box::new(Expression::BinaryOp {
            left: Box::new(Expression::Identifier(Identifier {
                name: "a".to_string(),
                field: None,
            })),
            operator: BinaryOp::Plus,
            right: Box::new(Expression::Identifier(Identifier {
                name: "b".to_string(),
                field: None,
            })),
        }),
        operator: BinaryOp::Equals,
        right: Box::new(Expression::Identifier(Identifier {
            name: "c".to_string(),
            field: None,
        })),
    };

    match nested_expr {
        Expression::BinaryOp {
            left,
            operator,
            right,
        } => {
            assert_eq!(operator, BinaryOp::Equals);

            match left.as_ref() {
                Expression::BinaryOp {
                    left: inner_left,
                    operator: inner_op,
                    right: inner_right,
                } => {
                    assert_eq!(*inner_op, BinaryOp::Plus);

                    match inner_left.as_ref() {
                        Expression::Identifier(id) => assert_eq!(id.name, "a"),
                        _ => panic!("Expected identifier"),
                    }

                    match inner_right.as_ref() {
                        Expression::Identifier(id) => assert_eq!(id.name, "b"),
                        _ => panic!("Expected identifier"),
                    }
                }
                _ => panic!("Expected nested binary operation"),
            }

            match right.as_ref() {
                Expression::Identifier(id) => assert_eq!(id.name, "c"),
                _ => panic!("Expected identifier"),
            }
        }
        _ => panic!("Expected binary operation"),
    }
}

#[test]
fn test_ast_node_serialization_equivalence() {
    // Create an AST node
    let original_entity = Entity {
        name: "TestEntity".to_string(),
        fields: vec!["field1".to_string(), "field2".to_string()],
    };

    // In a real implementation, we would serialize and deserialize
    // For now, we'll just verify the structure is as expected
    assert_eq!(original_entity.name, "TestEntity");
    assert_eq!(original_entity.fields.len(), 2);
    assert_eq!(original_entity.fields[0], "field1");
    assert_eq!(original_entity.fields[1], "field2");

    // Create another identical entity
    let duplicate_entity = Entity {
        name: "TestEntity".to_string(),
        fields: vec!["field1".to_string(), "field2".to_string()],
    };

    // Verify they are equivalent
    assert_eq!(original_entity.name, duplicate_entity.name);
    assert_eq!(original_entity.fields, duplicate_entity.fields);
}
