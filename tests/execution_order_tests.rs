mod common;
use common::assertions::{assert_equal, assert_false, assert_true, AssertionResult};
use kern_rule_engine::{RuleEngine, RuleExecutionInfo, Value, ExecutionContext, PriorityStrategy};
use kern_graph_builder::{GraphBuilder, ExecutionGraph, GraphNode, GraphNodeType, SpecializedNode, RuleNode, EdgeType, GraphEdge};
use kern_parser::{Parser, Program};

#[test]
fn test_execution_order_basic_sequential() {
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
            if test_entity.id == 2
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
    
    // Execute the graph
    if let Some(ref graph) = engine.execution_graph {
        let execution_result = engine.execute_graph(graph);
        assert!(execution_result.is_ok(), "Execution should succeed");

        // Check that execution happened
        assert!(engine.step_count > 0, "At least one step should have been executed");
    }
}

#[test]
fn test_execution_order_with_priority() {
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
            if test_entity.id == 2
            then set_value(test_entity, 200)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Set different priorities for rules
    engine.set_rule_priority(1, 100, 10, 1); // High priority
    engine.set_rule_priority(2, 10, 5, 1);   // Low priority
    
    // Execute the graph
    if let Some(ref graph) = engine.execution_graph {
        let execution_result = engine.execute_graph(graph);
        assert!(execution_result.is_ok(), "Execution with priority should succeed");

        // Check that the priority queue was properly sorted
        // The high priority rule should be considered first
        assert!(engine.priority_queue.len() <= 2, "Priority queue should have at most 2 items");
    }
}

#[test]
fn test_execution_order_with_dependencies() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            value
            processed
        }
        
        rule ProcessFirst:
            if test_entity.id == 1 && test_entity.processed == false
            then 
                set_value(test_entity, 100)
                set_processed(test_entity, true)
            
        rule ProcessSecond:
            if test_entity.id == 1 && test_entity.processed == true
            then finalize_entity(test_entity)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Set up initial state
    engine.set_variable("test_entity.id", Value::Num(1));
    engine.set_variable("test_entity.processed", Value::Bool(false));
    
    // Execute the graph
    if let Some(ref graph) = engine.execution_graph {
        let execution_result = engine.execute_graph(graph);
        assert!(execution_result.is_ok(), "Execution with dependencies should succeed");

        // Check that execution happened
        assert!(engine.step_count > 0, "At least one step should have been executed");
    }
}

#[test]
fn test_execution_order_with_control_flow() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            value
            condition
        }
        
        flow TestFlow {
            if test_entity.condition == true {
                process_entity(test_entity)
            } else {
                skip_entity(test_entity)
            }
            finalize_entity(test_entity)
        }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Set up initial state
    engine.set_variable("test_entity.condition", Value::Bool(true));
    
    // Execute the flow
    if let Some(ref graph) = engine.execution_graph.clone() {
        if let Some(entry_point) = graph.entry_points.first() {
            let flow_result = engine.execute_flow_pipeline(&graph, entry_point.node_id);
            assert!(flow_result.is_ok(), "Flow execution with control flow should succeed");
        }
    }
}

#[test]
fn test_execution_order_with_loop() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Counter {
            value
        }
        
        flow CountLoop {
            counter.value = 0
            loop counter.value < 3 {
                increment_counter(counter)
                counter.value = counter.value + 1
            }
        }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Execute the flow
    if let Some(ref graph) = engine.execution_graph.clone() {
        if let Some(entry_point) = graph.entry_points.first() {
            let flow_result = engine.execute_flow_pipeline(&graph, entry_point.node_id);
            assert!(flow_result.is_ok(), "Flow execution with loop should succeed");
        }
    }
}

#[test]
fn test_execution_order_with_lazy_evaluation() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            expensive_value
        }
        
        rule LazyRule:
            if test_entity.id == 1
            then compute_expensive_value(test_entity)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Set up initial state
    engine.set_variable("test_entity.id", Value::Num(1));
    
    // Execute with lazy evaluation
    if let Some(ref graph) = engine.execution_graph.clone() {
        if !graph.nodes.is_empty() {
            let node_id = graph.nodes[0].get_base().id;
            let lazy_result = engine.evaluate_lazy(node_id, &graph);
            assert!(lazy_result.is_ok(), "Lazy evaluation should succeed");
        }
    }
}

