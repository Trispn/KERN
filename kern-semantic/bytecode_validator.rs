//! KERN Bytecode Validator
//! 
//! Validates that KERN programs can be safely compiled to bytecode.

use crate::types::{TypeDescriptor, TypeKind};
use crate::resolver::Resolver;
use crate::type_checker::TypeChecker;
use kern_parser::{AstNode, Program, Definition, EntityDef, RuleDef, FlowDef, ConstraintDef, Condition, Expression, Term, Predicate, Action, IfAction, LoopAction, HaltAction, Assignment, ControlAction, Comparator};

#[derive(Debug)]
pub struct BytecodeValidator {
    resolver: Resolver,
    type_checker: TypeChecker,
    errors: Vec<String>,
}

#[derive(Debug)]
pub enum BytecodeValidationError {
    UnsupportedType {
        ty: TypeDescriptor,
        location: crate::symbol::SourceLocation,
    },
    DynamicTypeRequired {
        description: String,
        location: crate::symbol::SourceLocation,
    },
    StackUnderflowRisk {
        operation: String,
        location: crate::symbol::SourceLocation,
    },
    InvalidOpcode {
        opcode: String,
        location: crate::symbol::SourceLocation,
    },
}

impl BytecodeValidationError {
    pub fn message(&self) -> String {
        match self {
            BytecodeValidationError::UnsupportedType { ty, location } => {
                format!(
                    "Unsupported type {:?} for bytecode generation at {}:{}", 
                    ty, location.file, location.line
                )
            },
            BytecodeValidationError::DynamicTypeRequired { description, location } => {
                format!(
                    "Dynamic type required: {} at {}:{}", 
                    description, location.file, location.line
                )
            },
            BytecodeValidationError::StackUnderflowRisk { operation, location } => {
                format!(
                    "Stack underflow risk in operation '{}' at {}:{}", 
                    operation, location.file, location.line
                )
            },
            BytecodeValidationError::InvalidOpcode { opcode, location } => {
                format!(
                    "Invalid opcode '{}' at {}:{}", 
                    opcode, location.file, location.line
                )
            },
        }
    }
}

impl BytecodeValidator {
    pub fn new(resolver: Resolver, type_checker: TypeChecker) -> Self {
        BytecodeValidator {
            resolver,
            type_checker,
            errors: Vec::new(),
        }
    }

    /// Validates that a program can be safely compiled to bytecode
    pub fn validate_program(&mut self, program: &Program) -> Result<(), Vec<String>> {
        // First, perform type checking to ensure all types are valid
        let type_check_result = self.type_checker.check_program(program);
        if let Err(type_errors) = type_check_result {
            self.errors.extend(type_errors);
        }

        // Then validate each definition for bytecode compatibility
        for definition in &program.definitions {
            self.validate_definition(definition);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn validate_definition(&mut self, definition: &Definition) {
        match definition {
            Definition::Entity(entity_def) => {
                self.validate_entity(entity_def);
            },
            Definition::Rule(rule_def) => {
                self.validate_rule(rule_def);
            },
            Definition::Flow(flow_def) => {
                self.validate_flow(flow_def);
            },
            Definition::Constraint(constraint_def) => {
                self.validate_constraint(constraint_def);
            },
        }
    }

    fn validate_entity(&mut self, entity_def: &EntityDef) {
        // Validate that all field types are supported by the bytecode system
        for field in &entity_def.fields {
            // In a real implementation, we'd validate the type of each field
            // For now, we'll just ensure all fields have valid names
            if field.is_empty() {
                let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                self.errors.push(BytecodeValidationError::DynamicTypeRequired {
                    description: format!("Entity '{}' has a field with an empty name", entity_def.name),
                    location,
                }.message());
            }
        }
    }

    fn validate_rule(&mut self, rule_def: &RuleDef) {
        // Validate the condition expression
        self.validate_condition(&rule_def.condition);
        
        // Validate all actions
        for action in &rule_def.actions {
            self.validate_action(action);
        }
    }

    fn validate_flow(&mut self, flow_def: &FlowDef) {
        // Validate all actions in the flow
        for action in &flow_def.actions {
            self.validate_action(action);
        }
    }

    fn validate_constraint(&mut self, constraint_def: &ConstraintDef) {
        // Validate the constraint condition
        self.validate_condition(&constraint_def.condition);
    }

    fn validate_condition(&mut self, condition: &Condition) {
        match condition {
            Condition::Expression(expr) => {
                self.validate_expression(expr);
            },
            Condition::LogicalOp(left, _op, right) => {
                self.validate_condition(left);
                self.validate_condition(right);
            },
        }
    }

    fn validate_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Comparison { left, op, right } => {
                // Validate both operands
                self.validate_term(left);
                self.validate_term(right);
                
                // Validate that the operator is supported in bytecode
                match op {
                    Comparator::Equal | Comparator::NotEqual | 
                    Comparator::Greater | Comparator::Less | 
                    Comparator::GreaterEqual | Comparator::LessEqual => {
                        // These are all supported in bytecode
                    },
                }
            },
            Expression::Predicate(predicate) => {
                self.validate_predicate(predicate);
            },
        }
    }

    fn validate_term(&mut self, term: &Term) {
        match term {
            Term::Identifier(name) => {
                // Validate that the identifier refers to a valid symbol
                if self.resolver.scope_manager().resolve_symbol(name).is_none() {
                    let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                    self.errors.push(format!("Undeclared identifier '{}'", name));
                }
            },
            Term::Number(_value) => {
                // Numbers are always valid in bytecode
            },
            Term::QualifiedRef(entity, field) => {
                // Validate that both entity and field exist
                if self.resolver.scope_manager().resolve_symbol(entity).is_none() {
                    let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                    self.errors.push(format!("Undeclared entity '{}'", entity));
                }
                
                // In a real implementation, we'd also validate that the field exists on the entity
            },
        }
    }

