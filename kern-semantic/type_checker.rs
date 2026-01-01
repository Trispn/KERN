//! KERN Type Checker
//! 
//! Performs type checking on the KERN AST after symbol resolution.

use crate::resolver::Resolver;
use crate::types::{TypeDescriptor, TypeKind, TypeChecker as TypeCheckerUtil};
use crate::symbol::{Symbol, SymbolKind};
use kern_parser::{AstNode, Program, Definition, EntityDef, RuleDef, FlowDef, ConstraintDef, Condition, Expression, Term, Predicate, Action, IfAction, LoopAction, HaltAction, Assignment, ControlAction, Comparator, LogicalOp};

#[derive(Debug)]
pub struct TypeChecker {
    resolver: Resolver,
    errors: Vec<String>,
}

#[derive(Debug)]
pub enum TypeError {
    TypeMismatch {
        expected: TypeDescriptor,
        actual: TypeDescriptor,
        location: crate::symbol::SourceLocation,
    },
    InvalidOperator {
        operator: String,
        left_type: TypeDescriptor,
        right_type: TypeDescriptor,
        location: crate::symbol::SourceLocation,
    },
    InvalidUnaryOperator {
        operator: String,
        operand_type: TypeDescriptor,
        location: crate::symbol::SourceLocation,
    },
    UnknownType {
        name: String,
        location: crate::symbol::SourceLocation,
    },
    InvalidAssignment {
        target_type: TypeDescriptor,
        value_type: TypeDescriptor,
        location: crate::symbol::SourceLocation,
    },
}

impl TypeError {
    pub fn message(&self) -> String {
        match self {
            TypeError::TypeMismatch { expected, actual, location } => {
                format!(
                    "Type mismatch: expected {:?}, found {:?} at {}:{}", 
                    expected, actual, location.file, location.line
                )
            },
            TypeError::InvalidOperator { operator, left_type, right_type, location } => {
                format!(
                    "Invalid operator '{}' for types {:?} and {:?} at {}:{}", 
                    operator, left_type, right_type, location.file, location.line
                )
            },
            TypeError::InvalidUnaryOperator { operator, operand_type, location } => {
                format!(
                    "Invalid unary operator '{}' for type {:?} at {}:{}", 
                    operator, operand_type, location.file, location.line
                )
            },
            TypeError::UnknownType { name, location } => {
                format!(
                    "Unknown type '{}', at {}:{}", 
                    name, location.file, location.line
                )
            },
            TypeError::InvalidAssignment { target_type, value_type, location } => {
                format!(
                    "Cannot assign value of type {:?} to target of type {:?} at {}:{}", 
                    value_type, target_type, location.file, location.line
                )
            },
        }
    }
}

impl TypeChecker {
    pub fn new(resolver: Resolver) -> Self {
        TypeChecker {
            resolver,
            errors: Vec::new(),
        }
    }

