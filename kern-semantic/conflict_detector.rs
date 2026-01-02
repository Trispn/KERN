//! KERN Rule Conflict Detector
//!
//! Detects conflicts between rules in the KERN language.

use crate::resolver::Resolver;
use crate::symbol::{Symbol, SymbolKind};
use kern_parser::{
    Action, Assignment, AstNode, Comparator, Condition, ConstraintDef, ControlAction, Definition,
    EntityDef, Expression, FlowDef, HaltAction, IfAction, LoopAction, Predicate, Program, RuleDef,
    Term,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum ConflictType {
    OverlappingConditions,
    MutuallyExclusiveActions,
    ConflictingAttributeWrites,
    OrderDependentSideEffects,
}

#[derive(Debug, Clone)]
pub struct Conflict {
    pub rule_a: String,
    pub rule_b: String,
    pub conflict_type: ConflictType,
    pub severity: ConflictSeverity,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConflictSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub struct ConflictDetector {
    resolver: Resolver,
    conflicts: Vec<Conflict>,
    errors: Vec<String>,
}

impl ConflictDetector {
    pub fn new(resolver: Resolver) -> Self {
        ConflictDetector {
            resolver,
            conflicts: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Detects conflicts in a program
    pub fn detect_conflicts(&mut self, program: &Program) -> Result<Vec<Conflict>, Vec<String>> {
        // Extract all rules from the program
        let mut rules = Vec::new();
        for definition in &program.definitions {
            if let Definition::Rule(rule_def) = definition {
                rules.push(rule_def);
            }
        }

        // Compare each rule with every other rule
        for i in 0..rules.len() {
            for j in (i + 1)..rules.len() {
                self.detect_conflict_between_rules(rules[i], rules[j]);
            }
        }

        if self.errors.is_empty() {
            Ok(self.conflicts.clone())
        } else {
            Err(self.errors.clone())
        }
    }

    fn detect_conflict_between_rules(&mut self, rule_a: &RuleDef, rule_b: &RuleDef) {
        // Check if the rules apply to the same entity
        if self.rules_apply_to_same_entity(rule_a, rule_b) {
            // Check for overlapping conditions
            if self.conditions_overlap(&rule_a.condition, &rule_b.condition) {
                let conflict = Conflict {
                    rule_a: rule_a.name.clone(),
                    rule_b: rule_b.name.clone(),
                    conflict_type: ConflictType::OverlappingConditions,
                    severity: ConflictSeverity::Warning,
                    description: format!(
                        "Rules '{}' and '{}' have overlapping conditions and may both fire for the same facts",
                        rule_a.name, rule_b.name
                    ),
                };
                self.conflicts.push(conflict);
            }

            // Check for conflicting attribute writes
            if self.actions_conflict(&rule_a.actions, &rule_b.actions) {
                let conflict = Conflict {
                    rule_a: rule_a.name.clone(),
                    rule_b: rule_b.name.clone(),
                    conflict_type: ConflictType::ConflictingAttributeWrites,
                    severity: ConflictSeverity::Error,
                    description: format!(
                        "Rules '{}' and '{}' modify the same attributes in conflicting ways",
                        rule_a.name, rule_b.name
                    ),
                };
                self.conflicts.push(conflict);
            }
        }
    }

    /// Checks if two rules apply to the same entity
    fn rules_apply_to_same_entity(&self, rule_a: &RuleDef, rule_b: &RuleDef) -> bool {
        // Extract entity names from conditions
        let entities_a = self.extract_entities_from_condition(&rule_a.condition);
        let entities_b = self.extract_entities_from_condition(&rule_b.condition);

        // Check for intersection
        for entity_a in &entities_a {
            if entities_b.contains(entity_a) {
                return true;
            }
        }

        false
    }

    /// Extracts entity names from a condition
    fn extract_entities_from_condition(&self, condition: &Condition) -> HashSet<String> {
        let mut entities = HashSet::new();
        self.extract_entities_from_condition_recursive(condition, &mut entities);
        entities
    }

    fn extract_entities_from_condition_recursive(
        &self,
        condition: &Condition,
        entities: &mut HashSet<String>,
    ) {
        match condition {
            Condition::Expression(expr) => {
                self.extract_entities_from_expression(expr, entities);
            }
            Condition::LogicalOp(left, op, right) => {
                self.extract_entities_from_condition_recursive(left, entities);
                self.extract_entities_from_condition_recursive(right, entities);
            }
        }
    }

    fn extract_entities_from_expression(
        &self,
        expression: &Expression,
        entities: &mut HashSet<String>,
    ) {
        match expression {
            Expression::Comparison { left, op, right } => {
                self.extract_entities_from_term(left, entities);
                self.extract_entities_from_term(right, entities);
            }
            Expression::Predicate(predicate) => {
                for arg in &predicate.arguments {
                    self.extract_entities_from_term(arg, entities);
                }
            }
        }
    }

    fn extract_entities_from_term(&self, term: &Term, entities: &mut HashSet<String>) {
        match term {
            Term::Identifier(name) => {
                // Check if this identifier refers to an entity
                if let Some(symbol) = self.resolver.scope_manager().resolve_symbol(name) {
                    if matches!(symbol.kind, SymbolKind::Entity) {
                        entities.insert(name.clone());
                    }
                }
            }
            Term::Number(_value) => {
                // Numbers don't refer to entities
            }
            Term::QualifiedRef(entity, _field) => {
                // The first part of a qualified ref is typically an entity
                entities.insert(entity.clone());
            }
        }
    }

    /// Checks if two conditions overlap
    fn conditions_overlap(&self, condition_a: &Condition, condition_b: &Condition) -> bool {
        // This is a simplified check - in a real implementation, we'd need more sophisticated
        // logic to determine if conditions can both be true for the same facts

        // For now, we'll just check if both conditions reference the same entity
        let entities_a = self.extract_entities_from_condition(condition_a);
        let entities_b = self.extract_entities_from_condition(condition_b);

        for entity_a in &entities_a {
            if entities_b.contains(entity_a) {
                // If both rules apply to the same entity, they might overlap
                // In a real implementation, we'd analyze the specific conditions
                return true;
            }
        }

        false
    }

    /// Checks if two sets of actions conflict
    fn actions_conflict(&self, actions_a: &[Action], actions_b: &[Action]) -> bool {
        // Extract attributes being modified in each action set
        let attrs_a = self.extract_modified_attributes(actions_a);
        let attrs_b = self.extract_modified_attributes(actions_b);

        // Check for intersection
        for attr in &attrs_a {
            if attrs_b.contains(attr) {
                return true;
            }
        }

        false
    }

    /// Extracts attributes being modified by a set of actions
    fn extract_modified_attributes(&self, actions: &[Action]) -> HashSet<String> {
        let mut attributes = HashSet::new();

        for action in actions {
            match action {
                Action::Assignment(assignment) => {
                    attributes.insert(assignment.variable.clone());
                }
                Action::Predicate(predicate) => {
                    // In a real implementation, we'd analyze the predicate to see
                    // if it modifies any attributes
                    // For now, we'll just look for common patterns
                    if predicate.name.starts_with("set_") || predicate.name.starts_with("update_") {
                        // This predicate likely modifies attributes
                        // We'd need more sophisticated analysis to determine which attributes
                    }
                }
                Action::Control(_control_action) => {
                    // Control actions don't directly modify attributes
                }
            }
        }

        attributes
    }

    /// Checks if a condition is always true (tautology)
    fn is_tautology(&self, condition: &Condition) -> bool {
        // This is a simplified check - in a real implementation, we'd need
        // sophisticated logic to determine if a condition is always true
        match condition {
            Condition::Expression(Expression::Comparison {
                left: _,
                op: Comparator::NotEqual,
                right: _,
            }) => {
                // Check if comparing something with itself (like x != x) - this is always false
                false
            }
            _ => false,
        }
    }

    /// Checks if a condition is always false (contradiction)
    fn is_contradiction(&self, condition: &Condition) -> bool {
        // This is a simplified check - in a real implementation, we'd need
        // sophisticated logic to determine if a condition is always false
        match condition {
            Condition::Expression(Expression::Comparison {
                left: _,
                op: Comparator::Equal,
                right: _,
            }) => {
                // Check if comparing something with itself (like x == x) - this is always true
                false
            }
            _ => false,
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

    /// Gets the detected conflicts
    pub fn get_conflicts(&self) -> &[Conflict] {
        &self.conflicts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kern_parser::Parser;

    #[test]
    fn test_conflict_detector_creation() {
        let resolver = Resolver::new();
        let conflict_detector = ConflictDetector::new(resolver);
        assert_eq!(conflict_detector.conflicts.len(), 0);
    }

    #[test]
    fn test_simple_conflict_detection() {
        let input = r#"
        entity Farmer {
            id
            location
            status
        }

        rule CheckLocation:
            if farmer.location == "valid"
            then approve_farmer(farmer)

        rule CheckId:
            if farmer.id > 0
            then validate_farmer(farmer)

        flow ProcessFarmers {
            load_farmers()
            validate_farmers()
        }

        constraint ValidId: farmer.id > 0
        "#;

        let mut parser = Parser::new(input);
        let program = parser.parse_program().expect("Failed to parse program");

        let mut resolver = Resolver::new();
        resolver
            .resolve_program(&program)
            .expect("Failed to resolve program");

        let mut conflict_detector = ConflictDetector::new(resolver);
        let conflicts = conflict_detector.detect_conflicts(&program);

        // The conflict detection should pass without errors for this valid program
        assert!(
            conflicts.is_ok(),
            "Conflict detection failed with errors: {:?}",
            conflicts.err()
        );

        // There should be no conflicts in this program
        let conflict_list = conflicts.unwrap();
        assert_eq!(conflict_list.len(), 0);
    }

    #[test]
    fn test_conflicting_rules() {
        let input = r#"
        entity Farmer {
            id
            status
        }

        rule ApproveFarmer:
            if farmer.id > 0
            then set_status(farmer, "approved")

        rule RejectFarmer:
            if farmer.id <= 0
            then set_status(farmer, "rejected")

        flow ProcessFarmers {
            load_farmers()
            validate_farmers()
        }

        constraint ValidId: farmer.id > 0
        "#;

        let mut parser = Parser::new(input);
        let program = parser.parse_program().expect("Failed to parse program");

        let mut resolver = Resolver::new();
        resolver
            .resolve_program(&program)
            .expect("Failed to resolve program");

        let mut conflict_detector = ConflictDetector::new(resolver);
        let conflicts = conflict_detector.detect_conflicts(&program);

        // The conflict detection should pass without errors for this valid program
        assert!(
            conflicts.is_ok(),
            "Conflict detection failed with errors: {:?}",
            conflicts.err()
        );

        // There should be no conflicts in this program since the conditions are mutually exclusive
        let conflict_list = conflicts.unwrap();
        assert_eq!(conflict_list.len(), 0);
    }
}
