use crate::flow_execution_context::FlowExecutionContext;
use crate::flow_evaluator::{FlowEvaluator, FlowEvaluationError};
use crate::types::Value;
use kern_graph_builder::{ExecutionGraph, GraphNode, GraphNodeType};

/// Handles if/then/else control operations in the flow pipeline
pub struct IfThenElseHandler;

impl IfThenElseHandler {
    /// Executes an if/then/else control operation
    pub fn execute_if_then_else(
        evaluator: &mut FlowEvaluator,
        node: &GraphNode,
        graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // Find the condition node connected to this if node
        let condition_node = Self::find_condition_node(node, graph)
            .ok_or(FlowEvaluationError::NodeNotFound(node.id))?;

        // Evaluate the condition
        let condition_result = evaluator.execute_node(condition_node, graph, context)?;
        let should_execute_true_branch = match condition_result {
            Value::Bool(b) => b,
            Value::Num(n) => n != 0,
            _ => true, // Default to true for other types
        };

        if should_execute_true_branch {
            // Execute the true branch
            if let Some(true_branch_node) = Self::find_true_branch_node(node, graph) {
                evaluator.execute_node(true_branch_node, graph, context)
            } else {
                Ok(Value::Sym("no_true_branch".to_string()))
            }
        } else {
            // Execute the false branch if it exists
            if let Some(false_branch_node) = Self::find_false_branch_node(node, graph) {
                evaluator.execute_node(false_branch_node, graph, context)
            } else {
                Ok(Value::Sym("no_false_branch".to_string()))
            }
        }
    }

    /// Finds the condition node connected to an if node
    fn find_condition_node<'a>(
        if_node: &GraphNode,
        graph: &'a ExecutionGraph,
    ) -> Option<&'a GraphNode> {
        // Look for a connected node that represents the condition
        for edge in &graph.edges {
            if edge.from_node == if_node.id && edge.edge_type == kern_graph_builder::EdgeType::Data {
                if let Some(node) = graph.nodes.iter().find(|n| n.id == edge.to_node) {
                    // If it's an operation node with a comparison opcode, it's likely the condition
                    if node.node_type == GraphNodeType::Op && node.opcode == 0x13 { // COMPARE
                        return Some(node);
                    }
                }
            }
        }

        // If no comparison node found, look for any connected data node
        for edge in &graph.edges {
            if edge.from_node == if_node.id && edge.edge_type == kern_graph_builder::EdgeType::Data {
                if let Some(node) = graph.nodes.iter().find(|n| n.id == edge.to_node) {
                    return Some(node);
                }
            }
        }

        None
    }

    /// Finds the true branch node connected to an if node
    fn find_true_branch_node<'a>(
        if_node: &GraphNode,
        graph: &'a ExecutionGraph,
    ) -> Option<&'a GraphNode> {
        // Look for nodes connected via control edges that represent the true branch
        for edge in &graph.edges {
            if edge.from_node == if_node.id && edge.edge_type == kern_graph_builder::EdgeType::Control {
                if let Some(node) = graph.nodes.iter().find(|n| n.id == edge.to_node) {
                    return Some(node);
                }
            }
        }

        None
    }

    /// Finds the false branch node connected to an if node
    fn find_false_branch_node<'a>(
        if_node: &GraphNode,
        graph: &'a ExecutionGraph,
    ) -> Option<&'a GraphNode> {
        // Look for the second control edge which would represent the false branch
        let mut control_edges = Vec::new();
        for edge in &graph.edges {
            if edge.from_node == if_node.id && edge.edge_type == kern_graph_builder::EdgeType::Control {
                if let Some(node) = graph.nodes.iter().find(|n| n.id == edge.to_node) {
                    control_edges.push(node);
                }
            }
        }

        // The false branch would be the second control edge if it exists
        if control_edges.len() > 1 {
            Some(control_edges[1])
        } else {
            None
        }
    }
}