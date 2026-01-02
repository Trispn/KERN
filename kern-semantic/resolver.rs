//! KERN Symbol Resolver
//!
//! Resolves symbol references in the KERN AST against the symbol table.

use crate::scope::ScopeManager;
use crate::symbol::{SourceLocation, Symbol, SymbolKind};
use crate::types::{TypeDescriptor, TypeKind};
use kern_parser::{
    Action, Assignment, AstNode, Condition, ConstraintDef, ControlAction, Definition, EntityDef,
    Expression, FlowDef, HaltAction, IfAction, LoopAction, Predicate, Program, RuleDef, Term,
};

#[derive(Debug, Clone)]
pub struct Resolver {
    scope_manager: ScopeManager,
    errors: Vec<String>,
}

#[derive(Debug)]
pub enum ResolutionError {
    UndeclaredSymbol(String, SourceLocation),
    DuplicateDeclaration(String, SourceLocation),
    IllegalShadowing(String, SourceLocation),
    TypeMismatch(String, SourceLocation),
}

impl ResolutionError {
    pub fn message(&self) -> String {
        match self {
            ResolutionError::UndeclaredSymbol(name, loc) => {
                format!("Undeclared symbol '{}' at {}:{}", name, loc.file, loc.line)
            }
            ResolutionError::DuplicateDeclaration(name, loc) => {
                format!(
                    "Duplicate declaration of symbol '{}' at {}:{}",
                    name, loc.file, loc.line
                )
            }
            ResolutionError::IllegalShadowing(name, loc) => {
                format!(
                    "Illegal shadowing of symbol '{}' at {}:{}",
                    name, loc.file, loc.line
                )
            }
            ResolutionError::TypeMismatch(msg, loc) => {
                format!("Type mismatch: {} at {}:{}", msg, loc.file, loc.line)
            }
        }
    }
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            scope_manager: ScopeManager::new(),
            errors: Vec::new(),
        }
    }

    /// Resolves all symbols in a program
    pub fn resolve_program(&mut self, program: &Program) -> Result<(), Vec<String>> {
        // First pass: register all top-level declarations
        for definition in &program.definitions {
            self.register_top_level_declaration(definition);
        }

        // Second pass: resolve all references
        for definition in &program.definitions {
            self.resolve_definition(definition);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn register_top_level_declaration(&mut self, definition: &Definition) {
        match definition {
            Definition::Entity(entity_def) => {
                self.register_entity(entity_def);
            }
            Definition::Rule(rule_def) => {
                self.register_rule(rule_def);
            }
            Definition::Flow(flow_def) => {
                self.register_flow(flow_def);
            }
            Definition::Constraint(constraint_def) => {
                self.register_constraint(constraint_def);
            }
        }
    }

    fn register_entity(&mut self, entity_def: &EntityDef) {
        let location = SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
        let entity_type = TypeDescriptor::new_named(
            TypeKind::Entity(entity_def.name.clone()),
            entity_def.name.clone(),
        );

        let entity_symbol = Symbol::new(
            entity_def.name.clone(),
            SymbolKind::Entity,
            entity_type,
            self.scope_manager.current_scope().unwrap().id,
            location.clone(),
        );

        if let Err(e) = self.scope_manager.declare_symbol(entity_symbol) {
            self.errors.push(e);
        }

        // Register attributes in the entity's scope
        self.scope_manager.enter_scope();
        for field in &entity_def.fields {
            let field_type = TypeDescriptor::new(TypeKind::Sym); // Default to Sym type for fields
            let field_symbol = Symbol::new(
                field.name.clone(),
                SymbolKind::Attribute,
                field_type,
                self.scope_manager.current_scope().unwrap().id,
                location.clone(),
            );

            if let Err(e) = self.scope_manager.declare_symbol(field_symbol) {
                self.errors.push(e);
            }
        }
        self.scope_manager.exit_scope().unwrap();
    }

    fn register_rule(&mut self, rule_def: &RuleDef) {
        let location = SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
        let rule_type = TypeDescriptor::new(TypeKind::Void); // Rules have void return type

        let rule_symbol = Symbol::new(
            rule_def.name.clone(),
            SymbolKind::Rule,
            rule_type,
            self.scope_manager.current_scope().unwrap().id,
            location,
        );

        if let Err(e) = self.scope_manager.declare_symbol(rule_symbol) {
            self.errors.push(e);
        }
    }

    fn register_flow(&mut self, flow_def: &FlowDef) {
        let location = SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
        let flow_type = TypeDescriptor::new(TypeKind::Void); // Flows have void return type

        let flow_symbol = Symbol::new(
            flow_def.name.clone(),
            SymbolKind::Flow,
            flow_type,
            self.scope_manager.current_scope().unwrap().id,
            location,
        );

        if let Err(e) = self.scope_manager.declare_symbol(flow_symbol) {
            self.errors.push(e);
        }
    }

    fn register_constraint(&mut self, constraint_def: &ConstraintDef) {
        let location = SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
        let constraint_type = TypeDescriptor::new(TypeKind::Bool); // Constraints evaluate to boolean

        let constraint_symbol = Symbol::new(
            constraint_def.name.clone(),
            SymbolKind::Constraint,
            constraint_type,
            self.scope_manager.current_scope().unwrap().id,
            location,
        );

        if let Err(e) = self.scope_manager.declare_symbol(constraint_symbol) {
            self.errors.push(e);
        }
    }

    fn resolve_definition(&mut self, definition: &Definition) {
        match definition {
            Definition::Entity(entity_def) => {
                self.resolve_entity(entity_def);
            }
            Definition::Rule(rule_def) => {
                self.resolve_rule(rule_def);
            }
            Definition::Flow(flow_def) => {
                self.resolve_flow(flow_def);
            }
            Definition::Constraint(constraint_def) => {
                self.resolve_constraint(constraint_def);
            }
        }
    }

    fn resolve_entity(&mut self, _entity_def: &EntityDef) {
        // Entity definitions are already resolved during registration
    }

    fn resolve_rule(&mut self, rule_def: &RuleDef) {
        // Enter rule scope for parameters
        self.scope_manager.enter_scope();

        // Resolve the condition
        self.resolve_condition(&rule_def.condition);

        // Resolve actions
        for action in &rule_def.actions {
            self.resolve_action(action);
        }

        self.scope_manager.exit_scope().unwrap();
    }

    fn resolve_flow(&mut self, flow_def: &FlowDef) {
        // Enter flow scope
        self.scope_manager.enter_scope();

        // Resolve each action in the flow
        for action in &flow_def.actions {
            self.resolve_action(action);
        }

        self.scope_manager.exit_scope().unwrap();
    }

    fn resolve_constraint(&mut self, constraint_def: &ConstraintDef) {
        // Resolve the constraint condition
        self.resolve_condition(&constraint_def.condition);
    }

    fn resolve_condition(&mut self, condition: &Condition) {
        match condition {
            Condition::Expression(expr) => {
                self.resolve_expression(expr);
            }
            Condition::LogicalOp(left, op, right) => {
                self.resolve_condition(left);
                self.resolve_condition(right);
            }
        }
    }

    fn resolve_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Comparison { left, op, right } => {
                self.resolve_term(left);
                self.resolve_term(right);
            }
            Expression::Predicate(predicate) => {
                self.resolve_predicate(predicate);
            }
        }
    }

    fn resolve_term(&mut self, term: &Term) {
        match term {
            Term::Identifier(name) => {
                // Try to resolve the identifier
                if self.scope_manager.resolve_symbol(name).is_none() {
                    let location = SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                    self.errors
                        .push(ResolutionError::UndeclaredSymbol(name.clone(), location).message());
                }
            }
            Term::Number(_value) => {
                // Numbers are self-resolved
            }
            Term::QualifiedRef(entity, field) => {
                // Resolve the entity
                if self.scope_manager.resolve_symbol(entity).is_none() {
                    let location = SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
                    self.errors.push(
                        ResolutionError::UndeclaredSymbol(entity.clone(), location).message(),
                    );
                }

                // In a real implementation, we'd also verify that the field exists on the entity
            }
        }
    }

    fn resolve_predicate(&mut self, predicate: &Predicate) {
        // Check if the predicate name is a known rule or builtin
        if self.scope_manager.resolve_symbol(&predicate.name).is_none() {
            let location = SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
            self.errors.push(
                ResolutionError::UndeclaredSymbol(predicate.name.clone(), location).message(),
            );
        }

        // Resolve arguments
        for arg in &predicate.arguments {
            self.resolve_term(arg);
        }
    }

    fn resolve_action(&mut self, action: &Action) {
        match action {
            Action::Predicate(predicate) => {
                self.resolve_predicate(predicate);
            }
            Action::Assignment(assignment) => {
                self.resolve_assignment(assignment);
            }
            Action::Control(control_action) => {
                self.resolve_control_action(control_action);
            }
        }
    }

    fn resolve_assignment(&mut self, assignment: &Assignment) {
        // Resolve the value being assigned
        self.resolve_term(&assignment.value);

        // Check if the target variable exists
        if self
            .scope_manager
            .resolve_symbol(&assignment.variable)
            .is_none()
        {
            let location = SourceLocation::new("unknown".to_string(), 0, 0); // In real implementation, get from AST
            self.errors.push(
                ResolutionError::UndeclaredSymbol(assignment.variable.clone(), location).message(),
            );
        }
    }

    fn resolve_control_action(&mut self, control_action: &ControlAction) {
        match control_action {
            ControlAction::If(if_action) => {
                self.resolve_if_action(if_action);
            }
            ControlAction::Loop(loop_action) => {
                self.resolve_loop_action(loop_action);
            }
            ControlAction::Halt(_halt_action) => {
                // Halt action has no symbols to resolve
            }
        }
    }

    fn resolve_if_action(&mut self, if_action: &IfAction) {
        // Resolve the condition
        self.resolve_condition(&if_action.condition);

        // Resolve then actions
        for action in &if_action.then_actions {
            self.resolve_action(action);
        }

        // Resolve else actions if they exist
        if let Some(else_actions) = &if_action.else_actions {
            for action in else_actions {
                self.resolve_action(action);
            }
        }
    }

    fn resolve_loop_action(&mut self, loop_action: &LoopAction) {
        // Resolve actions in the loop body
        for action in &loop_action.actions {
            self.resolve_action(action);
        }
    }

    /// Gets the scope manager (for access to symbols after resolution)
    pub fn scope_manager(&self) -> &ScopeManager {
        &self.scope_manager
    }

    /// Gets mutable access to the scope manager
    pub fn scope_manager_mut(&mut self) -> &mut ScopeManager {
        &mut self.scope_manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kern_parser::Parser;

    #[test]
    fn test_resolver_creation() {
        let resolver = Resolver::new();
        assert_eq!(resolver.errors.len(), 0);
    }

    #[test]
    fn test_simple_resolution() {
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
        let result = resolver.resolve_program(&program);

        // The resolution should pass without errors for this valid program
        assert!(
            result.is_ok(),
            "Resolution failed with errors: {:?}",
            result.err()
        );
    }
}
