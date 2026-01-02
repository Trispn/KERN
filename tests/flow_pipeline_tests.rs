mod common;
use common::assertions::{assert_equal, assert_false, assert_true, AssertionResult};
use kern_flow_pipeline::flow_evaluator::{FlowEvaluationError, FlowEvaluator};
use kern_flow_pipeline::flow_execution_context::FlowExecutionContext;
use kern_parser::ast::{Action, Definition, FlowDef, Program};

#[test]
fn test_flow_creation() {
    let flow = FlowDef {
        name: "TestDataFlow".to_string(),
        actions: vec![],
    };

    assert_eq!(flow.name, "TestDataFlow");
}

#[test]
fn test_flow_evaluator_initialization() {
    let evaluator = FlowEvaluator::new();

    assert_eq!(evaluator.max_iterations, 100);
}

#[test]
fn test_flow_evaluation() {
    let mut evaluator = FlowEvaluator::new();
    let mut context = FlowExecutionContext::new(1);

    // Evaluate the flow
    let result = evaluator.evaluate_flow(1, &mut context);

    // Should execute without errors
    assert!(result.is_ok());
}
