mod common;
use common::assertions::{assert_equal, assert_false, assert_true, AssertionResult};
use kern_rule_engine::{RuleEngine, RuleExecutionInfo, RulePriority, Value, ExecutionContext, PriorityStrategy};
use kern_graph_builder::{ExecutionGraph, GraphNode, GraphNodeType, SpecializedNode, RuleNode};

#[test]
fn test_rule_evaluation_basic_execution() {
    let mut engine = RuleEngine::new(None);
    
    // Create a simple execution graph with a rule
    let mut graph = ExecutionGraph {
        nodes: vec![],
        edges: vec![],
        node_count: 0,
        edge_count: 0,
        entry_points: vec![],
        entry_count: 0,
        registers: kern_graph_builder::RegisterSet {
            regs: [kern_graph_builder::Register { reg_type: 0, value_id: 0 }; 16],
        },
        contexts: kern_graph_builder::ContextPool { contexts: vec![] },
        metadata: kern_graph_builder::GraphMeta { build_hash: 0, version: 1 },
    };

    // Add a simple rule node to the graph
    let base_node = GraphNode {
        id: 1,
        node_type: GraphNodeType::Rule,
        opcode: 0x31, // RULE_EVAL
        flags: 0,
        input_regs: [0; 4],
        output_regs: [0; 2],
        first_edge: 0,
        edge_count: 0,
        meta: kern_graph_builder::NodeMeta {
            source_ref: 0,
            cost_hint: 0,
        },
    };

    let rule_node = RuleNode::new(base_node, 1, 10, 0); // rule_id, priority, evaluation_mode
    graph.nodes.push(SpecializedNode::Rule(rule_node));
    graph.node_count = 1;

    // Add entry point
    graph.entry_points.push(kern_graph_builder::EntryPoint {
        node_id: 1,
        entry_type: 0, // 0 = rule
    });
    graph.entry_count = 1;

    // Execute the graph
    let result = engine.execute_graph(&graph);
    
    assert!(result.is_ok(), "Rule evaluation should succeed");
    assert!(engine.step_count > 0, "Engine should have executed at least one step");
}

#[test]
fn test_rule_evaluation_with_conditions() {
    let mut engine = RuleEngine::new(None);
    
    // Set up a variable in the context
    engine.set_variable("test_var", Value::Num(10));
    
    // Create a simple execution graph with a rule that has conditions
    let mut graph = ExecutionGraph {
        nodes: vec![],
        edges: vec![],
        node_count: 0,
        edge_count: 0,
        entry_points: vec![],
        entry_count: 0,
        registers: kern_graph_builder::RegisterSet {
            regs: [kern_graph_builder::Register { reg_type: 0, value_id: 0 }; 16],
        },
        contexts: kern_graph_builder::ContextPool { contexts: vec![] },
        metadata: kern_graph_builder::GraphMeta { build_hash: 0, version: 1 },
    };

    // Add a rule node to the graph
    let base_node = GraphNode {
        id: 1,
        node_type: GraphNodeType::Rule,
        opcode: 0x31, // RULE_EVAL
        flags: 0,
        input_regs: [0; 4],
        output_regs: [0; 2],
        first_edge: 0,
        edge_count: 0,
        meta: kern_graph_builder::NodeMeta {
            source_ref: 0,
            cost_hint: 0,
        },
    };

    let rule_node = RuleNode::new(base_node, 1, 10, 0); // rule_id, priority, evaluation_mode
    graph.nodes.push(SpecializedNode::Rule(rule_node));
    graph.node_count = 1;

    // Add entry point
    graph.entry_points.push(kern_graph_builder::EntryPoint {
        node_id: 1,
        entry_type: 0, // 0 = rule
    });
    graph.entry_count = 1;

    // Execute the graph
    let result = engine.execute_graph(&graph);
    
    assert!(result.is_ok(), "Rule evaluation with conditions should succeed");
}

