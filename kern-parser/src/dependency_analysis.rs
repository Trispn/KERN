use crate::ast::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct Dependency {
    pub from: String,  // The entity/rule/flow that depends on something
    pub to: String,    // The entity/rule/flow that is being depended on
    pub kind: DependencyKind,
}

#[derive(Debug, Clone)]
pub enum DependencyKind {
    UsesEntity,
    UsesField,
    CallsRule,
    CallsFlow,
    CallsPredicate,
    ConditionRef,
    ActionRef,
}

#[derive(Debug)]
pub struct DependencyGraph {
    // Dependencies from each node to the nodes it depends on
    dependencies: HashMap<String, Vec<Dependency>>,
    // Reverse dependencies (what depends on each node)
    dependents: HashMap<String, Vec<Dependency>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        DependencyGraph {
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, dep: Dependency) {
        // Add to forward dependencies
        self.dependencies
            .entry(dep.from.clone())
            .or_insert_with(Vec::new)
            .push(dep.clone());
        
        // Add to reverse dependencies
        self.dependents
            .entry(dep.to.clone())
            .or_insert_with(Vec::new)
            .push(dep);
    }

    pub fn get_dependencies(&self, node: &str) -> Option<&Vec<Dependency>> {
        self.dependencies.get(node)
    }

    pub fn get_dependents(&self, node: &str) -> Option<&Vec<Dependency>> {
        self.dependents.get(node)
    }

