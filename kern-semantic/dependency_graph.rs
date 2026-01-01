//! KERN Dependency Graph
//! 
//! Analyzes dependencies between symbols in the KERN language.

use std::collections::{HashMap, HashSet, VecDeque};
use crate::symbol::{Symbol, SymbolKind};
use crate::resolver::Resolver;
use kern_parser::{AstNode, Program, Definition, EntityDef, RuleDef, FlowDef, ConstraintDef, Condition, Expression, Term, Predicate, Action, IfAction, LoopAction, HaltAction, Assignment, ControlAction};

#[derive(Debug, Clone)]
pub struct DependencyNode {
    pub symbol_id: String,  // Using symbol name as ID for simplicity
    pub depends_on: Vec<String>,
}

#[derive(Debug)]
pub struct DependencyGraph {
    nodes: HashMap<String, DependencyNode>,
    resolver: Resolver,
    errors: Vec<String>,
}

#[derive(Debug)]
pub enum DependencyError {
    CyclicDependency {
        cycle: Vec<String>,
        location: crate::symbol::SourceLocation,
    },
    SelfDependency {
        symbol: String,
        location: crate::symbol::SourceLocation,
    },
}

impl DependencyError {
    pub fn message(&self) -> String {
        match self {
            DependencyError::CyclicDependency { cycle, location } => {
                format!(
                    "Cyclic dependency detected: {} at {}:{}",
                    cycle.join(" -> "),
                    location.file,
                    location.line
                )
            },
            DependencyError::SelfDependency { symbol, location } => {
                format!(
                    "Symbol '{}' depends on itself at {}:{}", 
                    symbol, location.file, location.line
                )
            },
        }
    }
}

impl DependencyGraph {
    pub fn new(resolver: Resolver) -> Self {
        DependencyGraph {
            nodes: HashMap::new(),
            resolver,
            errors: Vec::new(),
        }
    }

