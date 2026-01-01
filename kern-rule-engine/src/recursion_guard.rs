use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct RecursionGuard {
    pub active_rules: HashSet<u32>,           // Currently executing rules
    pub execution_counts: HashMap<u32, u32>, // Count of executions per rule
    pub recursion_limits: HashMap<u32, u32>, // Max executions per rule
    pub call_stack: Vec<u32>,               // Track call order for debugging
    pub max_call_depth: u32,                // Maximum allowed call depth
    pub default_recursion_limit: u32,       // Default limit for rules without specific limits
}

#[derive(Debug, Clone)]
pub enum RecursionError {
    LimitExceeded(u32, u32),  // rule_id, current_count
    StackOverflow,            // Call stack too deep
    DirectRecursion(u32),     // Rule calling itself directly
    IndirectRecursion(Vec<u32>), // Rule calling chain that leads back to itself
}

impl RecursionGuard {
    pub fn new() -> Self {
        RecursionGuard {
            active_rules: HashSet::new(),
            execution_counts: HashMap::new(),
            recursion_limits: HashMap::new(),
            call_stack: Vec::new(),
            max_call_depth: 100,  // Default maximum call depth
            default_recursion_limit: 10,  // Default recursion limit
        }
    }

    /// Checks if a rule can be executed without exceeding recursion limits
    pub fn can_execute_rule(&self, rule_id: u32) -> Result<(), RecursionError> {
        // Check if the rule is already active (direct recursion)
        if self.active_rules.contains(&rule_id) {
            return Err(RecursionError::DirectRecursion(rule_id));
        }

        // Check if the call stack is too deep
        if self.call_stack.len() as u32 >= self.max_call_depth {
            return Err(RecursionError::StackOverflow);
        }

        // Check if the execution count exceeds the limit
        let current_count = *self.execution_counts.get(&rule_id).unwrap_or(&0);
        let limit = *self.recursion_limits.get(&rule_id).unwrap_or(&self.default_recursion_limit);

        if current_count >= limit {
            return Err(RecursionError::LimitExceeded(rule_id, current_count));
        }

        Ok(())
    }

    /// Starts tracking execution of a rule
    pub fn start_rule_execution(&mut self, rule_id: u32) -> Result<(), RecursionError> {
        // Check if we can execute this rule
        self.can_execute_rule(rule_id)?;

        // Add to active rules
        self.active_rules.insert(rule_id);

        // Add to call stack
        self.call_stack.push(rule_id);

        // Increment execution count
        let count = self.execution_counts.entry(rule_id).or_insert(0);
        *count += 1;

        Ok(())
    }

    /// Ends tracking execution of a rule
    pub fn end_rule_execution(&mut self, rule_id: u32) {
        // Remove from active rules
        self.active_rules.remove(&rule_id);

        // Remove from call stack (only the most recent occurrence)
        if let Some(pos) = self.call_stack.iter().rposition(|&x| x == rule_id) {
            self.call_stack.remove(pos);
        }
    }

    /// Sets the recursion limit for a specific rule
    pub fn set_recursion_limit(&mut self, rule_id: u32, limit: u32) {
        self.recursion_limits.insert(rule_id, limit);
    }

    /// Gets the recursion limit for a rule
    pub fn get_recursion_limit(&self, rule_id: u32) -> u32 {
        *self.recursion_limits.get(&rule_id).unwrap_or(&self.default_recursion_limit)
    }

    /// Gets the current execution count for a rule
    pub fn get_execution_count(&self, rule_id: u32) -> u32 {
        *self.execution_counts.get(&rule_id).unwrap_or(&0)
    }

    /// Checks for indirect recursion (a calls b, b calls c, ..., z calls a)
    pub fn detect_indirect_recursion(&self, rule_id: u32) -> Option<Vec<u32>> {
        // Check if this rule already appears in the call stack
        let positions: Vec<usize> = self.call_stack.iter()
            .enumerate()
            .filter_map(|(i, &r)| if r == rule_id { Some(i) } else { None })
            .collect();

        if positions.len() > 1 {
            // There's a cycle - return the cycle path
            let first_occurrence = positions[0];
            let cycle: Vec<u32> = self.call_stack[first_occurrence..].to_vec();
            return Some(cycle);
        }

        None
    }

