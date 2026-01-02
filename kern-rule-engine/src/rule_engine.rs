use std::collections::HashMap;
use crate::{ResolutionMode, ConflictEntry, RuleMatch, RuleEngineError, Value, ExecutionContext, RuleEngine};
use kern_graph_builder::{ExecutionGraph, GraphNode, GraphNodeType};

// Rule execution metadata
#[derive(Debug, Clone)]
pub struct RuleExecutionInfo {
    pub rule_id: u32,
    pub priority: u16,
    pub condition_graph_id: Option<u32>,
    pub action_graph_id: Option<u32>,
    pub dependencies: Vec<u32>,
    pub recursion_limit: u32,
    pub execution_count: u32,  // runtime tracker
}

impl RuleExecutionInfo {
    pub fn new(rule_id: u32) -> Self {
        RuleExecutionInfo {
            rule_id,
            priority: 10,  // Default normal priority
            condition_graph_id: None,
            action_graph_id: None,
            dependencies: Vec::new(),
            recursion_limit: 10,  // Default recursion limit
            execution_count: 0,
        }
    }
}