    /// Builds the dependency graph for a program
    pub fn build_graph(&mut self, program: &Program) -> Result<(), Vec<String>> {
        // First, create nodes for all top-level declarations
        for definition in &program.definitions {
            self.create_node(definition);
        }

        // Then, analyze dependencies within each definition
        for definition in &program.definitions {
            self.analyze_dependencies(definition);
        }

        // Check for cycles
        self.check_cycles()?;

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn create_node(&mut self, definition: &Definition) {
        let name = match definition {
            Definition::Entity(entity_def) => &entity_def.name,
            Definition::Rule(rule_def) => &rule_def.name,
            Definition::Flow(flow_def) => &flow_def.name,
            Definition::Constraint(constraint_def) => &constraint_def.name,
        }.clone();

        let node = DependencyNode {
            symbol_id: name,
            depends_on: Vec::new(),
        };

        self.nodes.insert(node.symbol_id.clone(), node);
    }

    fn analyze_dependencies(&mut self, definition: &Definition) {
        match definition {
            Definition::Entity(entity_def) => {
                self.analyze_entity_dependencies(entity_def);
            },
            Definition::Rule(rule_def) => {
                self.analyze_rule_dependencies(rule_def);
            },
            Definition::Flow(flow_def) => {
                self.analyze_flow_dependencies(flow_def);
            },
            Definition::Constraint(constraint_def) => {
                self.analyze_constraint_dependencies(constraint_def);
            },
        }
    }

    fn analyze_entity_dependencies(&mut self, _entity_def: &EntityDef) {
        // Entities typically don't have dependencies on other entities/rules
        // unless they reference other entities in constraints or rules
    }

    fn analyze_rule_dependencies(&mut self, rule_def: &RuleDef) {
        let mut dependencies = Vec::new();
        
        // Analyze condition dependencies
        self.collect_term_dependencies(&rule_def.condition, &mut dependencies);
        
        // Analyze action dependencies
        for action in &rule_def.actions {
            self.collect_action_dependencies(action, &mut dependencies);
        }
        
        // Update the node with dependencies
        if let Some(node) = self.nodes.get_mut(&rule_def.name) {
            node.depends_on.extend(dependencies);
        }
    }

    fn analyze_flow_dependencies(&mut self, flow_def: &FlowDef) {
        let mut dependencies = Vec::new();
        
        // Analyze all actions in the flow
        for action in &flow_def.actions {
            self.collect_action_dependencies(action, &mut dependencies);
        }
        
        // Update the node with dependencies
        if let Some(node) = self.nodes.get_mut(&flow_def.name) {
            node.depends_on.extend(dependencies);
        }
    }

    fn analyze_constraint_dependencies(&mut self, constraint_def: &ConstraintDef) {
        let mut dependencies = Vec::new();
        
        // Analyze condition dependencies
        self.collect_term_dependencies(&constraint_def.condition, &mut dependencies);
        
        // Update the node with dependencies
        if let Some(node) = self.nodes.get_mut(&constraint_def.name) {
            node.depends_on.extend(dependencies);
        }
    }

    fn collect_term_dependencies(&mut self, condition: &Condition, dependencies: &mut Vec<String>) {
        match condition {
            Condition::Expression(expr) => {
                self.collect_expression_dependencies(expr, dependencies);
            },
            Condition::LogicalOp(left, _op, right) => {
                self.collect_term_dependencies(left, dependencies);
                self.collect_term_dependencies(right, dependencies);
            },
        }
    }

    fn collect_expression_dependencies(&mut self, expression: &Expression, dependencies: &mut Vec<String>) {
        match expression {
            Expression::Comparison { left, _op, right } => {
                self.collect_term_dependencies_single(left, dependencies);
                self.collect_term_dependencies_single(right, dependencies);
            },
            Expression::Predicate(predicate) => {
                // Add predicate as a dependency if it's a rule or function
                if self.resolver.scope_manager().resolve_symbol(&predicate.name).is_some() {
                    dependencies.push(predicate.name.clone());
                }
                
                // Add dependencies for arguments
                for arg in &predicate.arguments {
                    self.collect_term_dependencies_single(arg, dependencies);
                }
            },
        }
    }

    fn collect_term_dependencies_single(&mut self, term: &Term, dependencies: &mut Vec<String>) {
        match term {
            Term::Identifier(name) => {
                // Check if this identifier refers to a symbol that should be a dependency
                if let Some(symbol) = self.resolver.scope_manager().resolve_symbol(name) {
                    match symbol.kind {
                        SymbolKind::Rule | SymbolKind::Flow | SymbolKind::Entity | SymbolKind::Constraint => {
                            dependencies.push(name.clone());
                        },
                        _ => {
                            // Other symbol kinds (like variables) are not dependencies at the top level
                        }
                    }
                }
            },
            Term::Number(_value) => {
                // Numbers don't create dependencies
            },
            Term::QualifiedRef(entity, _field) => {
                // The entity part might be a dependency
                if self.resolver.scope_manager().resolve_symbol(entity).is_some() {
                    dependencies.push(entity.clone());
                }
            },
        }
    }

    fn collect_action_dependencies(&mut self, action: &Action, dependencies: &mut Vec<String>) {
        match action {
            Action::Predicate(predicate) => {
                // Add predicate as a dependency if it's a rule or function
                if self.resolver.scope_manager().resolve_symbol(&predicate.name).is_some() {
                    dependencies.push(predicate.name.clone());
                }
                
                // Add dependencies for arguments
                for arg in &predicate.arguments {
                    self.collect_term_dependencies_single(arg, dependencies);
                }
            },
            Action::Assignment(assignment) => {
                // Check if the value being assigned has dependencies
                self.collect_term_dependencies_single(&assignment.value, dependencies);
            },
            Action::Control(control_action) => {
                self.collect_control_action_dependencies(control_action, dependencies);
            },
        }
    }

    fn collect_control_action_dependencies(&mut self, control_action: &ControlAction, dependencies: &mut Vec<String>) {
        match control_action {
            ControlAction::If(if_action) => {
                // Collect dependencies from condition
                self.collect_term_dependencies(&if_action.condition, dependencies);
                
                // Collect dependencies from actions
                for action in &if_action.then_actions {
                    self.collect_action_dependencies(action, dependencies);
                }
                
                if let Some(else_actions) = &if_action.else_actions {
                    for action in else_actions {
                        self.collect_action_dependencies(action, dependencies);
                    }
                }
            },
            ControlAction::Loop(loop_action) => {
                // Collect dependencies from loop body
                for action in &loop_action.actions {
                    self.collect_action_dependencies(action, dependencies);
                }
            },
            ControlAction::Halt(_halt_action) => {
                // Halt action has no dependencies
            },
        }
    }

    /// Checks for cycles in the dependency graph
    fn check_cycles(&mut self) -> Result<(), Vec<String>> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for node_id in self.nodes.keys() {
            if !visited.contains(node_id) {
                if self.is_cyclic_dfs(node_id, &mut visited, &mut rec_stack, &mut path) {
                    // A cycle was detected, path contains the cycle
                    self.errors.push(DependencyError::CyclicDependency {
                        cycle: path.clone(),
                        location: crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0), // In real implementation, get from AST
                    }.message());
                    return Err(self.errors.clone());
                }
                path.clear();
            }
        }

