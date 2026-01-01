use crate::flow_execution_context::FlowExecutionContext;
use crate::flow_evaluator::{FlowEvaluator, FlowEvaluationError};
use crate::types::Value;

/// Handles if/then/else control operations in the flow pipeline
pub struct IfThenElseHandler;

impl IfThenElseHandler {
    /// Executes an if/then/else control operation
    pub fn execute_if_then_else(
        evaluator: &mut FlowEvaluator,
        condition: bool,
        true_branch: Option<Value>,
        false_branch: Option<Value>,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        if condition {
            if let Some(true_val) = true_branch {
                Ok(true_val)
            } else {
                Ok(Value::Sym("no_true_branch".to_string()))
            }
        } else {
            if let Some(false_val) = false_branch {
                Ok(false_val)
            } else {
                Ok(Value::Sym("no_false_branch".to_string()))
            }
        }
    }
}