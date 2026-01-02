use crate::types::{Pattern, Value};
use crate::RuleEngine;
use kern_graph_builder::{ContextPool, ExecutionGraph, GraphMeta, Register, RegisterSet};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_graph() -> ExecutionGraph {
        ExecutionGraph {
            nodes: vec![],
            edges: vec![],
            node_count: 0,
            edge_count: 0,
            entry_points: vec![],
            entry_count: 0,
            registers: RegisterSet {
                regs: [Register {
                    reg_type: 0,
                    value_id: 0,
                }; 16],
            },
            contexts: ContextPool { contexts: vec![] },
            metadata: GraphMeta {
                build_hash: 0,
                version: 0,
            },
        }
    }

    #[test]
    fn test_rule_engine_initialization() {
        let graph = create_mock_graph();
        let engine = RuleEngine::new(Some(graph));
        assert!(engine.execution_graph.is_some());
    }

    #[test]
    fn test_pattern_matching_basic() {
        let graph = create_mock_graph();
        let engine = RuleEngine::new(Some(graph));

        // Test Value equality
        let v1 = Value::Num(42);
        let v2 = Value::Num(42);
        assert_eq!(v1, v2);

        // Test Pattern matching (using Variable)
        let _pattern = Pattern::Variable("x".to_string());
        let _value = Value::Num(100);
    }

    #[test]
    fn test_rule_execution_flow() {
        let graph = create_mock_graph();
        let mut engine = RuleEngine::new(Some(graph));

        if let Some(graph_clone) = engine.execution_graph.clone() {
            let result = engine.execute_graph(&graph_clone);
            assert!(result.is_ok());
        }
    }
}
