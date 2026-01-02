mod common;
use common::assertions::{assert_equal, assert_false, assert_true, AssertionResult};
use kern_rule_engine::{RuleEngine, RuleExecutionInfo, RulePriority, Value};
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

    assert_eq!(engine.rule_registry.len(), 0);
    assert_eq!(engine.step_count, 0);
    assert_eq!(engine.max_steps, 10000);
}

#[test]
fn test_adding_rules_to_registry() {
    let mut engine = RuleEngine::new(None);

    let rule_info = RuleExecutionInfo::new(1);
    engine.rule_registry.insert(1, rule_info);

    assert_eq!(engine.rule_registry.len(), 1);
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

    // Engine should track step count and max_steps
    assert_eq!(engine.step_count, engine.max_steps);
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

    match val_sym {
        Value::Sym(s) => assert_eq!(s, "test"),
        _ => panic!("Expected Sym"),
    }

    match val_bool {
        Value::Bool(b) => assert!(b),
        _ => panic!("Expected Bool"),
    }
}

#[test]
fn test_rule_engine_empty_execution_path() {
    let engine = RuleEngine::new(None);
    assert!(engine.execution_path.is_empty());
}

#[test]
fn test_rule_engine_priority_queue_empty() {
    let engine = RuleEngine::new(None);
    assert!(engine.priority_queue.is_empty());
}
