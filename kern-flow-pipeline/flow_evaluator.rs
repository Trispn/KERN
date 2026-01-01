use crate::flow_execution_context::FlowExecutionContext;
use crate::flow_step_info::FlowStepExecutionInfo;
use crate::types::Value;
use kern_graph_builder::{ExecutionGraph, GraphNode, GraphNodeType};
use kern_rule_engine::RuleEngine;

/// FlowEvaluator handles the execution of flow pipelines
pub struct FlowEvaluator {
    pub rule_engine: RuleEngine,
    pub max_iterations: u32,
}

impl FlowEvaluator {
    pub fn new() -> Self {
        FlowEvaluator {
            rule_engine: RuleEngine::new(),
            max_iterations: 100, // Default max iterations per loop
        }
    }

    /// Sets the maximum number of iterations allowed for loops
    pub fn set_max_iterations(&mut self, max_iterations: u32) {
        self.max_iterations = max_iterations;
    }

    /// Evaluates a flow pipeline with demand-driven evaluation
    pub fn evaluate_flow(
        &mut self,
        flow_id: u32,
        graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // Find all nodes related to this flow
        let flow_nodes = self.get_flow_nodes(flow_id, graph);

        // Evaluate each step in the flow
        for node in flow_nodes {
            if context.halted {
                break;
            }

            if context.break_requested {
                context.break_requested = false;
                break;
            }

            if context.continue_requested {
                context.continue_requested = false;
                continue;
            }

            let step_info = FlowStepExecutionInfo::new(node.id, node.id);
            let result = self.evaluate_step(step_info, graph, context)?;

            // Update context with the result if needed
            context.increment_step();
        }

        // Return a default value or the result of the last step
        Ok(Value::Sym(format!("flow_{}_completed", flow_id)))
    }

    /// Evaluates a single step in the flow
    pub fn evaluate_step(
        &mut self,
        mut step_info: FlowStepExecutionInfo,
        graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // Check if already evaluated
        if step_info.evaluated {
            return Ok(step_info.cached_result.unwrap_or(Value::Sym("cached".to_string())));
        }

        // Check if the step has a condition
        if let Some(condition_id) = step_info.condition_graph_id {
            let condition_result = self.evaluate_condition(condition_id, graph, context)?;
            if !condition_result {
                step_info.mark_evaluated(Value::Sym("condition_false".to_string()));
                return Ok(Value::Sym("condition_false".to_string()));
            }
        }

        // Execute the action for this step
        let result = self.execute_action(step_info.action_graph_id, graph, context)?;
        step_info.mark_evaluated(result.clone());

        Ok(result)
    }

