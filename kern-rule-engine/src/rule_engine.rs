use std::collections::HashMap;
use crate::{ResolutionMode, ConflictEntry, RuleMatch};
use kern_graph_builder::{ExecutionGraph, GraphNode, GraphNodeType};

// Rule execution metadata
#[derive(Debug, Clone)]
pub struct RuleExecutionInfo {
    pub rule_id: u32,
    pub priority: u16,
    pub condition_graph_id: Option<u32>,
    pub action_graph_id: Option<u32>,
    pub dependencies: Vec<u32>,
    pub recursion_limit: u32,
    pub execution_count: u32,  // runtime tracker
}

impl RuleExecutionInfo {
    pub fn new(rule_id: u32) -> Self {
        RuleExecutionInfo {
            rule_id,
            priority: 10,  // Default normal priority
            condition_graph_id: None,
            action_graph_id: None,
            dependencies: Vec::new(),
            recursion_limit: 10,  // Default recursion limit
            execution_count: 0,
        }
    }
}

// Rule engine main structure
pub struct RuleEngine {
    pub matched_rules_queue: Vec<RuleExecutionInfo>,
    pub active_rule_stack: Vec<u32>,
    pub conflict_table: Vec<ConflictEntry>,
    pub execution_graph: ExecutionGraph,
    pub program_state: HashMap<String, String>,  // SymbolTable + Scope
    pub rule_registry: HashMap<u32, RuleExecutionInfo>,
    pub max_steps: u32,
    pub step_count: u32,
}

impl RuleEngine {
    pub fn new(execution_graph: ExecutionGraph) -> Self {
        RuleEngine {
            matched_rules_queue: Vec::new(),
            active_rule_stack: Vec::new(),
            conflict_table: Vec::new(),
            execution_graph,
            program_state: HashMap::new(),
            rule_registry: HashMap::new(),
            max_steps: 10000,
            step_count: 0,
        }
    }

    /// Evaluates all rules in the execution graph and returns applicable ones
    pub fn evaluate_rules(&mut self) -> Result<Vec<RuleExecutionInfo>, String> {
        let mut matched_rules = Vec::new();

        // Process each rule in the execution graph
        for node in &self.execution_graph.nodes {
            if let GraphNodeType::Rule = node.node_type {
                let rule_id = node.id;
                
                // Create or get rule execution info
                let mut rule_info = self.rule_registry
                    .entry(rule_id)
                    .or_insert_with(|| RuleExecutionInfo::new(rule_id))
                    .clone();

                // Evaluate the rule's condition
                if self.evaluate_condition(&rule_info)? {
                    // Check dependencies
                    if self.check_dependencies(&rule_info)? {
                        matched_rules.push(rule_info);
                    }
                }
            }
        }

        // Sort by priority (descending) and then by rule_id (for deterministic tie-break)
        matched_rules.sort_by(|a, b| {
            b.priority.cmp(&a.priority)  // Higher priority first
                .then_with(|| a.rule_id.cmp(&b.rule_id))  // Then by rule_id for stability
        });

        Ok(matched_rules)
    }

    /// Evaluates a rule's condition against the current program state
    fn evaluate_condition(&self, rule_info: &RuleExecutionInfo) -> Result<bool, String> {
        // In a real implementation, this would evaluate the condition subgraph
        // For now, we'll return true to indicate the condition is satisfied
        Ok(true)
    }

