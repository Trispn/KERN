use crate::types::RuleExecutionInfo;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PriorityLevel {
    Lowest = 0,
    Normal = 50,
    High = 150,
    Critical = 250,
}

#[derive(Debug, Clone)]
pub struct PriorityManager {
    pub default_priority: u16,
    pub rule_priorities: HashMap<u32, u16>,
    pub priority_limits: PriorityLimits,
}

#[derive(Debug, Clone)]
pub struct PriorityLimits {
    pub min_priority: u16,
    pub max_priority: u16,
}

impl PriorityManager {
    pub fn new() -> Self {
        PriorityManager {
            default_priority: PriorityLevel::Normal as u16,
            rule_priorities: HashMap::new(),
            priority_limits: PriorityLimits {
                min_priority: 0,
                max_priority: 1000,
            },
        }
    }

    /// Sets the priority for a specific rule
    pub fn set_rule_priority(&mut self, rule_id: u32, priority: u16) -> Result<(), String> {
        // Validate priority is within limits
        if priority < self.priority_limits.min_priority
            || priority > self.priority_limits.max_priority
        {
            return Err(format!(
                "Priority {} is outside allowed range ({}-{})",
                priority, self.priority_limits.min_priority, self.priority_limits.max_priority
            ));
        }

        self.rule_priorities.insert(rule_id, priority);
        Ok(())
    }

    /// Gets the priority for a rule, defaulting to default_priority if not set
    pub fn get_rule_priority(&self, rule_id: u32) -> u16 {
        *self
            .rule_priorities
            .get(&rule_id)
            .unwrap_or(&self.default_priority)
    }

    /// Updates multiple rule priorities at once
    pub fn update_rule_priorities(&mut self, priorities: &[(u32, u16)]) -> Result<(), String> {
        for &(rule_id, priority) in priorities {
            self.set_rule_priority(rule_id, priority)?;
        }
        Ok(())
    }

    /// Applies a priority level to a rule
    pub fn set_rule_priority_level(
        &mut self,
        rule_id: u32,
        level: PriorityLevel,
    ) -> Result<(), String> {
        self.set_rule_priority(rule_id, level as u16)
    }

    /// Gets the priority level for a rule
    pub fn get_rule_priority_level(&self, rule_id: u32) -> PriorityLevel {
        let priority = self.get_rule_priority(rule_id);
        if priority <= PriorityLevel::Lowest as u16 {
            PriorityLevel::Lowest
        } else if priority <= PriorityLevel::Normal as u16 {
            PriorityLevel::Normal
        } else if priority <= PriorityLevel::High as u16 {
            PriorityLevel::High
        } else {
            PriorityLevel::Critical
        }
    }

    /// Adjusts priority based on rule dependencies
    pub fn adjust_priority_for_dependencies(&mut self, rule_info: &mut RuleExecutionInfo) {
        // Increase priority if rule has many dependencies (should execute after them)
        if rule_info.dependencies.len() > 5 {
            // Boost priority for rules with many dependencies
            let current_priority = self.get_rule_priority(rule_info.rule_id);
            let new_priority =
                std::cmp::min(current_priority + 10, self.priority_limits.max_priority);
            self.set_rule_priority(rule_info.rule_id, new_priority)
                .unwrap_or_default(); // Ignore errors in this helper function
        }
    }

    /// Adjusts priority based on rule complexity
    pub fn adjust_priority_for_complexity(
        &mut self,
        rule_info: &mut RuleExecutionInfo,
        complexity_score: u16,
    ) {
        // Increase priority for more complex rules
        let current_priority = self.get_rule_priority(rule_info.rule_id);
        let priority_boost = std::cmp::min(complexity_score / 10, 50); // Max 50 point boost
        let new_priority = std::cmp::min(
            current_priority + priority_boost,
            self.priority_limits.max_priority,
        );

        self.set_rule_priority(rule_info.rule_id, new_priority)
            .unwrap_or_default(); // Ignore errors in this helper function
    }

