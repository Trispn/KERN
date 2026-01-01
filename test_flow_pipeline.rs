use kern_flow_pipeline::{
    FlowExecutionContext, 
    FlowEvaluator, 
    LazyEvaluationManager, 
    ContextManager,
    FlowStepExecutionInfo,
    IfThenElseHandler,
    LoopHandler,
    BreakHaltHandler
};

fn main() {
    println!("Testing KERN Flow Pipeline Execution...");

    // Create a simple flow execution context
    let mut context = FlowExecutionContext::new(1);
    context.set_symbol("test_var", kern_flow_pipeline::Value::Num(42));
    
    println!("Created flow execution context with test_var = 42");
    
    // Create a context manager
    let mut context_manager = ContextManager::new();
    let context_id = context_manager.create_context(1);
    println!("Created context with ID: {}", context_id);
    
    // Create a lazy evaluation manager
    let mut lazy_manager = LazyEvaluationManager::new();
    println!("Created lazy evaluation manager");
    
    // Create a flow evaluator
    let mut evaluator = FlowEvaluator::new();
    println!("Created flow evaluator with max iterations: {}", evaluator.max_iterations);
    
    // Test flow execution
    match evaluator.evaluate_flow(1, &mut context) {
        Ok(result) => println!("Flow execution result: {:?}", result),
        Err(e) => println!("Flow execution error: {:?}", e),
    }
    
    // Test if/then/else handler
    let if_result = IfThenElseHandler::execute_if_then_else(
        &mut evaluator,
        true,
        Some(kern_flow_pipeline::Value::Sym("true_branch".to_string())),
        Some(kern_flow_pipeline::Value::Sym("false_branch".to_string())),
        &mut context
    );
    println!("If/then/else result: {:?}", if_result);
    
    // Test loop handler
    let loop_result = LoopHandler::execute_loop(
        &mut evaluator,
        3, // 3 iterations
        kern_flow_pipeline::Value::Sym("loop_body".to_string()),
        &mut context
    );
    println!("Loop result: {:?}", loop_result);
    
    // Test break handler
    let break_result = BreakHaltHandler::execute_break(
        &mut evaluator,
        &mut context
    );
    println!("Break result: {:?}", break_result);
    
    // Test continue handler
    let continue_result = BreakHaltHandler::execute_continue(
        &mut evaluator,
        &mut context
    );
    println!("Continue result: {:?}", continue_result);
    
    // Test halt handler
    let halt_result = BreakHaltHandler::execute_halt(
        &mut evaluator,
        &mut context
    );
    println!("Halt result: {:?}", halt_result);
    
    // Test lazy evaluation
    let lazy_result = lazy_manager.evaluate_lazy(
        1,
        &mut evaluator,
        kern_flow_pipeline::Value::Sym("lazy_step".to_string()),
        &mut context
    );
    println!("Lazy evaluation result: {:?}", lazy_result);
    
    println!("KERN Flow Pipeline test completed successfully!");
}