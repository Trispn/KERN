mod common;
use common::assertions::{assert_equal, assert_false, assert_true, AssertionResult};
use kern_graph_builder::{GraphBuilder, ExecutionGraph, GraphNode, GraphNodeType, SpecializedNode, RuleNode, EdgeType, GraphEdge};
use kern_parser::{Parser, Program};

#[test]
fn test_graph_building_basic_entity() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Farmer {
            id
            location
            produce
        }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);

    // Verify that the graph has the expected structure for entities
    assert!(!graph.nodes.is_empty(), "Graph should contain nodes for entity processing");
    assert!(graph.edges.is_empty(), "Entity definitions don't create execution edges");
    assert!(graph.entry_points.is_empty(), "Entity definitions don't create entry points");
    
    println!(
        "Generated execution graph for entity with {} nodes and {} edges",
        graph.nodes.len(),
        graph.edges.len()
    );
}

#[test]
fn test_graph_building_basic_rule() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Farmer {
            id
            location
        }
        
        rule CheckLocation:
            if farmer.location == "valid"
            then approve_farmer(farmer)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);

    // Verify that the graph has the expected structure for rules
    assert!(!graph.nodes.is_empty(), "Graph should contain nodes for rule processing");
    assert!(!graph.entry_points.is_empty(), "Rule should create an entry point");
    
    // Check that we have at least a rule node
    let rule_nodes: Vec<&SpecializedNode> = graph
        .nodes
        .iter()
        .filter(|node| matches!(node, SpecializedNode::Rule(_)))
        .collect();
    
    assert!(!rule_nodes.is_empty(), "Graph should contain at least one rule node");
    
    println!(
        "Generated execution graph for rule with {} nodes and {} edges",
        graph.nodes.len(),
        graph.edges.len()
    );
}

#[test]
fn test_graph_building_basic_flow() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Farmer {
            id
            location
        }
        
        flow ProcessFarmers {
            load_farmers()
            validate_farmers()
        }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);

    // Verify that the graph has the expected structure for flows
    assert!(!graph.nodes.is_empty(), "Graph should contain nodes for flow processing");
    assert!(!graph.entry_points.is_empty(), "Flow should create an entry point");
    
    println!(
        "Generated execution graph for flow with {} nodes and {} edges",
        graph.nodes.len(),
        graph.edges.len()
    );
}

#[test]
fn test_graph_building_basic_constraint() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Farmer {
            id
            location
        }
        
        constraint ValidId: farmer.id > 0
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);

    // Verify that the graph has the expected structure for constraints
    assert!(!graph.nodes.is_empty(), "Graph should contain nodes for constraint processing");
    assert!(!graph.entry_points.is_empty(), "Constraint should create an entry point");
    
    println!(
        "Generated execution graph for constraint with {} nodes and {} edges",
        graph.nodes.len(),
        graph.edges.len()
    );
}

#[test]
fn test_graph_building_with_conditions() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Farmer {
            id
            location
        }
        
        rule CheckLocation:
            if farmer.location == "valid" && farmer.id > 0
            then approve_farmer(farmer)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);

    // Verify that the graph has nodes for conditions
    assert!(!graph.nodes.is_empty(), "Graph should contain nodes for condition processing");
    
    // Check that we have comparison nodes
    let comparison_nodes: Vec<&SpecializedNode> = graph
        .nodes
        .iter()
        .filter(|node| {
            let base = node.get_base();
            base.node_type == GraphNodeType::Op && base.opcode == 0x13 // COMPARE
        })
        .collect();
    
    assert!(!comparison_nodes.is_empty(), "Graph should contain comparison nodes for conditions");
    
    println!(
        "Generated execution graph with conditions has {} nodes and {} edges",
        graph.nodes.len(),
        graph.edges.len()
    );
}

#[test]
fn test_graph_building_with_actions() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Farmer {
            id
            location
        }
        
        rule CheckLocation:
            if farmer.location == "valid"
            then 
                approve_farmer(farmer)
                update_status(farmer, "approved")
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);

    // Verify that the graph has nodes for actions
    assert!(!graph.nodes.is_empty(), "Graph should contain nodes for action processing");
    
    println!(
        "Generated execution graph with actions has {} nodes and {} edges",
        graph.nodes.len(),
        graph.edges.len()
    );
}

#[test]
fn test_graph_building_with_control_flow() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Farmer {
            id
            location
        }
        
        flow ProcessFarmers {
            if farmer.location == "valid" {
                approve_farmer(farmer)
            } else {
                reject_farmer(farmer)
            }
        }
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);

    // Verify that the graph has control flow nodes
    assert!(!graph.nodes.is_empty(), "Graph should contain nodes for control flow processing");
    
    // Check that we have control nodes
    let control_nodes: Vec<&SpecializedNode> = graph
        .nodes
        .iter()
        .filter(|node| node.get_base().node_type == GraphNodeType::Control)
        .collect();
    
    assert!(!control_nodes.is_empty(), "Graph should contain control flow nodes");
    
    println!(
        "Generated execution graph with control flow has {} nodes and {} edges",
        graph.nodes.len(),
        graph.edges.len()
    );
}

