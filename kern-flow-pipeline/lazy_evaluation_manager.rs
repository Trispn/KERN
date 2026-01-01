use std::collections::HashMap;
use crate::flow_execution_context::FlowExecutionContext;
use crate::flow_evaluator::{FlowEvaluator, FlowEvaluationError};
use crate::types::Value;

/// Manages lazy evaluation of flow steps
pub struct LazyEvaluationManager {
    pub evaluated_results: HashMap<String, Value>,
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
        step: Value,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        let cache_key = format!("step_{}", step_id);

        // Check if result is already cached
        if let Some(cached_result) = self.evaluated_results.get(&cache_key) {
            return Ok(cached_result.clone());
        }

        // Evaluate the step
        let result = evaluator.execute_node(context)?;

        // Cache the result
        self.evaluated_results.insert(cache_key, result.clone());

        Ok(result)
    }

    /// Evaluates a step with dependencies, evaluating dependencies lazily first
    pub fn evaluate_with_dependencies(
        &mut self,
        step_id: u32,
        evaluator: &mut FlowEvaluator,
        step: Value,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // For now, just evaluate the step
        // In a real implementation, this would handle dependencies
        self.evaluate_lazy(step_id, evaluator, step, context)
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