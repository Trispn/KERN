#[cfg(test)]
mod tests {
    use super::*;
    use kern_parser::Parser;
    use kern_graph_builder::GraphBuilder;

    #[test]
    fn test_rule_engine_complete_flow() {
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
        println!("Generated execution graph with {} nodes", graph.nodes.len());

        // Create the rule engine
        let mut engine = RuleEngine::new(graph);

        // Set up some initial values for testing
        engine.program_state.insert("farmer.location".to_string(), "valid".to_string());
        engine.program_state.insert("farmer.id".to_string(), "123".to_string());

        // Run a cycle of rule execution
        let execution_result = engine.run_cycle();
        assert!(execution_result.is_ok());

        println!("Rule engine executed successfully with {} steps", engine.step_count);
    }

    #[test]
    fn test_pattern_matching() {
        let matcher = PatternMatcher::new();
        
        // Test simple value matching
        let value_pattern = Pattern::Value("test_value".to_string());
        let result = matcher.match_pattern(&value_pattern, "test_value");
        assert!(result.is_some());
        
        // Test variable binding
        let var_pattern = Pattern::Variable("x".to_string());
        let result = matcher.match_pattern(&var_pattern, "bound_value");
        assert!(result.is_some());
        if let Some(bindings) = result {
            assert_eq!(bindings.get("x"), Some(&"bound_value".to_string()));
        }
        
        // Test composite pattern
        let composite_pattern = Pattern::Composite(
            "entity.field".to_string(),
            vec![Pattern::Value("location".to_string())]
        );
        let result = matcher.match_pattern(&composite_pattern, "location");
        assert!(result.is_some());
    }

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

    #[test]
    fn test_conflict_resolution() {
        let mut resolver = ConflictResolver::new();
        
        // Create test rules
        let mut rule1 = RuleExecutionInfo::new(1);
        rule1.priority = 50;
        
        let mut rule2 = RuleExecutionInfo::new(2);
        rule2.priority = 75;  // Higher priority
        
        let mut rules = vec![rule1, rule2];
        
        // Add a conflict between the rules
        resolver.add_conflict(ConflictEntry {
            target_symbol_id: 100,
            conflicting_rules: vec![1, 2],
            resolution_mode: ResolutionMode::Override,
        });
        
        // Resolve conflicts
        let result = resolver.resolve_conflicts(&mut rules);
        assert!(result.is_ok());
        
        // Check that the lower priority rule was deprioritized
        let lower_priority_rule = rules.iter().find(|r| r.rule_id == 1).unwrap();
        assert_eq!(lower_priority_rule.priority, 0);  // Should be set to 0
        
        let higher_priority_rule = rules.iter().find(|r| r.rule_id == 2).unwrap();
        assert_eq!(higher_priority_rule.priority, 75);  // Should remain unchanged
    }

    #[test]
    fn test_priority_manager() {
        let mut manager = PriorityManager::new();
        
        // Test default priority
        assert_eq!(manager.get_default_priority(), PriorityLevel::Normal as u16);
        
        // Create a test rule
        let mut rule_info = RuleExecutionInfo::new(1);
        
        // Test getting default priority
        assert_eq!(manager.get_rule_priority(1), PriorityLevel::Normal as u16);
        
        // Set a specific priority
        manager.set_rule_priority(1, 75).unwrap();
        assert_eq!(manager.get_rule_priority(1), 75);
        
        // Test priority level conversion
        manager.set_rule_priority(1, PriorityLevel::High as u16).unwrap();
        assert_eq!(manager.get_rule_priority_level(1), PriorityLevel::High);
        
        // Test setting priority level directly
        manager.set_rule_priority_level(1, PriorityLevel::Critical).unwrap();
        assert_eq!(manager.get_rule_priority(1), PriorityLevel::Critical as u16);
        
        // Test sorting rules
        let mut rules = vec![
            RuleExecutionInfo { rule_id: 1, priority: 50, ..RuleExecutionInfo::new(1) },
            RuleExecutionInfo { rule_id: 2, priority: 100, ..RuleExecutionInfo::new(2) },
            RuleExecutionInfo { rule_id: 3, priority: 75, ..RuleExecutionInfo::new(3) },
        ];
        
        manager.sort_rules_by_priority(&mut rules);
        
        // Rules should be sorted by priority descending
        assert_eq!(rules[0].rule_id, 2); // priority 100
        assert_eq!(rules[1].rule_id, 3); // priority 75
        assert_eq!(rules[2].rule_id, 1); // priority 50
    }

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
}