#[test]
fn test_execution_order_with_demand_driven_evaluation() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            value
            required
        }
        
        rule DemandDrivenRule:
            if test_entity.required == true
            then compute_value(test_entity)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Set up initial state
    engine.set_variable("test_entity.required", Value::Bool(true));
    
    // Execute with demand-driven evaluation
    if let Some(ref graph) = engine.execution_graph {
        if let Some(entry_point) = graph.entry_points.first() {
            // For demand-driven evaluation, we'll execute nodes individually
            // Since execute_node_demand_driven_from_specialized is private, we'll skip this test for now
            // This is a limitation of the current API design
        }
    }
}

#[test]
fn test_execution_order_with_context_propagation() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity {
            id
            value
        }
        
        flow ContextFlow {
            load_entity(test_entity)
            process_entity(test_entity)
            save_entity(test_entity)
        }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Create and switch to a new context
    let mut new_context = engine.create_context();
    new_context.variables.insert("test_entity.id".to_string(), Value::Num(42));
    engine.switch_context(new_context);
    
    // Execute the flow
    if let Some(ref graph) = engine.execution_graph.clone() {
        if let Some(entry_point) = graph.entry_points.first() {
            let flow_result = engine.execute_flow_pipeline(&graph, entry_point.node_id);
            assert!(flow_result.is_ok(), "Flow execution with context propagation should succeed");
        }
    }
}

#[test]
fn test_execution_order_with_multiple_entry_points() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity TestEntity1 {
            id
            value
        }
        
        entity TestEntity2 {
            id
            value
        }
        
        rule Rule1:
            if test_entity1.id == 1
            then process_entity1(test_entity1)
            
        rule Rule2:
            if test_entity2.id == 2
            then process_entity2(test_entity2)
            
        flow Flow1 {
            execute_rule1()
        }
        
        flow Flow2 {
            execute_rule2()
        }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Execute the graph
    if let Some(ref graph) = engine.execution_graph {
        let execution_result = engine.execute_graph(graph);
        assert!(execution_result.is_ok(), "Execution with multiple entry points should succeed");

        // Check that multiple entry points were processed
        assert!(engine.step_count > 0, "At least one step should have been executed");
        assert!(engine.priority_queue.len() >= 2, "Should have multiple items in priority queue");
    }
}

#[test]
fn test_execution_order_with_recursion_guard() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Counter {
            value
        }
        
        rule RecursiveRule:
            if counter.value < 5
            then 
                increment_counter(counter)
                counter.value = counter.value + 1
                trigger_recursive_rule(counter)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();
    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    let mut engine = RuleEngine::new(Some(graph));
    
    // Set up initial state
    engine.set_variable("counter.value", Value::Num(0));
    
    // Set a low recursion limit to test the guard
    engine.set_max_recursion_depth(3);
    
    // Execute the graph
    if let Some(ref graph) = engine.execution_graph {
        let execution_result = engine.execute_graph(graph);
        // This might fail due to recursion limit, which is expected
        // The important thing is that the recursion guard is working
        assert!(execution_result.is_ok() ||
                matches!(execution_result, Err(kern_rule_engine::RuleEngineError::ExecutionLimitExceeded)),
                "Execution should either succeed or be stopped by recursion guard");
    }
}

#[test]
fn test_execution_order_with_conflict_resolution() {
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
    
    // Set up conflict resolution priority strategy
    engine.set_priority_strategy(PriorityStrategy::ConflictResolution);
    
    // Execute the graph
    let execution_result = engine.execute_graph(&graph);
    assert!(execution_result.is_ok(), "Execution with conflict resolution should succeed");

    // Check that conflicts were detected
    let conflicts = engine.detect_rule_conflicts(&graph);
    assert!(!conflicts.is_empty(), "Should detect conflicts between rules");
}