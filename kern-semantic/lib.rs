//! KERN Semantic Analysis System
//! 
//! This crate provides the complete semantic analysis and type-checking system for the KERN language.
//! It includes symbol resolution, type checking, dependency analysis, conflict detection,
//! bytecode validation, and diagnostic reporting.

pub mod types;
pub mod symbol;
pub mod scope;
pub mod resolver;
pub mod type_checker;
pub mod dependency_graph;
pub mod conflict_detector;
pub mod bytecode_validator;
pub mod diagnostics;

// Re-export important types for easier access
pub use types::{TypeDescriptor, TypeKind, TypeChecker as TypeCheckerUtil};
pub use symbol::{Symbol, SymbolKind, SourceLocation};
pub use scope::ScopeManager;
pub use resolver::{Resolver, ResolutionError};
pub use type_checker::{TypeChecker, TypeError};
pub use dependency_graph::{DependencyGraph, DependencyNode, DependencyError};
pub use conflict_detector::{ConflictDetector, Conflict, ConflictType, ConflictSeverity};
pub use bytecode_validator::{BytecodeValidator, BytecodeValidationError};
pub use diagnostics::{Diagnostic, DiagnosticCode, DiagnosticReporter, Severity, SourceLocation as DiagnosticSourceLocation};

/// The main semantic analysis pipeline for KERN programs
pub struct SemanticAnalyzer {
    diagnostic_reporter: DiagnosticReporter,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            diagnostic_reporter: DiagnosticReporter::new(),
        }
    }

    /// Performs complete semantic analysis on a KERN program
    pub fn analyze(&mut self, program: &kern_parser::Program) -> Result<(), Vec<String>> {
        // Reset diagnostic reporter
        self.diagnostic_reporter = DiagnosticReporter::new();

        // Step 1: Resolve symbols
        let mut resolver = Resolver::new();
        match resolver.resolve_program(program) {
            Ok(()) => {
                // Step 2: Type check
                let mut type_checker = TypeChecker::new(resolver);
                match type_checker.check_program(program) {
                    Ok(()) => {
                        // Step 3: Build dependency graph
                        let mut dep_graph = DependencyGraph::new(type_checker.resolver().clone());
                        match dep_graph.build_graph(program) {
                            Ok(()) => {
                                // Step 4: Detect conflicts
                                let mut conflict_detector = ConflictDetector::new(dep_graph.resolver().clone());
                                match conflict_detector.detect_conflicts(program) {
                                    Ok(conflicts) => {
                                        // Report conflicts as warnings/errors
                                        for conflict in conflicts {
                                            let location = DiagnosticSourceLocation::new("unknown".to_string(), 0, 0);
                                            let message = format!("Rule conflict: {}", conflict.description);

                                            match conflict.severity {
                                                ConflictSeverity::Error => {
                                                    self.diagnostic_reporter.error(
                                                        DiagnosticCode::RULE_CONFLICT,
                                                        message,
                                                        location,
                                                    );
                                                },
                                                ConflictSeverity::Warning => {
                                                    self.diagnostic_reporter.warning(
                                                        DiagnosticCode::RULE_CONFLICT,
                                                        message,
                                                        location,
                                                    );
                                                },
                                                ConflictSeverity::Info => {
                                                    self.diagnostic_reporter.info(
                                                        DiagnosticCode::RULE_CONFLICT,
                                                        message,
                                                        location,
                                                    );
                                                },
                                            }
                                        }

                                        // Step 5: Validate for bytecode generation
                                        let resolver = conflict_detector.resolver().clone();
                                        let type_checker = TypeChecker::new(resolver);
                                        let mut bytecode_validator = BytecodeValidator::new(
                                            type_checker.resolver().clone(),
                                            type_checker
                                        );
                                        match bytecode_validator.validate_program(program) {
                                            Ok(()) => {
                                                // All checks passed
                                                Ok(())
                                            },
                                            Err(errors) => Err(errors),
                                        }
                                    },
                                    Err(errors) => Err(errors),
                                }
                            },
                            Err(errors) => Err(errors),
                        }
                    },
                    Err(errors) => Err(errors),
                }
            },
            Err(errors) => Err(errors),
        }
    }

    /// Gets the diagnostic reporter
    pub fn diagnostic_reporter(&self) -> &DiagnosticReporter {
        &self.diagnostic_reporter
    }

    /// Gets mutable access to the diagnostic reporter
    pub fn diagnostic_reporter_mut(&mut self) -> &mut DiagnosticReporter {
        &mut self.diagnostic_reporter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kern_parser::Parser;

    #[test]
    fn test_semantic_analyzer_creation() {
        let analyzer = SemanticAnalyzer::new();
        assert_eq!(analyzer.diagnostic_reporter().diagnostics().len(), 0);
    }

    #[test]
    fn test_complete_semantic_analysis() {
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

        let mut parser = Parser::new(input);
        let program = parser.parse_program().expect("Failed to parse program");

        let mut analyzer = SemanticAnalyzer::new();
        let result = analyzer.analyze(&program);
        
        // The complete semantic analysis should pass without errors for this valid program
        assert!(result.is_ok(), "Semantic analysis failed with errors: {:?}", result.err());
        
        // There should be no errors reported
        assert!(!analyzer.diagnostic_reporter().has_errors());
    }
}