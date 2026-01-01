use crate::ast::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum ConflictType {
    ContradictoryConditions,  // Two rules have contradictory conditions
    ConflictingActions,       // Two rules perform conflicting actions
    PriorityConflict,         // Rules have conflicting priorities
    ResourceConflict,         // Rules compete for the same resources
    StateConflict,            // Rules modify the same state in conflicting ways
}

#[derive(Debug, Clone)]
pub struct RuleConflict {
    pub rule1: String,
    pub rule2: String,
    pub conflict_type: ConflictType,
    pub description: String,
}

#[derive(Debug)]
pub struct RuleConflictDetector {
    conflicts: Vec<RuleConflict>,
    rules: Vec<RuleDef>,
}

impl RuleConflictDetector {
    pub fn new() -> Self {
        RuleConflictDetector {
            conflicts: Vec::new(),
            rules: Vec::new(),
        }
    }

    pub fn detect_conflicts(&mut self, program: &Program) -> Result<Vec<RuleConflict>, String> {
        // Extract all rules from the program
        self.rules = program.definitions.iter()
            .filter_map(|def| {
                if let Definition::Rule(rule) = def {
                    Some(rule.clone())
                } else {
                    None
                }
            })
            .collect();

        // Check for conflicts between all pairs of rules
        for i in 0..self.rules.len() {
            for j in (i + 1)..self.rules.len() {
                self.check_rule_pair(&self.rules[i], &self.rules[j]);
            }
        }

        Ok(self.conflicts.clone())
    }

    fn check_rule_pair(&mut self, rule1: &RuleDef, rule2: &RuleDef) {
        // Check for contradictory conditions
        self.check_for_contradictory_conditions(rule1, rule2);
        
        // Check for conflicting actions
        self.check_for_conflicting_actions(rule1, rule2);
        
        // Check for state conflicts
        self.check_for_state_conflicts(rule1, rule2);
    }

    fn check_for_contradictory_conditions(&mut self, rule1: &RuleDef, rule2: &RuleDef) {
        // This is a simplified check - in a real implementation, 
        // we would need more sophisticated logic to determine if conditions are contradictory
        // For now, we'll just check for simple cases
        
        // Check if both rules have the same condition (which would be redundant, not contradictory)
        if self.conditions_are_equivalent(&rule1.condition, &rule2.condition) {
            self.conflicts.push(RuleConflict {
                rule1: rule1.name.clone(),
                rule2: rule2.name.clone(),
                conflict_type: ConflictType::ContradictoryConditions,
                description: format!("Rules '{}' and '{}' have equivalent conditions", rule1.name, rule2.name),
            });
        }
        
        // In a more sophisticated implementation, we would check for logical contradictions
        // For example: one rule fires when x > 5, another when x <= 5, and they have conflicting actions
    }

    fn check_for_conflicting_actions(&mut self, rule1: &RuleDef, rule2: &RuleDef) {
        // Check if rules perform conflicting assignments to the same variable
        let rule1_assignments = self.extract_assignments(&rule1.actions);
        let rule2_assignments = self.extract_assignments(&rule2.actions);
        
        for (var1, _) in &rule1_assignments {
            for (var2, _) in &rule2_assignments {
                if var1 == var2 {
                    // Same variable is assigned in both rules - potential conflict
                    // This would be a conflict if both rules can fire simultaneously
                    self.conflicts.push(RuleConflict {
                        rule1: rule1.name.clone(),
                        rule2: rule2.name.clone(),
                        conflict_type: ConflictType::ConflictingActions,
                        description: format!("Rules '{}' and '{}' both assign to variable '{}'", 
                                          rule1.name, rule2.name, var1),
                    });
                }
            }
        }
    }

    fn check_for_state_conflicts(&mut self, rule1: &RuleDef, rule2: &RuleDef) {
        // Check if rules modify the same entities or fields
        let rule1_entity_modifications = self.extract_entity_modifications(&rule1.actions);
        let rule2_entity_modifications = self.extract_entity_modifications(&rule2.actions);
        
        for entity1 in &rule1_entity_modifications {
            for entity2 in &rule2_entity_modifications {
                if entity1 == entity2 {
                    self.conflicts.push(RuleConflict {
                        rule1: rule1.name.clone(),
                        rule2: rule2.name.clone(),
                        conflict_type: ConflictType::StateConflict,
                        description: format!("Rules '{}' and '{}' both modify entity '{}'", 
                                          rule1.name, rule2.name, entity1),
                    });
                }
            }
        }
    }

    fn conditions_are_equivalent(&self, cond1: &Condition, cond2: &Condition) -> bool {
        // This is a simplified check - in a real implementation, 
        // we would need sophisticated logic to determine logical equivalence
        match (cond1, cond2) {
            (Condition::Expression(expr1), Condition::Expression(expr2)) => {
                self.expressions_are_equivalent(expr1, expr2)
            },
            _ => false, // For now, only handle simple expression cases
        }
    }

