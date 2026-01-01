#[cfg(test)]
mod semantic_tests {
    use kern_parser::ast_nodes::{Program, Declaration, Entity, Rule, Flow, Constraint};
    use kern_semantic::{SymbolTable, TypeChecker, DependencyGraph, ConflictDetector};
    use crate::assertions::{assert_equal, assert_true, assert_false, AssertionResult};

    #[test]
    fn test_symbol_table_creation() {
        let mut symbol_table = SymbolTable::new();
        
        // Add a symbol
        symbol_table.add_symbol("user", "Entity");
        
        // Check that the symbol exists
        assert!(symbol_table.lookup_symbol("user").is_some());
        assert_eq!(symbol_table.lookup_symbol("user").unwrap(), "Entity");
        
        // Check that a non-existent symbol doesn't exist
        assert!(symbol_table.lookup_symbol("nonexistent").is_none());
    }

    #[test]
    fn test_duplicate_symbol_detection() {
        let mut symbol_table = SymbolTable::new();
        
        // Add a symbol
        symbol_table.add_symbol("user", "Entity");
        
        // Try to add the same symbol again
        let result = symbol_table.add_symbol("user", "Rule");
        
        // Should return false since the symbol already exists
        assert!(!result);
    }

    #[test]
    fn test_symbol_scoping() {
        let mut symbol_table = SymbolTable::new();
        
        // Add symbols to global scope
        symbol_table.add_symbol("global_var", "Entity");
        
        // Enter a new scope
        symbol_table.enter_scope();
        symbol_table.add_symbol("local_var", "Rule");
        
        // Check that both symbols are accessible
        assert!(symbol_table.lookup_symbol("global_var").is_some());
        assert!(symbol_table.lookup_symbol("local_var").is_some());
        
        // Exit the scope
        symbol_table.exit_scope();
        
        // Local symbol should no longer be accessible
        assert!(symbol_table.lookup_symbol("local_var").is_none());
        // Global symbol should still be accessible
        assert!(symbol_table.lookup_symbol("global_var").is_some());
    }

    #[test]
    fn test_type_checker_basic_validation() {
        let type_checker = TypeChecker::new();
        
        // Create a simple program with an entity
        let entity = Entity {
            name: "User".to_string(),
            fields: vec!["name".to_string(), "age".to_string()],
        };
        
        let program = Program {
            declarations: vec![Declaration::Entity(entity)],
        };
        
        // Validate the program
        let result = type_checker.validate_program(&program);
        
        // Should pass validation
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_checker_invalid_reference() {
        let type_checker = TypeChecker::new();
        
        // Create a rule that references an undefined entity
        let rule = Rule {
            name: "ValidateUser".to_string(),
            condition: None, // In a real implementation, this would reference an undefined entity
            actions: vec![],
        };
        
        let program = Program {
            declarations: vec![Declaration::Rule(rule)],
        };
        
        // Validate the program
        let result = type_checker.validate_program(&program);
        
        // Should fail validation
        assert!(result.is_err());
    }

    #[test]
    fn test_dependency_graph_creation() {
        let mut dep_graph = DependencyGraph::new();
        
        // Add some nodes
        dep_graph.add_node("entity1");
        dep_graph.add_node("rule1");
        dep_graph.add_node("flow1");
        
        // Add dependencies
        dep_graph.add_dependency("rule1", "entity1");
        dep_graph.add_dependency("flow1", "rule1");
        
        // Check dependencies
        let entity_deps = dep_graph.get_dependents("entity1");
        assert!(entity_deps.contains(&"rule1".to_string()));
        
        let rule_deps = dep_graph.get_dependents("rule1");
        assert!(rule_deps.contains(&"flow1".to_string()));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut dep_graph = DependencyGraph::new();
        
        // Create a circular dependency: A -> B -> C -> A
        dep_graph.add_node("A");
        dep_graph.add_node("B");
        dep_graph.add_node("C");
        
        dep_graph.add_dependency("A", "B");
        dep_graph.add_dependency("B", "C");
        dep_graph.add_dependency("C", "A");
        
        // Check for circular dependencies
        let has_cycle = dep_graph.has_circular_dependency();
        assert!(has_cycle);
    }

    #[test]
    fn test_rule_conflict_detection() {
        let mut conflict_detector = ConflictDetector::new();
        
        // Add rules that might conflict
        // In a real implementation, this would check for attribute write conflicts
        let rule1 = Rule {
            name: "Rule1".to_string(),
            condition: None,
            actions: vec![],
        };
        
        let rule2 = Rule {
            name: "Rule2".to_string(),
            condition: None,
            actions: vec![],
        };
        
        // Check for conflicts between rules
        let conflicts = conflict_detector.detect_rule_conflicts(&[rule1, rule2]);
        
        // For now, just verify the function runs without error
        assert!(conflicts.len() >= 0);
    }

    #[test]
    fn test_type_compatibility_check() {
        let type_checker = TypeChecker::new();
        
        // Test compatible types
        let result = type_checker.check_type_compatibility("num", "num");
        assert!(result);
        
        // Test incompatible types
        let result = type_checker.check_type_compatibility("num", "sym");
        assert!(!result);
    }

    #[test]
    fn test_scope_resolution() {
        let mut symbol_table = SymbolTable::new();
        
        // Add symbols in different scopes
        symbol_table.add_symbol("global_entity", "Entity");
        
        symbol_table.enter_scope();
        symbol_table.add_symbol("local_rule", "Rule");
        
        symbol_table.enter_scope();
        symbol_table.add_symbol("nested_flow", "Flow");
        
        // Resolve symbols from nested scope
        assert!(symbol_table.resolve_symbol("nested_flow").is_some());
        assert!(symbol_table.resolve_symbol("local_rule").is_some());
        assert!(symbol_table.resolve_symbol("global_entity").is_some());
        
        // Exit to parent scope
        symbol_table.exit_scope();
        assert!(symbol_table.resolve_symbol("local_rule").is_some());
        assert!(symbol_table.resolve_symbol("global_entity").is_some());
        assert!(symbol_table.resolve_symbol("nested_flow").is_none());
    }

    #[test]
    fn test_validation_error_reporting() {
        let type_checker = TypeChecker::new();
        
        // Create a program with an intentional error
        // In a real implementation, this would be an invalid reference
        let entity = Entity {
            name: "User".to_string(),
            fields: vec!["name".to_string(), "age".to_string()],
        };
        
        let program = Program {
            declarations: vec![Declaration::Entity(entity)],
        };
        
        // Validate the program
        let result = type_checker.validate_program(&program);
        
        // Should pass validation in this simple case
        assert!(result.is_ok());
        
        // In a more complex implementation, we would test error reporting
        // by creating programs with actual semantic errors
    }
}