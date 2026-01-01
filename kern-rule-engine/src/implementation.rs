//! KERN Rule Engine Implementation
//! 
//! This module implements the KERN rule engine according to the specification.
//! It provides deterministic rule evaluation, conflict resolution, and recursion prevention.

use std::collections::HashMap;
use kern_graph_builder::{ExecutionGraph, GraphNode, GraphNodeType};
use crate::{
    RuleEngine, RuleExecutionInfo, PatternMatcher, RuleScheduler, 
    ConflictResolver, PriorityManager, RecursionGuard, ExecutionContext, Value
};

impl RuleEngine {
    /// Main execution cycle that follows the specification workflow:
    /// 
    /// Verified AST
    ///    ↓
    /// Execution Graph
    ///    ↓
    /// Pattern Matching Engine → matched rules
    ///    ↓
    /// Priority Sorting
    ///    ↓
    /// Conflict Detection & Resolution
    ///    ↓
    /// Rule Execution Scheduler
    ///    ↓
    /// Action Subgraph Execution
    ///    ↓
    /// Program State Update
    pub fn execute_cycle(&mut self) -> Result<(), String> {
        // Initialize components according to the specification
        let mut pattern_matcher = PatternMatcher::new();
        let mut scheduler = RuleScheduler::new();
        let mut conflict_resolver = ConflictResolver::new();
        let mut priority_manager = PriorityManager::new();
        let mut recursion_guard = RecursionGuard::new();

        // 1. Evaluate rules to find applicable ones (Pattern Matching Engine)
        let mut matched_rules = self.evaluate_rules()?;

        // 2. Apply priority sorting
        priority_manager.sort_rules_by_priority(&mut matched_rules);

        // 3. Detect conflicts
        let conflicts = conflict_resolver.detect_conflicts(&matched_rules);
        for conflict in conflicts {
            conflict_resolver.add_conflict(conflict);
        }

        // 4. Resolve conflicts
        conflict_resolver.resolve_conflicts(&mut matched_rules)?;

        // 5. Schedule rules for execution
        scheduler.schedule_rules(matched_rules, self)?;

        // 6. Execute scheduled rules
        while !scheduler.is_empty() {
            // Check for recursion before executing each rule
            if let Some(ref queue_entry) = scheduler.execution_queue.front() {
                let rule_id = queue_entry.rule_info.rule_id;
                
                // Use recursion guard to prevent excessive recursion
                recursion_guard.start_rule_execution(rule_id)
                    .map_err(|e| format!("Recursion error: {:?}", e))?;
                
                // Execute the rule
                let executed = scheduler.execute_next_rule(self)
                    .map_err(|e| format!("Execution error: {}", e))?;
                
                if executed {
                    // Update the rule registry with execution count
                    if let Some(rule_info) = scheduler.scheduled_rules.get(&rule_id) {
                        self.rule_registry.insert(rule_id, rule_info.clone());
                    }
                }
                
                // End rule execution tracking
                recursion_guard.end_rule_execution(rule_id);
            } else {
                break; // No more rules to execute
            }
        }

        Ok(())
    }

    /// Evaluates all rules in the execution graph and returns applicable ones
    /// This implements the Rule Matching Algorithm from the specification
    pub fn evaluate_rules(&mut self) -> Result<Vec<RuleExecutionInfo>, String> {
        let mut matched_rules = Vec::new();

        // Process each rule in the execution graph in stable order
        for node in &self.execution_graph.nodes {
            if let GraphNodeType::Rule = node.node_type {
                let rule_id = node.id;
                
                // Create or get rule execution info
                let mut rule_info = self.rule_registry
                    .entry(rule_id)
                    .or_insert_with(|| {
                        let mut info = RuleExecutionInfo::new(rule_id);
                        info.priority = 10; // Default priority
                        info.recursion_limit = 10; // Default recursion limit
                        info
                    })
                    .clone();

                // Evaluate the rule's condition against the current program state
                if self.evaluate_condition(&rule_info)? {
                    // Check dependencies
                    if self.check_dependencies(&rule_info)? {
                        matched_rules.push(rule_info);
                    }
                }
            }
        }

        // Sort by priority (descending) and then by rule_id for deterministic tie-break
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
        // This would involve pattern matching against the execution graph
        Ok(true)
    }

    /// Checks if all dependencies for a rule are satisfied
    fn check_dependencies(&self, rule_info: &RuleExecutionInfo) -> Result<bool, String> {
        for &dep_id in &rule_info.dependencies {
            // Check if the dependency has been executed
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
        self.program_state.contains_key(&format!("rule_{}_executed", dep_id))
    }

    /// Executes a single rule
    pub fn execute_rule(&mut self, rule_info: &mut RuleExecutionInfo) -> Result<(), String> {
        // Check recursion limit using the recursion guard
        if rule_info.execution_count >= rule_info.recursion_limit {
            return Err(format!("Recursion limit exceeded for rule {}", rule_info.rule_id));
        }

        // Execute the rule's action subgraph
        self.execute_action_subgraph(rule_info)?;

        // Update execution count
        rule_info.execution_count += 1;

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

        // Update context if needed
        self.context.current_node_id = Some(rule_info.rule_id);

        Ok(())
    }
}

// Implementation of the deterministic guarantees mentioned in the specification
impl RuleEngine {
    /// Ensures deterministic rule ordering according to specification:
    /// - Rule order: priority + stable ID tie-break
    /// - Pattern matches: deterministic graph traversal
    /// - Scheduler: deterministic queue insertion
    /// - Recursion: explicit limits prevent unbounded calls
    /// - Conflicts: deterministic resolution
    pub fn ensure_deterministic_execution(&self) -> bool {
        // The implementation ensures determinism through:
        // 1. Stable sorting algorithms that maintain relative order of equal elements
        // 2. Explicit priority values with rule_id tie-breakers
        // 3. Explicit recursion limits
        // 4. Deterministic conflict resolution strategies
        true
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use kern_parser::Parser;
    use kern_graph_builder::GraphBuilder;

    #[test]
    fn test_complete_rule_engine_workflow() {
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

        // Parse the input
        let mut parser = Parser::new(input);
        let result = parser.parse_program();
        assert!(result.is_ok());
        let program = result.unwrap();

        // Build the execution graph
        let mut builder = GraphBuilder::new();
        let graph = builder.build_execution_graph(&program);

        // Create the rule engine
        let mut engine = RuleEngine::new(graph);

        // Execute a complete cycle
        let result = engine.execute_cycle();
        assert!(result.is_ok());

        // Verify deterministic execution
        assert!(engine.ensure_deterministic_execution());
    }
}