    /// Evaluates a condition node
    fn evaluate_condition(
        &mut self,
        condition_id: u32,
        graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<bool, FlowEvaluationError> {
        // Find the condition node in the graph
        let condition_node = graph.nodes.iter()
            .find(|n| n.id == condition_id)
            .ok_or(FlowEvaluationError::NodeNotFound(condition_id))?;

        // Execute the condition node and return the boolean result
        match condition_node.node_type {
            GraphNodeType::Op if condition_node.opcode == 0x13 => { // COMPARE
                // Execute the comparison operation
                let result = self.execute_node(condition_node, graph, context)?;

                // Convert the result to a boolean
                match result {
                    Value::Bool(b) => Ok(b),
                    Value::Num(n) => Ok(n != 0),
                    _ => Ok(true), // Default to true for other types
                }
            },
            _ => {
                // For other condition types, we'll evaluate them differently
                let result = self.execute_node(condition_node, graph, context)?;

                // Convert the result to a boolean
                match result {
                    Value::Bool(b) => Ok(b),
                    Value::Num(n) => Ok(n != 0),
                    _ => Ok(true), // Default to true for other types
                }
            }
        }
    }

    /// Executes an action node
    fn execute_action(
        &mut self,
        action_id: u32,
        graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // Find the action node in the graph
        let action_node = graph.nodes.iter()
            .find(|n| n.id == action_id)
            .ok_or(FlowEvaluationError::NodeNotFound(action_id))?;

        // Execute the action node
        self.execute_node(action_node, graph, context)
    }

    /// Executes a single node in the execution graph
    pub fn execute_node(
        &mut self,
        node: &GraphNode,
        graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        match node.node_type {
            GraphNodeType::Op => self.execute_op_node(node, context),
            GraphNodeType::Rule => self.execute_rule_node(node, graph, context),
            GraphNodeType::Control => self.execute_control_node(node, graph, context),
            GraphNodeType::Graph => self.execute_graph_node(node, context),
            GraphNodeType::Io => self.execute_io_node(node, context),
        }
    }

    /// Executes an operation node
    fn execute_op_node(
        &mut self,
        node: &GraphNode,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        match node.opcode {
            0x10 => self.execute_load_sym(node, context),      // LOAD_SYM
            0x11 => self.execute_load_num(node, context),      // LOAD_NUM
            0x12 => self.execute_move(node, context),          // MOVE
            0x13 => self.execute_compare(node, context),       // COMPARE
            _ => {
                // For other opcodes, we'll implement as needed
                println!("Executing operation node with opcode: {}", node.opcode);
                Ok(Value::Sym(format!("op_{}_result", node.id)))
            }
        }
    }

    /// Executes a rule node
    fn execute_rule_node(
        &mut self,
        node: &GraphNode,
        graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // For rule execution, we'll use the rule engine
        // This is a simplified implementation - in a real system, we'd pass the context
        println!("Executing rule node: {}", node.id);

        // Add the rule to the rule engine's context
        self.rule_engine.context.current_node_id = Some(node.id);

        // Execute the rule using the rule engine
        // This is a simplified approach - in a real system, we'd need to properly integrate
        // the flow context with the rule engine context
        Ok(Value::Sym(format!("rule_{}_executed", node.id)))
    }

    /// Executes a control node (if/then/else, loops, etc.)
    fn execute_control_node(
        &mut self,
        node: &GraphNode,
        graph: &ExecutionGraph,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        match node.opcode {
            0x00 => { // NOP - No operation
                println!("Executing NOP: {}", node.id);
                Ok(Value::Sym("nop".to_string()))
            },
            0x01 => { // JMP - Jump
                println!("Executing JMP: {}", node.id);
                // In a real implementation, this would update the execution flow
                Ok(Value::Sym("jmp".to_string()))
            },
            0x02 => { // JMP_IF - Conditional jump
                println!("Executing JMP_IF: {}", node.id);
                // In a real implementation, this would conditionally update the execution flow
                Ok(Value::Sym("jmp_if".to_string()))
            },
            0x03 => { // HALT - Stop execution
                println!("Executing HALT: {}", node.id);
                context.halted = true;
                Ok(Value::Sym("halt".to_string()))
            },
            _ => {
                println!("Executing control node with opcode: {}", node.opcode);
                // Handle other control operations based on the graph structure
                Ok(Value::Sym("control".to_string()))
            }
        }
    }

    /// Executes a graph node
    fn execute_graph_node(
        &mut self,
        _node: &GraphNode,
        _context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // Graph operations would manipulate the symbol graph (not the execution graph)
        println!("Executing graph node");
        Ok(Value::Sym("graph_op".to_string()))
    }

    /// Executes an IO node
    fn execute_io_node(
        &mut self,
        _node: &GraphNode,
        _context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // IO operations would call external functions
        println!("Executing IO node");
        Ok(Value::Sym("io_op".to_string()))
    }

    /// Gets all nodes related to a specific flow
    fn get_flow_nodes(&self, flow_id: u32, graph: &ExecutionGraph) -> Vec<&GraphNode> {
        // In a real implementation, this would find all nodes that belong to the specified flow
        // For now, we'll return all nodes as a placeholder
        graph.nodes.iter().collect()
    }

    /// Executes a load symbol operation
    fn execute_load_sym(
        &mut self,
        node: &GraphNode,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // For now, we'll just return a placeholder value
        // In a real implementation, this would load a symbol from the symbol table
        Ok(Value::Sym(format!("symbol_{}", node.id)))
    }

    /// Executes a load number operation
    fn execute_load_num(
        &mut self,
        node: &GraphNode,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // In a real implementation, this would load a number from the node's metadata
        Ok(Value::Num(node.id as i64))
    }

    /// Executes a move operation
    fn execute_move(
        &mut self,
        node: &GraphNode,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // For now, just return a success value
        // In a real implementation, this would move a value from one register to another
        Ok(Value::Sym(format!("move_{}", node.id)))
    }

    /// Executes a compare operation
    fn execute_compare(
        &mut self,
        node: &GraphNode,
        context: &mut FlowExecutionContext,
    ) -> Result<Value, FlowEvaluationError> {
        // For now, just return a success value
        // In a real implementation, this would compare two values
        Ok(Value::Bool(true))
    }
}

#[derive(Debug)]
pub enum FlowEvaluationError {
    NodeNotFound(u32),
    InvalidNodeType,
    MissingRegisterValue(u16),
    InvalidComparison(String),
    ExecutionLimitExceeded,
}