    fn validate_predicate(&mut self, predicate: &Predicate) {
        // Validate that the predicate name is a known function/rule
        if self.resolver.scope_manager().resolve_symbol(&predicate.name).is_none() {
            let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
            self.errors.push(format!("Unknown predicate '{}'", predicate.name));
        }
        
        // Validate all arguments
        for arg in &predicate.arguments {
            self.validate_term(arg);
        }
    }

    fn validate_action(&mut self, action: &Action) {
        match action {
            Action::Predicate(predicate) => {
                self.validate_predicate(predicate);
            },
            Action::Assignment(assignment) => {
                // Validate the value being assigned
                self.validate_term(&assignment.value);
                
                // Validate that the target exists
                if self.resolver.scope_manager().resolve_symbol(&assignment.target).is_none() {
                    let location = crate::symbol::SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                    self.errors.push(format!("Cannot assign to undeclared variable '{}'", assignment.target));
                }
            },
            Action::Control(control_action) => {
                self.validate_control_action(control_action);
            },
        }
    }

    fn validate_control_action(&mut self, control_action: &ControlAction) {
        match control_action {
            ControlAction::If(if_action) => {
                // Validate the condition
                self.validate_condition(&if_action.condition);
                
                // Validate then actions
                for action in &if_action.then_actions {
                    self.validate_action(action);
                }
                
                // Validate else actions if they exist
                if let Some(else_actions) = &if_action.else_actions {
                    for action in else_actions {
                        self.validate_action(action);
                    }
                }
            },
            ControlAction::Loop(loop_action) => {
                // Validate all actions in the loop body
                for action in &loop_action.actions {
                    self.validate_action(action);
                }
            },
            ControlAction::Halt(_halt_action) => {
                // Halt action is always valid in bytecode
            },
        }
    }

    /// Validates that a type can be represented in bytecode
    pub fn validate_type_for_bytecode(&self, ty: &TypeDescriptor) -> bool {
        match &ty.kind {
            TypeKind::Int | TypeKind::Float | TypeKind::Bool | TypeKind::String | 
            TypeKind::Void | TypeKind::Sym | TypeKind::Num | TypeKind::Ref | 
            TypeKind::Vec | TypeKind::Ctx => true,
            TypeKind::Entity(_) => true,  // Entities are supported
            TypeKind::List(inner) => self.validate_type_for_bytecode(inner),
            TypeKind::Optional(inner) => self.validate_type_for_bytecode(inner),
        }
    }

    /// Maps a KERN type to its bytecode representation
    pub fn map_type_to_bytecode(&self, ty: &TypeDescriptor) -> Option<&'static str> {
        match &ty.kind {
            TypeKind::Int => Some("I32"),
            TypeKind::Float => Some("F64"),
            TypeKind::Bool => Some("BOOL"),
            TypeKind::String => Some("STR"),
            TypeKind::Sym => Some("SYM"),
            TypeKind::Num => Some("NUM"),
            TypeKind::Ref => Some("REF"),
            TypeKind::Vec => Some("VEC"),
            TypeKind::Ctx => Some("CTX"),
            TypeKind::Entity(_) => Some("ENTITY_REF"),
            TypeKind::List(_) => Some("LIST"),
            TypeKind::Optional(_) => Some("OPTIONAL"),
            TypeKind::Void => Some("VOID"),
        }
    }

    /// Gets the resolver (for access to symbols)
    pub fn resolver(&self) -> &Resolver {
        &self.resolver
    }

    /// Gets mutable access to the resolver
    pub fn resolver_mut(&mut self) -> &mut Resolver {
        &mut self.resolver
    }

    /// Gets the type checker
    pub fn type_checker(&self) -> &TypeChecker {
        &self.type_checker
    }

    /// Gets mutable access to the type checker
    pub fn type_checker_mut(&mut self) -> &mut TypeChecker {
        &mut self.type_checker
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kern_parser::Parser;

    #[test]
    fn test_bytecode_validator_creation() {
        let resolver = Resolver::new();
        let type_checker = TypeChecker::new(resolver);
        let resolver = type_checker.resolver().clone(); // Get resolver back
        let bytecode_validator = BytecodeValidator::new(resolver, type_checker);
        assert_eq!(bytecode_validator.errors.len(), 0);
    }

    #[test]
    fn test_simple_bytecode_validation() {
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

        let type_checker = TypeChecker::new(resolver);
        let resolver = type_checker.resolver().clone(); // Get resolver back
        let mut bytecode_validator = BytecodeValidator::new(resolver, type_checker);

        let result = bytecode_validator.validate_program(&program);
        
        // The bytecode validation should pass without errors for this valid program
        assert!(result.is_ok(), "Bytecode validation failed with errors: {:?}", result.err());
    }

    #[test]
    fn test_type_to_bytecode_mapping() {
        let resolver = Resolver::new();
        let type_checker = TypeChecker::new(resolver);
        let resolver = type_checker.resolver().clone(); // Get resolver back
        let bytecode_validator = BytecodeValidator::new(resolver, type_checker);

        let int_type = TypeDescriptor::new(TypeKind::Int);
        assert_eq!(bytecode_validator.map_type_to_bytecode(&int_type), Some("I32"));

        let bool_type = TypeDescriptor::new(TypeKind::Bool);
        assert_eq!(bytecode_validator.map_type_to_bytecode(&bool_type), Some("BOOL"));

        let string_type = TypeDescriptor::new(TypeKind::String);
        assert_eq!(bytecode_validator.map_type_to_bytecode(&string_type), Some("STR"));
    }
}