    /// Performs type checking on a program
    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<String>> {
        for definition in &program.definitions {
            self.check_definition(definition);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn check_definition(&mut self, definition: &Definition) {
        match definition {
            Definition::Entity(entity_def) => {
                self.check_entity(entity_def);
            },
            Definition::Rule(rule_def) => {
                self.check_rule(rule_def);
            },
            Definition::Flow(flow_def) => {
                self.check_flow(flow_def);
            },
            Definition::Constraint(constraint_def) => {
                self.check_constraint(constraint_def);
            },
        }
    }

    fn check_entity(&mut self, _entity_def: &EntityDef) {
        // Entity type validation is done during definition
        // For now, we just validate that all attributes have valid types
    }

    fn check_rule(&mut self, rule_def: &RuleDef) {
        // Enter rule scope
        self.resolver.scope_manager_mut().enter_scope();
        
        // Check that the condition evaluates to Bool
        let condition_type = self.check_condition(&rule_def.condition);
        if !TypeCheckerUtil::validate_boolean(&condition_type) {
            let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
            self.errors.push(TypeError::TypeMismatch {
                expected: TypeDescriptor::new(TypeKind::Bool),
                actual: condition_type,
                location,
            }.message());
        }
        
        // Check each action
        for action in &rule_def.actions {
            self.check_action(action);
        }
        
        self.resolver.scope_manager_mut().exit_scope().unwrap();
    }

    fn check_flow(&mut self, flow_def: &FlowDef) {
        // Enter flow scope
        self.resolver.scope_manager_mut().enter_scope();
        
        // Check each action in the flow
        for action in &flow_def.actions {
            self.check_action(action);
        }
        
        self.resolver.scope_manager_mut().exit_scope().unwrap();
    }

    fn check_constraint(&mut self, constraint_def: &ConstraintDef) {
        // Check that the constraint condition evaluates to Bool
        let condition_type = self.check_condition(&constraint_def.condition);
        if !TypeCheckerUtil::validate_boolean(&condition_type) {
            let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
            self.errors.push(TypeError::TypeMismatch {
                expected: TypeDescriptor::new(TypeKind::Bool),
                actual: condition_type,
                location,
            }.message());
        }
    }

    fn check_condition(&mut self, condition: &Condition) -> TypeDescriptor {
        match condition {
            Condition::Expression(expr) => {
                self.check_expression(expr)
            },
            Condition::LogicalOp(left, _op, right) => {
                let left_type = self.check_condition(left);
                let right_type = self.check_condition(right);
                
                // Both operands of logical operators must be boolean
                if !TypeCheckerUtil::validate_boolean(&left_type) {
                    let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                    self.errors.push(TypeError::TypeMismatch {
                        expected: TypeDescriptor::new(TypeKind::Bool),
                        actual: left_type,
                        location,
                    }.message());
                }
                
                if !TypeCheckerUtil::validate_boolean(&right_type) {
                    let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                    self.errors.push(TypeError::TypeMismatch {
                        expected: TypeDescriptor::new(TypeKind::Bool),
                        actual: right_type,
                        location,
                    }.message());
                }
                
                // Result is always boolean
                TypeDescriptor::new(TypeKind::Bool)
            },
        }
    }

    fn check_expression(&mut self, expression: &Expression) -> TypeDescriptor {
        match expression {
            Expression::Comparison { left, op, right } => {
                let left_type = self.check_term(left);
                let right_type = self.check_term(right);
                
                // Check if the operator is valid for the types
                match op {
                    Comparator::Equal | Comparator::NotEqual => {
                        // These operators work on any comparable types, but both operands must be the same type
                        if left_type != right_type {
                            let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                            self.errors.push(TypeError::TypeMismatch {
                                expected: left_type.clone(),
                                actual: right_type,
                                location,
                            }.message());
                        }
                    },
                    Comparator::Greater | Comparator::Less | Comparator::GreaterEqual | Comparator::LessEqual => {
                        // These operators require numeric types
                        if !TypeCheckerUtil::validate_numeric(&left_type) || !TypeCheckerUtil::validate_numeric(&right_type) {
                            let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                            self.errors.push(TypeError::InvalidOperator {
                                operator: format!("{:?}", op),
                                left_type: left_type.clone(),
                                right_type: right_type.clone(),
                                location,
                            }.message());
                        }
                        
                        // Both operands must be the same type
                        if left_type != right_type {
                            let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                            self.errors.push(TypeError::TypeMismatch {
                                expected: left_type.clone(),
                                actual: right_type,
                                location,
                            }.message());
                        }
                    },
                }
                
                // Comparison operations always return boolean
                TypeDescriptor::new(TypeKind::Bool)
            },
            Expression::Predicate(predicate) => {
                self.check_predicate(predicate)
            },
        }
    }

    fn check_term(&mut self, term: &Term) -> TypeDescriptor {
        match term {
            Term::Identifier(name) => {
                // Look up the type of the identifier
                if let Some(symbol) = self.resolver.scope_manager().resolve_symbol(name) {
                    symbol.ty.clone()
                } else {
                    // If the symbol is not found, return a default type and add an error
                    let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                    self.errors.push(format!("Undeclared identifier '{}'", name));
                    TypeDescriptor::new(TypeKind::Void) // Default to void for unknown types
                }
            },
            Term::Number(_value) => {
                // For now, assume all numbers are Int
                // In a real implementation, we'd distinguish between Int and Float based on the literal
                TypeDescriptor::new(TypeKind::Int)
            },
            Term::QualifiedRef(entity, field) => {
                // First check if the entity exists
                if let Some(entity_symbol) = self.resolver.scope_manager().resolve_symbol(entity) {
                    // In a real implementation, we'd check if the field exists on the entity
                    // For now, we'll just return a default type
                    TypeDescriptor::new(TypeKind::Sym)
                } else {
                    // Entity doesn't exist
                    let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                    self.errors.push(format!("Undeclared entity '{}'", entity));
                    TypeDescriptor::new(TypeKind::Void)
                }
            },
        }
    }

    fn check_predicate(&mut self, predicate: &Predicate) -> TypeDescriptor {
        // Check if the predicate name is a known function/rule
        if let Some(symbol) = self.resolver.scope_manager().resolve_symbol(&predicate.name) {
            // For now, we'll assume predicates return Void
            // In a real implementation, we'd check the function signature
            TypeDescriptor::new(TypeKind::Void)
        } else {
            // Predicate doesn't exist
            let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
            self.errors.push(format!("Unknown predicate '{}'", predicate.name));
            TypeDescriptor::new(TypeKind::Void)
        }
        
        // Check argument types
        for arg in &predicate.arguments {
            self.check_term(arg);
        }
        
        // Predicates typically don't return values, so Void
        TypeDescriptor::new(TypeKind::Void)
    }

    fn check_action(&mut self, action: &Action) {
        match action {
            Action::Predicate(predicate) => {
                self.check_predicate(predicate);
            },
            Action::Assignment(assignment) => {
                self.check_assignment(assignment);
            },
            Action::Control(control_action) => {
                self.check_control_action(control_action);
            },
        }
    }

    fn check_assignment(&mut self, assignment: &Assignment) {
        // Check the type of the value being assigned
        let value_type = self.check_term(&assignment.value);
        
        // Look up the type of the target variable
        if let Some(target_symbol) = self.resolver.scope_manager().resolve_symbol(&assignment.target) {
            // Check if the assignment is valid
            if !TypeCheckerUtil::validate_compatibility(&target_symbol.ty, &value_type) {
                let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                self.errors.push(TypeError::InvalidAssignment {
                    target_type: target_symbol.ty.clone(),
                    value_type,
                    location,
                }.message());
            }
        } else {
            // Target variable doesn't exist
            let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
            self.errors.push(format!("Cannot assign to undeclared variable '{}'", assignment.target));
        }
    }

    fn check_control_action(&mut self, control_action: &ControlAction) {
        match control_action {
            ControlAction::If(if_action) => {
                self.check_if_action(if_action);
            },
            ControlAction::Loop(loop_action) => {
                self.check_loop_action(loop_action);
            },
            ControlAction::Halt(_halt_action) => {
                // Halt action has no type requirements
            },
        }
    }

    fn check_if_action(&mut self, if_action: &IfAction) {
        // Check that the condition evaluates to Bool
        let condition_type = self.check_condition(&if_action.condition);
        if !TypeCheckerUtil::validate_boolean(&condition_type) {
            let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
            self.errors.push(TypeError::TypeMismatch {
                expected: TypeDescriptor::new(TypeKind::Bool),
                actual: condition_type,
                location,
            }.message());
        }
        
        // Check then actions
        for action in &if_action.then_actions {
            self.check_action(action);
        }
        
        // Check else actions if they exist
        if let Some(else_actions) = &if_action.else_actions {
            for action in else_actions {
                self.check_action(action);
            }
        }
    }

    fn check_loop_action(&mut self, loop_action: &LoopAction) {
        // For now, we just check the actions inside the loop
        // In a real implementation, we might check for termination conditions
        for action in &loop_action.actions {
            self.check_action(action);
        }
    }

    /// Gets the resolver (for access to symbols after type checking)
    pub fn resolver(&self) -> &Resolver {
        &self.resolver
    }

    /// Gets mutable access to the resolver
    pub fn resolver_mut(&mut self) -> &mut Resolver {
        &mut self.resolver
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kern_parser::Parser;

    #[test]
    fn test_type_checker_creation() {
        let resolver = Resolver::new();
        let type_checker = TypeChecker::new(resolver);
        assert_eq!(type_checker.errors.len(), 0);
    }

    #[test]
    fn test_simple_type_checking() {
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

        let mut resolver = Resolver::new();
        resolver.resolve_program(&program).expect("Failed to resolve program");

        let mut type_checker = TypeChecker::new(resolver);
        let result = type_checker.check_program(&program);
        
        // The type checking should pass without errors for this valid program
        assert!(result.is_ok(), "Type checking failed with errors: {:?}", result.err());
    }
}