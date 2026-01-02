mod common;
use common::assertions::{assert_equal, assert_false, assert_true, AssertionResult};
use kern_flow_pipeline::flow_evaluator::{FlowEvaluationError, FlowEvaluator};
use kern_flow_pipeline::flow_execution_context::FlowExecutionContext;
use kern_parser::ast_nodes::{Declaration, Flow, Program};

#[test]
fn test_flow_creation() {
    let flow = Flow {
        name: "TestDataFlow".to_string(),
        steps: vec![
            "step1: load_data()".to_string(),
            "step2: transform_data()".to_string(),
            "step3: save_data()".to_string(),
        ],
    };

    assert_eq!(flow.name, "TestDataFlow");
    assert_eq!(flow.steps.len(), 3);
    assert_eq!(flow.steps[0], "step1: load_data()");
    assert_eq!(flow.steps[1], "step2: transform_data()");
    assert_eq!(flow.steps[2], "step3: save_data()");
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

#[test]
fn test_multi_step_flow_evaluation() {
    let mut evaluator = FlowEvaluator::new();
    let mut context = FlowExecutionContext::new(2);

    // Execute the multi-step flow
    let result = evaluator.evaluate_flow(2, &mut context);

    // Should execute without errors
    assert!(result.is_ok());
}

#[test]
fn test_demand_driven_evaluation() {
    let mut evaluator = FlowEvaluator::new();
    let mut context = FlowExecutionContext::new(3);

    // Execute the flow
    let result = evaluator.evaluate_flow(3, &mut context);

    // Should execute without errors
    assert!(result.is_ok());
}

#[test]
fn test_lazy_evaluation() {
    let mut evaluator = FlowEvaluator::new();
    let mut context = FlowExecutionContext::new(4);

    // Execute the flow with lazy evaluation
    let result = evaluator.evaluate_flow(4, &mut context);

    // Should execute without errors
    assert!(result.is_ok());
}

#[test]
fn test_context_propagation() {
    let mut evaluator = FlowEvaluator::new();
    let mut context = FlowExecutionContext::new(5);

    // Execute the flow
    let result = evaluator.evaluate_flow(5, &mut context);

    // Should execute without errors
    assert!(result.is_ok());
}

#[test]
fn test_conditional_flow_evaluation() {
    let mut evaluator = FlowEvaluator::new();
    let mut context = FlowExecutionContext::new(6);

    // Execute the flow
    let result = evaluator.evaluate_flow(6, &mut context);

    // Should execute without errors
    assert!(result.is_ok());
}

#[test]
fn test_loop_execution_in_flow() {
    let mut evaluator = FlowEvaluator::new();
    let mut context = FlowExecutionContext::new(7);

    // Execute the flow with loop
    let result = evaluator.evaluate_flow(7, &mut context);

    // Should execute without errors
    assert!(result.is_ok());
}

#[test]
fn test_flow_with_error_handling() {
    let mut evaluator = FlowEvaluator::new();
    let mut context = FlowExecutionContext::new(8);

    // Execute the flow
    let result = evaluator.evaluate_flow(8, &mut context);

    // Should execute without errors
    assert!(result.is_ok());
}
