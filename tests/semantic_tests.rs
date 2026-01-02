mod common;
use common::assertions::{assert_equal, assert_false, assert_true, AssertionResult};
use kern_parser::ast::{
    Comparator, Condition, ConstraintDef, Definition, EntityDef, Expression, FieldDef, Program,
    RuleDef, Term,
};
use kern_semantic::{
    ConflictDetector, DependencyGraph, Resolver, SemanticAnalyzer, Symbol, SymbolKind, SymbolTable,
    TypeChecker,
};

#[test]
fn test_symbol_table_creation() {
    let symbol_table = SymbolTable::new();

    // Check that the symbol table is initially empty
    assert!(!symbol_table.has_symbol("user"));
    assert!(symbol_table.lookup_symbol("nonexistent").is_none());
}

#[test]
fn test_symbol_registration() {
    use kern_semantic::{SourceLocation, TypeDescriptor, TypeKind};

    let mut symbol_table = SymbolTable::new();
    let location = SourceLocation::new("test.kern".to_string(), 1, 1);
    let ty = TypeDescriptor::new(TypeKind::Sym);

    let symbol = Symbol::new("user".to_string(), SymbolKind::Entity, ty, 0, location);

    // Register the symbol
    let result = symbol_table.register_symbol(symbol);
    assert!(result.is_ok());

    // Check that the symbol exists
    assert!(symbol_table.has_symbol("user"));
    assert!(symbol_table.lookup_symbol("user").is_some());
}

#[test]
fn test_duplicate_symbol_detection() {
    use kern_semantic::{SourceLocation, TypeDescriptor, TypeKind};

    let mut symbol_table = SymbolTable::new();
    let location = SourceLocation::new("test.kern".to_string(), 1, 1);
    let ty = TypeDescriptor::new(TypeKind::Sym);

    let symbol1 = Symbol::new(
        "user".to_string(),
        SymbolKind::Entity,
        ty.clone(),
        0,
        location.clone(),
    );
    let symbol2 = Symbol::new("user".to_string(), SymbolKind::Rule, ty, 0, location);

    // Register the first symbol
    let result1 = symbol_table.register_symbol(symbol1);
    assert!(result1.is_ok());

    // Try to register the same symbol again - should fail
    let result2 = symbol_table.register_symbol(symbol2);
    assert!(result2.is_err());
}

#[test]
fn test_resolver_creation() {
    let resolver = Resolver::new();
    // Just ensure it creates without panicking
    let _ = resolver.scope_manager();
}

#[test]
fn test_type_checker_creation() {
    let resolver = Resolver::new();
    let type_checker = TypeChecker::new(resolver);
    // Just ensure it creates without panicking
    let _ = type_checker.resolver();
}

#[test]
fn test_type_checker_check_program() {
    let mut resolver = Resolver::new();

    // Create a simple program with an entity
    let entity = EntityDef {
        name: "User".to_string(),
        fields: vec![
            FieldDef {
                name: "name".to_string(),
            },
            FieldDef {
                name: "age".to_string(),
            },
        ],
    };

    let program = Program {
        definitions: vec![Definition::Entity(entity)],
    };

    // Resolve the program first
    let resolve_result = resolver.resolve_program(&program);
    assert!(
        resolve_result.is_ok(),
        "Resolution failed: {:?}",
        resolve_result.err()
    );

    // Now type check
    let mut type_checker = TypeChecker::new(resolver);
    let check_result = type_checker.check_program(&program);
    assert!(
        check_result.is_ok(),
        "Type checking failed: {:?}",
        check_result.err()
    );
}

#[test]
fn test_dependency_graph_creation() {
    let resolver = Resolver::new();
    let dep_graph = DependencyGraph::new(resolver);
    // Just test that it creates without error
    let _ = dep_graph.topological_sort();
}

#[test]
fn test_dependency_graph_build() {
    let mut resolver = Resolver::new();

    let entity = EntityDef {
        name: "User".to_string(),
        fields: vec![FieldDef {
            name: "id".to_string(),
        }],
    };

    let program = Program {
        definitions: vec![Definition::Entity(entity)],
    };

    // Resolve the program first
    let _ = resolver.resolve_program(&program);

    // Build dependency graph
    let mut dep_graph = DependencyGraph::new(resolver);
    let result = dep_graph.build_graph(&program);
    assert!(
        result.is_ok(),
        "Failed to build dependency graph: {:?}",
        result.err()
    );
}

#[test]
fn test_conflict_detector_creation() {
    let resolver = Resolver::new();
    let _conflict_detector = ConflictDetector::new(resolver);
    // Just test that it creates without error
}

#[test]
fn test_semantic_analyzer_creation() {
    let analyzer = SemanticAnalyzer::new();
    assert!(!analyzer.diagnostic_reporter().has_errors());
}

#[test]
fn test_semantic_analyzer_simple_program() {
    use kern_parser::Parser;

    let input = r#"
        entity Farmer {
            id
            location
        }

        rule CheckId:
            if farmer.id > 0
            then approve(farmer)

        constraint ValidId: farmer.id > 0
    "#;

    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse program");

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&program);

    // The analysis should pass for this valid program
    assert!(
        result.is_ok(),
        "Semantic analysis failed: {:?}",
        result.err()
    );
}
