use crate::flow_execution_context::FlowExecutionContext;
use crate::flow_evaluator::{FlowEvaluator, FlowEvaluationError};
use crate::types::Value;
use kern_graph_builder::{ExecutionGraph, GraphNode};

/// Handles break, continue, and halt control operations in the flow pipeline
pub struct BreakHaltHandler;

impl BreakHaltHandler {
    /// Executes a break operation
    pub fn execute_break(
        _evaluator: &mut FlowEvaluator,
        _node: &GraphNode,
        _graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        context.break_requested = true;
        Ok(Value::Sym("break_executed".to_string()))
    }

    /// Executes a continue operation
    pub fn execute_continue(
        _evaluator: &mut FlowEvaluator,
        _node: &GraphNode,
        _graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        context.continue_requested = true;
        Ok(Value::Sym("continue_executed".to_string()))
    }

    /// Executes a halt operation
    pub fn execute_halt(
        _evaluator: &mut FlowEvaluator,
        _node: &GraphNode,
        _graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        context.halted = true;
        Ok(Value::Sym("halt_executed".to_string()))
    }
}