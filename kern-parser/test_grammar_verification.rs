//! Verification tests to ensure all grammar productions match KERN specification
//! Based on the formal KERN grammar specification (EBNF)

use kern_parser::{Parser, Program, Definition, AstNode};

// Test that all grammar productions are correctly parsed according to the formal specification
#[test]
fn test_entity_grammar_specification() {
    // According to spec: entity_def = "entity" , identifier , "{" , { field_def } , "}" ;
    let input = r#"
        entity Farmer {
            id
            name
            location
        }
    "#;
    
    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);
    
    if let Definition::Entity(entity) = &program.definitions[0] {
        assert_eq!(entity.name, "Farmer");
        assert_eq!(entity.fields.len(), 3);
        assert_eq!(entity.fields[0].name, "id");
        assert_eq!(entity.fields[1].name, "name");
        assert_eq!(entity.fields[2].name, "location");
    } else {
        panic!("Expected entity definition");
    }
}

#[test]
fn test_rule_grammar_specification() {
    // According to spec: rule_def = "rule" , identifier , ":" , "if" , condition , "then" , action_list ;
    let input = r#"
        rule ValidateFarmer:
            if farmer.id != 0
            then mark_valid(farmer)
    "#;
    
    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);
    
    if let Definition::Rule(rule) = &program.definitions[0] {
        assert_eq!(rule.name, "ValidateFarmer");
        // Verify that rule has condition and actions
        // Additional verification would go here
    } else {
        panic!("Expected rule definition");
    }
}

#[test]
fn test_flow_grammar_specification() {
    // According to spec: flow_def = "flow" , identifier , "{" , action_list , "}" ;
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
        panic!("Expected flow definition");
    }
}

#[test]
fn test_constraint_grammar_specification() {
    // According to spec: constraint_def = "constraint" , identifier , ":" , condition ;
    let input = r#"
        constraint ValidFarmerId: farmer.id > 0
    "#;
    
    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);
    
    if let Definition::Constraint(constraint) = &program.definitions[0] {
        assert_eq!(constraint.name, "ValidFarmerId");
        // Verify that constraint has a condition
        // Additional verification would go here
    } else {
        panic!("Expected constraint definition");
    }
}

// Test complex conditions according to grammar:
// condition = expression | expression , logical_op , condition ;
#[test]
fn test_complex_conditions_grammar_specification() {
    let input = r#"
        rule ComplexCondition:
            if farmer.id != 0 and farmer.name != "" or farmer.location != "unknown"
            then validate_farmer(farmer)
    "#;
    
    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);
    
    if let Definition::Rule(rule) = &program.definitions[0] {
        assert_eq!(rule.name, "ComplexCondition");
    } else {
        panic!("Expected rule definition");
    }
}

// Test expressions according to grammar:
// expression = term , comparator , term | predicate ;
#[test]
fn test_expressions_grammar_specification() {
    let input = r#"
        rule ExpressionTest:
            if value == 42
            then do_something()
    "#;
    
    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);
}

// Test terms according to grammar:
// term = identifier | number | qualified_ref ;
// qualified_ref = identifier , "." , identifier ;
#[test]
fn test_terms_grammar_specification() {
    let input = r#"
        rule TermTest:
            if farmer.id != 0
            then validate_farmer(farmer.id, 42)
    "#;
    
    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);
}

// Test predicates according to grammar:
// predicate = identifier , "(" , [ argument_list ] , ")" ;
// argument_list = term , { "," , term } ;
#[test]
fn test_predicates_grammar_specification() {
    let input = r#"
        rule PredicateTest:
            if condition(x, y, 42)
            then action(a, b)
    "#;
    
    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);
}

// Test actions according to grammar:
// action_list = action , { "," , action } ;
// action = predicate | assignment | control_action ;
#[test]
fn test_actions_grammar_specification() {
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
}

// Test control actions according to grammar:
// control_action = if_action | loop_action | halt_action ;
#[test]
fn test_control_actions_grammar_specification() {
    let input = r#"
        flow ControlActionTest {
            if x > 0 then positive_action() else negative_action(),
            loop { process_item() },
            halt
        }
    "#;
    
    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.definitions.len(), 1);
}

// Test that all definition types can coexist in one program
#[test]
fn test_all_grammar_productions_coexist() {
    let input = r#"
        entity Farmer {
            id
            name
            location
        }
        
        rule ValidateFarmer:
            if farmer.id != 0
            then mark_valid(farmer)
            
        flow ProcessFarmers {
            load_farmers(),
            validate_farmers()
        }
        
        constraint ValidFarmerId: farmer.id > 0
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

// Test error cases to ensure proper error handling
#[test]
fn test_malformed_entity_definition() {
    let input = "entity IncompleteEntity { id";
    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    
    // This should produce an error since the entity definition is incomplete
    assert!(result.is_err() || !parser.get_errors().is_empty());
}

#[test]
fn test_malformed_rule_definition() {
    let input = "rule IncompleteRule: if condition";
    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    
    // This should produce an error since the rule definition is incomplete
    assert!(result.is_err() || !parser.get_errors().is_empty());
}