use crate::flow_execution_context::FlowExecutionContext;
use crate::flow_evaluator::{FlowEvaluator, FlowEvaluationError};
use crate::types::Value;
use kern_graph_builder::{ExecutionGraph, GraphNode};

/// Handles loop control operations in the flow pipeline
pub struct LoopHandler;

impl LoopHandler {
    /// Executes a loop control operation
    pub fn execute_loop(
        evaluator: &mut FlowEvaluator,
        node: &GraphNode,
        graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // Initialize iteration counter for this loop
        let counter_key = format!("loop_{}_counter", node.id);
        let limit_key = format!("loop_{}_limit", node.id);

        // Set the iteration limit
        let limit = evaluator.max_iterations;
        context.set_symbol(&limit_key, Value::Num(limit as i64));

        // Initialize counter if not already set
        if context.get_symbol(&counter_key).is_none() {
            context.set_symbol(&counter_key, Value::Num(0));
        }

        // Get the loop condition node
        let condition_node = Self::find_condition_node(node, graph);
        let body_nodes = Self::find_body_nodes(node, graph);

        // Execute the loop
        loop {
            // Check iteration counter
            let counter_val = context.get_symbol(&counter_key)
                .unwrap_or(&Value::Num(0));

            let counter = match counter_val {
                Value::Num(n) => *n,
                _ => 0,
            };

            if counter >= limit as i64 {
                // Max iterations reached
                break;
            }

            // Update the counter in context
            context.set_symbol(&counter_key, Value::Num(counter + 1));

            // Check the loop condition
            let should_continue = if let Some(condition_node) = &condition_node {
                let result = evaluator.execute_node(condition_node, graph, context)?;
                match result {
                    Value::Bool(b) => b,
                    Value::Num(n) => n != 0,
                    _ => true, // Default to true for other types
                }
            } else {
                // If no condition node, continue indefinitely (up to max iterations)
                true
            };

            if !should_continue {
                break;
            }

            // Execute the loop body
            for body_node in &body_nodes {
                let result = evaluator.execute_node(body_node, graph, context)?;

                // Check for break/continue flags
                if context.break_requested {
                    context.break_requested = false;
                    break;
                }

                if context.continue_requested {
                    context.continue_requested = false;
                    break; // This will continue to the next iteration
                }

                // If halted, stop everything
                if context.halted {
                    return Ok(result);
                }
            }

            // If continue was requested, reset the flag and continue to next iteration
            if context.continue_requested {
                context.continue_requested = false;
            }
        }

        Ok(Value::Sym("loop_completed".to_string()))
    }

    /// Finds the condition node for a loop
    fn find_condition_node<'a>(
        loop_node: &GraphNode,
        graph: &'a ExecutionGraph,
    ) -> Option<&'a GraphNode> {
        // Look for a connected node that represents the condition
        for edge in &graph.edges {
            if edge.from_node == loop_node.id && edge.edge_type == kern_graph_builder::EdgeType::Data {
                if let Some(node) = graph.nodes.iter().find(|n| n.id == edge.to_node) {
                    // If it's an operation node with a comparison opcode, it's likely the condition
                    if node.node_type == kern_graph_builder::GraphNodeType::Op && node.opcode == 0x13 { // COMPARE
                        return Some(node);
                    }
                }
            }
        }

        // If no comparison node found, look for any connected data node
        for edge in &graph.edges {
            if edge.from_node == loop_node.id && edge.edge_type == kern_graph_builder::EdgeType::Data {
                if let Some(node) = graph.nodes.iter().find(|n| n.id == edge.to_node) {
                    return Some(node);
                }
            }
        }

        None
    }

    /// Finds all nodes that belong to the loop body
    fn find_body_nodes<'a>(
        loop_node: &GraphNode,
        graph: &'a ExecutionGraph,
    ) -> Vec<&'a GraphNode> {
        let mut body_nodes = Vec::new();

        // Look for nodes connected via control edges that represent the loop body
        for edge in &graph.edges {
            if edge.from_node == loop_node.id && edge.edge_type == kern_graph_builder::EdgeType::Control {
                if let Some(node) = graph.nodes.iter().find(|n| n.id == edge.to_node) {
                    body_nodes.push(node);
                }
            }
        }

        body_nodes
    }
}