    /// Resets the execution count for a rule
    pub fn reset_rule_count(&mut self, rule_id: u32) {
        self.execution_counts.insert(rule_id, 0);
    }

    /// Resets all execution counts
    pub fn reset_all_counts(&mut self) {
        self.execution_counts.clear();
    }

    /// Sets the default recursion limit
    pub fn set_default_recursion_limit(&mut self, limit: u32) {
        self.default_recursion_limit = limit;
    }

    /// Gets the default recursion limit
    pub fn get_default_recursion_limit(&self) -> u32 {
        self.default_recursion_limit
    }

    /// Sets the maximum call stack depth
    pub fn set_max_call_depth(&mut self, depth: u32) {
        self.max_call_depth = depth;
    }

    /// Gets the current call stack
    pub fn get_call_stack(&self) -> &[u32] {
        &self.call_stack
    }

    /// Checks if any rules are currently executing
    pub fn has_active_rules(&self) -> bool {
        !self.active_rules.is_empty()
    }

    /// Gets all active rules
    pub fn get_active_rules(&self) -> &HashSet<u32> {
        &self.active_rules
    }

    /// Checks if a specific rule is currently executing
    pub fn is_rule_active(&self, rule_id: u32) -> bool {
        self.active_rules.contains(&rule_id)
    }
}

impl Default for RecursionGuard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursion_guard() {
        let mut guard = RecursionGuard::new();
        
        // Test basic execution tracking
        assert!(guard.start_rule_execution(1).is_ok());
        assert!(guard.is_rule_active(1));
        assert_eq!(guard.get_execution_count(1), 1);
        
        guard.end_rule_execution(1);
        assert!(!guard.is_rule_active(1));
        
        // Test recursion limit
        guard.set_recursion_limit(1, 2);
        assert!(guard.start_rule_execution(1).is_ok());
        assert!(guard.start_rule_execution(1).is_ok());
        
        // Third execution should fail
        assert!(matches!(guard.start_rule_execution(1), Err(RecursionError::LimitExceeded(1, 2))));
        
        // Reset and test again
        guard.reset_rule_count(1);
        assert!(guard.start_rule_execution(1).is_ok());
    }

    #[test]
    fn test_direct_recursion_detection() {
        let mut guard = RecursionGuard::new();
        
        // Start a rule
        assert!(guard.start_rule_execution(1).is_ok());
        
        // Try to start the same rule again - should fail
        assert!(matches!(guard.start_rule_execution(1), Err(RecursionError::DirectRecursion(1))));
        
        guard.end_rule_execution(1);
        // Now it should be OK to start again
        assert!(guard.start_rule_execution(1).is_ok());
    }

    #[test]
    fn test_call_stack_depth() {
        let mut guard = RecursionGuard::new();
        guard.set_max_call_depth(3);
        
        // Add 3 rules to the stack
        assert!(guard.start_rule_execution(1).is_ok());
        assert!(guard.start_rule_execution(2).is_ok());
        assert!(guard.start_rule_execution(3).is_ok());
        
        // Adding a 4th should fail
        assert!(matches!(guard.start_rule_execution(4), Err(RecursionError::StackOverflow)));
        
        // Clean up
        guard.end_rule_execution(3);
        guard.end_rule_execution(2);
        guard.end_rule_execution(1);
    }

    #[test]
    fn test_indirect_recursion_detection() {
        let mut guard = RecursionGuard::new();
        
        // Simulate a call chain: 1 -> 2 -> 3 -> 1 (indirect recursion)
        assert!(guard.start_rule_execution(1).is_ok());
        assert!(guard.start_rule_execution(2).is_ok());
        assert!(guard.start_rule_execution(3).is_ok());
        
        // Check for indirect recursion
        let cycle = guard.detect_indirect_recursion(1);
        assert!(cycle.is_some());
        assert_eq!(cycle.unwrap(), vec![1, 2, 3, 1]);
        
        // Clean up
        guard.end_rule_execution(3);
        guard.end_rule_execution(2);
        guard.end_rule_execution(1);
    }
}