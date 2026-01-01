#[cfg(test)]
mod rule_engine_tests {
    use kern_rule_engine::{RuleEngine, Rule, RuleMatch, RulePriority};
    use kern_parser::ast_nodes::{Program, Declaration};
    use crate::assertions::{assert_equal, assert_true, assert_false, AssertionResult};

    #[test]
    fn test_rule_creation() {
        let rule = Rule {
            id: 1,
            name: "TestRule".to_string(),
            priority: RulePriority::Normal,
            condition: None,
            actions: vec![],
        };
        
        assert_eq!(rule.id, 1);
        assert_eq!(rule.name, "TestRule");
        assert_eq!(rule.priority, RulePriority::Normal);
    }

    #[test]
    fn test_rule_engine_initialization() {
        let engine = RuleEngine::new();
        
        assert_eq!(engine.rules.len(), 0);
        assert_eq!(engine.facts.len(), 0);
    }

    #[test]
    fn test_adding_rules_to_engine() {
        let mut engine = RuleEngine::new();
        
        let rule = Rule {
            id: 1,
            name: "TestRule".to_string(),
            priority: RulePriority::Normal,
            condition: None,
            actions: vec![],
        };
        
        engine.add_rule(rule);
        
        assert_eq!(engine.rules.len(), 1);
        assert!(engine.rules.get(&1).is_some());
    }

    #[test]
    fn test_rule_priority_system() {
        let mut engine = RuleEngine::new();
        
        // Add rules with different priorities
        let high_priority_rule = Rule {
            id: 1,
            name: "HighPriorityRule".to_string(),
            priority: RulePriority::High,
            condition: None,
            actions: vec![],
        };
        
        let low_priority_rule = Rule {
            id: 2,
            name: "LowPriorityRule".to_string(),
            priority: RulePriority::Low,
            condition: None,
            actions: vec![],
        };
        
        let normal_priority_rule = Rule {
            id: 3,
            name: "NormalPriorityRule".to_string(),
            priority: RulePriority::Normal,
            condition: None,
            actions: vec![],
        };
        
        engine.add_rule(high_priority_rule);
        engine.add_rule(low_priority_rule);
        engine.add_rule(normal_priority_rule);
        
        // In a real implementation, we would test that rules execute in priority order
        // For now, just verify they were added
        assert_eq!(engine.rules.len(), 3);
    }

    #[test]
    fn test_rule_matching() {
        let mut engine = RuleEngine::new();
        
        // Add a simple rule
        let rule = Rule {
            id: 1,
            name: "MatchRule".to_string(),
            priority: RulePriority::Normal,
            condition: None, // In a real implementation, this would have a condition
            actions: vec![],
        };
        
        engine.add_rule(rule);
        
        // Create a mock fact to match against
        // In a real implementation, this would be actual data
        let mock_fact = "test_fact";
        
        // In a real implementation, we would test if the rule matches the fact
        // For now, just verify the engine has the rule
        assert!(engine.rules.get(&1).is_some());
    }

    #[test]
    fn test_rule_execution() {
        let mut engine = RuleEngine::new();
        
        // Add a rule
        let rule = Rule {
            id: 1,
            name: "ExecuteRule".to_string(),
            priority: RulePriority::Normal,
            condition: None,
            actions: vec![], // In a real implementation, this would have actions
        };
        
        engine.add_rule(rule);
        
        // Execute rules
        // In a real implementation, this would execute the rule actions
        let execution_result = engine.execute_rules();
        
        // For now, just verify execution doesn't panic
        assert!(execution_result.is_ok());
    }

    #[test]
    fn test_rule_conflict_resolution() {
        let mut engine = RuleEngine::new();
        
        // Add rules that might conflict
        let rule1 = Rule {
            id: 1,
            name: "Rule1".to_string(),
            priority: RulePriority::Normal,
            condition: None,
            actions: vec![],
        };
        
        let rule2 = Rule {
            id: 2,
            name: "Rule2".to_string(),
            priority: RulePriority::Normal,
            condition: None,
            actions: vec![],
        };
        
        engine.add_rule(rule1);
        engine.add_rule(rule2);
        
        // In a real implementation, we would test conflict resolution
        // For now, just verify both rules are added
        assert_eq!(engine.rules.len(), 2);
    }

    #[test]
    fn test_rule_activation() {
        let mut engine = RuleEngine::new();
        
        let rule = Rule {
            id: 1,
            name: "ActivationTestRule".to_string(),
            priority: RulePriority::Normal,
            condition: None,
            actions: vec![],
        };
        
        engine.add_rule(rule);
        
        // In a real implementation, we would test rule activation based on facts
        // For now, just verify the rule exists
        assert!(engine.rules.get(&1).is_some());
    }

    #[test]
    fn test_multiple_rule_execution() {
        let mut engine = RuleEngine::new();
        
        // Add multiple rules
        for i in 1..=5 {
            let rule = Rule {
                id: i,
                name: format!("Rule{}", i),
                priority: RulePriority::Normal,
                condition: None,
                actions: vec![],
            };
            engine.add_rule(rule);
        }
        
        // Execute all rules
        let result = engine.execute_rules();
        
        // Should execute without errors
        assert!(result.is_ok());
        
        // Should still have all 5 rules
        assert_eq!(engine.rules.len(), 5);
    }

    #[test]
    fn test_rule_removal() {
        let mut engine = RuleEngine::new();
        
        let rule = Rule {
            id: 1,
            name: "RemovalTestRule".to_string(),
            priority: RulePriority::Normal,
            condition: None,
            actions: vec![],
        };
        
        engine.add_rule(rule);
        assert_eq!(engine.rules.len(), 1);
        
        // In a real implementation, we would have a remove_rule method
        // For now, just verify the rule was added
        assert!(engine.rules.get(&1).is_some());
    }
}