    pub fn has_cycle(&self) -> bool {
        // Check for cycles in the dependency graph using DFS
        let all_nodes: HashSet<&String> = self.dependencies.keys().chain(self.dependents.keys()).collect();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node in all_nodes {
            if !visited.contains(node) {
                if self.has_cycle_dfs(node, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }

        false
    }

    fn has_cycle_dfs(&self, node: &str, visited: &mut HashSet<&String>, rec_stack: &mut HashSet<&String>) -> bool {
        visited.insert(node);
        rec_stack.insert(node);

        if let Some(dependencies) = self.dependencies.get(node) {
            for dep in dependencies {
                if !visited.contains(&dep.to) && self.has_cycle_dfs(&dep.to, visited, rec_stack) {
                    return true;
                } else if rec_stack.contains(&dep.to) {
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        false
    }

    pub fn get_topological_order(&self) -> Result<Vec<String>, String> {
        // Perform topological sort to determine the order of processing
        let all_nodes: HashSet<String> = self.dependencies.keys()
            .chain(self.dependents.keys())
            .map(|s| s.clone())
            .collect();
        
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        
        // Initialize in-degree for all nodes
        for node in &all_nodes {
            in_degree.insert(node.clone(), 0);
        }
        
        // Calculate in-degrees
        for deps in self.dependencies.values() {
            for dep in deps {
                *in_degree.get_mut(&dep.to).unwrap() += 1;
            }
        }
        
        // Find nodes with in-degree 0
        let mut queue: VecDeque<String> = VecDeque::new();
        for (node, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(node.clone());
            }
        }
        
        let mut topological_order = Vec::new();
        
        while let Some(node) = queue.pop_front() {
            topological_order.push(node.clone());
            
            // Reduce in-degree for all dependents of this node
            if let Some(dependents) = self.dependents.get(&node) {
                for dep in dependents {
                    let current_degree = in_degree.get_mut(&dep.from).unwrap();
                    *current_degree -= 1;
                    
                    if *current_degree == 0 {
                        queue.push_back(dep.from.clone());
                    }
                }
            }
        }
        
        // If topological sort includes all nodes, there's no cycle
        if topological_order.len() == all_nodes.len() {
            Ok(topological_order)
        } else {
            Err("Cycle detected in dependency graph".to_string())
        }
    }
}

pub struct DependencyAnalyzer {
    graph: DependencyGraph,
    errors: Vec<String>,
}

impl DependencyAnalyzer {
    pub fn new() -> Self {
        DependencyAnalyzer {
            graph: DependencyGraph::new(),
            errors: Vec::new(),
        }
    }

    pub fn analyze_program(&mut self, program: &Program) -> Result<&DependencyGraph, Vec<String>> {
        // Analyze each definition for dependencies
        for def in &program.definitions {
            self.analyze_definition(def);
        }

        if self.errors.is_empty() {
            Ok(&self.graph)
        } else {
            Err(self.errors.clone())
        }
    }

    fn analyze_definition(&mut self, def: &Definition) {
        match def {
            Definition::Entity(entity_def) => self.analyze_entity_def(entity_def),
            Definition::Rule(rule_def) => self.analyze_rule_def(rule_def),
            Definition::Flow(flow_def) => self.analyze_flow_def(flow_def),
            Definition::Constraint(constraint_def) => self.analyze_constraint_def(constraint_def),
        }
    }

    fn analyze_entity_def(&mut self, entity_def: &EntityDef) {
        // Entities may depend on other entities if they have fields of entity types
        // For now, we'll just register the entity itself
        // In a more complex system, we might track field type dependencies
    }

    fn analyze_rule_def(&mut self, rule_def: &RuleDef) {
        // Analyze condition for dependencies
        self.analyze_condition(&rule_def.condition, &rule_def.name, DependencyKind::ConditionRef);
        
        // Analyze actions for dependencies
        for action in &rule_def.actions {
            self.analyze_action(action, &rule_def.name, DependencyKind::ActionRef);
        }
    }

    fn analyze_flow_def(&mut self, flow_def: &FlowDef) {
        // Analyze actions in the flow for dependencies
        for action in &flow_def.actions {
            self.analyze_action(action, &flow_def.name, DependencyKind::ActionRef);
        }
    }

    fn analyze_constraint_def(&mut self, constraint_def: &ConstraintDef) {
        // Analyze condition for dependencies
        self.analyze_condition(&constraint_def.condition, &constraint_def.name, DependencyKind::ConditionRef);
    }

    fn analyze_condition(&mut self, condition: &Condition, parent_name: &str, context: DependencyKind) {
        match condition {
            Condition::Expression(expr) => self.analyze_expression(expr, parent_name, context),
            Condition::LogicalOp(left, _, right) => {
                self.analyze_condition(left, parent_name, context.clone());
                self.analyze_condition(right, parent_name, context);
            }
        }
    }

    fn analyze_expression(&mut self, expression: &Expression, parent_name: &str, context: DependencyKind) {
        match expression {
            Expression::Comparison { left, right, .. } => {
                self.analyze_term(left, parent_name, context.clone());
                self.analyze_term(right, parent_name, context);
            },
            Expression::Predicate(predicate) => {
                // Add dependency on the predicate
                self.graph.add_dependency(Dependency {
                    from: parent_name.to_string(),
                    to: predicate.name.clone(),
                    kind: DependencyKind::CallsPredicate,
                });
                
                // Analyze predicate arguments
                for arg in &predicate.arguments {
                    self.analyze_term(arg, parent_name, context.clone());
                }
            }
        }
    }

    fn analyze_term(&mut self, term: &Term, parent_name: &str, context: DependencyKind) {
        match term {
            Term::Identifier(name) => {
                // Add dependency on the identifier
                self.graph.add_dependency(Dependency {
                    from: parent_name.to_string(),
                    to: name.clone(),
                    kind: context,
                });
            },
            Term::Number(_) => {
                // Numbers don't create dependencies
            },
            Term::QualifiedRef(entity_name, field_name) => {
                // Add dependency on the entity
                self.graph.add_dependency(Dependency {
                    from: parent_name.to_string(),
                    to: entity_name.clone(),
                    kind: DependencyKind::UsesEntity,
                });
                
                // Add dependency on the field
                self.graph.add_dependency(Dependency {
                    from: parent_name.to_string(),
                    to: format!("{}.{}", entity_name, field_name),
                    kind: DependencyKind::UsesField,
                });
            }
        }
    }

    fn analyze_action(&mut self, action: &Action, parent_name: &str, context: DependencyKind) {
        match action {
            Action::Predicate(predicate) => {
                // Add dependency on the predicate
                self.graph.add_dependency(Dependency {
                    from: parent_name.to_string(),
                    to: predicate.name.clone(),
                    kind: DependencyKind::CallsPredicate,
                });
                
                // Analyze predicate arguments
                for arg in &predicate.arguments {
                    self.analyze_term(arg, parent_name, context.clone());
                }
            },
            Action::Assignment(assignment) => {
                // Analyze the value being assigned
                self.analyze_term(&assignment.value, parent_name, context.clone());
            },
            Action::Control(control_action) => {
                self.analyze_control_action(control_action, parent_name, context);
            }
        }
    }

    fn analyze_control_action(&mut self, control_action: &ControlAction, parent_name: &str, context: DependencyKind) {
        match control_action {
            ControlAction::If(if_action) => {
                // Analyze the condition
                self.analyze_condition(&if_action.condition, parent_name, context.clone());
                
                // Analyze then actions
                for action in &if_action.then_actions {
                    self.analyze_action(action, parent_name, context.clone());
                }
                
                // Analyze else actions if present
                if let Some(else_actions) = &if_action.else_actions {
                    for action in else_actions {
                        self.analyze_action(action, parent_name, context.clone());
                    }
                }
            },
            ControlAction::Loop(loop_action) => {
                // Analyze loop actions
                for action in &loop_action.actions {
                    self.analyze_action(action, parent_name, context.clone());
                }
            },
            ControlAction::Halt(_) => {
                // Halt action doesn't create dependencies
            }
        }
    }

    pub fn get_graph(&self) -> &DependencyGraph {
        &self.graph
    }

    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_analyzer_basic() {
        let mut analyzer = DependencyAnalyzer::new();
        
        // Create a simple program with dependencies
        let program = Program {
            definitions: vec![
                Definition::Entity(EntityDef {
                    name: "Farmer".to_string(),
                    fields: vec![
                        FieldDef { name: "id".to_string() },
                        FieldDef { name: "location".to_string() },
                    ],
                }),
                Definition::Rule(RuleDef {
                    name: "CheckFarmer".to_string(),
                    condition: Condition::Expression(Expression::Comparison {
                        left: Box::new(Term::QualifiedRef("Farmer".to_string(), "id".to_string())),
                        op: Comparator::Greater,
                        right: Box::new(Term::Number(0)),
                    }),
                    actions: vec![
                        Action::Predicate(Predicate {
                            name: "validate_farmer".to_string(),
                            arguments: vec![Term::Identifier("Farmer".to_string())],
                        })
                    ],
                })
            ]
        };

        let result = analyzer.analyze_program(&program);
        assert!(result.is_ok());
        
        let graph = result.unwrap();
        
        // Check that there's a dependency from CheckFarmer to Farmer
        let farmer_deps = graph.get_dependents("Farmer");
        assert!(farmer_deps.is_some());
        assert!(!farmer_deps.unwrap().is_empty());
        
        // Check that there's a dependency from CheckFarmer to validate_farmer
        let predicate_deps = graph.get_dependents("validate_farmer");
        assert!(predicate_deps.is_some());
        assert!(!predicate_deps.unwrap().is_empty());
    }

    #[test]
    fn test_dependency_cycle_detection() {
        let mut analyzer = DependencyAnalyzer::new();
        
        // Create a program with a cycle
        // This is a simplified example - in practice, cycles would be more complex
        let program = Program {
            definitions: vec![
                Definition::Rule(RuleDef {
                    name: "RuleA".to_string(),
                    condition: Condition::Expression(Expression::Predicate(Predicate {
                        name: "RuleB".to_string(),
                        arguments: vec![],
                    })),
                    actions: vec![],
                }),
                Definition::Rule(RuleDef {
                    name: "RuleB".to_string(),
                    condition: Condition::Expression(Expression::Predicate(Predicate {
                        name: "RuleA".to_string(),
                        arguments: vec![],
                    })),
                    actions: vec![],
                })
            ]
        };

        let result = analyzer.analyze_program(&program);
        assert!(result.is_ok());
        
        // The dependency graph should detect the cycle
        assert!(analyzer.graph.has_cycle());
    }
}