use std::collections::HashMap;
use crate::flow_execution_context::FlowExecutionContext;
use crate::flow_evaluator::{FlowEvaluator, FlowEvaluationError};
use crate::types::Value;
use kern_graph_builder::{ExecutionGraph, GraphNode};

/// Manages lazy evaluation of flow steps
pub struct LazyEvaluationManager {
    evaluated_results: HashMap<String, Value>,
}

impl LazyEvaluationManager {
    pub fn new() -> Self {
        LazyEvaluationManager {
            evaluated_results: HashMap::new(),
        }
    }

    /// Evaluates a step lazily, only if not already evaluated
    pub fn evaluate_lazy(
        &mut self,
        step_id: u32,
        evaluator: &mut FlowEvaluator,
        graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        let cache_key = format!("step_{}", step_id);

        // Check if result is already cached
        if let Some(cached_result) = self.evaluated_results.get(&cache_key) {
            return Ok(cached_result.clone());
        }

        // Find the step node in the graph
        let step_node = graph.nodes.iter()
            .find(|n| n.id == step_id)
            .ok_or(FlowEvaluationError::NodeNotFound(step_id))?;

        // Evaluate the step
        let result = evaluator.execute_node(step_node, graph, context)?;

        // Cache the result
        self.evaluated_results.insert(cache_key, result.clone());

        Ok(result)
    }

    /// Evaluates a step with dependencies, evaluating dependencies lazily first
    pub fn evaluate_with_dependencies(
        &mut self,
        step_id: u32,
        evaluator: &mut FlowEvaluator,
        graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // First, evaluate all dependencies lazily
        let dependencies = self.get_dependencies(step_id, graph);
        for dep_id in dependencies {
            self.evaluate_lazy(dep_id, evaluator, graph, context)?;
        }

        // Then evaluate the target step
        self.evaluate_lazy(step_id, evaluator, graph, context)
    }

    /// Gets all dependencies for a given step
    fn get_dependencies(&self, step_id: u32, graph: &ExecutionGraph) -> Vec<u32> {
        let mut dependencies = Vec::new();

        // Find all nodes that this step depends on via data edges
        for edge in &graph.edges {
            if edge.to_node == step_id && edge.edge_type == kern_graph_builder::EdgeType::Data {
                dependencies.push(edge.from_node);
            }
        }

        dependencies
    }

    /// Clears all cached results
    pub fn clear_cache(&mut self) {
        self.evaluated_results.clear();
    }

    /// Checks if a step has been evaluated
    pub fn is_evaluated(&self, step_id: u32) -> bool {
        self.evaluated_results.contains_key(&format!("step_{}", step_id))
    }
}