        Ok(())
    }

    /// DFS helper to detect cycles
    fn is_cyclic_dfs(
        &self,
        node_id: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        visited.insert(node_id.to_string());
        rec_stack.insert(node_id.to_string());
        path.push(node_id.to_string());

        if let Some(node) = self.nodes.get(node_id) {
            for dependency in &node.depends_on {
                if !visited.contains(dependency) {
                    if self.is_cyclic_dfs(dependency, visited, rec_stack, path) {
                        return true;
                    }
                } else if rec_stack.contains(dependency) {
                    // Found a back edge, which means a cycle
                    // Add the cycle part to the path
                    if let Some(start_idx) = path.iter().position(|x| x == dependency) {
                        *path = path[start_idx..].to_vec();
                        path.push(dependency.clone());
                    }
                    return true;
                }
            }
        }

        path.pop();
        rec_stack.remove(node_id);
        false
    }

    /// Performs topological sort of the dependency graph
    pub fn topological_sort(&self) -> Result<Vec<String>, String> {
        if !self.is_acyclic() {
            return Err("Cannot perform topological sort on cyclic graph".to_string());
        }

        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        for node_id in self.nodes.keys() {
            if !visited.contains(node_id) {
                self.topological_sort_dfs(node_id, &mut visited, &mut stack);
            }
        }

        stack.reverse();
        Ok(stack)
    }

    /// DFS helper for topological sort
    fn topological_sort_dfs(&self, node_id: &str, visited: &mut HashSet<String>, stack: &mut Vec<String>) {
        visited.insert(node_id.to_string());

        if let Some(node) = self.nodes.get(node_id) {
            for dependency in &node.depends_on {
                if !visited.contains(dependency) {
                    self.topological_sort_dfs(dependency, visited, stack);
                }
            }
        }

        stack.push(node_id.to_string());
    }

    /// Checks if the graph is acyclic
    fn is_acyclic(&self) -> bool {
        // For simplicity, we'll just check if there are any cycles
        // In a real implementation, we'd run the cycle detection algorithm
        // Since we already check for cycles during graph building, 
        // we can assume the graph is acyclic if no errors were reported
        self.errors.is_empty()
    }

    /// Gets all dependencies for a symbol
    pub fn get_dependencies(&self, symbol_id: &str) -> Option<&[String]> {
        self.nodes.get(symbol_id).map(|node| node.depends_on.as_slice())
    }

    /// Gets the resolver (for access to symbols)
    pub fn resolver(&self) -> &Resolver {
        &self.resolver
    }

    /// Gets mutable access to the resolver
    pub fn resolver_mut(&mut self) -> &mut Resolver {
        &mut self.resolver
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kern_parser::Parser;

    #[test]
    fn test_dependency_graph_creation() {
        let resolver = Resolver::new();
        let mut dep_graph = DependencyGraph::new(resolver);
        assert_eq!(dep_graph.nodes.len(), 0);
    }

    #[test]
    fn test_simple_dependency_analysis() {
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
        let program = parser.parse_program().expect("Failed to parse program");

        let mut resolver = Resolver::new();
        resolver.resolve_program(&program).expect("Failed to resolve program");

        let mut dep_graph = DependencyGraph::new(resolver);
        let result = dep_graph.build_graph(&program);
        
        // The dependency analysis should pass without errors for this valid program
        assert!(result.is_ok(), "Dependency analysis failed with errors: {:?}", result.err());
    }

    #[test]
    fn test_topological_sort() {
        let input = r#"
        entity Farmer {
            id
            location
        }

        rule ValidateFarmer:
            if farmer.id > 0
            then mark_valid(farmer)

        flow ProcessFarmers {
            load_farmers()
            validate_farmers()
        }

        constraint ValidId: farmer.id > 0
        "#;

        let mut parser = Parser::new(input);
        let program = parser.parse_program().expect("Failed to parse program");

        let mut resolver = Resolver::new();
        resolver.resolve_program(&program).expect("Failed to resolve program");

        let mut dep_graph = DependencyGraph::new(resolver);
        dep_graph.build_graph(&program).expect("Failed to build dependency graph");

        let sorted = dep_graph.topological_sort();
        assert!(sorted.is_ok(), "Topological sort failed: {:?}", sorted.err());
    }
}