    fn expressions_are_equivalent(&self, expr1: &Expression, expr2: &Expression) -> bool {
        match (expr1, expr2) {
            (Expression::Comparison { left: left1, op: op1, right: right1 }, 
             Expression::Comparison { left: left2, op: op2, right: right2 }) => {
                // Check if the comparisons are equivalent
                self.terms_are_equivalent(left1, left2) && 
                op1 == op2 && 
                self.terms_are_equivalent(right1, right2)
            },
            (Expression::Predicate(pred1), Expression::Predicate(pred2)) => {
                pred1.name == pred2.name && 
                pred1.arguments.len() == pred2.arguments.len() &&
                pred1.arguments.iter().zip(&pred2.arguments).all(|(a, b)| self.terms_are_equivalent(a, b))
            },
            _ => false,
        }
    }

    fn terms_are_equivalent(&self, term1: &Term, term2: &Term) -> bool {
        match (term1, term2) {
            (Term::Identifier(id1), Term::Identifier(id2)) => id1 == id2,
            (Term::Number(n1), Term::Number(n2)) => n1 == n2,
            (Term::QualifiedRef(entity1, field1), Term::QualifiedRef(entity2, field2)) => {
                entity1 == entity2 && field1 == field2
            },
            _ => false,
        }
    }

    fn extract_assignments(&self, actions: &[Action]) -> Vec<(String, Term)> {
        let mut assignments = Vec::new();
        
        for action in actions {
            if let Action::Assignment(assignment) = action {
                assignments.push((assignment.variable.clone(), assignment.value.clone()));
            }
        }
        
        assignments
    }

    fn extract_entity_modifications(&self, actions: &[Action]) -> HashSet<String> {
        let mut entities = HashSet::new();
        
        for action in actions {
            match action {
                Action::Assignment(assignment) => {
                    // If the assignment is to a qualified reference, extract the entity
                    if let Term::QualifiedRef(entity, _) = &assignment.value {
                        entities.insert(entity.clone());
                    }
                },
                Action::Predicate(predicate) => {
                    // Check if the predicate modifies any entities
                    // This is a simplified check - in a real implementation, 
                    // we would need to know which predicates modify which entities
                    for arg in &predicate.arguments {
                        if let Term::QualifiedRef(entity, _) = arg {
                            entities.insert(entity.clone());
                        }
                    }
                },
                Action::Control(control_action) => {
                    // Recursively check control actions
                    match control_action {
                        ControlAction::If(if_action) => {
                            entities.extend(self.extract_entity_modifications(&if_action.then_actions));
                            if let Some(else_actions) = &if_action.else_actions {
                                entities.extend(self.extract_entity_modifications(else_actions));
                            }
                        },
                        ControlAction::Loop(loop_action) => {
                            entities.extend(self.extract_entity_modifications(&loop_action.actions));
                        },
                        ControlAction::Halt(_) => {}
                    }
                }
            }
        }
        
        entities
    }

    pub fn get_conflicts(&self) -> &[RuleConflict] {
        &self.conflicts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_conflict_detector_no_conflicts() {
        let mut detector = RuleConflictDetector::new();
        
        // Create a program with non-conflicting rules
        let program = Program {
            definitions: vec![
                Definition::Rule(RuleDef {
                    name: "RuleA".to_string(),
                    condition: Condition::Expression(Expression::Comparison {
                        left: Box::new(Term::Identifier("x".to_string())),
                        op: Comparator::Greater,
                        right: Box::new(Term::Number(5)),
                    }),
                    actions: vec![
                        Action::Predicate(Predicate {
                            name: "action_a".to_string(),
                            arguments: vec![],
                        })
                    ],
                }),
                Definition::Rule(RuleDef {
                    name: "RuleB".to_string(),
                    condition: Condition::Expression(Expression::Comparison {
                        left: Box::new(Term::Identifier("y".to_string())),
                        op: Comparator::Less,
                        right: Box::new(Term::Number(10)),
                    }),
                    actions: vec![
                        Action::Predicate(Predicate {
                            name: "action_b".to_string(),
                            arguments: vec![],
                        })
                    ],
                })
            ]
        };

        let conflicts = detector.detect_conflicts(&program).unwrap();
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_rule_conflict_detector_assignment_conflict() {
        let mut detector = RuleConflictDetector::new();
        
        // Create a program with rules that assign to the same variable
        let program = Program {
            definitions: vec![
                Definition::Rule(RuleDef {
                    name: "RuleA".to_string(),
                    condition: Condition::Expression(Expression::Comparison {
                        left: Box::new(Term::Identifier("x".to_string())),
                        op: Comparator::Greater,
                        right: Box::new(Term::Number(5)),
                    }),
                    actions: vec![
                        Action::Assignment(Assignment {
                            variable: "result".to_string(),
                            value: Term::Number(1),
                        })
                    ],
                }),
                Definition::Rule(RuleDef {
                    name: "RuleB".to_string(),
                    condition: Condition::Expression(Expression::Comparison {
                        left: Box::new(Term::Identifier("y".to_string())),
                        op: Comparator::Less,
                        right: Box::new(Term::Number(10)),
                    }),
                    actions: vec![
                        Action::Assignment(Assignment {
                            variable: "result".to_string(),
                            value: Term::Number(2),
                        })
                    ],
                })
            ]
        };

        let conflicts = detector.detect_conflicts(&program).unwrap();
        assert!(!conflicts.is_empty());
        assert_eq!(conflicts[0].conflict_type, ConflictType::ConflictingActions);
        assert!(conflicts[0].description.contains("both assign to variable 'result'"));
    }
}