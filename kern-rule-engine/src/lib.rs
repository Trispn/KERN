mod rule_engine;
mod pattern_matcher;
mod scheduler;
mod conflict_resolver;
mod priority_manager;
mod recursion_guard;

mod implementation;

pub use rule_engine::*;
pub use pattern_matcher::*;
pub use scheduler::*;
pub use conflict_resolver::*;
pub use priority_manager::*;
pub use recursion_guard::*;
pub use implementation::*;

use kern_graph_builder::{ExecutionGraph, GraphNode, GraphNodeType, EdgeType};
use kern_parser::{Comparator, LogicalOp};
use std::collections::HashMap;

#[cfg(test)]
mod tests;

// Define the rule engine execution context
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub registers: [Option<Value>; 16],  // R0-R15, using Option for uninitialized values
    pub variables: HashMap<String, Value>,
    pub facts: HashMap<String, Value>,
    pub rule_results: HashMap<String, bool>,
    pub current_node_id: Option<u32>,
}

// Pattern matching structures
#[derive(Debug, Clone)]
pub enum Pattern {
    Value(Value),
    Variable(String),  // A variable that can match any value
    Composite(String, Vec<Pattern>),  // A composite pattern like (entity.field value)
}

#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub bindings: HashMap<String, Value>,  // Variable bindings from the match
    pub matched_node: u32,  // The node that matched
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Sym(String),
    Num(i64),
    Bool(bool),
    Vec(Vec<Value>),
    Ref(String),  // External reference
}

#[derive(Debug)]
pub enum RuleEngineError {
    InvalidNodeType,
    MissingRegisterValue(u16),
    InvalidComparison(Comparator, Value, Value),
    InvalidPredicate(String),
    ExecutionLimitExceeded,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RulePriority {
    pub rule_id: u32,
    pub priority: u32,  // Higher number means higher priority
    pub specificity: u32,  // More specific rules have higher priority
    pub recency: u32,  // More recently added facts might affect priority
    pub activation_count: u32,  // How many times the rule has been activated
    pub conflict_score: u32,  // Score based on conflicts with other rules
}

#[derive(Debug, Clone, PartialEq)]
pub enum PriorityStrategy {
    /// Standard priority based on explicit settings
    Standard,
    /// Priority based on rule specificity (more specific rules fire first)
    SpecificityFirst,
    /// Priority based on recency (newer facts/rules have higher priority)
    RecencyBased,
    /// Priority based on how frequently the rule has been activated
    FrequencyBased,
    /// Priority based on conflict resolution needs
    ConflictResolution,
    /// Custom priority function
    Custom(Box<dyn Fn(&RulePriority) -> u32>),
}

// The RuleEngine executes rules based on the execution graph
pub struct RuleEngine {
    pub context: ExecutionContext,
    pub step_count: u32,
    pub max_steps: u32,
    pub priority_queue: Vec<u32>,  // Node IDs sorted by priority
    pub rule_priorities: HashMap<u32, RulePriority>,  // Map of rule ID to priority
    pub activation_records: Vec<u32>,  // List of activated rule nodes
    pub priority_strategy: PriorityStrategy,  // Strategy for determining rule priority
    pub execution_path: Vec<u32>,  // Track the current execution path to detect recursion
    pub max_recursion_depth: u32,  // Maximum allowed recursion depth
    pub rule_execution_counts: HashMap<u32, u32>,  // Track how many times each rule has been executed in the current path
}

impl RuleEngine {
    pub fn new() -> Self {
        RuleEngine {
            context: ExecutionContext {
                registers: [None; 16],
                variables: HashMap::new(),
                facts: HashMap::new(),
                rule_results: HashMap::new(),
                current_node_id: None,
            },
            step_count: 0,
            max_steps: 10000,  // Prevent infinite loops
            priority_queue: Vec::new(),
            rule_priorities: HashMap::new(),
            activation_records: Vec::new(),
            priority_strategy: PriorityStrategy::Standard,
            execution_path: Vec::new(),
            max_recursion_depth: 100,  // Default maximum recursion depth
            rule_execution_counts: HashMap::new(),
        }
    }

    /// Sets the priority for a specific rule
    pub fn set_rule_priority(&mut self, rule_id: u32, priority: u32, specificity: u32, recency: u32) {
        let rule_priority = RulePriority {
            rule_id,
            priority,
            specificity,
            recency,
            activation_count: 0,
            conflict_score: 0,
        };
        self.rule_priorities.insert(rule_id, rule_priority);
    }

    /// Sets the priority strategy for the rule engine
    pub fn set_priority_strategy(&mut self, strategy: PriorityStrategy) {
        self.priority_strategy = strategy;
    }

    /// Updates the activation count for a rule
    pub fn increment_rule_activation(&mut self, rule_id: u32) {
        if let Some(rule_priority) = self.rule_priorities.get_mut(&rule_id) {
            rule_priority.activation_count += 1;
        }
    }

    /// Updates the conflict score for a rule
    pub fn update_rule_conflict_score(&mut self, rule_id: u32, score: u32) {
        if let Some(rule_priority) = self.rule_priorities.get_mut(&rule_id) {
            rule_priority.conflict_score = score;
        }
    }

    /// Gets the priority for a rule, defaulting to 0 if not set
    fn get_rule_priority(&self, rule_id: u32) -> u32 {
        if let Some(rule_priority) = self.rule_priorities.get(&rule_id) {
            match &self.priority_strategy {
                PriorityStrategy::Standard => {
                    // Calculate effective priority based on all factors
                    rule_priority.priority * 1000 +
                    rule_priority.specificity * 100 +
                    rule_priority.recency * 10 +
                    rule_priority.activation_count / 10  // Lower priority for frequently activated rules to avoid loops
                },
                PriorityStrategy::SpecificityFirst => {
                    // Prioritize more specific rules
                    rule_priority.specificity * 1000 +
                    rule_priority.priority * 100 +
                    rule_priority.recency * 10
                },
                PriorityStrategy::RecencyBased => {
                    // Prioritize based on recency
                    rule_priority.recency * 1000 +
                    rule_priority.priority * 100 +
                    rule_priority.specificity * 10
                },
                PriorityStrategy::FrequencyBased => {
                    // Prioritize based on activation frequency (less frequent rules first to avoid loops)
                    rule_priority.priority * 1000 +
                    (u32::MAX - rule_priority.activation_count) * 100 +  // Inverse of activation count
                    rule_priority.specificity * 10
                },
                PriorityStrategy::ConflictResolution => {
                    // Prioritize based on conflict resolution needs
                    rule_priority.priority * 1000 +
                    (u32::MAX - rule_priority.conflict_score) * 100 +  // Lower conflict score = higher priority
                    rule_priority.specificity * 10
                },
                PriorityStrategy::Custom(priority_fn) => {
                    // Use custom priority function
                    priority_fn(rule_priority)
                }
            }
        } else {
            0  // Default priority
        }
    }

