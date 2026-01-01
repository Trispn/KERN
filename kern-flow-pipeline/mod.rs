//! KERN Flow Pipeline Execution
//! 
//! This module implements the flow pipeline execution system for the KERN language.
//! It handles demand-driven evaluation, lazy evaluation strategies, context passing,
//! and control flow operations.

pub mod flow_execution_context;
pub mod flow_step_info;
pub mod flow_evaluator;
pub mod control_ops_if_then_else;
pub mod control_ops_loop;
pub mod control_ops_break_halt;
pub mod lazy_evaluation_manager;
pub mod context_manager;

pub use flow_execution_context::{FlowExecutionContext, SymbolTable, Value};
pub use flow_step_info::FlowStepExecutionInfo;
pub use flow_evaluator::{FlowEvaluator, FlowEvaluationError};
pub use control_ops_if_then_else::IfThenElseHandler;
pub use control_ops_loop::LoopHandler;
pub use control_ops_break_halt::BreakHaltHandler;
pub use lazy_evaluation_manager::LazyEvaluationManager;
pub use context_manager::{ContextManager, ContextError};