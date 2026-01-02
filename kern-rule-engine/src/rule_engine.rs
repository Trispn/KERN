use crate::types::{
    ConflictEntry, ExecutionContext, PriorityStrategy, ResolutionMode, RuleEngineError,
    RuleExecutionInfo, RuleMatch, RulePriority, Value,
};
use kern_graph_builder::{ExecutionGraph, GraphNode, GraphNodeType, SpecializedNode};
use std::collections::HashMap;
