use crate::ast::*;
use crate::symbol_table::{Symbol, SymbolKind, SymbolTable};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Sym,
    Num,
    Bool,
    Vec,
    Ref,
    Ctx,
    // For entities, we'll use the entity name as the type
    Entity(String),
    // For functions/predicates, we'll track input and output types
    Function(Vec<Type>, Box<Type>),
    // Unknown type for type inference
    Unknown,
    // Error type for type checking errors
    Error,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Sym => write!(f, "sym"),
            Type::Num => write!(f, "num"),
            Type::Bool => write!(f, "bool"),
            Type::Vec => write!(f, "vec"),
            Type::Ref => write!(f, "ref"),
            Type::Ctx => write!(f, "ctx"),
            Type::Entity(name) => write!(f, "entity({})", name),
            Type::Function(inputs, output) => {
                write!(f, "fn(")?;
                for (i, input) in inputs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", input)?;
                }
                write!(f, ") -> {}", output)
            }
            Type::Unknown => write!(f, "unknown"),
            Type::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeError {
    pub message: String,
    pub node: AstNode, // The AST node where the error occurred
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type error: {}", self.message)
    }
}

pub struct TypeChecker {
    // Symbol table for scope resolution
    symbol_table: SymbolTable,
    // Map from variable/identifier names to their types
    type_env: HashMap<String, Type>,
    // Map from entity names to their field types
    entity_fields: HashMap<String, HashMap<String, Type>>,
    // Map from predicate names to their signature
    #[allow(dead_code)]
    predicate_signatures: HashMap<String, Type>,
    // List of type errors encountered
    errors: Vec<TypeError>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            symbol_table: SymbolTable::new(),
            type_env: HashMap::new(),
            entity_fields: HashMap::new(),
            predicate_signatures: HashMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<TypeError>> {
        // First pass: register all definitions in the symbol table
        for def in &program.definitions {
            match def {
                Definition::Entity(entity_def) => {
                    if let Err(e) = self.symbol_table.register_entity(entity_def) {
                        self.errors.push(TypeError {
                            message: e,
                            node: AstNode::EntityDef(entity_def.clone()),
                        });
                    }
                    self.register_entity(entity_def);
                }
                Definition::Rule(rule_def) => {
                    if let Err(e) = self.symbol_table.register_rule(rule_def) {
                        self.errors.push(TypeError {
                            message: e,
                            node: AstNode::RuleDef(rule_def.clone()),
                        });
                    }
                }
                Definition::Flow(flow_def) => {
                    if let Err(e) = self.symbol_table.register_flow(flow_def) {
                        self.errors.push(TypeError {
                            message: e,
                            node: AstNode::FlowDef(flow_def.clone()),
                        });
                    }
                }
                Definition::Constraint(constraint_def) => {
                    if let Err(e) = self.symbol_table.register_constraint(constraint_def) {
                        self.errors.push(TypeError {
                            message: e,
                            node: AstNode::ConstraintDef(constraint_def.clone()),
                        });
                    }
                }
            }
        }

        // Second pass: check all definitions
        for def in &program.definitions {
            self.check_definition(def);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn register_entity(&mut self, entity_def: &EntityDef) {
        let mut fields = HashMap::new();
        for field in &entity_def.fields {
            // For now, we'll assign unknown type to fields
            // In a real implementation, we might have type annotations
            fields.insert(field.name.clone(), Type::Unknown);
        }
        self.entity_fields.insert(entity_def.name.clone(), fields);
        self.type_env.insert(
            entity_def.name.clone(),
            Type::Entity(entity_def.name.clone()),
        );
    }

    fn check_definition(&mut self, def: &Definition) {
        match def {
            Definition::Entity(entity_def) => self.check_entity_def(entity_def),
            Definition::Rule(rule_def) => self.check_rule_def(rule_def),
            Definition::Flow(flow_def) => self.check_flow_def(flow_def),
            Definition::Constraint(constraint_def) => self.check_constraint_def(constraint_def),
        }
    }

    fn check_entity_def(&mut self, entity_def: &EntityDef) {
        // Check for duplicate field names within the entity
        let mut field_names = std::collections::HashSet::new();
        for field in &entity_def.fields {
            if field_names.contains(&field.name) {
                self.errors.push(TypeError {
                    message: format!(
                        "Duplicate field name '{}' in entity '{}'",
                        field.name, entity_def.name
                    ),
                    node: AstNode::FieldDef(field.clone()),
                });
            } else {
                field_names.insert(&field.name);
            }
        }

        // In a more advanced implementation, we might also check for:
        // - Reserved field names
        // - Field type annotations if the language supports them
    }

    fn check_rule_def(&mut self, rule_def: &RuleDef) {
        // Check the condition type - it should evaluate to a boolean
        let condition_type = self.check_condition(&rule_def.condition);
        if condition_type != Type::Bool && condition_type != Type::Unknown {
            self.errors.push(TypeError {
                message: format!(
                    "Rule condition must evaluate to bool, got {}",
                    condition_type
                ),
                node: AstNode::Condition(rule_def.condition.clone()),
            });
        }

        // Check each action in the rule
        for action in &rule_def.actions {
            self.check_action(action);
        }

        // Additional rule-specific checks could include:
        // - Ensuring variables used in the condition are defined
        // - Checking for side effects in conditions (if not allowed)
        // - Validating that actions are consistent with the rule's purpose
    }

    fn check_flow_def(&mut self, flow_def: &FlowDef) {
        // Check each action in the flow
        for action in &flow_def.actions {
            self.check_action(action);
        }

        // Additional flow-specific checks could include:
        // - Ensuring the flow doesn't have unreachable code
        // - Checking for proper termination if required
        // - Validating that variables used are properly initialized
    }

    fn check_constraint_def(&mut self, constraint_def: &ConstraintDef) {
        // Check the condition type - it should evaluate to a boolean
        let condition_type = self.check_condition(&constraint_def.condition);
        if condition_type != Type::Bool && condition_type != Type::Unknown {
            self.errors.push(TypeError {
                message: format!(
                    "Constraint condition must evaluate to bool, got {}",
                    condition_type
                ),
                node: AstNode::Condition(constraint_def.condition.clone()),
            });
        }

        // Additional constraint-specific checks could include:
        // - Ensuring the constraint doesn't have side effects
        // - Checking that the constraint only refers to valid entities and fields
        // - Validating that the constraint expression is pure (no side effects)
    }

    fn check_condition(&mut self, condition: &Condition) -> Type {
        match condition {
            Condition::Expression(expr) => self.check_expression(expr),
            Condition::LogicalOp(left, _op, right) => {
                let left_type = self.check_condition(left);
                let right_type = self.check_condition(right);

                // Both operands of logical operators should be boolean
                if left_type != Type::Bool && left_type != Type::Unknown {
                    self.errors.push(TypeError {
                        message: format!(
                            "Left operand of logical operation must be bool, got {}",
                            left_type
                        ),
                        node: AstNode::Condition(*left.clone()),
                    });
                }

                if right_type != Type::Bool && right_type != Type::Unknown {
                    self.errors.push(TypeError {
                        message: format!(
                            "Right operand of logical operation must be bool, got {}",
                            right_type
                        ),
                        node: AstNode::Condition(*right.clone()),
                    });
                }

                // Result of logical operations is always boolean
                Type::Bool
            }
        }
    }

    fn check_expression(&mut self, expression: &Expression) -> Type {
        match expression {
            Expression::Comparison { left, op: _, right } => {
                let left_type = self.check_term(left);
                let right_type = self.check_term(right);

                // For comparison, both operands should have the same type
                if left_type != right_type
                    && left_type != Type::Unknown
                    && right_type != Type::Unknown
                {
                    self.errors.push(TypeError {
                        message: format!("Cannot compare {} with {}", left_type, right_type),
                        node: AstNode::Expression(expression.clone()),
                    });
                }

                // Result of comparison is always boolean
                Type::Bool
            }
            Expression::Predicate(predicate) => self.check_predicate(predicate),
        }
    }

    fn check_term(&mut self, term: &Term) -> Type {
        match term {
            Term::Identifier(name) => {
                // Use the symbol table for scope resolution
                match self.symbol_table.lookup(name) {
                    Some(symbol) => {
                        // Return the appropriate type based on the symbol kind
                        match symbol.kind {
                            SymbolKind::Entity => Type::Entity(name.clone()),
                            SymbolKind::Variable | SymbolKind::Parameter => {
                                // Look up the type in the environment
                                match self.type_env.get(name) {
                                    Some(ty) => ty.clone(),
                                    None => {
                                        // If not in type environment, return unknown
                                        Type::Unknown
                                    }
                                }
                            }
                            SymbolKind::Field => {
                                // For fields, we need to determine the type from context
                                // This would typically be handled by qualified references
                                Type::Unknown
                            }
                            SymbolKind::Rule | SymbolKind::Flow => Type::Unknown, // Rules and flows don't have a direct type
                            SymbolKind::Predicate => Type::Unknown, // Predicates return types based on their signature
                        }
                    }
                    None => {
                        // Undefined variable
                        self.errors.push(TypeError {
                            message: format!("Undefined variable: {}", name),
                            node: AstNode::Term(term.clone()),
                        });
                        Type::Error
                    }
                }
            }
            Term::Number(_) => Type::Num,
            Term::QualifiedRef(entity_name, field_name) => {
                // Check if the entity exists in the symbol table
                if self.symbol_table.entity_exists(entity_name) {
                    // Check if the field exists in the entity
                    if let Some(entity_def) = self.symbol_table.get_entity(entity_name) {
                        // Look for the field in the entity definition
                        let field_exists = entity_def
                            .fields
                            .iter()
                            .any(|field| field.name == *field_name);

                        if field_exists {
                            // Return the type of the field if we have it
                            if let Some(fields) = self.entity_fields.get(entity_name) {
                                if let Some(field_type) = fields.get(field_name) {
                                    return field_type.clone();
                                }
                            }
                            // If we don't have the specific field type, return unknown
                            Type::Unknown
                        } else {
                            self.errors.push(TypeError {
                                message: format!(
                                    "Field '{}' does not exist in entity '{}'",
                                    field_name, entity_name
                                ),
                                node: AstNode::Term(term.clone()),
                            });
                            Type::Error
                        }
                    } else {
                        self.errors.push(TypeError {
                            message: format!("Entity '{}' does not exist", entity_name),
                            node: AstNode::Term(term.clone()),
                        });
                        Type::Error
                    }
                } else {
                    self.errors.push(TypeError {
                        message: format!("Entity '{}' does not exist", entity_name),
                        node: AstNode::Term(term.clone()),
                    });
                    Type::Error
                }
            }
        }
    }

    fn check_predicate(&mut self, predicate: &Predicate) -> Type {
        // For now, we'll assume all predicates return boolean
        // In a more sophisticated system, we'd have predicate signatures

        // Check argument types
        for arg in &predicate.arguments {
            self.check_term(arg);
        }

        // For now, assume predicates return bool
        // In a real implementation, we'd look up the predicate signature
        Type::Bool
    }

    fn check_action(&mut self, action: &Action) {
        match action {
            Action::Predicate(predicate) => {
                self.check_predicate(predicate);
            }
            Action::Assignment(assignment) => {
                self.check_assignment(assignment);
            }
            Action::Control(control_action) => {
                self.check_control_action(control_action);
            }
        }
    }

    fn check_assignment(&mut self, assignment: &Assignment) {
        let value_type = self.check_term(&assignment.value);

        // Check if the variable already exists in the symbol table
        if self.symbol_table.lookup(&assignment.variable).is_none() {
            // If the variable doesn't exist, create it in the current scope
            let var_symbol = Symbol::new(
                assignment.variable.clone(),
                SymbolKind::Variable,
                None, // No AST node for runtime variables
                self.symbol_table.get_current_scope(),
                Some(format!("{}", value_type)), // Store the type as string
            );

            if let Err(e) = self
                .symbol_table
                .insert(assignment.variable.clone(), var_symbol)
            {
                self.errors.push(TypeError {
                    message: e,
                    node: AstNode::Assignment(assignment.clone()),
                });
            }
        }

        // Look up or create the variable type in the environment
        let var_type = self
            .type_env
            .entry(assignment.variable.clone())
            .or_insert(value_type.clone());

        // Check if types are compatible
        if *var_type != value_type && *var_type != Type::Unknown && value_type != Type::Unknown {
            self.errors.push(TypeError {
                message: format!(
                    "Cannot assign {} to variable of type {}",
                    value_type, var_type
                ),
                node: AstNode::Assignment(assignment.clone()),
            });
        } else {
            // Update the variable type if it was unknown
            if *var_type == Type::Unknown {
                *var_type = value_type;
            }
        }
    }

    fn check_control_action(&mut self, control_action: &ControlAction) {
        match control_action {
            ControlAction::If(if_action) => self.check_if_action(if_action),
            ControlAction::Loop(loop_action) => self.check_loop_action(loop_action),
            ControlAction::Halt(_) => {} // Halt action has no type requirements
        }
    }

    fn check_if_action(&mut self, if_action: &IfAction) {
        // Check the condition type - it should be boolean
        let condition_type = self.check_condition(&if_action.condition);
        if condition_type != Type::Bool && condition_type != Type::Unknown {
            self.errors.push(TypeError {
                message: format!("If condition must evaluate to bool, got {}", condition_type),
                node: AstNode::IfAction(if_action.clone()),
            });
        }

        // Enter a new scope for the then branch
        self.symbol_table.enter_scope();

        // Check then actions
        for action in &if_action.then_actions {
            self.check_action(action);
        }

        // Exit the then branch scope
        self.symbol_table.exit_scope();

        // Enter a new scope for the else branch (if it exists)
        if if_action.else_actions.is_some() {
            self.symbol_table.enter_scope();

            // Check else actions if present
            if let Some(else_actions) = &if_action.else_actions {
                for action in else_actions {
                    self.check_action(action);
                }
            }

            // Exit the else branch scope
            self.symbol_table.exit_scope();
        }
    }

    fn check_loop_action(&mut self, loop_action: &LoopAction) {
        // Enter a new scope for the loop body
        self.symbol_table.enter_scope();

        // Check each action in the loop
        for action in &loop_action.actions {
            self.check_action(action);
        }

        // Exit the loop body scope
        self.symbol_table.exit_scope();
    }

    pub fn get_errors(&self) -> &[TypeError] {
        &self.errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_checker_basic() {
        let mut type_checker = TypeChecker::new();

        // Create a simple program with an entity and a rule
        let program = Program {
            definitions: vec![
                Definition::Entity(EntityDef {
                    name: "Farmer".to_string(),
                    fields: vec![
                        FieldDef {
                            name: "id".to_string(),
                        },
                        FieldDef {
                            name: "location".to_string(),
                        },
                    ],
                }),
                Definition::Rule(RuleDef {
                    name: "CheckFarmer".to_string(),
                    condition: Condition::Expression(Expression::Comparison {
                        left: Box::new(Term::QualifiedRef("Farmer".to_string(), "id".to_string())),
                        op: Comparator::Greater,
                        right: Box::new(Term::Number(0)),
                    }),
                    actions: vec![Action::Predicate(Predicate {
                        name: "validate_farmer".to_string(),
                        arguments: vec![Term::Identifier("Farmer".to_string())],
                    })],
                }),
            ],
        };

        // This should not cause any type errors
        let result = type_checker.check_program(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_checker_with_error() {
        let mut type_checker = TypeChecker::new();

        // Create a program with a type error
        let program = Program {
            definitions: vec![Definition::Rule(RuleDef {
                name: "InvalidRule".to_string(),
                condition: Condition::Expression(Expression::Comparison {
                    left: Box::new(Term::Number(5)),
                    op: Comparator::Greater,
                    right: Box::new(Term::Identifier("undefined_var".to_string())), // This variable doesn't exist
                }),
                actions: vec![],
            })],
        };

        // This should cause a type error
        let result = type_checker.check_program(&program);
        assert!(result.is_err());

        let errors = result.err().unwrap();
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("Undefined variable"));
    }

    #[test]
    fn test_entity_duplicate_fields() {
        let mut type_checker = TypeChecker::new();

        // Create an entity with duplicate field names
        let program = Program {
            definitions: vec![Definition::Entity(EntityDef {
                name: "TestEntity".to_string(),
                fields: vec![
                    FieldDef {
                        name: "field1".to_string(),
                    },
                    FieldDef {
                        name: "field1".to_string(),
                    }, // Duplicate field name
                ],
            })],
        };

        // This should cause a type error
        let result = type_checker.check_program(&program);
        assert!(result.is_err());

        let errors = result.err().unwrap();
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("Duplicate field name"));
    }

    #[test]
    fn test_qualified_reference_to_nonexistent_entity() {
        let mut type_checker = TypeChecker::new();

        // Create a rule that references a non-existent entity
        let program = Program {
            definitions: vec![Definition::Rule(RuleDef {
                name: "InvalidRule".to_string(),
                condition: Condition::Expression(Expression::Comparison {
                    left: Box::new(Term::QualifiedRef(
                        "nonexistent".to_string(),
                        "field".to_string(),
                    )),
                    op: Comparator::Equal,
                    right: Box::new(Term::Number(42)),
                }),
                actions: vec![],
            })],
        };

        // This should cause a type error
        let result = type_checker.check_program(&program);
        assert!(result.is_err());

        let errors = result.err().unwrap();
        assert!(!errors.is_empty());
        assert!(errors[0]
            .message
            .contains("Entity 'nonexistent' does not exist"));
    }

    #[test]
    fn test_qualified_reference_to_nonexistent_field() {
        let mut type_checker = TypeChecker::new();

        // Create an entity and then try to access a non-existent field
        let program = Program {
            definitions: vec![
                Definition::Entity(EntityDef {
                    name: "TestEntity".to_string(),
                    fields: vec![FieldDef {
                        name: "field1".to_string(),
                    }],
                }),
                Definition::Rule(RuleDef {
                    name: "InvalidRule".to_string(),
                    condition: Condition::Expression(Expression::Comparison {
                        left: Box::new(Term::QualifiedRef(
                            "TestEntity".to_string(),
                            "nonexistent_field".to_string(),
                        )),
                        op: Comparator::Equal,
                        right: Box::new(Term::Number(42)),
                    }),
                    actions: vec![],
                }),
            ],
        };

        // This should cause a type error
        let result = type_checker.check_program(&program);
        assert!(result.is_err());

        let errors = result.err().unwrap();
        assert!(!errors.is_empty());
        assert!(errors[0]
            .message
            .contains("Field 'nonexistent_field' does not exist in entity 'TestEntity'"));
    }

    #[test]
    fn test_constraint_returns_bool() {
        let mut type_checker = TypeChecker::new();

        // Create a constraint with a valid boolean condition
        let program = Program {
            definitions: vec![
                Definition::Entity(EntityDef {
                    name: "TestEntity".to_string(),
                    fields: vec![FieldDef {
                        name: "value".to_string(),
                    }],
                }),
                Definition::Constraint(ConstraintDef {
                    name: "ValidConstraint".to_string(),
                    condition: Condition::Expression(Expression::Comparison {
                        left: Box::new(Term::QualifiedRef(
                            "TestEntity".to_string(),
                            "value".to_string(),
                        )),
                        op: Comparator::Greater,
                        right: Box::new(Term::Number(0)),
                    }),
                }),
            ],
        };

        // This should not cause any type errors
        let result = type_checker.check_program(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_constraint_with_non_bool_condition() {
        let mut type_checker = TypeChecker::new();

        // Create a constraint with a non-boolean condition (which should be an error)
        let program = Program {
            definitions: vec![
                Definition::Entity(EntityDef {
                    name: "TestEntity".to_string(),
                    fields: vec![FieldDef {
                        name: "value".to_string(),
                    }],
                }),
                Definition::Constraint(ConstraintDef {
                    name: "InvalidConstraint".to_string(),
                    condition: Condition::Expression(Expression::Predicate(Predicate {
                        name: "some_function".to_string(),
                        arguments: vec![],
                    })),
                }),
            ],
        };

        // This should cause a type error
        let result = type_checker.check_program(&program);
        assert!(result.is_err());

        let errors = result.err().unwrap();
        assert!(!errors.is_empty());
        assert!(errors[0]
            .message
            .contains("Constraint condition must evaluate to bool"));
    }
}