    /// Adds a node to the priority queue, maintaining priority order
    fn add_to_priority_queue(&mut self, node_id: u32) {
        // Check if the node is already in the queue
        if self.priority_queue.contains(&node_id) {
            return;
        }

        self.priority_queue.push(node_id);

        // Sort the queue based on priority (higher priority first)
        self.priority_queue.sort_by(|&a, &b| {
            let priority_a = self.get_rule_priority(a);
            let priority_b = self.get_rule_priority(b);
            priority_b.cmp(&priority_a)  // Reverse order for higher priority first
        });
    }

    /// Adds a node to the priority queue using the current strategy
    fn add_to_priority_queue_with_strategy(&mut self, node_id: u32) {
        // Check if the node is already in the queue
        if self.priority_queue.contains(&node_id) {
            return;
        }

        self.priority_queue.push(node_id);

        // Sort the queue based on the current priority strategy
        self.priority_queue.sort_by(|&a, &b| {
            let priority_a = self.get_rule_priority(a);
            let priority_b = self.get_rule_priority(b);
            priority_b.cmp(&priority_a)  // Reverse order for higher priority first
        });
    }

    /// Selects the next node to execute based on priority and scheduling strategy
    fn select_next_node(&mut self) -> Option<u32> {
        if self.priority_queue.is_empty() {
            return None;
        }

        // Sort the priority queue based on the current priority strategy
        self.priority_queue.sort_by(|&a, &b| {
            let priority_a = self.get_rule_priority(a);
            let priority_b = self.get_rule_priority(b);
            priority_b.cmp(&priority_a)  // Reverse order for higher priority first
        });

        // Return the highest priority node
        self.priority_queue.pop()  // pop from the end since it's sorted in descending order
    }

    /// Schedules a rule for execution based on its eligibility
    pub fn schedule_rule(&mut self, rule_id: u32, graph: &ExecutionGraph) -> bool {
        // Check if the rule is eligible for execution
        if self.is_rule_eligible(rule_id, graph) {
            self.add_to_priority_queue_with_strategy(rule_id);
            true
        } else {
            false
        }
    }

    /// Checks if a rule is eligible for execution
    fn is_rule_eligible(&self, rule_id: u32, graph: &ExecutionGraph) -> bool {
        // Find the rule node in the graph
        if let Some(node) = graph.nodes.iter().find(|n| n.id == rule_id && n.node_type == GraphNodeType::Rule) {
            // Check if the rule's condition is satisfied
            // For now, we'll just check if it's a valid rule node
            // In a real implementation, we'd evaluate the rule's condition
            node.node_type == GraphNodeType::Rule
        } else {
            false
        }
    }

    /// Performs conflict-aware scheduling
    pub fn conflict_aware_schedule(&mut self, graph: &ExecutionGraph) {
        // Detect conflicts in the current rule set
        let conflicts = self.detect_rule_conflicts(graph);

        // Resolve conflicts if any exist
        if !conflicts.is_empty() {
            self.resolve_conflicts(&conflicts);

            // Re-sort the priority queue based on conflict resolution strategy
            self.priority_queue.sort_by(|&a, &b| {
                let priority_a = self.get_rule_priority(a);
                let priority_b = self.get_rule_priority(b);
                priority_b.cmp(&priority_a)  // Reverse order for higher priority first
            });
        }
    }

    pub fn execute_graph(&mut self, graph: &ExecutionGraph) -> Result<(), RuleEngineError> {
        // Initialize the priority queue with entry points
        for entry_point in &graph.entry_points {
            self.add_to_priority_queue_with_strategy(entry_point.node_id);
        }

        // Execute nodes in priority order
        while !self.priority_queue.is_empty() && self.step_count < self.max_steps {
            // Get the next node to execute based on priority
            if let Some(node_id) = self.select_next_node() {
                self.context.current_node_id = Some(node_id);

                // Find the node in the graph
                if let Some(node) = graph.nodes.iter().find(|n| n.id == node_id) {
                    // Perform conflict-aware scheduling before execution
                    self.conflict_aware_schedule(graph);

                    self.execute_node(node, graph)?;
                }
            } else {
                // If no node was selected, break the loop
                break;
            }

            self.step_count += 1;
        }

        if self.step_count >= self.max_steps {
            return Err(RuleEngineError::ExecutionLimitExceeded);
        }

        Ok(())
    }

    /// Executes a flow pipeline with demand-driven evaluation
    pub fn execute_flow_pipeline(&mut self, graph: &ExecutionGraph, flow_node_id: u32) -> Result<(), RuleEngineError> {
        // Find the flow node in the graph
        let flow_node = graph.nodes.iter()
            .find(|n| n.id == flow_node_id)
            .ok_or(RuleEngineError::InvalidNodeType)?;

        // Get all connected nodes for this flow
        let connected_nodes = self.get_connected_nodes_for_flow(flow_node_id, graph);

        // Execute nodes in the flow with demand-driven evaluation
        for node_id in connected_nodes {
            if let Some(node) = graph.nodes.iter().find(|n| n.id == node_id) {
                self.execute_node_demand_driven(node, graph)?;
            }
        }

        Ok(())
    }

    /// Executes a node with demand-driven evaluation
    fn execute_node_demand_driven(&mut self, node: &GraphNode, graph: &ExecutionGraph) -> Result<(), RuleEngineError> {
        // Check if all required inputs are available
        if !self.are_inputs_available(node) {
            // If inputs are not available, defer execution
            return Ok(());
        }

        // Execute the node
        self.execute_node(node, graph)?;

        // Propagate outputs to dependent nodes
        self.propagate_outputs(node, graph)?;

        Ok(())
    }