#[test]
fn test_graph_building_with_edges() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Farmer {
            id
            location
        }
        
        rule CheckLocation:
            if farmer.location == "valid"
            then approve_farmer(farmer)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);

    // Verify that the graph has edges connecting nodes
    if !graph.edges.is_empty() {
        // Check that edges have valid from and to node IDs
        for edge in &graph.edges {
            assert!(edge.from_node < graph.node_count, "Edge from_node should be valid");
            assert!(edge.to_node < graph.node_count, "Edge to_node should be valid");
        }
    }
    
    println!(
        "Generated execution graph with {} nodes and {} edges",
        graph.nodes.len(),
        graph.edges.len()
    );
}

#[test]
fn test_graph_building_validation() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Farmer {
            id
            location
        }
        
        rule CheckLocation:
            if farmer.location == "valid"
            then approve_farmer(farmer)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    // Validate the graph structure
    let validation_result = builder.validate_graph(&graph);
    assert!(validation_result.is_ok(), "Generated graph should be valid: {:?}", validation_result.err());
    
    println!(
        "Generated execution graph with {} nodes and {} edges passed validation",
        graph.nodes.len(),
        graph.edges.len()
    );
}

#[test]
fn test_graph_building_cycles_detection() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Farmer {
            id
            location
        }
        
        rule CheckLocation:
            if farmer.location == "valid"
            then approve_farmer(farmer)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);
    
    // Check for cycles in the graph
    let has_cycles = builder.has_cycles(&graph);
    assert!(!has_cycles, "Simple rule graph should not have cycles");
    
    println!(
        "Generated execution graph with {} nodes and {} edges has no cycles",
        graph.nodes.len(),
        graph.edges.len()
    );
}

#[test]
fn test_graph_building_optimization() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Farmer {
            id
            location
        }
        
        rule CheckLocation:
            if farmer.location == "valid"
            then approve_farmer(farmer)
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();

    let mut graph = builder.build_execution_graph(&program);
    
    // Get initial stats
    let initial_node_count = graph.nodes.len();
    let initial_edge_count = graph.edges.len();
    
    // Optimize the graph
    builder.optimize_graph(&mut graph);
    
    // After optimization, the graph should still be valid
    let validation_result = builder.validate_graph(&graph);
    assert!(validation_result.is_ok(), "Optimized graph should be valid: {:?}", validation_result.err());
    
    println!(
        "Optimized execution graph from {} nodes/{} edges to {} nodes/{} edges",
        initial_node_count,
        initial_edge_count,
        graph.nodes.len(),
        graph.edges.len()
    );
}

#[test]
fn test_graph_building_multiple_entities_rules_flows() {
    let mut builder = GraphBuilder::new();
    
    let input = r#"
        entity Farmer {
            id
            location
            produce
        }
        
        entity Crop {
            id
            type
            season
        }
        
        rule CheckLocation:
            if farmer.location == "valid"
            then approve_farmer(farmer)
            
        rule CheckCrop:
            if crop.season == "spring"
            then plant_crop(crop)
            
        flow ProcessFarmers {
            load_farmers()
            validate_farmers()
        }
        
        flow ProcessCrops {
            load_crops()
            validate_crops()
        }
        
        constraint ValidId: farmer.id > 0
        constraint ValidCropId: crop.id > 0
    "#;

    let mut parser = Parser::new(input);
    let result = parser.parse_program();

    assert!(result.is_ok());
    let program = result.unwrap();

    let graph = builder.build_execution_graph(&program);

    // Verify that the graph has nodes for all entities, rules, flows, and constraints
    assert!(!graph.nodes.is_empty(), "Graph should contain nodes for all definitions");
    assert!(!graph.entry_points.is_empty(), "Graph should have entry points for rules, flows, and constraints");
    
    // Count different types of nodes
    let rule_nodes: Vec<&SpecializedNode> = graph
        .nodes
        .iter()
        .filter(|node| matches!(node, SpecializedNode::Rule(_)))
        .collect();
    
    let control_nodes: Vec<&SpecializedNode> = graph
        .nodes
        .iter()
        .filter(|node| node.get_base().node_type == GraphNodeType::Control)
        .collect();
    
    assert!(rule_nodes.len() >= 2, "Graph should contain at least 2 rule nodes");
    assert!(control_nodes.len() >= 2, "Graph should contain at least 2 control nodes for flows");
    
    println!(
        "Generated execution graph with multiple entities/rules/flows has {} nodes and {} edges",
        graph.nodes.len(),
        graph.edges.len()
    );
}