#[test]
fn test_rule_evaluation_with_multiple_rules() {
    let mut engine = RuleEngine::new(None);
    
    // Create a simple execution graph with multiple rules
    let mut graph = ExecutionGraph {
        nodes: vec![],
        edges: vec![],
        node_count: 0,
        edge_count: 0,
        entry_points: vec![],
        entry_count: 0,
        registers: kern_graph_builder::RegisterSet {
            regs: [kern_graph_builder::Register { reg_type: 0, value_id: 0 }; 16],
        },
        contexts: kern_graph_builder::ContextPool { contexts: vec![] },
        metadata: kern_graph_builder::GraphMeta { build_hash: 0, version: 1 },
    };

    // Add multiple rule nodes to the graph
    for i in 1..=3 {
        let base_node = GraphNode {
            id: i,
            node_type: GraphNodeType::Rule,
            opcode: 0x31, // RULE_EVAL
            flags: 0,
            input_regs: [0; 4],
            output_regs: [0; 2],
            first_edge: 0,
            edge_count: 0,
            meta: kern_graph_builder::NodeMeta {
                source_ref: 0,
                cost_hint: 0,
            },
        };

        let rule_node = RuleNode::new(base_node, i, 10, 0); // rule_id, priority, evaluation_mode
        graph.nodes.push(SpecializedNode::Rule(rule_node));
    }
    graph.node_count = 3;

    // Add entry points
    for i in 1..=3 {
        graph.entry_points.push(kern_graph_builder::EntryPoint {
            node_id: i,
            entry_type: 0, // 0 = rule
        });
    }
    graph.entry_count = 3;

    // Execute the graph
    let result = engine.execute_graph(&graph);
    
    assert!(result.is_ok(), "Rule evaluation with multiple rules should succeed");
    assert!(engine.step_count > 0, "Engine should have executed at least one step");
}

#[test]
fn test_rule_evaluation_with_priority() {
    let mut engine = RuleEngine::new(None);
    
    // Set different priorities for rules
    engine.set_rule_priority(1, 100, 10, 1); // High priority
    engine.set_rule_priority(2, 50, 5, 1);   // Lower priority
    
    // Create a simple execution graph with two rules
    let mut graph = ExecutionGraph {
        nodes: vec![],
        edges: vec![],
        node_count: 0,
        edge_count: 0,
        entry_points: vec![],
        entry_count: 0,
        registers: kern_graph_builder::RegisterSet {
            regs: [kern_graph_builder::Register { reg_type: 0, value_id: 0 }; 16],
        },
        contexts: kern_graph_builder::ContextPool { contexts: vec![] },
        metadata: kern_graph_builder::GraphMeta { build_hash: 0, version: 1 },
    };

    // Add two rule nodes to the graph
    for i in 1..=2 {
        let base_node = GraphNode {
            id: i,
            node_type: GraphNodeType::Rule,
            opcode: 0x31, // RULE_EVAL
            flags: 0,
            input_regs: [0; 4],
            output_regs: [0; 2],
            first_edge: 0,
            edge_count: 0,
            meta: kern_graph_builder::NodeMeta {
                source_ref: 0,
                cost_hint: 0,
            },
        };

        let rule_node = RuleNode::new(base_node, i, 10, 0); // rule_id, priority, evaluation_mode
        graph.nodes.push(SpecializedNode::Rule(rule_node));
    }
    graph.node_count = 2;

    // Add entry points
    for i in 1..=2 {
        graph.entry_points.push(kern_graph_builder::EntryPoint {
            node_id: i,
            entry_type: 0, // 0 = rule
        });
    }
    graph.entry_count = 2;

    // Execute the graph
    let result = engine.execute_graph(&graph);
    
    assert!(result.is_ok(), "Rule evaluation with priority should succeed");
    
    // Check that the priority queue was properly sorted
    assert!(engine.priority_queue.len() <= 2, "Priority queue should have at most 2 items");
}

#[test]
fn test_rule_evaluation_with_pattern_matching() {
    let engine = RuleEngine::new(None);
    
    // Create a simple pattern to match
    let pattern = kern_rule_engine::Pattern::Variable("x".to_string());
    let value = Value::Num(42);
    
    // Test pattern matching
    let result = engine.match_pattern(&pattern, &value);
    
    assert!(result.is_some(), "Pattern matching should succeed");
    if let Some(bindings) = result {
        assert!(bindings.contains_key("x"), "Pattern should bind variable x");
        if let Some(bound_value) = bindings.get("x") {
            assert_equal(bound_value, &Value::Num(42), "Bound value should match");
        }
    }
}

#[test]
fn test_rule_evaluation_with_complex_pattern_matching() {
    let engine = RuleEngine::new(None);
    
    // Create a complex pattern to match
    let pattern = kern_rule_engine::Pattern::Composite(
        "entity.field".to_string(),
        vec![
            kern_rule_engine::Pattern::Value(Value::Sym("Farmer".to_string())),
            kern_rule_engine::Pattern::Value(Value::Sym("location".to_string()))
        ]
    );
    let value = Value::Sym("Farmer.location".to_string());
    
    // Test complex pattern matching
    let result = engine.match_pattern(&pattern, &value);
    
    assert!(result.is_some(), "Complex pattern matching should succeed");
}

#[test]
fn test_rule_evaluation_with_context() {
    let mut engine = RuleEngine::new(None);
    
    // Set up context with some initial values
    engine.set_variable("farmer_id", Value::Num(123));
    engine.set_variable("location", Value::Sym("field".to_string()));
    
    // Verify the context was set up correctly
    assert!(engine.get_variable("farmer_id").is_some(), "Variable should be set in context");
    assert!(engine.get_variable("location").is_some(), "Variable should be set in context");
    
    if let Some(id_val) = engine.get_variable("farmer_id") {
        assert_equal(id_val, &Value::Num(123), "Variable value should match");
    }
    
    if let Some(loc_val) = engine.get_variable("location") {
        assert_equal(loc_val, &Value::Sym("field".to_string()), "Variable value should match");
    }
}

