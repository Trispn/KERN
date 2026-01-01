use kern_compiler::flow_pipeline::{
    FlowExecutionContext, 
    FlowEvaluator, 
    LazyEvaluationManager, 
    ContextManager,
    FlowStepExecutionInfo
};
use kern_parser::Parser;
use kern_graph_builder::GraphBuilder;

fn main() {
    println!("Testing KERN Flow Pipeline Execution...");

    // Create a simple flow execution context
    let mut context = FlowExecutionContext::new(1);
    context.set_symbol("test_var", kern_compiler::flow_pipeline::Value::Num(42));
    
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
    
    // Parse a simple KERN program to test integration
    let input = r#"
    entity TestEntity {
        value
    }

    rule TestRule:
        if value == 42
        then output(value)

    flow TestFlow {
        step1: load_data()
        step2: process_data()
    }
    "#;

    let mut parser = Parser::new(input);
    match parser.parse_program() {
        Ok(program) => {
            println!("Successfully parsed KERN program with {} declarations", program.declarations.len());
            
            // Build execution graph
            let mut builder = GraphBuilder::new();
            let graph = builder.build_execution_graph(&program);
            println!("Built execution graph with {} nodes", graph.nodes.len());
            
            // Test creating a flow step execution info
            if let Some(node) = graph.nodes.first() {
                let step_info = FlowStepExecutionInfo::new(node.id, node.id);
                println!("Created flow step execution info for node ID: {}", step_info.step_id);
            }
        }
        Err(e) => {
            eprintln!("Failed to parse program: {:?}", e);
        }
    }

    println!("KERN Flow Pipeline test completed successfully!");
}