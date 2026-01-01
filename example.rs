use kern_parser::Parser;
use kern_graph_builder::GraphBuilder;
use kern_rule_engine::RuleEngine;

fn main() {
    let input = r#"
    entity Farmer {
        id
        location
        produce
    }

    rule CheckLocation:
        if farmer.location == "valid"
        then approve_farmer(farmer)

    flow ProcessFarmers {
        load_farmers()
        validate_farmers()
    }

    constraint ValidId: farmer.id > 0
    "#;

    // Parse the KERN code
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse program");

    // Build the execution graph
    let mut builder = GraphBuilder::new();
    let graph = builder.build_execution_graph(&program);
    println!("Generated execution graph with {} nodes", graph.nodes.len());

    // Create and run the rule engine
    let mut engine = RuleEngine::new();

    // Execute the graph
    match engine.execute_graph(&graph) {
        Ok(_) => println!("Execution completed successfully"),
        Err(e) => println!("Execution failed: {:?}", e),
    }

    // Test flow pipeline execution
    if let Some(flow_node) = graph.nodes.iter().find(|n| n.node_type == kern_graph_builder::GraphNodeType::Control) {
        match engine.execute_flow_pipeline(&graph, flow_node.id) {
            Ok(_) => println!("Flow pipeline executed successfully"),
            Err(e) => println!("Flow pipeline execution failed: {:?}", e),
        }
    }

    // Test lazy evaluation
    if let Some(node) = graph.nodes.first() {
        match engine.evaluate_lazy(node.id, &graph) {
            Ok(result) => println!("Lazy evaluation result: {:?}", result),
            Err(e) => println!("Lazy evaluation failed: {:?}", e),
        }
    }

    println!("Total execution steps: {}", engine.step_count);
}