    /// Checks if all required inputs for a node are available
    fn are_inputs_available(&self, node: &GraphNode) -> bool {
        for &input_reg in &node.input_regs {
            if input_reg != 0 {  // Non-zero means there's an input register
                let reg_idx = input_reg as usize;
                if reg_idx < self.context.registers.len() {
                    if self.context.registers[reg_idx].is_none() {
                        return false;  // Required input is not available
                    }
                } else {
                    return false;  // Invalid register index
                }
            }
        }
        true
    }

    /// Propagates outputs from a node to dependent nodes
    fn propagate_outputs(&mut self, node: &GraphNode, graph: &ExecutionGraph) -> Result<(), RuleEngineError> {
        // Find all nodes that depend on this node's outputs
        for edge in &graph.edges {
            if edge.from_node == node.id {
                // Add the dependent node to the execution queue
                if !self.priority_queue.contains(&edge.to_node) {
                    self.priority_queue.push(edge.to_node);
                }
            }
        }
        Ok(())
    }

    /// Gets all connected nodes for a specific flow
    fn get_connected_nodes_for_flow(&self, flow_node_id: u32, graph: &ExecutionGraph) -> Vec<u32> {
        let mut connected_nodes = Vec::new();
        let mut visited = std::collections::HashSet::new();

        // Start with the flow node
        let mut queue = vec![flow_node_id];
        visited.insert(flow_node_id);

        while let Some(current_node_id) = queue.pop() {
            // Add all directly connected nodes
            for edge in &graph.edges {
                if edge.from_node == current_node_id && !visited.contains(&edge.to_node) {
                    connected_nodes.push(edge.to_node);
                    queue.push(edge.to_node);
                    visited.insert(edge.to_node);
                }
            }
        }

        connected_nodes
    }

    /// Implements lazy evaluation for a node
    pub fn evaluate_lazy(&mut self, node_id: u32, graph: &ExecutionGraph) -> Result<Value, RuleEngineError> {
        // Check if the result is already computed and cached
        let cache_key = format!("lazy_result_{}", node_id);
        if let Some(cached_result) = self.context.variables.get(&cache_key) {
            return Ok(cached_result.clone());
        }

        // Execute the node to get the result
        let node = graph.nodes.iter()
            .find(|n| n.id == node_id)
            .ok_or(RuleEngineError::InvalidNodeType)?;

        // Execute the node and get its result
        self.execute_node(node, graph)?;

        // Get the result from the output register
        let result = if !node.output_regs.is_empty() && node.output_regs[0] != 0 {
            let reg_idx = node.output_regs[0] as usize;
            if reg_idx < self.context.registers.len() {
                if let Some(value) = &self.context.registers[reg_idx] {
                    value.clone()
                } else {
                    // If no value in register, return a default
                    Value::Sym(format!("node_{}_result", node_id))
                }
            } else {
                Value::Sym(format!("node_{}_result", node_id))
            }
        } else {
            Value::Sym(format!("node_{}_result", node_id))
        };

        // Cache the result for future use
        self.context.variables.insert(cache_key, result.clone());

        Ok(result)
    }

    /// Implements lazy evaluation for a graph with dependencies
    pub fn evaluate_lazy_with_dependencies(&mut self, node_id: u32, graph: &ExecutionGraph) -> Result<Value, RuleEngineError> {
        // First, evaluate all dependencies lazily
        for edge in &graph.edges {
            if edge.to_node == node_id && edge.edge_type == EdgeType::Data {
                // Evaluate the dependency node
                self.evaluate_lazy(edge.from_node, graph)?;
            }
        }

        // Then evaluate the target node
        self.evaluate_lazy(node_id, graph)
    }

    /// Creates a new execution context
    pub fn create_context(&mut self) -> ExecutionContext {
        ExecutionContext {
            registers: [None; 16],
            variables: HashMap::new(),
            facts: HashMap::new(),
            rule_results: HashMap::new(),
            current_node_id: self.context.current_node_id,
        }
    }

    /// Switches to a different execution context
    pub fn switch_context(&mut self, new_context: ExecutionContext) {
        self.context = new_context;
    }

    /// Clones the current context
    pub fn clone_context(&self) -> ExecutionContext {
        self.context.clone()
    }

    /// Passes context to a sub-flow or rule
    pub fn pass_context_to_subflow(&mut self, subflow_node_id: u32, graph: &ExecutionGraph) -> Result<(), RuleEngineError> {
        // Create a new context based on the current one
        let mut sub_context = self.clone_context();
        sub_context.current_node_id = Some(subflow_node_id);

        // Execute the subflow with the new context
        let original_context = std::mem::replace(&mut self.context, sub_context);

        // Execute the subflow
        let result = self.execute_flow_pipeline(graph, subflow_node_id);

        // Restore the original context
        self.context = original_context;

        result
    }

    fn execute_node(&mut self, node: &GraphNode, graph: &ExecutionGraph) -> Result<(), RuleEngineError> {
        match node.node_type {
            GraphNodeType::Op => self.execute_op_node(node),
            GraphNodeType::Rule => self.execute_rule_node(node, graph),
            GraphNodeType::Control => self.execute_control_node(node, graph),
            GraphNodeType::Graph => self.execute_graph_node(node),
            GraphNodeType::Io => self.execute_io_node(node),
        }
    }

    fn execute_op_node(&mut self, node: &GraphNode) -> Result<(), RuleEngineError> {
        match node.opcode {
            0x10 => self.execute_load_sym(node),      // LOAD_SYM
            0x11 => self.execute_load_num(node),      // LOAD_NUM
            0x12 => self.execute_move(node),          // MOVE
            0x13 => self.execute_compare(node),       // COMPARE
            _ => {
                // For other opcodes, we'll implement as needed
                println!("Executing operation node with opcode: {}", node.opcode);
                Ok(())
            }
        }
    }

    fn execute_load_sym(&mut self, node: &GraphNode) -> Result<(), RuleEngineError> {
        // Extract the symbol name from metadata or context
        // For now, we'll use a placeholder approach
        let dest_reg = node.output_regs[0] as usize;
        if dest_reg < self.context.registers.len() {
            // In a real implementation, we'd get the symbol name from metadata or elsewhere
            self.context.registers[dest_reg] = Some(Value::Sym(format!("symbol_{}", node.id)));
        }
        Ok(())
    }

