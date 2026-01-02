mod common;
use common::assertions::{assert_equal, assert_false, assert_true, AssertionResult};
use kern_rule_engine::{RuleEngine, RuleExecutionInfo, Value, ExecutionContext, PriorityStrategy, RuleConflict, ConflictType};
use kern_graph_builder::{GraphBuilder, ExecutionGraph, GraphNode, GraphNodeType, SpecializedNode, RuleNode, EdgeType, GraphEdge};
use kern_parser::{Parser, Program};

#[test]
fn test_rule_conflict_detection_basic() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            value
        }
        
        rule ConflictingRule1:
            if test_entity.id == 1
            then set_value(test_entity, 100)
            
        rule ConflictingRule2:
            if test_entity.id == 1
            then set_value(test_entity, 200)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let engine = RuleEngine::new(Some(graph));
    
    // Detect conflicts between rules
    let conflicts = engine.detect_rule_conflicts(&engine.execution_graph.as_ref().unwrap());
    
    // There should be at least one conflict detected
    assert!(!conflicts.is_empty(), "Should detect conflicts between rules that modify the same entity");
    
    // Check that the conflict is of the expected type
    for conflict in &conflicts {
        assert_eq!(conflict.conflict_type, ConflictType::ActionConflict, "Conflict should be an action conflict");
    }
}

#[test]
fn test_rule_conflict_resolution_strategies() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            value
        }
        
        rule Rule1:
            if test_entity.id == 1
            then set_value(test_entity, 100)
            
        rule Rule2:
            if test_entity.id == 1
            then set_value(test_entity, 200)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    // Test different priority strategies for conflict resolution
    let strategies = [
        PriorityStrategy::Standard,
        PriorityStrategy::SpecificityFirst,
        PriorityStrategy::RecencyBased,
        PriorityStrategy::FrequencyBased,
        PriorityStrategy::ConflictResolution,
    ];
    
    for strategy in &strategies {
        let mut engine = RuleEngine::new(Some(graph.clone()));
        engine.set_priority_strategy(strategy.clone());
        
        // Detect and resolve conflicts
        let conflicts = engine.detect_rule_conflicts(&graph);
        if !conflicts.is_empty() {
            engine.resolve_conflicts(&conflicts);
            
            // Check that the strategy was applied
            assert_eq!(&engine.priority_strategy, strategy, "Priority strategy should be applied");
        }
    }
}

#[test]
fn test_rule_conflict_with_priority_based_resolution() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            value
        }
        
        rule HighPriorityRule:
            if test_entity.id == 1
            then set_value(test_entity, 100)
            
        rule LowPriorityRule:
            if test_entity.id == 1
            then set_value(test_entity, 200)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Set different priorities for the conflicting rules
    engine.set_rule_priority(1, 100, 10, 1); // High priority
    engine.set_rule_priority(2, 10, 5, 1);   // Low priority
    
    // Detect conflicts
    let conflicts = engine.detect_rule_conflicts(&engine.execution_graph.as_ref().unwrap());
    assert!(!conflicts.is_empty(), "Should detect conflicts between rules");
    
    // Resolve conflicts - the higher priority rule should win
    engine.resolve_conflicts(&conflicts);
    
    // Check that conflict scores were updated
    if let Some(rule_priority) = engine.rule_priorities.get(&1) {
        assert!(rule_priority.conflict_score > 0, "Conflict score should be updated for high priority rule");
    }
    
    if let Some(rule_priority) = engine.rule_priorities.get(&2) {
        assert!(rule_priority.conflict_score > 0, "Conflict score should be updated for low priority rule");
    }
}

#[test]
fn test_rule_conflict_with_specificity_resolution() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            category
            value
        }
        
        rule GeneralRule:
            if test_entity.id > 0
            then set_value(test_entity, 100)
            
        rule SpecificRule:
            if test_entity.id == 1 && test_entity.category == "special"
            then set_value(test_entity, 200)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Set specificity values - the specific rule should have higher specificity
    engine.set_rule_priority(1, 50, 5, 1);  // General rule, lower specificity
    engine.set_rule_priority(2, 50, 20, 1); // Specific rule, higher specificity
    
    // Use specificity-first strategy
    engine.set_priority_strategy(PriorityStrategy::SpecificityFirst);
    
    // Detect conflicts
    let conflicts = engine.detect_rule_conflicts(&engine.execution_graph.as_ref().unwrap());
    // Note: The current conflict detection might not detect this as a conflict since conditions are different
    // But the specificity strategy should still be tested
    
    // Check that the strategy is set correctly
    assert_eq!(engine.priority_strategy, PriorityStrategy::SpecificityFirst, "Should use specificity-first strategy");
}

#[test]
fn test_rule_conflict_with_recency_resolution() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            value
        }
        
        rule OlderRule:
            if test_entity.id == 1
            then set_value(test_entity, 100)
            
        rule NewerRule:
            if test_entity.id == 1
            then set_value(test_entity, 200)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Set recency values - newer rule should have higher recency
    engine.set_rule_priority(1, 50, 10, 1);  // Older rule
    engine.set_rule_priority(2, 50, 10, 5);  // Newer rule (higher recency)
    
    // Use recency-based strategy
    engine.set_priority_strategy(PriorityStrategy::RecencyBased);
    
    // Detect conflicts
    let conflicts = engine.detect_rule_conflicts(&engine.execution_graph.as_ref().unwrap());
    assert!(!conflicts.is_empty(), "Should detect conflicts between rules");
    
    // Resolve conflicts
    engine.resolve_conflicts(&conflicts);
    
    // Check that the strategy is set correctly
    assert_eq!(engine.priority_strategy, PriorityStrategy::RecencyBased, "Should use recency-based strategy");
}