    /// Sorts rules by priority (descending) with stable tie-breaking by rule ID
    pub fn sort_rules_by_priority(&self, rules: &mut [RuleExecutionInfo]) {
        rules.sort_by(|a, b| {
            // Sort by priority descending, then by rule_id ascending for stable sorting
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.rule_id.cmp(&b.rule_id))
        });
    }

    /// Sets the default priority for new rules
    pub fn set_default_priority(&mut self, priority: u16) -> Result<(), String> {
        if priority < self.priority_limits.min_priority
            || priority > self.priority_limits.max_priority
        {
            return Err(format!(
                "Default priority {} is outside allowed range ({}-{})",
                priority, self.priority_limits.min_priority, self.priority_limits.max_priority
            ));
        }

        self.default_priority = priority;
        Ok(())
    }

    /// Gets the current default priority
    pub fn get_default_priority(&self) -> u16 {
        self.default_priority
    }

    /// Resets all priorities to default
    pub fn reset_all_priorities(&mut self) {
        self.rule_priorities.clear();
    }

    /// Gets all currently set priorities
    pub fn get_all_priorities(&self) -> &HashMap<u32, u16> {
        &self.rule_priorities
    }

    /// Removes a rule's priority setting (will use default)
    pub fn remove_rule_priority(&mut self, rule_id: u32) {
        self.rule_priorities.remove(&rule_id);
    }
}

impl Default for PriorityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RuleExecutionInfo;

    #[test]
    fn test_priority_manager() {
        let mut manager = PriorityManager::new();

        // Test default priority
        assert_eq!(manager.get_default_priority(), PriorityLevel::Normal as u16);

        // Create a test rule
        let _rule_info = RuleExecutionInfo::new(1);

        // Test getting default priority
        assert_eq!(manager.get_rule_priority(1), PriorityLevel::Normal as u16);

        // Set a specific priority
        manager.set_rule_priority(1, 75).unwrap();
        assert_eq!(manager.get_rule_priority(1), 75);

        // Test priority level conversion
        manager
            .set_rule_priority(1, PriorityLevel::High as u16)
            .unwrap();
        assert_eq!(manager.get_rule_priority_level(1), PriorityLevel::High);

        // Test setting priority level directly
        manager
            .set_rule_priority_level(1, PriorityLevel::Critical)
            .unwrap();
        assert_eq!(manager.get_rule_priority(1), PriorityLevel::Critical as u16);

        // Test sorting rules
        let mut rules = vec![
            RuleExecutionInfo {
                rule_id: 1,
                priority: 50,
                ..RuleExecutionInfo::new(1)
            },
            RuleExecutionInfo {
                rule_id: 2,
                priority: 100,
                ..RuleExecutionInfo::new(2)
            },
            RuleExecutionInfo {
                rule_id: 3,
                priority: 75,
                ..RuleExecutionInfo::new(3)
            },
        ];

        manager.sort_rules_by_priority(&mut rules);

        // Rules should be sorted by priority descending
        assert_eq!(rules[0].rule_id, 2); // priority 100
        assert_eq!(rules[1].rule_id, 3); // priority 75
        assert_eq!(rules[2].rule_id, 1); // priority 50
    }

    #[test]
    fn test_priority_adjustments() {
        let mut manager = PriorityManager::new();
        let mut rule_info = RuleExecutionInfo::new(1);

        // Add dependencies to trigger adjustment
        rule_info.dependencies = vec![2, 3, 4, 5, 6, 7]; // More than 5 dependencies

        // Adjust priority for dependencies
        manager.adjust_priority_for_dependencies(&mut rule_info);

        // The priority should be increased due to many dependencies
        assert!(manager.get_rule_priority(1) > manager.get_default_priority());
    }

    #[test]
    fn test_priority_limits() {
        let mut manager = PriorityManager::new();

        // Test setting priority within limits
        assert!(manager.set_rule_priority(1, 500).is_ok());

        // Test setting priority outside limits
        assert!(manager.set_rule_priority(2, 2000).is_err());

        // Test setting default priority outside limits
        assert!(manager.set_default_priority(1500).is_err());
    }
}
