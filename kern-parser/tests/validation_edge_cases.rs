//! Validation edge case tests
//! These tests cover edge cases and boundary conditions for the parser

use kern_parser::{Definition, Parser};

#[test]
fn test_empty_entity() {
    let input = "entity EmptyEntity { }";
    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Entity(entity) = &program.definitions[0] {
        assert_eq!(entity.name, "EmptyEntity");
        assert_eq!(entity.fields.len(), 0);
    } else {
        panic!("Expected entity definition");
    }
}

#[test]
fn test_entity_with_very_long_name() {
    let long_name = "a".repeat(1000);
    let input = format!("entity {} {{ field1 }}", long_name);
    let mut parser = Parser::new(&input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Entity(entity) = &program.definitions[0] {
        assert_eq!(entity.name, long_name);
        assert_eq!(entity.fields.len(), 1);
    } else {
        panic!("Expected entity definition");
    }
}

#[test]
fn test_entity_with_many_fields() {
    let fields: Vec<String> = (0..100).map(|i| format!("field{}", i)).collect();
    let fields_str = fields.join(" ");
    let input = format!("entity ManyFields {{ {} }}", fields_str);
    let mut parser = Parser::new(&input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Entity(entity) = &program.definitions[0] {
        assert_eq!(entity.name, "ManyFields");
        assert_eq!(entity.fields.len(), 100);
        for (i, field) in entity.fields.iter().enumerate() {
            assert_eq!(field.name, format!("field{}", i));
        }
    } else {
        panic!("Expected entity definition");
    }
}

#[test]
fn test_rule_with_complex_nested_conditions() {
    let input = r#"
        rule ComplexNestedRule:
            if (a > 0 and b < 10) or (c == 5 and (d != 3 or e >= 7))
            then action()
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Rule(rule) = &program.definitions[0] {
        assert_eq!(rule.name, "ComplexNestedRule");
    } else {
        panic!("Expected rule definition");
    }
}

#[test]
fn test_rule_with_large_number_of_actions() {
    let actions: Vec<String> = (0..50).map(|i| format!("action{}()", i)).collect();
    let actions_str = actions.join(", ");
    let input = format!("rule ManyActions: if condition then {}", actions_str);
    let mut parser = Parser::new(&input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Rule(rule) = &program.definitions[0] {
        assert_eq!(rule.name, "ManyActions");
        assert_eq!(rule.actions.len(), 50);
    } else {
        panic!("Expected rule definition");
    }
}

#[test]
fn test_flow_with_nested_control_structures() {
    let input = r#"
        flow NestedControlFlow {
            if condition1 then action1() else action2(),
            loop { if inner_condition then inner_action() },
            action3()
        }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Flow(flow) = &program.definitions[0] {
        assert_eq!(flow.name, "NestedControlFlow");
        assert!(flow.actions.len() >= 1);
    } else {
        panic!("Expected flow definition");
    }
}

#[test]
fn test_constraint_with_complex_expression() {
    let input = r#"
        constraint ComplexConstraint: ((value1 > 0 and value2 < 10) or value3 == 5) and value4 != 0
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Constraint(constraint) = &program.definitions[0] {
        assert_eq!(constraint.name, "ComplexConstraint");
    } else {
        panic!("Expected constraint definition");
    }
}

#[test]
fn test_multiple_definitions_sequential() {
    // Test a long sequence of different definition types
    let input = r#"
        entity Entity1 { field1 }
        rule Rule1: if condition1 then action1()
        flow Flow1 { action1() }
        constraint Constraint1: value1 > 0
        entity Entity2 { field2 }
        rule Rule2: if condition2 then action2()
        flow Flow2 { action2() }
        constraint Constraint2: value2 > 0
        entity Entity3 { field3 }
        rule Rule3: if condition3 then action3()
        flow Flow3 { action3() }
        constraint Constraint3: value3 > 0
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 12);

    // Verify the sequence of definitions
    let mut entity_count = 0;
    let mut rule_count = 0;
    let mut flow_count = 0;
    let mut constraint_count = 0;

    for def in &program.definitions {
        match def {
            Definition::Entity(_) => entity_count += 1,
            Definition::Rule(_) => rule_count += 1,
            Definition::Flow(_) => flow_count += 1,
            Definition::Constraint(_) => constraint_count += 1,
        }
    }

    assert_eq!(entity_count, 3);
    assert_eq!(rule_count, 3);
    assert_eq!(flow_count, 3);
    assert_eq!(constraint_count, 3);
}

#[test]
fn test_whitespace_and_comments_handling() {
    let input = r#"
        entity WhitespaceTest { 
            field1   
            
            field2
                    field3
        }
        
        rule WhitespaceRule   
            :   
            if   
            condition   
            then   
            action   
            (   
            )   
        "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 2);

    if let Definition::Entity(entity) = &program.definitions[0] {
        assert_eq!(entity.name, "WhitespaceTest");
        assert_eq!(entity.fields.len(), 3);
    } else {
        panic!("Expected entity definition");
    }

    if let Definition::Rule(rule) = &program.definitions[1] {
        assert_eq!(rule.name, "WhitespaceRule");
    } else {
        panic!("Expected rule definition");
    }
}

#[test]
fn test_number_parsing_edge_cases() {
    let input = r#"
        rule NumberEdgeCases:
            if value == 0 or value == -1 or value == 9223372036854775807 or value == -9223372036854775808
            then process_numbers(0, -1, 9223372036854775807, -9223372036854775808)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Rule(rule) = &program.definitions[0] {
        assert_eq!(rule.name, "NumberEdgeCases");
    } else {
        panic!("Expected rule definition");
    }
}

#[test]
fn test_identifier_edge_cases() {
    let input = r#"
        entity Test_entity123 { field_name456 }
        rule RuleWithNumbers123: if var123abc > 0 then action_123()
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 2);

    if let Definition::Entity(entity) = &program.definitions[0] {
        assert_eq!(entity.name, "Test_entity123");
    } else {
        panic!("Expected entity definition");
    }

    if let Definition::Rule(rule) = &program.definitions[1] {
        assert_eq!(rule.name, "RuleWithNumbers123");
    } else {
        panic!("Expected rule definition");
    }
}