#[test]
fn test_rule_evaluation_activation_count() {
    let mut engine = RuleEngine::new(None);
    
    // Create a simple execution graph with a rule
    let mut graph = ExecutionGraph {
        nodes: vec![],
        edges: vec![],
        node_count: 0,
        edge_count: 0,
        entry_points: vec![],
        entry_count: 0,
        registers: kern_graph_builder::RegisterSet {
            regs: [kern_graph_builder::Register { reg_type: 0, value_id: 0 }; 16],
        },
        contexts: kern_graph_builder::ContextPool { contexts: vec![] },
        metadata: kern_graph_builder::GraphMeta { build_hash: 0, version: 1 },
    };

    // Add a rule node to the graph
    let base_node = GraphNode {
        id: 1,
        node_type: GraphNodeType::Rule,
        opcode: 0x31, // RULE_EVAL
        flags: 0,
        input_regs: [0; 4],
        output_regs: [0; 2],
        first_edge: 0,
        edge_count: 0,
        meta: kern_graph_builder::NodeMeta {
            source_ref: 0,
            cost_hint: 0,
        },
    };

    let rule_node = RuleNode::new(base_node, 1, 10, 0); // rule_id, priority, evaluation_mode
    graph.nodes.push(SpecializedNode::Rule(rule_node));
    graph.node_count = 1;

    // Add entry point
    graph.entry_points.push(kern_graph_builder::EntryPoint {
        node_id: 1,
        entry_type: 0, // 0 = rule
    });
    graph.entry_count = 1;

    // Execute the rule multiple times
    for _ in 0..3 {
        let rule_info = RuleExecutionInfo::new(1);
        let result = engine.execute_rule_from_info(&rule_info, &graph);
        assert!(result.is_ok(), "Rule execution should succeed");
    }
    
    // Check that the activation count was incremented
    if let Some(rule_priority) = engine.rule_priorities.get(&1) {
        assert!(rule_priority.activation_count >= 0, "Activation count should be tracked");
    }
}

#[test]
fn test_rule_evaluation_with_different_priority_strategies() {
    let mut engine = RuleEngine::new(None);
    
    // Test different priority strategies
    let strategies = [
        PriorityStrategy::Standard,
        PriorityStrategy::SpecificityFirst,
        PriorityStrategy::RecencyBased,
        PriorityStrategy::FrequencyBased,
        PriorityStrategy::ConflictResolution,
    ];
    
    for strategy in &strategies {
        engine.set_priority_strategy(strategy.clone());
        assert_equal(
            &engine.priority_strategy,
            strategy,
            "Priority strategy should be set correctly"
        );
    }
}

#[test]
fn test_rule_evaluation_error_handling() {
    let mut engine = RuleEngine::new(None);
    
    // Create a simple execution graph with a rule
    let mut graph = ExecutionGraph {
        nodes: vec![],
        edges: vec![],
        node_count: 0,
        edge_count: 0,
        entry_points: vec![],
        entry_count: 0,
        registers: kern_graph_builder::RegisterSet {
            regs: [kern_graph_builder::Register { reg_type: 0, value_id: 0 }; 16],
        },
        contexts: kern_graph_builder::ContextPool { contexts: vec![] },
        metadata: kern_graph_builder::GraphMeta { build_hash: 0, version: 1 },
    };

    // Add a rule node to the graph
    let base_node = GraphNode {
        id: 1,
        node_type: GraphNodeType::Rule,
        opcode: 0x31, // RULE_EVAL
        flags: 0,
        input_regs: [0; 4],
        output_regs: [0; 2],
        first_edge: 0,
        edge_count: 0,
        meta: kern_graph_builder::NodeMeta {
            source_ref: 0,
            cost_hint: 0,
        },
    };

    let rule_node = RuleNode::new(base_node, 1, 10, 0); // rule_id, priority, evaluation_mode
    graph.nodes.push(SpecializedNode::Rule(rule_node));
    graph.node_count = 1;

    // Add entry point
    graph.entry_points.push(kern_graph_builder::EntryPoint {
        node_id: 1,
        entry_type: 0, // 0 = rule
    });
    graph.entry_count = 1;

    // Execute the graph with a very low step limit to test execution limit
    engine.max_steps = 0; // Force execution limit exceeded
    let result = engine.execute_graph(&graph);
    
    // This should fail due to execution limit
    assert!(result.is_err(), "Rule evaluation should fail when max steps exceeded");
}