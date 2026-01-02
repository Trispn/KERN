//! Comprehensive test suite for all KERN grammar productions
//! Based on the formal KERN grammar specification (EBNF)

use kern_parser::{Definition, Parser};

// Test entity definitions according to grammar:
// entity_def = "entity" , identifier , "{" , { field_def } , "}" ;
// field_def = identifier ;
#[test]
fn test_entity_definition() {
    let input = r#"
        entity Farmer {
            id
            location
            produce
        }

        entity Crop {
            type
            season
            yield
        }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 2);

    // Verify first entity
    if let Definition::Entity(entity) = &program.definitions[0] {
        assert_eq!(entity.name, "Farmer");
        assert_eq!(entity.fields.len(), 3);
        assert_eq!(entity.fields[0].name, "id");
        assert_eq!(entity.fields[1].name, "location");
        assert_eq!(entity.fields[2].name, "produce");
    } else {
        panic!("Expected first definition to be an entity");
    }

    // Verify second entity
    if let Definition::Entity(entity) = &program.definitions[1] {
        assert_eq!(entity.name, "Crop");
        assert_eq!(entity.fields.len(), 3);
        assert_eq!(entity.fields[0].name, "type");
        assert_eq!(entity.fields[1].name, "season");
        assert_eq!(entity.fields[2].name, "yield");
    } else {
        panic!("Expected second definition to be an entity");
    }
}

// Test rule definitions according to grammar:
// rule_def = "rule" , identifier , ":" , "if" , condition , "then" , action_list ;
#[test]
fn test_rule_definition() {
    let input = r#"
        rule CheckFarmer:
            if farmer.id != 0
            then validate_farmer(farmer)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Rule(rule) = &program.definitions[0] {
        assert_eq!(rule.name, "CheckFarmer");
        // Additional assertions for condition and actions would go here
    } else {
        panic!("Expected definition to be a rule");
    }
}

// Test conditions according to grammar:
// condition = expression | expression , logical_op , condition ;
// logical_op = "and" | "or" ;
#[test]
fn test_conditions_with_logical_ops() {
    let input = r#"
        rule ComplexCondition:
            if farmer.id != 0 and farmer.location != ""
            then validate_farmer(farmer)

        rule AnotherCondition:
            if value > 10 or value < 5
            then adjust_value(value)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 2);

    // Verify first rule with AND condition
    if let Definition::Rule(rule) = &program.definitions[0] {
        assert_eq!(rule.name, "ComplexCondition");
    } else {
        panic!("Expected first definition to be a rule");
    }

    // Verify second rule with OR condition
    if let Definition::Rule(rule) = &program.definitions[1] {
        assert_eq!(rule.name, "AnotherCondition");
    } else {
        panic!("Expected second definition to be a rule");
    }
}

// Test expressions according to grammar:
// expression = term , comparator , term | predicate ;
// comparator = "==" | "!=" | ">" | "<" | ">=" | "<=" ;
#[test]
fn test_expressions_with_comparators() {
    let input = r#"
        rule EqualityCheck: if value == 42 then do_something()
        rule InequalityCheck: if value != 0 then do_something()
        rule GreaterCheck: if value > 10 then do_something()
        rule LessCheck: if value < 5 then do_something()
        rule GreaterEqualCheck: if value >= 15 then do_something()
        rule LessEqualCheck: if value <= 3 then do_something()
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 6);
}

// Test terms according to grammar:
// term = identifier | number | qualified_ref ;
// qualified_ref = identifier , "." , identifier ;
#[test]
fn test_terms_and_qualified_refs() {
    let input = r#"
        rule QualifiedRefCheck:
            if farmer.id != 0
            then validate_farmer(farmer.id, 42)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Rule(rule) = &program.definitions[0] {
        assert_eq!(rule.name, "QualifiedRefCheck");
    } else {
        panic!("Expected definition to be a rule");
    }
}

// Test predicates according to grammar:
// predicate = identifier , "(" , [ argument_list ] , ")" ;
// argument_list = term , { "," , term } ;
#[test]
fn test_predicates_with_arguments() {
    let input = r#"
        rule PredicateWithArgs:
            if condition(x, y, 42)
            then action(a, b, farmer.id)

        rule PredicateNoArgs:
            if simple_condition()
            then simple_action()
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 2);
}