    fn execute_load_num(&mut self, node: &GraphNode) -> Result<(), RuleEngineError> {
        let dest_reg = node.output_regs[0] as usize;
        if dest_reg < self.context.registers.len() {
            // In a real implementation, we'd get the number from metadata or elsewhere
            self.context.registers[dest_reg] = Some(Value::Num(node.id as i64));
        }
        Ok(())
    }

    fn execute_move(&mut self, node: &GraphNode) -> Result<(), RuleEngineError> {
        let src_reg = node.input_regs[0] as usize;
        let dest_reg = node.output_regs[0] as usize;

        if src_reg >= self.context.registers.len() || dest_reg >= self.context.registers.len() {
            return Err(RuleEngineError::MissingRegisterValue(src_reg as u16));
        }

        if let Some(value) = &self.context.registers[src_reg] {
            self.context.registers[dest_reg] = Some(value.clone());
        } else {
            return Err(RuleEngineError::MissingRegisterValue(src_reg as u16));
        }

        Ok(())
    }

    fn execute_compare(&mut self, node: &GraphNode) -> Result<(), RuleEngineError> {
        let reg_a = node.input_regs[0] as usize;
        let reg_b = node.input_regs[1] as usize;

        if reg_a >= self.context.registers.len() || reg_b >= self.context.registers.len() {
            return Err(RuleEngineError::MissingRegisterValue(reg_a as u16));
        }

        if let (Some(val_a), Some(val_b)) = (&self.context.registers[reg_a], &self.context.registers[reg_b]) {
            let result = match node.flags as u8 {
                0 => Value::Bool(self.compare_values(val_a, val_b, &Comparator::Equal)?),      // ==
                1 => Value::Bool(self.compare_values(val_a, val_b, &Comparator::NotEqual)?),   // !=
                2 => Value::Bool(self.compare_values(val_a, val_b, &Comparator::Greater)?),    // >
                3 => Value::Bool(self.compare_values(val_a, val_b, &Comparator::Less)?),       // <
                4 => Value::Bool(self.compare_values(val_a, val_b, &Comparator::GreaterEqual)?), // >=
                5 => Value::Bool(self.compare_values(val_a, val_b, &Comparator::LessEqual)?),  // <=
                _ => Value::Bool(false), // Default to false for unknown comparators
            };

            // Store result in output register
            let result_reg = node.output_regs[0] as usize;
            if result_reg < self.context.registers.len() {
                self.context.registers[result_reg] = Some(result);
            }
        } else {
            return Err(RuleEngineError::MissingRegisterValue(reg_a as u16));
        }

        Ok(())
    }

    fn compare_values(&self, val_a: &Value, val_b: &Value, op: &Comparator) -> Result<bool, RuleEngineError> {
        match (val_a, val_b, op) {
            (Value::Num(a), Value::Num(b), Comparator::Equal) => Ok(a == b),
            (Value::Num(a), Value::Num(b), Comparator::NotEqual) => Ok(a != b),
            (Value::Num(a), Value::Num(b), Comparator::Greater) => Ok(a > b),
            (Value::Num(a), Value::Num(b), Comparator::Less) => Ok(a < b),
            (Value::Num(a), Value::Num(b), Comparator::GreaterEqual) => Ok(a >= b),
            (Value::Num(a), Value::Num(b), Comparator::LessEqual) => Ok(a <= b),
            (Value::Sym(a), Value::Sym(b), Comparator::Equal) => Ok(a == b),
            (Value::Sym(a), Value::Sym(b), Comparator::NotEqual) => Ok(a != b),
            (Value::Bool(a), Value::Bool(b), Comparator::Equal) => Ok(a == b),
            (Value::Bool(a), Value::Bool(b), Comparator::NotEqual) => Ok(a != b),
            _ => Err(RuleEngineError::InvalidComparison(op.clone(), val_a.clone(), val_b.clone())),
        }
    }

    fn execute_rule_node(&mut self, node: &GraphNode, graph: &ExecutionGraph) -> Result<(), RuleEngineError> {
        println!("Executing rule node: {}", node.id);

        // Start tracking execution of this rule (with recursion prevention)
        self.start_rule_execution(node.id)?;

        // Increment the activation count for this rule
        self.increment_rule_activation(node.id);

        // Evaluate the rule's condition by traversing the connected nodes
        let condition_result = self.evaluate_rule_condition(node, graph)?;

        if condition_result {
            // Execute the rule's actions if the condition is satisfied
            self.execute_rule_actions(node, graph)?;
        }

        // Add connected nodes to the priority queue
        self.add_connected_nodes(node, graph);

        // End tracking execution of this rule
        self.end_rule_execution(node.id);

        Ok(())
    }

