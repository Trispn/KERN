use kern_parser::{
    Action, Assignment, AstNode, Condition, ConstraintDef, ControlAction, Definition, EntityDef,
    Expression, FlowDef, HaltAction, IfAction, LoopAction, Predicate, Program, RuleDef, Term,
};
use std::collections::HashMap;

// Define the execution graph data structures as specified in the KERN language documentation
#[derive(Debug, Clone, PartialEq)]
pub enum GraphNodeType {
    Op,      // bytecode operation
    Rule,    // rule evaluation
    Control, // if / loop / jump
    Graph,   // graph manipulation
    Io,      // external interface
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum EdgeType {
    Control,   // execution order
    Data,      // value dependency
    Condition, // conditional routing
}

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: u32,
    pub node_type: GraphNodeType,
    pub opcode: u8, // bytecode opcode
    pub flags: u16,
    pub input_regs: [u16; 4], // register indices
    pub output_regs: [u16; 2],
    pub first_edge: u32, // edge index
    pub edge_count: u16,
    pub meta: NodeMeta,
}

// Specialized control nodes as specified in the KERN language documentation
#[derive(Debug, Clone)]
pub struct IfNode {
    pub base: GraphNode,
    pub condition_reg: u8,
    pub true_edge: Option<u32>,
    pub false_edge: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct LoopNode {
    pub base: GraphNode,
    pub body_entry: Option<u32>,
    pub exit_edge: Option<u32>,
    pub iteration_limit: u32,
}

#[derive(Debug, Clone)]
pub struct RuleNode {
    pub base: GraphNode,
    pub rule_id: u32,
    pub priority: u16,
    pub evaluation_mode: u8, // 0 = eager, 1 = lazy
}

#[derive(Debug, Clone)]
pub struct GraphOpNode {
    pub base: GraphNode,
    pub graph_op_type: u8, // 0 = create, 1 = match, 2 = traverse
    pub operand_id: u32,
}

impl IfNode {
    pub fn new(base: GraphNode, condition_reg: u8) -> Self {
        IfNode {
            base,
            condition_reg,
            true_edge: None,
            false_edge: None,
        }
    }
}

impl LoopNode {
    pub fn new(base: GraphNode, iteration_limit: u32) -> Self {
        LoopNode {
            base,
            body_entry: None,
            exit_edge: None,
            iteration_limit,
        }
    }
}

impl RuleNode {
    pub fn new(base: GraphNode, rule_id: u32, priority: u16, evaluation_mode: u8) -> Self {
        RuleNode {
            base,
            rule_id,
            priority,
            evaluation_mode,
        }
    }
}

impl GraphOpNode {
    pub fn new(base: GraphNode, graph_op_type: u8, operand_id: u32) -> Self {
        GraphOpNode {
            base,
            graph_op_type,
            operand_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub from_node: u32,
    pub to_node: u32,
    pub edge_type: EdgeType,
    pub condition_flag: u8, // used only for conditional edges
}

impl GraphEdge {
    /// Creates a new control edge
    pub fn new_control(from_node: u32, to_node: u32) -> Self {
        GraphEdge {
            from_node,
            to_node,
            edge_type: EdgeType::Control,
            condition_flag: 0,
        }
    }

    /// Creates a new data edge
    pub fn new_data(from_node: u32, to_node: u32) -> Self {
        GraphEdge {
            from_node,
            to_node,
            edge_type: EdgeType::Data,
            condition_flag: 0,
        }
    }

    /// Creates a new conditional edge
    pub fn new_condition(from_node: u32, to_node: u32, condition_flag: u8) -> Self {
        GraphEdge {
            from_node,
            to_node,
            edge_type: EdgeType::Condition,
            condition_flag,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeMeta {
    pub source_ref: u32, // mapping to KERN source
    pub cost_hint: u16,  // heuristic cost
}

#[derive(Debug, Clone, Copy)]
pub struct Register {
    pub reg_type: u8,  // sym, num, ref, vec (represented as u8)
    pub value_id: u32, // index into value table
}

#[derive(Debug, Clone)]
pub struct RegisterSet {
    pub regs: [Register; 16], // R0–R15
}

#[derive(Debug, Clone)]
pub struct Context {
    pub id: u32,
    pub registers: RegisterSet,
    pub flags: u8,
}

#[derive(Debug, Clone)]
pub struct ContextPool {
    pub contexts: Vec<Context>,
}

#[derive(Debug, Clone)]
pub struct EntryPoint {
    pub node_id: u32,
    pub entry_type: u8, // 0=rule, 1=flow, 2=external call
}

#[derive(Debug, Clone)]
pub enum SpecializedNode {
    Base(GraphNode),
    If(IfNode),
    Loop(LoopNode),
    Rule(RuleNode),
    GraphOp(GraphOpNode),
}

impl SpecializedNode {
    pub fn get_base(&self) -> &GraphNode {
        match self {
            SpecializedNode::Base(node) => node,
            SpecializedNode::If(node) => &node.base,
            SpecializedNode::Loop(node) => &node.base,
            SpecializedNode::Rule(node) => &node.base,
            SpecializedNode::GraphOp(node) => &node.base,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionGraph {
    pub nodes: Vec<SpecializedNode>,
    pub edges: Vec<GraphEdge>,
    pub node_count: u32,
    pub edge_count: u32,
    pub entry_points: Vec<EntryPoint>,
    pub entry_count: u16,
    pub registers: RegisterSet,
    pub contexts: ContextPool,
    pub metadata: GraphMeta,
}

#[derive(Debug, Clone)]
pub struct GraphMeta {
    pub build_hash: u32,
    pub version: u16,
}

// The GraphBuilder converts AST nodes to execution graphs
pub struct GraphBuilder {
    node_id_counter: u32,
    edge_id_counter: u32,
    nodes: Vec<SpecializedNode>,
    edges: Vec<GraphEdge>,
    entry_points: Vec<EntryPoint>,
    registers: RegisterSet,
    contexts: ContextPool,
    source_map: HashMap<u32, String>, // Maps node IDs to source locations for debugging
}

impl GraphBuilder {
    pub fn new() -> Self {
        GraphBuilder {
            node_id_counter: 0,
            edge_id_counter: 0,
            nodes: Vec::new(),
            edges: Vec::new(),
            entry_points: Vec::new(),
            registers: RegisterSet {
                regs: [Register {
                    reg_type: 0,
                    value_id: 0,
                }; 16],
            },
            contexts: ContextPool {
                contexts: Vec::new(),
            },
            source_map: HashMap::new(),
        }
    }

    // Main function to convert a Program AST to an ExecutionGraph
    pub fn build_execution_graph(&mut self, program: &Program) -> ExecutionGraph {
        // Process each definition in the program
        for definition in &program.definitions {
            match definition {
                Definition::Entity(entity_def) => {
                    self.process_entity_def(entity_def);
                }
                Definition::Rule(rule_def) => {
                    self.process_rule_def(rule_def);
                }
                Definition::Flow(flow_def) => {
                    self.process_flow_def(flow_def);
                }
                Definition::Constraint(constraint_def) => {
                    self.process_constraint_def(constraint_def);
                }
            }
        }

        // Create the final execution graph
        ExecutionGraph {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
            node_count: self.nodes.len() as u32,
            edge_count: self.edges.len() as u32,
            entry_points: self.entry_points.clone(),
            entry_count: self.entry_points.len() as u16,
            registers: self.registers.clone(),
            contexts: self.contexts.clone(),
            metadata: GraphMeta {
                build_hash: 0, // In a real implementation, this would be a proper hash
                version: 1,
            },
        }
    }

    fn process_entity_def(&mut self, entity_def: &EntityDef) {
        // Entities define data structures but don't create executable nodes
        // However, we might create nodes for validation or initialization
        // For now, we'll just track the entity definition for reference
        println!("Processing entity: {}", entity_def.name);
    }

    fn process_rule_def(&mut self, rule_def: &RuleDef) {
        // Create a rule node
        let rule_node_id = self.node_id_counter;
        self.node_id_counter += 1;

        // Create the base rule evaluation node
        let base_node = GraphNode {
            id: rule_node_id,
            node_type: GraphNodeType::Rule,
            opcode: 0x31, // RULE_EVAL from KERN bytecode spec
            flags: 0,
            input_regs: [0; 4],
            output_regs: [0; 2],
            first_edge: self.edge_id_counter,
            edge_count: 0,
            meta: NodeMeta {
                source_ref: 0, // Would map to actual source location
                cost_hint: 0,
            },
        };

        // Create the specialized RuleNode
        let rule_node = RuleNode::new(base_node, rule_node_id, 10, 0); // rule_id, priority, evaluation_mode

        // Store the RuleNode in our nodes vector using the SpecializedNode enum
        self.nodes.push(SpecializedNode::Rule(rule_node));

        // Add the rule to entry points
        self.entry_points.push(EntryPoint {
            node_id: rule_node_id,
            entry_type: 0, // 0 = rule
        });

        // Process the rule's condition
        self.process_condition(&rule_def.condition, rule_node_id);

        // Process the rule's actions
        for action in &rule_def.actions {
            self.process_action(action, rule_node_id);
        }
    }

    fn process_flow_def(&mut self, flow_def: &FlowDef) {
        // Create a flow execution node
        let flow_node_id = self.node_id_counter;
        self.node_id_counter += 1;

        // Create the flow execution node
        let flow_node = GraphNode {
            id: flow_node_id,
            node_type: GraphNodeType::Control,
            opcode: 0x00, // NOP as a placeholder
            flags: 0,
            input_regs: [0; 4],
            output_regs: [0; 2],
            first_edge: self.edge_id_counter,
            edge_count: 0,
            meta: NodeMeta {
                source_ref: 0, // Would map to actual source location
                cost_hint: 0,
            },
        };

        self.nodes.push(SpecializedNode::Base(flow_node));

        // Add the flow to entry points
        self.entry_points.push(EntryPoint {
            node_id: flow_node_id,
            entry_type: 1, // 1 = flow
        });

        // Process the flow's actions
        for action in &flow_def.actions {
            self.process_action(action, flow_node_id);
        }
    }

    fn process_constraint_def(&mut self, constraint_def: &ConstraintDef) {
        // Create a constraint validation node
        let constraint_node_id = self.node_id_counter;
        self.node_id_counter += 1;

        // Create the constraint validation node
        let constraint_node = GraphNode {
            id: constraint_node_id,
            node_type: GraphNodeType::Op,
            opcode: 0x13, // COMPARE as a placeholder for validation
            flags: 0,
            input_regs: [0; 4],
            output_regs: [0; 2],
            first_edge: self.edge_id_counter,
            edge_count: 0,
            meta: NodeMeta {
                source_ref: 0, // Would map to actual source location
                cost_hint: 0,
            },
        };

        self.nodes.push(SpecializedNode::Base(constraint_node));

        // Add the constraint to entry points
        self.entry_points.push(EntryPoint {
            node_id: constraint_node_id,
            entry_type: 2, // 2 = constraint
        });

        // Process the constraint's condition
        self.process_condition(&constraint_def.condition, constraint_node_id);
    }

    fn process_condition(&mut self, condition: &Condition, parent_node_id: u32) {
        match condition {
            Condition::Expression(expr) => {
                self.process_expression(expr, parent_node_id);
            }
            Condition::LogicalOp(left, op, right) => {
                self.process_condition(left, parent_node_id);
                // Process the logical operator
                match op {
                    kern_parser::LogicalOp::And => {
                        // Handle AND operation
                        println!("Processing AND operation");
                    }
                    kern_parser::LogicalOp::Or => {
                        // Handle OR operation
                        println!("Processing OR operation");
                    }
                }
                self.process_condition(right, parent_node_id);
            }
        }
    }

    fn process_expression(&mut self, expression: &Expression, parent_node_id: u32) {
        match expression {
            Expression::Comparison { left, op, right } => {
                // Create a comparison node
                let compare_node_id = self.node_id_counter;
                self.node_id_counter += 1;

                let opcode = match op {
                    kern_parser::Comparator::Equal => 0x13, // COMPARE with flags for ==
                    kern_parser::Comparator::NotEqual => 0x13, // COMPARE with flags for !=
                    kern_parser::Comparator::Greater => 0x13, // COMPARE with flags for >
                    kern_parser::Comparator::Less => 0x13,  // COMPARE with flags for <
                    kern_parser::Comparator::GreaterEqual => 0x13, // COMPARE with flags for >=
                    kern_parser::Comparator::LessEqual => 0x13, // COMPARE with flags for <=
                };

                let flags = match op {
                    kern_parser::Comparator::Equal => 0,
                    kern_parser::Comparator::NotEqual => 1,
                    kern_parser::Comparator::Greater => 2,
                    kern_parser::Comparator::Less => 3,
                    kern_parser::Comparator::GreaterEqual => 4,
                    kern_parser::Comparator::LessEqual => 5,
                } as u16;

                let compare_node = GraphNode {
                    id: compare_node_id,
                    node_type: GraphNodeType::Op,
                    opcode,
                    flags,
                    input_regs: [0; 4],
                    output_regs: [0; 2],
                    first_edge: self.edge_id_counter,
                    edge_count: 0,
                    meta: NodeMeta {
                        source_ref: 0,
                        cost_hint: 0,
                    },
                };

                self.nodes.push(SpecializedNode::Base(compare_node));

                // Process the left and right terms
                self.process_term(left, compare_node_id);
                self.process_term(right, compare_node_id);

                // Create an edge from the parent to this comparison node
                self.create_edge(parent_node_id, compare_node_id, EdgeType::Data);
            }
            Expression::Predicate(predicate) => {
                self.process_predicate(predicate, parent_node_id);
            }
        }
    }

    fn process_term(&mut self, term: &Term, parent_node_id: u32) {
        match term {
            Term::Identifier(_name) => {
                // Create a node to load the identifier value
                let load_node_id = self.node_id_counter;
                self.node_id_counter += 1;

                let load_node = GraphNode {
                    id: load_node_id,
                    node_type: GraphNodeType::Op,
                    opcode: 0x10, // LOAD_SYM
                    flags: 0,
                    input_regs: [0; 4],
                    output_regs: [0; 2],
                    first_edge: self.edge_id_counter,
                    edge_count: 0,
                    meta: NodeMeta {
                        source_ref: 0,
                        cost_hint: 0,
                    },
                };

                self.nodes.push(SpecializedNode::Base(load_node));
                self.create_edge(parent_node_id, load_node_id, EdgeType::Data);
            }
            Term::Number(_value) => {
                // Create a node to load the number value
                let load_node_id = self.node_id_counter;
                self.node_id_counter += 1;

                let load_node = GraphNode {
                    id: load_node_id,
                    node_type: GraphNodeType::Op,
                    opcode: 0x11, // LOAD_NUM
                    flags: 0,
                    input_regs: [0; 4],
                    output_regs: [0; 2],
                    first_edge: self.edge_id_counter,
                    edge_count: 0,
                    meta: NodeMeta {
                        source_ref: 0,
                        cost_hint: 0,
                    },
                };

                self.nodes.push(SpecializedNode::Base(load_node));
                self.create_edge(parent_node_id, load_node_id, EdgeType::Data);
            }
            Term::QualifiedRef(_entity, _field) => {
                // Create a node to load the qualified reference
                let load_node_id = self.node_id_counter;
                self.node_id_counter += 1;

                let load_node = GraphNode {
                    id: load_node_id,
                    node_type: GraphNodeType::Op,
                    opcode: 0x10, // LOAD_SYM
                    flags: 0,
                    input_regs: [0; 4],
                    output_regs: [0; 2],
                    first_edge: self.edge_id_counter,
                    edge_count: 0,
                    meta: NodeMeta {
                        source_ref: 0,
                        cost_hint: 0,
                    },
                };

                self.nodes.push(SpecializedNode::Base(load_node));
                self.create_edge(parent_node_id, load_node_id, EdgeType::Data);
            }
        }
    }

    fn process_predicate(&mut self, predicate: &Predicate, parent_node_id: u32) {
        // Create a predicate call node
        let pred_node_id = self.node_id_counter;
        self.node_id_counter += 1;

        let pred_node = GraphNode {
            id: pred_node_id,
            node_type: GraphNodeType::Io,
            opcode: 0x60, // EXT_CALL
            flags: 0,
            input_regs: [0; 4],
            output_regs: [0; 2],
            first_edge: self.edge_id_counter,
            edge_count: 0,
            meta: NodeMeta {
                source_ref: 0,
                cost_hint: 0,
            },
        };

        self.nodes.push(SpecializedNode::Base(pred_node));

        // Process the arguments
        for arg in &predicate.arguments {
            self.process_term(arg, pred_node_id);
        }

        // Create an edge from the parent to this predicate node
        self.create_edge(parent_node_id, pred_node_id, EdgeType::Data);
    }

    fn process_action(&mut self, action: &Action, parent_node_id: u32) {
        match action {
            Action::Predicate(predicate) => {
                self.process_predicate(predicate, parent_node_id);
            }
            Action::Assignment(assignment) => {
                self.process_assignment(assignment, parent_node_id);
            }
            Action::Control(control_action) => {
                self.process_control_action(control_action, parent_node_id);
            }
        }
    }

    fn process_assignment(&mut self, assignment: &Assignment, parent_node_id: u32) {
        // Create an assignment node
        let assign_node_id = self.node_id_counter;
        self.node_id_counter += 1;

        let assign_node = GraphNode {
            id: assign_node_id,
            node_type: GraphNodeType::Op,
            opcode: 0x12, // MOVE
            flags: 0,
            input_regs: [0; 4],
            output_regs: [0; 2],
            first_edge: self.edge_id_counter,
            edge_count: 0,
            meta: NodeMeta {
                source_ref: 0,
                cost_hint: 0,
            },
        };

        self.nodes.push(SpecializedNode::Base(assign_node));

        // Process the value being assigned
        self.process_term(&assignment.value, assign_node_id);

        // Create an edge from the parent to this assignment node
        self.create_edge(parent_node_id, assign_node_id, EdgeType::Data);
    }

    fn process_control_action(&mut self, control_action: &ControlAction, parent_node_id: u32) {
        match control_action {
            ControlAction::If(if_action) => {
                self.process_if_action(if_action, parent_node_id);
            }
            ControlAction::Loop(loop_action) => {
                self.process_loop_action(loop_action, parent_node_id);
            }
            ControlAction::Halt(halt_action) => {
                self.process_halt_action(halt_action, parent_node_id);
            }
        }
    }

    fn process_if_action(&mut self, if_action: &IfAction, parent_node_id: u32) {
        // Create an if control node
        let if_node_id = self.node_id_counter;
        self.node_id_counter += 1;

        let base_node = GraphNode {
            id: if_node_id,
            node_type: GraphNodeType::Control,
            opcode: 0x02, // JMP_IF
            flags: 0,
            input_regs: [0; 4],
            output_regs: [0; 2],
            first_edge: self.edge_id_counter,
            edge_count: 0,
            meta: NodeMeta {
                source_ref: 0,
                cost_hint: 0,
            },
        };

        // Create the specialized IfNode
        let if_node = IfNode::new(base_node, 0); // condition_reg will be updated later

        // Store the IfNode in our nodes vector using the SpecializedNode enum
        self.nodes.push(SpecializedNode::If(if_node));

        // Process the condition
        self.process_condition(&if_action.condition, if_node_id);

        // Process then actions
        for action in &if_action.then_actions {
            self.process_action(action, if_node_id);
        }

        // Process else actions if they exist
        if let Some(else_actions) = &if_action.else_actions {
            for action in else_actions {
                self.process_action(action, if_node_id);
            }
        }

        // Create an edge from the parent to this if node
        self.create_edge(parent_node_id, if_node_id, EdgeType::Control);
    }

    fn process_loop_action(&mut self, loop_action: &LoopAction, parent_node_id: u32) {
        // Create a loop control node
        let loop_node_id = self.node_id_counter;
        self.node_id_counter += 1;

        let base_node = GraphNode {
            id: loop_node_id,
            node_type: GraphNodeType::Control,
            opcode: 0x01, // JMP (for loop back)
            flags: 0,
            input_regs: [0; 4],
            output_regs: [0; 2],
            first_edge: self.edge_id_counter,
            edge_count: 0,
            meta: NodeMeta {
                source_ref: 0,
                cost_hint: 0,
            },
        };

        // Create the specialized LoopNode
        let loop_node = LoopNode::new(base_node, 100); // Default iteration limit

        // Store the LoopNode in our nodes vector using the SpecializedNode enum
        self.nodes.push(SpecializedNode::Loop(loop_node));

        // Process the loop body actions
        for action in &loop_action.actions {
            self.process_action(action, loop_node_id);
        }

        // Create an edge from the parent to this loop node
        self.create_edge(parent_node_id, loop_node_id, EdgeType::Control);
    }

    fn process_halt_action(&mut self, _halt_action: &HaltAction, parent_node_id: u32) {
        // Create a halt control node
        let halt_node_id = self.node_id_counter;
        self.node_id_counter += 1;

        let halt_node = GraphNode {
            id: halt_node_id,
            node_type: GraphNodeType::Control,
            opcode: 0x03, // HALT
            flags: 0,
            input_regs: [0; 4],
            output_regs: [0; 2],
            first_edge: self.edge_id_counter,
            edge_count: 0,
            meta: NodeMeta {
                source_ref: 0,
                cost_hint: 0,
            },
        };

        self.nodes.push(SpecializedNode::Base(halt_node));

        // Create an edge from the parent to this halt node
        self.create_edge(parent_node_id, halt_node_id, EdgeType::Control);
    }

    fn create_edge(&mut self, from_node: u32, to_node: u32, edge_type: EdgeType) {
        let edge = GraphEdge {
            from_node,
            to_node,
            edge_type,
            condition_flag: 0,
        };

        self.edges.push(edge);
        self.edge_id_counter += 1;
    }

    /// Validates the execution graph according to KERN specifications
    pub fn validate_graph(&self, graph: &ExecutionGraph) -> Result<(), String> {
        // Check for unreachable nodes
        let reachable = self.find_reachable_nodes(graph);

        for (i, node) in graph.nodes.iter().enumerate() {
            let node_id = match node {
                SpecializedNode::Base(n) => n.id,
                SpecializedNode::If(n) => n.base.id,
                SpecializedNode::Loop(n) => n.base.id,
                SpecializedNode::Rule(n) => n.base.id,
                SpecializedNode::GraphOp(n) => n.base.id,
            };

            if !reachable.contains(&(i as u32)) {
                return Err(format!("Unreachable node: {}", node_id));
            }
        }

        // Check for dangling edges
        for edge in &graph.edges {
            if edge.from_node >= graph.node_count || edge.to_node >= graph.node_count {
                return Err(format!(
                    "Dangling edge: ({}, {})",
                    edge.from_node, edge.to_node
                ));
            }
        }

        // Check for illegal cycles
        if self.has_cycles(graph) {
            return Err("Graph contains illegal cycles".to_string());
        }

        Ok(())
    }

    /// Finds all reachable nodes in the graph using DFS
    fn find_reachable_nodes(&self, graph: &ExecutionGraph) -> std::collections::HashSet<u32> {
        let mut visited = std::collections::HashSet::new();
        let mut stack = Vec::new();

        // Start from all entry points
        for entry_point in &graph.entry_points {
            stack.push(entry_point.node_id);
        }

        while let Some(node_id) = stack.pop() {
            if visited.insert(node_id) {
                // Find all outgoing edges from this node
                for edge in &graph.edges {
                    if edge.from_node == node_id {
                        stack.push(edge.to_node);
                    }
                }
            }
        }

        visited
    }

    /// Checks if the graph contains cycles
    pub fn has_cycles(&self, graph: &ExecutionGraph) -> bool {
        let mut visited = vec![false; graph.node_count as usize];
        let mut rec_stack = vec![false; graph.node_count as usize];

        for entry_point in &graph.entry_points {
            if !visited[entry_point.node_id as usize] {
                if self.is_cyclic_util(graph, entry_point.node_id, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }

        false
    }

    /// Finds all cycles in the graph and returns them as paths
    pub fn find_all_cycles(&self, graph: &ExecutionGraph) -> Vec<Vec<u32>> {
        let mut all_cycles = Vec::new();
        let mut visited = vec![false; graph.node_count as usize];

        for entry_point in &graph.entry_points {
            if !visited[entry_point.node_id as usize] {
                let mut path: Vec<u32> = Vec::new();
                let mut rec_stack = vec![false; graph.node_count as usize];
                let mut path_stack = Vec::new();

                if self.find_cycle_util(
                    graph,
                    entry_point.node_id,
                    &mut visited,
                    &mut rec_stack,
                    &mut path_stack,
                    &mut all_cycles,
                ) {
                    // At least one cycle was found, continue checking
                }
            }
        }

        all_cycles
    }

    /// Utility function for cycle detection using DFS
    fn is_cyclic_util(
        &self,
        graph: &ExecutionGraph,
        node_id: u32,
        visited: &mut [bool],
        rec_stack: &mut [bool],
    ) -> bool {
        let idx = node_id as usize;
        visited[idx] = true;
        rec_stack[idx] = true;

        // Check all outgoing edges
        for edge in &graph.edges {
            if edge.from_node == node_id {
                let to_idx = edge.to_node as usize;

                if !visited[to_idx] {
                    if self.is_cyclic_util(graph, edge.to_node, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack[to_idx] {
                    return true; // Back edge found, cycle exists
                }
            }
        }

        rec_stack[idx] = false;
        false
    }

    /// Utility function to find all cycles in the graph
    fn find_cycle_util(
        &self,
        graph: &ExecutionGraph,
        node_id: u32,
        visited: &mut [bool],
        rec_stack: &mut [bool],
        path_stack: &mut Vec<u32>,
        all_cycles: &mut Vec<Vec<u32>>,
    ) -> bool {
        let idx = node_id as usize;
        visited[idx] = true;
        rec_stack[idx] = true;
        path_stack.push(node_id);

        // Check all outgoing edges
        for edge in &graph.edges {
            if edge.from_node == node_id {
                let to_idx = edge.to_node as usize;

                if !visited[to_idx] {
                    if self.find_cycle_util(
                        graph,
                        edge.to_node,
                        visited,
                        rec_stack,
                        path_stack,
                        all_cycles,
                    ) {
                        return true;
                    }
                } else if rec_stack[to_idx] {
                    // Found a back edge, which indicates a cycle
                    // Extract the cycle from the path stack
                    if let Some(cycle_start_idx) =
                        path_stack.iter().position(|&n| n == edge.to_node)
                    {
                        let cycle = path_stack[cycle_start_idx..].to_vec();
                        all_cycles.push(cycle);
                        return true; // Found at least one cycle
                    }
                }
            }
        }

        path_stack.pop();
        rec_stack[idx] = false;
        false
    }

    /// Optimizes the execution graph by removing redundant nodes and edges
    pub fn optimize_graph(&mut self, graph: &mut ExecutionGraph) {
        // Apply optimization passes until no more changes occur
        let mut changed = true;
        while changed {
            changed = false;

            // Pass 1: Remove unreachable nodes
            changed |= self.remove_unreachable_nodes(graph);

            // Pass 2: Remove redundant edges
            changed |= self.remove_redundant_edges(graph);

            // Pass 3: Combine similar operations
            changed |= self.combine_similar_operations(graph);
        }
    }

    /// Removes nodes that are not reachable from entry points
    fn remove_unreachable_nodes(&mut self, graph: &mut ExecutionGraph) -> bool {
        let reachable = self.find_reachable_nodes(graph);
        let mut removed = false;

        // Create a mapping from old indices to new indices
        let mut node_mapping = vec![None; graph.node_count as usize];
        let mut new_nodes = Vec::new();

        for (i, node) in graph.nodes.iter().enumerate() {
            if reachable.contains(&(i as u32)) {
                node_mapping[i] = Some(new_nodes.len() as u32);
                new_nodes.push(node.clone());
            } else {
                removed = true;
            }
        }

        if removed {
            // Update edges to use new indices
            let mut new_edges = Vec::new();
            for edge in &graph.edges {
                if let (Some(new_from), Some(new_to)) = (
                    node_mapping[edge.from_node as usize],
                    node_mapping[edge.to_node as usize],
                ) {
                    new_edges.push(GraphEdge {
                        from_node: new_from,
                        to_node: new_to,
                        ..edge.clone()
                    });
                }
            }

            // Update entry points
            for entry_point in &mut graph.entry_points {
                if let Some(new_id) = node_mapping[entry_point.node_id as usize] {
                    entry_point.node_id = new_id;
                }
            }

            // Update graph with optimized structures
            graph.nodes = new_nodes;
            graph.edges = new_edges;
            graph.node_count = graph.nodes.len() as u32;
            graph.edge_count = graph.edges.len() as u32;
        }

        removed
    }

    /// Removes redundant edges (e.g., duplicate edges between same nodes)
    fn remove_redundant_edges(&mut self, graph: &mut ExecutionGraph) -> bool {
        let mut removed = false;
        let mut unique_edges = std::collections::HashSet::new();
        let mut new_edges = Vec::new();

        for edge in &graph.edges {
            let edge_key = (edge.from_node, edge.to_node, edge.edge_type.clone());
            if !unique_edges.contains(&edge_key) {
                unique_edges.insert(edge_key);
                new_edges.push(edge.clone());
            } else {
                removed = true; // Duplicate edge found and skipped
            }
        }

        if removed {
            graph.edges = new_edges;
            graph.edge_count = graph.edges.len() as u32;
        }

        removed
    }

    /// Combines similar operations that can be merged
    fn combine_similar_operations(&mut self, graph: &mut ExecutionGraph) -> bool {
        // This is a basic implementation - in a real system, we would have more
        // sophisticated optimization techniques
        let mut combined = false;

        // Look for consecutive LOAD operations that can be combined
        for i in 0..graph.nodes.len() {
            if let GraphNodeType::Op = graph.nodes[i].get_base().node_type {
                // Check if this is a simple operation that could be combined
                // with the next one (in a more complex implementation)
            }
        }

        combined
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kern_parser::Parser;

    #[test]
    fn test_graph_builder() {
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

        // Verify that the graph has the expected structure
        assert!(!graph.nodes.is_empty());
        assert!(!graph.edges.is_empty());
        assert!(!graph.entry_points.is_empty());

        println!(
            "Generated execution graph with {} nodes and {} edges",
            graph.nodes.len(),
            graph.edges.len()
        );
    }
}
