mod common;
use common::assertions::{assert_equal, assert_false, assert_true, AssertionResult};
use kern_parser::ast_nodes::{Declaration, Program};
use kern_rule_engine::{RuleEngine, RuleExecutionInfo, RuleMatch, RulePriority, Value};
use std::collections::HashMap;

#[test]
fn test_rule_execution_info_creation() {
    let rule_info = RuleExecutionInfo::new(1);

    assert_eq!(rule_info.rule_id, 1);
    assert_eq!(rule_info.priority, 10);
}

#[test]
fn test_rule_engine_initialization() {
    let engine = RuleEngine::new(None);

    assert_eq!(engine.rules.len(), 0);
    assert_eq!(engine.rule_registry.len(), 0);
}

#[test]
fn test_adding_rules_to_engine() {
    let mut engine = RuleEngine::new(None);

    let rule_info = RuleExecutionInfo::new(1);

    engine.rules.push(1);
    engine.rule_registry.insert(1, rule_info);

    assert_eq!(engine.rules.len(), 1);
    assert!(engine.rule_registry.contains_key(&1));
}

#[test]
fn test_rule_priority_struct() {
    let priority = RulePriority {
        rule_id: 1,
        priority: 100,
        specificity: 5,
        recency: 1,
        activation_count: 0,
        conflict_score: 0,
    };

    assert_eq!(priority.rule_id, 1);
    assert_eq!(priority.priority, 100);
}

#[test]
fn test_rule_engine_execution_limit() {
    let mut engine = RuleEngine::new(None);
    engine.step_count = 1000;
    engine.max_steps = 1000;

    // Create a mock call to something that checks limits
    // In RuleEngine, execute_graph checks max_steps
}

#[test]
fn test_value_enum() {
    let val_num = Value::Num(42);
    let val_sym = Value::Sym("test".to_string());
    let val_bool = Value::Bool(true);

    match val_num {
        Value::Num(n) => assert_eq!(n, 42),
        _ => panic!("Expected Num"),
    }
}