    /// Evaluates the condition part of a rule
    fn evaluate_rule_condition(&mut self, rule_node: &GraphNode, graph: &ExecutionGraph) -> Result<bool, RuleEngineError> {
        // Find all nodes connected to this rule node that represent conditions
        let mut condition_nodes = Vec::new();

        for edge in &graph.edges {
            if edge.from_node == rule_node.id && edge.edge_type == EdgeType::Data {
                if let Some(node) = graph.nodes.iter().find(|n| n.id == edge.to_node) {
                    condition_nodes.push(node.clone());
                }
            }
        }

        // For now, we'll evaluate each condition node and return true if any condition is met
        // In a real implementation, we'd properly evaluate the logical expressions
        for condition_node in condition_nodes {
            if condition_node.node_type == GraphNodeType::Op && condition_node.opcode == 0x13 { // COMPARE
                // Execute the comparison operation
                self.execute_compare(&condition_node)?;

                // Check if the comparison result is true
                let result_reg = condition_node.output_regs[0] as usize;
                if result_reg < self.context.registers.len() {
                    if let Some(Value::Bool(result)) = &self.context.registers[result_reg] {
                        if *result {
                            return Ok(true);
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    /// Executes the action part of a rule
    fn execute_rule_actions(&mut self, rule_node: &GraphNode, graph: &ExecutionGraph) -> Result<(), RuleEngineError> {
        // Find all nodes connected to this rule node that represent actions
        let mut action_nodes = Vec::new();

        for edge in &graph.edges {
            if edge.from_node == rule_node.id && edge.edge_type == EdgeType::Data {
                if let Some(node) = graph.nodes.iter().find(|n| n.id == edge.to_node) {
                    if node.node_type != GraphNodeType::Op || node.opcode != 0x13 { // Not a comparison
                        action_nodes.push(node.clone());
                    }
                }
            }
        }

        // Execute each action node
        for action_node in action_nodes {
            self.execute_node(&action_node, graph)?;
        }

        Ok(())
    }

    fn execute_control_node(&mut self, node: &GraphNode, graph: &ExecutionGraph) -> Result<(), RuleEngineError> {
        match node.opcode {
            0x00 => { // NOP - No operation
                println!("Executing NOP: {}", node.id);
            },
            0x01 => { // JMP - Jump
                println!("Executing JMP: {}", node.id);
                // Get the target from metadata or input register
                if !node.input_regs.is_empty() {
                    let target_reg = node.input_regs[0] as usize;
                    if target_reg < self.context.registers.len() {
                        if let Some(Value::Num(target_id)) = &self.context.registers[target_reg] {
                            // Add the target node to the priority queue
                            if !self.priority_queue.contains(&(*target_id as u32)) {
                                self.priority_queue.push(*target_id as u32);
                            }
                        }
                    }
                }
            },
            0x02 => { // JMP_IF - Conditional jump
                println!("Executing JMP_IF: {}", node.id);
                // Check the condition register
                if node.input_regs.len() >= 2 {
                    let condition_reg = node.input_regs[0] as usize;
                    let target_reg = node.input_regs[1] as usize;

                    if condition_reg < self.context.registers.len() &&
                       target_reg < self.context.registers.len() {
                        if let (Some(Value::Bool(condition)), Some(Value::Num(target_id))) =
                            (&self.context.registers[condition_reg], &self.context.registers[target_reg]) {
                            if *condition {
                                // Add the target node to the priority queue if condition is true
                                if !self.priority_queue.contains(&(*target_id as u32)) {
                                    self.priority_queue.push(*target_id as u32);
                                }
                            }
                        }
                    }
                }
            },
            0x03 => { // HALT - Stop execution
                println!("Executing HALT: {}", node.id);
                // In a real implementation, this would stop the execution
                // For now, we'll just clear the priority queue to stop execution
                self.priority_queue.clear();
            },
            _ => {
                println!("Executing control node with opcode: {}", node.opcode);
                // Handle other control operations based on the graph structure
                self.add_connected_control_nodes(node, graph)?;
            }
        }

        // Add connected nodes to the priority queue based on control flow logic
        self.add_connected_control_nodes(node, graph)?;

        Ok(())
    }

    /// Adds connected control nodes based on execution flow
    fn add_connected_control_nodes(&mut self, node: &GraphNode, graph: &ExecutionGraph) -> Result<(), RuleEngineError> {
        // For control nodes, we need to handle special cases like if/then/else and loops
        match node.opcode {
            0x02 => { // JMP_IF - Handle conditional flow
                // Find the conditional edges from this node
                for edge in &graph.edges {
                    if edge.from_node == node.id {
                        // For conditional nodes, only add the appropriate branch
                        // based on the condition value in the register
                        let condition_reg = node.input_regs[0] as usize;
                        if condition_reg < self.context.registers.len() {
                            if let Some(Value::Bool(condition)) = &self.context.registers[condition_reg] {
                                // Add the appropriate branch based on condition
                                // In a real implementation, we'd check the edge's condition_flag
                                if !self.priority_queue.contains(&edge.to_node) {
                                    self.priority_queue.push(edge.to_node);
                                }
                            }
                        }
                    }
                }
            },
            _ => {
                // For other control nodes, add all connected nodes
                for edge in &graph.edges {
                    if edge.from_node == node.id {
                        if !self.priority_queue.contains(&edge.to_node) {
                            self.priority_queue.push(edge.to_node);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Executes a loop control node
    fn execute_loop_node(&mut self, node: &GraphNode, graph: &ExecutionGraph) -> Result<(), RuleEngineError> {
        // Get the iteration counter register
        let counter_reg = node.input_regs[0] as usize;
        let limit_reg = node.input_regs[1] as usize;

        // Initialize counter if not already set
        if counter_reg < self.context.registers.len() {
            if self.context.registers[counter_reg].is_none() {
                self.context.registers[counter_reg] = Some(Value::Num(0));
            }
        }

        // Check the counter against the limit
        if counter_reg < self.context.registers.len() &&
           limit_reg < self.context.registers.len() {
            if let (Some(Value::Num(counter)), Some(Value::Num(limit))) =
                (&self.context.registers[counter_reg], &self.context.registers[limit_reg]) {
                if *counter < *limit {
                    // Continue the loop - add the loop body to the queue
                    for edge in &graph.edges {
                        if edge.from_node == node.id && edge.edge_type == EdgeType::Control {
                            if !self.priority_queue.contains(&edge.to_node) {
                                self.priority_queue.push(edge.to_node);
                            }
                        }
                    }

                    // Increment the counter
                    self.context.registers[counter_reg] = Some(Value::Num(counter + 1));
                } else {
                    // Loop finished - add exit nodes to the queue
                    for edge in &graph.edges {
                        if edge.from_node == node.id && edge.edge_type == EdgeType::Control {
                            // In a real implementation, we'd distinguish between body and exit edges
                            if !self.priority_queue.contains(&edge.to_node) {
                                self.priority_queue.push(edge.to_node);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn execute_graph_node(&mut self, _node: &GraphNode) -> Result<(), RuleEngineError> {
        // Graph operations would manipulate the symbol graph (not the execution graph)
        println!("Executing graph node");
        Ok(())
    }

    fn execute_io_node(&mut self, _node: &GraphNode) -> Result<(), RuleEngineError> {
        // IO operations would call external functions
        println!("Executing IO node");
        Ok(())
    }

    fn process_node_dependencies(&mut self, node: &GraphNode, graph: &ExecutionGraph) -> Result<(), RuleEngineError> {
        // Process dependencies for the current node
        // This would involve evaluating inputs and preparing for execution
        for i in 0..node.input_regs.len() {
            if node.input_regs[i] != 0 {  // Non-zero means there's an input register
                // In a real implementation, we'd validate that the input register has a value
                // and that all dependencies are satisfied
            }
        }
        
        Ok(())
    }

    fn add_connected_nodes(&mut self, node: &GraphNode, graph: &ExecutionGraph) {
        // Add nodes connected by edges to the priority queue
        for edge in &graph.edges {
            if edge.from_node == node.id {
                // Add the destination node to the queue if it's not already there
                if !self.priority_queue.contains(&edge.to_node) {
                    self.priority_queue.push(edge.to_node);
                }
            }
        }
    }

    // Helper function to set a variable in the context
    pub fn set_variable(&mut self, name: &str, value: Value) {
        self.context.variables.insert(name.to_string(), value);
    }

    // Helper function to get a variable from the context
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.context.variables.get(name)
    }

    /// Matches a pattern against the current context
    pub fn match_pattern(&self, pattern: &Pattern, value: &Value) -> Option<HashMap<String, Value>> {
        let mut bindings = HashMap::new();
        if self.match_pattern_with_bindings(pattern, value, &mut bindings) {
            Some(bindings)
        } else {
            None
        }
    }

    /// Internal function to match a pattern with variable bindings
    fn match_pattern_with_bindings(&self, pattern: &Pattern, value: &Value, bindings: &mut HashMap<String, Value>) -> bool {
        match pattern {
            Pattern::Value(expected) => {
                // Direct value comparison
                expected == value
            },
            Pattern::Variable(var_name) => {
                // Check if this variable is already bound
                if let Some(bound_value) = bindings.get(var_name) {
                    // If already bound, the value must match
                    bound_value == value
                } else {
                    // If not bound, bind it to the current value
                    bindings.insert(var_name.clone(), value.clone());
                    true
                }
            },
            Pattern::Composite(pattern_name, pattern_parts) => {
                // Match structured data patterns
                match (pattern_name.as_str(), value) {
                    ("entity.field", Value::Sym(field_value)) => {
                        // Match entity.field pattern against symbol value
                        if let Some(expected_field) = pattern_parts.get(0) {
                            if let Pattern::Value(Value::Sym(expected)) = expected_field {
                                return field_value == expected;
                            }
                        }
                        false
                    },
                    ("entity", Value::Sym(entity_value)) => {
                        // Match entity pattern against symbol value
                        if let Some(expected_entity) = pattern_parts.get(0) {
                            if let Pattern::Value(Value::Sym(expected)) = expected_entity {
                                return entity_value == expected;
                            }
                        }
                        false
                    },
                    ("entity.fields", Value::Vec(field_values)) => {
                        // Match entity fields pattern against vector of values
                        if pattern_parts.len() == field_values.len() {
                            for (pattern_part, field_value) in pattern_parts.iter().zip(field_values.iter()) {
                                if !self.match_pattern_with_bindings(pattern_part, field_value, bindings) {
                                    return false;
                                }
                            }
                            true
                        } else {
                            false
                        }
                    },
                    _ => false,
                }
            }
        }
    }

    /// Enhanced rule matching algorithm that matches against facts in the context
    pub fn match_rule_condition(&self, condition: &kern_parser::Condition) -> Result<bool, RuleEngineError> {
        match condition {
            kern_parser::Condition::Expression(expr) => {
                self.evaluate_expression(expr)
            },
            kern_parser::Condition::LogicalOp(left, op, right) => {
                let left_result = self.match_rule_condition(left)?;
                let right_result = self.match_rule_condition(right)?;

                match op {
                    kern_parser::LogicalOp::And => Ok(left_result && right_result),
                    kern_parser::LogicalOp::Or => Ok(left_result || right_result),
                }
            }
        }
    }

    /// Evaluates an expression to determine if it matches the current context
    fn evaluate_expression(&self, expression: &kern_parser::Expression) -> Result<bool, RuleEngineError> {
        match expression {
            kern_parser::Expression::Comparison { left, op, right } => {
                let left_value = self.get_term_value(left)?;
                let right_value = self.get_term_value(right)?;

                match op {
                    kern_parser::Comparator::Equal => Ok(left_value == right_value),
                    kern_parser::Comparator::NotEqual => Ok(left_value != right_value),
                    kern_parser::Comparator::Greater => {
                        match (&left_value, &right_value) {
                            (Value::Num(a), Value::Num(b)) => Ok(a > b),
                            _ => Err(RuleEngineError::InvalidComparison(op.clone(), left_value, right_value)),
                        }
                    },
                    kern_parser::Comparator::Less => {
                        match (&left_value, &right_value) {
                            (Value::Num(a), Value::Num(b)) => Ok(a < b),
                            _ => Err(RuleEngineError::InvalidComparison(op.clone(), left_value, right_value)),
                        }
                    },
                    kern_parser::Comparator::GreaterEqual => {
                        match (&left_value, &right_value) {
                            (Value::Num(a), Value::Num(b)) => Ok(a >= b),
                            _ => Err(RuleEngineError::InvalidComparison(op.clone(), left_value, right_value)),
                        }
                    },
                    kern_parser::Comparator::LessEqual => {
                        match (&left_value, &right_value) {
                            (Value::Num(a), Value::Num(b)) => Ok(a <= b),
                            _ => Err(RuleEngineError::InvalidComparison(op.clone(), left_value, right_value)),
                        }
                    },
                }
            },
            kern_parser::Expression::Predicate(predicate) => {
                // For now, we'll return true for any predicate
                // In a real implementation, this would call external functions
                println!("Evaluating predicate: {}", predicate.name);
                Ok(true)
            }
        }
    }

    /// Gets the value of a term from the execution context
    fn get_term_value(&self, term: &kern_parser::Term) -> Result<Value, RuleEngineError> {
        match term {
            kern_parser::Term::Identifier(name) => {
                // Look up the value in variables or facts
                if let Some(value) = self.context.variables.get(name) {
                    Ok(value.clone())
                } else if let Some(value) = self.context.facts.get(name) {
                    Ok(value.clone())
                } else {
                    // If not found, return a default value or error
                    Err(RuleEngineError::InvalidPredicate(format!("Undefined identifier: {}", name)))
                }
            },
            kern_parser::Term::Number(n) => Ok(Value::Num(*n)),
            kern_parser::Term::QualifiedRef(entity, field) => {
                // Look up qualified reference (entity.field)
                let var_name = format!("{}.{}", entity, field);
                if let Some(value) = self.context.variables.get(&var_name) {
                    Ok(value.clone())
                } else if let Some(value) = self.context.facts.get(&var_name) {
                    Ok(value.clone())
                } else {
                    // If not found, return a default value or error
                    Err(RuleEngineError::InvalidPredicate(format!("Undefined qualified reference: {}", var_name)))
                }
            }
        }
    }

    /// Matches a pattern against the graph nodes
    pub fn match_graph_pattern(&mut self, pattern: &Pattern, graph: &ExecutionGraph) -> Vec<PatternMatch> {
        let mut matches = Vec::new();

        // Iterate through all nodes in the graph to find matches
        for node in &graph.nodes {
            // Extract the value represented by this node
            let node_value = self.get_node_value(node);

            if let Some(bindings) = self.match_pattern(pattern, &node_value) {
                matches.push(PatternMatch {
                    bindings,
                    matched_node: node.id,
                });
            }
        }

        matches
    }

    /// Enhanced pattern matching engine that supports complex pattern matching
    pub fn match_complex_pattern(&self, pattern: &Pattern, value: &Value) -> Option<HashMap<String, Value>> {
        let mut bindings = HashMap::new();
        if self.match_complex_pattern_with_bindings(pattern, value, &mut bindings) {
            Some(bindings)
        } else {
            None
        }
    }

    /// Internal function to match complex patterns with variable bindings
    fn match_complex_pattern_with_bindings(&self, pattern: &Pattern, value: &Value, bindings: &mut HashMap<String, Value>) -> bool {
        match pattern {
            Pattern::Value(expected) => {
                // Direct value comparison
                expected == value
            },
            Pattern::Variable(var_name) => {
                // Check if this variable is already bound
                if let Some(bound_value) = bindings.get(var_name) {
                    // If already bound, the value must match
                    bound_value == value
                } else {
                    // If not bound, bind it to the current value
                    bindings.insert(var_name.clone(), value.clone());
                    true
                }
            },
            Pattern::Composite(pattern_name, pattern_parts) => {
                match (pattern_name.as_str(), value) {
                    // Match entity.field pattern
                    ("entity.field", Value::Sym(field_value)) => {
                        if let Some(expected_field) = pattern_parts.get(0) {
                            if let Pattern::Value(Value::Sym(expected)) = expected_field {
                                return field_value == expected;
                            }
                        }
                        false
                    },
                    // Match entity pattern
                    ("entity", Value::Sym(entity_value)) => {
                        if let Some(expected_entity) = pattern_parts.get(0) {
                            if let Pattern::Value(Value::Sym(expected)) = expected_entity {
                                return entity_value == expected;
                            }
                        }
                        false
                    },
                    // Match entity fields pattern
                    ("entity.fields", Value::Vec(field_values)) => {
                        if pattern_parts.len() == field_values.len() {
                            for (pattern_part, field_value) in pattern_parts.iter().zip(field_values.iter()) {
                                if !self.match_complex_pattern_with_bindings(pattern_part, field_value, bindings) {
                                    return false;
                                }
                            }
                            true
                        } else {
                            false
                        }
                    },
                    // Match vector pattern
                    ("vec", Value::Vec(vec_values)) => {
                        if pattern_parts.len() == vec_values.len() {
                            for (pattern_part, vec_value) in pattern_parts.iter().zip(vec_values.iter()) {
                                if !self.match_complex_pattern_with_bindings(pattern_part, vec_value, bindings) {
                                    return false;
                                }
                            }
                            true
                        } else {
                            false
                        }
                    },
                    // Match any pattern (wildcard)
                    ("any", _) => true,
                    // Match type pattern
                    ("type.sym", Value::Sym(_)) |
                    ("type.num", Value::Num(_)) |
                    ("type.bool", Value::Bool(_)) |
                    ("type.vec", Value::Vec(_)) |
                    ("type.ref", Value::Ref(_)) => {
                        let expected_type = pattern_name.strip_prefix("type.").unwrap_or("");
                        match expected_type {
                            "sym" => matches!(value, Value::Sym(_)),
                            "num" => matches!(value, Value::Num(_)),
                            "bool" => matches!(value, Value::Bool(_)),
                            "vec" => matches!(value, Value::Vec(_)),
                            "ref" => matches!(value, Value::Ref(_)),
                            _ => false,
                        }
                    },
                    _ => false,
                }
            }
        }
    }

    /// Matches multiple patterns against a set of values (for rule conditions)
    pub fn match_multiple_patterns(&self, patterns: &[Pattern], values: &[Value]) -> Option<Vec<HashMap<String, Value>>> {
        if patterns.len() != values.len() {
            return None;
        }

        let mut all_bindings = Vec::new();
        let mut global_bindings = HashMap::new();

        for (pattern, value) in patterns.iter().zip(values.iter()) {
            let mut local_bindings = global_bindings.clone();
            if self.match_pattern_with_bindings(pattern, value, &mut local_bindings) {
                // Update global bindings with new bindings
                global_bindings = local_bindings;
                all_bindings.push(global_bindings.clone());
            } else {
                return None; // Pattern didn't match
            }
        }

        Some(all_bindings)
    }

    /// Gets the value represented by a graph node
    fn get_node_value(&self, node: &GraphNode) -> Value {
        // This is a simplified implementation
        // In a real system, this would extract the actual value from the node
        match node.node_type {
            GraphNodeType::Op => {
                // Check the output registers for the actual value
                for (i, &reg_idx) in node.output_regs.iter().enumerate() {
                    if reg_idx != 0 {
                        if let Some(Some(value)) = self.context.registers.get(reg_idx as usize) {
                            return value.clone();
                        }
                    }
                }
                // If no output register has a value, return a default
                Value::Sym(format!("op_{}_{}", node.opcode, node.id))
            },
            GraphNodeType::Rule => Value::Sym(format!("rule_{}", node.id)),
            GraphNodeType::Control => Value::Sym(format!("control_{}", node.id)),
            GraphNodeType::Graph => Value::Sym(format!("graph_{}", node.id)),
            GraphNodeType::Io => Value::Sym(format!("io_{}", node.id)),
        }
    }

    /// Detects conflicts between rules based on their conditions and actions
    pub fn detect_rule_conflicts(&self, graph: &ExecutionGraph) -> Vec<RuleConflict> {
        let mut conflicts = Vec::new();

        // Get all rule nodes in the graph
        let rule_nodes: Vec<&GraphNode> = graph.nodes.iter()
            .filter(|node| node.node_type == GraphNodeType::Rule)
            .collect();

        // Compare each pair of rules for potential conflicts
        for i in 0..rule_nodes.len() {
            for j in (i + 1)..rule_nodes.len() {
                let rule1 = rule_nodes[i];
                let rule2 = rule_nodes[j];

                if let Some(conflict) = self.check_rule_conflict(rule1, rule2, graph) {
                    conflicts.push(conflict);
                }
            }
        }

        conflicts
    }

    /// Checks if two rules conflict with each other
    fn check_rule_conflict(&self, rule1: &GraphNode, rule2: &GraphNode, graph: &ExecutionGraph) -> Option<RuleConflict> {
        // For now, we'll implement a basic check
        // In a real implementation, this would be more sophisticated

        // Check if rules modify the same variables or entities
        let rule1_actions = self.get_rule_actions(rule1, graph);
        let rule2_actions = self.get_rule_actions(rule2, graph);

        // Check for conflicting assignments
        for action1 in &rule1_actions {
            for action2 in &rule2_actions {
                if self.actions_conflict(action1, action2) {
                    return Some(RuleConflict {
                        rule1_id: rule1.id,
                        rule2_id: rule2.id,
                        conflict_type: ConflictType::ActionConflict,
                        description: format!("Rules {} and {} have conflicting actions", rule1.id, rule2.id),
                    });
                }
            }
        }

        None
    }

    /// Gets the actions associated with a rule node
    fn get_rule_actions(&self, rule_node: &GraphNode, graph: &ExecutionGraph) -> Vec<GraphNode> {
        let mut actions = Vec::new();

        // Find all nodes connected to this rule node that represent actions
        for edge in &graph.edges {
            if edge.from_node == rule_node.id && edge.edge_type == EdgeType::Data {
                if let Some(node) = graph.nodes.iter().find(|n| n.id == edge.to_node) {
                    // Exclude comparison nodes which are part of conditions
                    if !(node.node_type == GraphNodeType::Op && node.opcode == 0x13) { // COMPARE
                        actions.push(node.clone());
                    }
                }
            }
        }

        actions
    }

    /// Checks if two actions conflict with each other
    fn actions_conflict(&self, action1: &GraphNode, action2: &GraphNode) -> bool {
        // For now, we'll check if both actions are assignments to the same variable
        // In a real implementation, this would be more sophisticated

        // This is a simplified check - in a real system, we'd need to analyze
        // the actual operations and their effects on shared state
        action1.opcode == 0x12 && action2.opcode == 0x12  // Both are MOVE operations
    }

    /// Resolves conflicts between rules using the current priority strategy
    pub fn resolve_conflicts(&mut self, conflicts: &[RuleConflict]) {
        for conflict in conflicts {
            // Update conflict scores based on the detected conflicts
            self.update_rule_conflict_score(conflict.rule1_id, conflict.conflict_type as u32);
            self.update_rule_conflict_score(conflict.rule2_id, conflict.conflict_type as u32);
        }

        // Switch to conflict resolution priority strategy if there are conflicts
        if !conflicts.is_empty() {
            self.priority_strategy = PriorityStrategy::ConflictResolution;
        }
    }

    /// Sets the maximum recursion depth allowed
    pub fn set_max_recursion_depth(&mut self, depth: u32) {
        self.max_recursion_depth = depth;
    }

    /// Checks if executing a rule would cause recursion beyond the allowed depth
    fn would_exceed_recursion_limit(&self, rule_id: u32) -> bool {
        // Check if this rule is already in the current execution path
        let current_rule_count = self.execution_path.iter().filter(|&&id| id == rule_id).count() as u32;

        // If we've already executed this rule more than the allowed depth, it would exceed the limit
        current_rule_count >= self.max_recursion_depth
    }

    /// Starts tracking execution of a rule
    fn start_rule_execution(&mut self, rule_id: u32) -> Result<(), RuleEngineError> {
        // Check if executing this rule would exceed the recursion limit
        if self.would_exceed_recursion_limit(rule_id) {
            return Err(RuleEngineError::ExecutionLimitExceeded);
        }

        // Add the rule to the execution path
        self.execution_path.push(rule_id);

        // Increment the execution count for this rule
        *self.rule_execution_counts.entry(rule_id).or_insert(0) += 1;

        Ok(())
    }

    /// Ends tracking execution of a rule
    fn end_rule_execution(&mut self, rule_id: u32) {
        // Remove the rule from the execution path
        if let Some(pos) = self.execution_path.iter().rposition(|&x| x == rule_id) {
            self.execution_path.remove(pos);
        }

        // Decrement the execution count for this rule
        if let Some(count) = self.rule_execution_counts.get_mut(&rule_id) {
            if *count > 0 {
                *count -= 1;
            } else {
                self.rule_execution_counts.remove(&rule_id);
            }
        }
    }

    /// Checks if a rule is currently in the execution path (potential recursion)
    fn is_in_execution_path(&self, rule_id: u32) -> bool {
        self.execution_path.contains(&rule_id)
    }
}

/// Represents a conflict between two rules
#[derive(Debug, Clone)]
pub struct RuleConflict {
    pub rule1_id: u32,
    pub rule2_id: u32,
    pub conflict_type: ConflictType,
    pub description: String,
}

/// Types of conflicts that can occur between rules
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConflictType {
    /// Rules have contradictory conditions
    ContradictoryConditions = 1,
    /// Rules perform conflicting actions
    ActionConflict = 2,
    /// Rules compete for the same resources
    ResourceConflict = 3,
    /// Rules modify the same state in conflicting ways
    StateConflict = 4,
}

#[cfg(test)]
mod tests {
    use super::*;
    use kern_parser::Parser;
    use kern_graph_builder::GraphBuilder;

    #[test]
    fn test_rule_engine_execution() {
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

        let mut parser = Parser::new(input);
        let result = parser.parse_program();
        assert!(result.is_ok());
        let program = result.unwrap();

        let mut builder = GraphBuilder::new();
        let graph = builder.build_execution_graph(&program);
        println!("Generated execution graph with {} nodes", graph.nodes.len());

        let mut engine = RuleEngine::new();
        
        // Set up some initial values for testing
        engine.set_variable("farmer.location", Value::Sym("valid".to_string()));
        engine.set_variable("farmer.id", Value::Num(123));
        
        let execution_result = engine.execute_graph(&graph);
        assert!(execution_result.is_ok());
        
        println!("Rule engine executed successfully with {} steps", engine.step_count);
    }
}