// Test actions according to grammar:
// action_list = action , { "," , action } ;
// action = predicate | assignment | control_action ;
#[test]
fn test_action_lists() {
    let input = r#"
        rule MultipleActions:
            if condition
            then action1(), action2(), action3()
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Rule(rule) = &program.definitions[0] {
        assert_eq!(rule.name, "MultipleActions");
        assert!(rule.actions.len() >= 1); // At least one action
    } else {
        panic!("Expected definition to be a rule");
    }
}

// Test assignments according to grammar:
// assignment = identifier , "=" , term ;
#[test]
fn test_assignments_in_actions() {
    let input = r#"
        rule AssignmentRule:
            if value > 0
            then result = value, log_result(result)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Rule(rule) = &program.definitions[0] {
        assert_eq!(rule.name, "AssignmentRule");
    } else {
        panic!("Expected definition to be a rule");
    }
}

// Test control actions according to grammar:
// control_action = if_action | loop_action | halt_action ;
#[test]
fn test_control_actions() {
    let input = r#"
        rule ControlActionRule:
            if condition
            then if x > 0 then positive_action() else negative_action(), halt
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Rule(rule) = &program.definitions[0] {
        assert_eq!(rule.name, "ControlActionRule");
    } else {
        panic!("Expected definition to be a rule");
    }
}

// Test if actions according to grammar:
// if_action = "if" , condition , "then" , action_list , [ "else" , action_list ] ;
#[test]
fn test_if_control_action() {
    let input = r#"
        flow IfActionFlow {
            if value > 0 then positive_action() else negative_action()
        }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Flow(flow) = &program.definitions[0] {
        assert_eq!(flow.name, "IfActionFlow");
    } else {
        panic!("Expected definition to be a flow");
    }
}

// Test loop actions according to grammar:
// loop_action = "loop" , "{" , action_list , "}" ;
#[test]
fn test_loop_control_action() {
    let input = r#"
        flow LoopActionFlow {
            loop { process_item() }
        }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Flow(flow) = &program.definitions[0] {
        assert_eq!(flow.name, "LoopActionFlow");
    } else {
        panic!("Expected definition to be a flow");
    }
}

// Test halt actions according to grammar:
// halt_action = "halt" ;
#[test]
fn test_halt_control_action() {
    let input = r#"
        flow HaltActionFlow {
            validate_input(), halt
        }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Flow(flow) = &program.definitions[0] {
        assert_eq!(flow.name, "HaltActionFlow");
    } else {
        panic!("Expected definition to be a flow");
    }
}

// Test flow definitions according to grammar:
// flow_def = "flow" , identifier , "{" , action_list , "}" ;
#[test]
fn test_flow_definition() {
    let input = r#"
        flow ProcessFarmers {
            load_farmers(),
            validate_farmers(),
            generate_reports()
        }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Flow(flow) = &program.definitions[0] {
        assert_eq!(flow.name, "ProcessFarmers");
        assert!(flow.actions.len() >= 1);
    } else {
        panic!("Expected definition to be a flow");
    }
}

// Test constraint definitions according to grammar:
// constraint_def = "constraint" , identifier , ":" , condition ;
#[test]
fn test_constraint_definition() {
    let input = r#"
        constraint ValidLocation: farmer.location != ""
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);

    if let Definition::Constraint(constraint) = &program.definitions[0] {
        assert_eq!(constraint.name, "ValidLocation");
        // Additional assertions for condition would go here
    } else {
        panic!("Expected definition to be a constraint");
    }
}

// Test multiple definitions in one program
#[test]
fn test_multiple_definition_types() {
    let input = r#"
        entity Farmer {
            id
            location
        }

        rule CheckFarmer:
            if farmer.id != 0
            then validate_farmer(farmer)

        flow ProcessFarmers {
            load_farmers(),
            validate_farmers()
        }

        constraint ValidFarmer: farmer.id > 0
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 4);

    // Verify each definition type is present
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

    assert_eq!(entity_count, 1);
    assert_eq!(rule_count, 1);
    assert_eq!(flow_count, 1);
    assert_eq!(constraint_count, 1);
}
