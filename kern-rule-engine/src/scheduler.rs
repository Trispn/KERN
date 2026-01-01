use std::collections::{HashMap, VecDeque};
use crate::rule_engine::{RuleExecutionInfo, RuleEngine};

#[derive(Debug, Clone)]
pub struct RuleQueueEntry {
    pub rule_info: RuleExecutionInfo,
    pub execution_order: u32,  // For deterministic tie-breaking
}

pub struct RuleScheduler {
    pub execution_queue: VecDeque<RuleQueueEntry>,
    pub scheduled_rules: HashMap<u32, RuleExecutionInfo>,
    pub execution_order_counter: u32,
}

impl RuleScheduler {
    pub fn new() -> Self {
        RuleScheduler {
            execution_queue: VecDeque::new(),
            scheduled_rules: HashMap::new(),
            execution_order_counter: 0,
        }
    }

    /// Schedules a rule for execution based on priority and dependencies
    pub fn schedule_rule(&mut self, mut rule_info: RuleExecutionInfo, rule_engine: &RuleEngine) -> Result<bool, String> {
        // Check if all dependencies are satisfied
        if !self.check_dependencies(&rule_info, rule_engine)? {
            return Ok(false); // Dependencies not satisfied, don't schedule
        }

        // Check recursion limit
        if rule_info.execution_count >= rule_info.recursion_limit {
            return Err(format!("Recursion limit exceeded for rule {}", rule_info.rule_id));
        }

        // Add to execution queue
        let queue_entry = RuleQueueEntry {
            rule_info,
            execution_order: self.execution_order_counter,
        };
        self.execution_queue.push_back(queue_entry);
        self.execution_order_counter += 1;

        Ok(true)
    }

    /// Checks if all dependencies for a rule are satisfied
    fn check_dependencies(&self, rule_info: &RuleExecutionInfo, rule_engine: &RuleEngine) -> Result<bool, String> {
        for &dep_id in &rule_info.dependencies {
            // Check if the dependency has been executed
            if !self.is_dependency_satisfied(dep_id, rule_engine) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Checks if a specific dependency is satisfied
    fn is_dependency_satisfied(&self, dep_id: u32, rule_engine: &RuleEngine) -> bool {
        // Check if the dependency rule has already been executed in this cycle
        rule_engine.program_state.contains_key(&format!("rule_{}_executed", dep_id))
    }

    /// Schedules multiple rules with priority sorting
    pub fn schedule_rules(&mut self, mut rules: Vec<RuleExecutionInfo>, rule_engine: &RuleEngine) -> Result<(), String> {
        // Sort by priority (descending) and then by rule_id (for deterministic tie-break)
        rules.sort_by(|a, b| {
            b.priority.cmp(&a.priority)  // Higher priority first
                .then_with(|| a.rule_id.cmp(&b.rule_id))  // Then by rule_id for stability
        });

        for mut rule_info in rules {
            // Check recursion limit
            if rule_info.execution_count >= rule_info.recursion_limit {
                continue; // Skip this rule if recursion limit is reached
            }

            // Schedule the rule
            match self.schedule_rule(rule_info, rule_engine) {
                Ok(scheduled) => {
                    if !scheduled {
                        // Dependencies not satisfied, skip this rule
                        continue;
                    }
                },
                Err(e) => {
                    eprintln!("Error scheduling rule: {}", e);
                    continue;
                }
            }
        }

        Ok(())
    }

    /// Executes the next rule in the queue
    pub fn execute_next_rule(&mut self, rule_engine: &mut RuleEngine) -> Result<bool, String> {
        if let Some(queue_entry) = self.execution_queue.pop_front() {
            let mut rule_info = queue_entry.rule_info;
            
            // Execute the rule
            rule_engine.execute_rule(&mut rule_info)?;
            
            // Update the scheduled rules registry
            self.scheduled_rules.insert(rule_info.rule_id, rule_info);
            
            Ok(true) // Rule executed
        } else {
            Ok(false) // No more rules to execute
        }
    }

    /// Executes all scheduled rules
    pub fn execute_all_scheduled(&mut self, rule_engine: &mut RuleEngine) -> Result<(), String> {
        while !self.execution_queue.is_empty() {
            self.execute_next_rule(rule_engine)?;
        }
        Ok(())
    }

    /// Gets the number of scheduled rules
    pub fn scheduled_count(&self) -> usize {
        self.execution_queue.len()
    }

    /// Checks if the scheduler is empty
    pub fn is_empty(&self) -> bool {
        self.execution_queue.is_empty()
    }

    /// Clears the execution queue
    pub fn clear_queue(&mut self) {
        self.execution_queue.clear();
    }

    /// Sorts the queue based on priority and execution order
    pub fn sort_queue(&mut self) {
        // Convert to vector, sort, and convert back to deque
        let mut queue_vec: Vec<RuleQueueEntry> = self.execution_queue.drain(..).collect();
        
        queue_vec.sort_by(|a, b| {
            b.rule_info.priority.cmp(&a.rule_info.priority)  // Higher priority first
                .then_with(|| a.execution_order.cmp(&b.execution_order))  // Then by execution order for stability
        });
        
        for entry in queue_vec {
            self.execution_queue.push_back(entry);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_engine::RuleExecutionInfo;
    use kern_graph_builder::ExecutionGraph;
    
    #[test]
    fn test_scheduler() {
        let graph = ExecutionGraph {
            nodes: vec![],
            edges: vec![],
            node_count: 0,
            edge_count: 0,
            entry_points: vec![],
            entry_count: 0,
            registers: kern_graph_builder::RegisterSet { regs: [kern_graph_builder::Register { reg_type: 0, value_id: 0 }; 16] },
            contexts: kern_graph_builder::ContextPool { contexts: vec![] },
            metadata: kern_graph_builder::GraphMeta { build_hash: 0, version: 0 },
        };
        
        let mut rule_engine = RuleEngine::new(graph);
        let mut scheduler = RuleScheduler::new();
        
        // Create a test rule
        let mut rule_info = RuleExecutionInfo::new(1);
        rule_info.priority = 50;
        
        // Schedule the rule
        let result = scheduler.schedule_rule(rule_info, &rule_engine);
        assert!(result.is_ok());
        assert_eq!(scheduler.scheduled_count(), 1);
        
        // Sort the queue
        scheduler.sort_queue();
        
        // Execute the rule
        let result = scheduler.execute_next_rule(&mut rule_engine);
        assert!(result.is_ok());
        assert_eq!(scheduler.scheduled_count(), 0);
    }
}