    /// Checks if all dependencies for a rule are satisfied
    fn check_dependencies(&self, rule_info: &RuleExecutionInfo) -> Result<bool, String> {
        for &dep_id in &rule_info.dependencies {
            // Check if the dependency has been executed
            // In a real implementation, this would check the actual dependency status
            if !self.is_dependency_satisfied(dep_id) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Checks if a specific dependency is satisfied
    fn is_dependency_satisfied(&self, dep_id: u32) -> bool {
        // In a real implementation, this would check if the dependency rule has executed
        // For now, we'll assume all dependencies are satisfied
        true
    }

    /// Detects conflicts between rules
    pub fn detect_conflicts(&self, rules: &[RuleExecutionInfo]) -> Vec<ConflictEntry> {
        let mut conflicts = Vec::new();

        // Compare each pair of rules for potential conflicts
        for i in 0..rules.len() {
            for j in (i + 1)..rules.len() {
                let rule1 = &rules[i];
                let rule2 = &rules[j];

                // Check for conflicts (simplified for this example)
                if self.rules_conflict(rule1, rule2) {
                    conflicts.push(ConflictEntry {
                        target_symbol_id: 0, // Placeholder
                        conflicting_rules: vec![rule1.rule_id, rule2.rule_id],
                        resolution_mode: ResolutionMode::Override,
                    });
                }
            }
        }

        conflicts
    }

    /// Checks if two rules conflict with each other
    fn rules_conflict(&self, rule1: &RuleExecutionInfo, rule2: &RuleExecutionInfo) -> bool {
        // In a real implementation, this would check for actual conflicts
        // For now, we'll return false to indicate no conflicts
        false
    }

    /// Executes a single rule
    pub fn execute_rule(&mut self, rule_info: &mut RuleExecutionInfo) -> Result<(), String> {
        // Check recursion limit
        if rule_info.execution_count >= rule_info.recursion_limit {
            return Err(format!("Recursion limit exceeded for rule {}", rule_info.rule_id));
        }

        // Add to active rule stack to prevent recursion
        if self.active_rule_stack.contains(&rule_info.rule_id) {
            return Err(format!("Recursive rule execution detected for rule {}", rule_info.rule_id));
        }
        self.active_rule_stack.push(rule_info.rule_id);

        // Execute the rule's action subgraph
        self.execute_action_subgraph(rule_info)?;

        // Update execution count
        rule_info.execution_count += 1;

        // Remove from active stack
        if let Some(pos) = self.active_rule_stack.iter().rposition(|&x| x == rule_info.rule_id) {
            self.active_rule_stack.remove(pos);
        }

        Ok(())
    }

    /// Executes the action subgraph for a rule
    fn execute_action_subgraph(&mut self, rule_info: &RuleExecutionInfo) -> Result<(), String> {
        // In a real implementation, this would execute the action subgraph
        // For now, we'll just update the program state with a dummy value
        self.program_state.insert(
            format!("rule_{}_executed", rule_info.rule_id),
            "true".to_string(),
        );

        Ok(())
    }

    /// Runs the complete rule execution cycle
    pub fn run_cycle(&mut self) -> Result<(), String> {
        // Evaluate rules to find applicable ones
        let mut matched_rules = self.evaluate_rules()?;

        // Detect conflicts
        let conflicts = self.detect_conflicts(&matched_rules);
        self.conflict_table = conflicts;

        // Execute rules in priority order
        for mut rule_info in matched_rules {
            // Resolve conflicts if any exist
            if self.has_conflicts(&rule_info) {
                self.resolve_conflicts(&mut rule_info)?;
            }

            // Execute the rule
            match self.execute_rule(&mut rule_info) {
                Ok(()) => {
                    // Update the registry with the new execution count
                    self.rule_registry.insert(rule_info.rule_id, rule_info);
                },
                Err(e) => {
                    eprintln!("Error executing rule {}: {}", rule_info.rule_id, e);
                }
            }

            self.step_count += 1;
            if self.step_count >= self.max_steps {
                return Err("Maximum execution steps exceeded".to_string());
            }
        }

        Ok(())
    }

    /// Checks if a rule has conflicts
    fn has_conflicts(&self, rule_info: &RuleExecutionInfo) -> bool {
        self.conflict_table.iter().any(|conflict| {
            conflict.conflicting_rules.contains(&rule_info.rule_id)
        })
    }

    /// Resolves conflicts for a rule
    fn resolve_conflicts(&mut self, rule_info: &mut RuleExecutionInfo) -> Result<(), String> {
        // In a real implementation, this would resolve actual conflicts
        // For now, we'll just return Ok
        Ok(())
    }
}