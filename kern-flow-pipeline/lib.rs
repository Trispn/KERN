//! KERN Flow Pipeline Execution
//!
//! This crate implements the flow pipeline execution system for the KERN language.
//! It handles demand-driven evaluation, lazy evaluation strategies, context passing,
//! and control flow operations.

pub mod context_manager;
pub mod control_ops_break_halt;
pub mod control_ops_if_then_else;
pub mod control_ops_loop;
pub mod flow_evaluator;
pub mod flow_execution_context;
pub mod flow_step_info;
pub mod lazy_evaluation_manager;
pub mod types;

pub use context_manager::{ContextError, ContextManager};
pub use control_ops_break_halt::BreakHaltHandler;
pub use control_ops_if_then_else::IfThenElseHandler;
pub use control_ops_loop::LoopHandler;
pub use flow_evaluator::{FlowEvaluationError, FlowEvaluator};
pub use flow_execution_context::FlowExecutionContext;
pub use flow_step_info::FlowStepExecutionInfo;
pub use lazy_evaluation_manager::LazyEvaluationManager;
pub use types::{SymbolTable, Value};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_pipeline_creation() {
        let mut context_manager = ContextManager::new();
        let context_id = context_manager.create_context(1);

        assert_eq!(context_id, 1); // First context after initial should have ID 1

        let context = context_manager.get_context(context_id).unwrap();
        assert_eq!(context.flow_id, 1);
    }

    #[test]
    fn test_flow_execution() {
        let evaluator = FlowEvaluator::new();
        let context = FlowExecutionContext::new(1);

        // For now, just verify that we can create the necessary components
        assert_eq!(context.flow_id, 1);
        assert_eq!(evaluator.max_iterations, 100);
    }

    #[test]
    fn test_lazy_evaluation() {
        let mut lazy_manager = LazyEvaluationManager::new();
        assert_eq!(lazy_manager.is_evaluated(1), false);

        // Clear cache to ensure it's empty
        lazy_manager.clear_cache();
        assert_eq!(lazy_manager.evaluated_results.len(), 0);
    }
}