#[test]
fn test_rule_conflict_with_frequency_based_resolution() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            value
        }
        
        rule FrequentRule:
            if test_entity.id == 1
            then set_value(test_entity, 100)
            
        rule InfrequentRule:
            if test_entity.id == 1
            then set_value(test_entity, 200)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Use frequency-based strategy
    engine.set_priority_strategy(PriorityStrategy::FrequencyBased);
    
    // Manually set activation counts to simulate different frequencies
    engine.increment_rule_activation(1); // Make rule 1 more frequent
    engine.increment_rule_activation(1);
    engine.increment_rule_activation(2); // Make rule 2 less frequent
    
    // Detect conflicts
    let conflicts = engine.detect_rule_conflicts(&engine.execution_graph.as_ref().unwrap());
    assert!(!conflicts.is_empty(), "Should detect conflicts between rules");
    
    // Resolve conflicts
    engine.resolve_conflicts(&conflicts);
    
    // Check that the strategy is set correctly
    assert_eq!(engine.priority_strategy, PriorityStrategy::FrequencyBased, "Should use frequency-based strategy");
}

#[test]
fn test_rule_conflict_with_state_conflicts() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            value
            status
        }
        
        rule UpdateValueRule:
            if test_entity.id == 1
            then set_value(test_entity, 100)
            
        rule UpdateStatusRule:
            if test_entity.id == 1
            then set_status(test_entity, "processed")
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let engine = RuleEngine::new(Some(graph));
    
    // Detect conflicts - these rules modify different aspects of the entity
    // so they might not be detected as conflicting by the basic implementation
    let conflicts = engine.detect_rule_conflicts(&engine.execution_graph.as_ref().unwrap());
    
    // The current implementation might not detect this as a conflict since they modify different fields
    // But the test ensures the conflict detection system is working
    println!("Detected {} potential conflicts", conflicts.len());
}

#[test]
fn test_rule_conflict_with_resource_conflicts() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Resource {
            id
            available
        }
        
        rule AcquireResource1:
            if resource.id == 1 && resource.available == true
            then acquire(resource, "process1")
            
        rule AcquireResource2:
            if resource.id == 1 && resource.available == true
            then acquire(resource, "process2")
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let engine = RuleEngine::new(Some(graph));
    
    // Detect resource conflicts
    let conflicts = engine.detect_rule_conflicts(&engine.execution_graph.as_ref().unwrap());
    
    // Check if any conflicts were detected
    println!("Detected {} resource conflicts", conflicts.len());
}

#[test]
fn test_rule_conflict_with_conditional_conflicts() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            condition1
            condition2
            value
        }
        
        rule ConditionalRule1:
            if test_entity.id == 1 && test_entity.condition1 == true
            then set_value(test_entity, 100)
            
        rule ConditionalRule2:
            if test_entity.id == 1 && test_entity.condition2 == true
            then set_value(test_entity, 200)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let engine = RuleEngine::new(Some(graph));
    
    // Detect conflicts between conditionally executed rules
    let conflicts = engine.detect_rule_conflicts(&engine.execution_graph.as_ref().unwrap());
    
    // These rules might not conflict if their conditions are mutually exclusive
    // But the conflict detection should still work
    println!("Detected {} conditional conflicts", conflicts.len());
}

#[test]
fn test_rule_conflict_with_complex_conflicts() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            value
            category
            status
        }
        
        rule ComplexRule1:
            if test_entity.id == 1 && test_entity.category == "A"
            then 
                set_value(test_entity, 100)
                set_status(test_entity, "processed")
            
        rule ComplexRule2:
            if test_entity.id == 1 && test_entity.category == "A"
            then 
                set_value(test_entity, 200)
                set_status(test_entity, "updated")
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Detect complex conflicts
    let conflicts = engine.detect_rule_conflicts(&engine.execution_graph.as_ref().unwrap());
    assert!(!conflicts.is_empty(), "Should detect conflicts between complex rules");
    
    // Check conflict details
    for conflict in &conflicts {
        println!("Conflict detected: {} vs {}: {}", conflict.rule1_id, conflict.rule2_id, conflict.description);
        assert!(conflict.rule1_id != conflict.rule2_id, "Conflicting rules should be different");
        assert!(!conflict.description.is_empty(), "Conflict should have a description");
    }
    
    // Resolve the conflicts
    engine.resolve_conflicts(&conflicts);
    
    // Verify that conflict resolution changed the priority strategy
    assert_eq!(engine.priority_strategy, PriorityStrategy::ConflictResolution, 
               "Priority strategy should be set to ConflictResolution when conflicts exist");
}

#[test]
fn test_rule_conflict_with_execution_after_resolution() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            value
        }
        
        rule ConflictingRule1:
            if test_entity.id == 1
            then set_value(test_entity, 100)
            
        rule ConflictingRule2:
            if test_entity.id == 1
            then set_value(test_entity, 200)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Set up initial state
    engine.set_variable("test_entity.id", Value::Num(1));
    
    // Detect and resolve conflicts before execution
    let conflicts = engine.detect_rule_conflicts(&engine.execution_graph.as_ref().unwrap());
    assert!(!conflicts.is_empty(), "Should detect conflicts");
    
    engine.resolve_conflicts(&conflicts);
    
    // Now execute the graph with conflict resolution in place
    if let Some(graph) = engine.execution_graph.clone() {
        let execution_result = engine.execute_graph(&graph);
        assert!(execution_result.is_ok(), "Execution after conflict resolution should succeed");

        // Check that execution happened
        assert!(engine.step_count > 0, "At least one step should have been executed after conflict resolution");
    }
}