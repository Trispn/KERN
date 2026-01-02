use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum BytecodeValidationError {
    InvalidInstruction(String),
    InvalidOperand(String),
    TypeMismatch { expected: String, actual: String },
    UndefinedSymbol(String),
    InvalidJumpTarget(usize),
    StackOverflow,
    StackUnderflow,
    UnsupportedFeature(String),
}

impl std::fmt::Display for BytecodeValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BytecodeValidationError::InvalidInstruction(msg) => {
                write!(f, "Invalid instruction: {}", msg)
            }
            BytecodeValidationError::InvalidOperand(msg) => write!(f, "Invalid operand: {}", msg),
            BytecodeValidationError::TypeMismatch { expected, actual } => {
                write!(f, "Type mismatch: expected {}, got {}", expected, actual)
            }
            BytecodeValidationError::UndefinedSymbol(symbol) => {
                write!(f, "Undefined symbol: {}", symbol)
            }
            BytecodeValidationError::InvalidJumpTarget(target) => {
                write!(f, "Invalid jump target: {}", target)
            }
            BytecodeValidationError::StackOverflow => write!(f, "Stack overflow"),
            BytecodeValidationError::StackUnderflow => write!(f, "Stack underflow"),
            BytecodeValidationError::UnsupportedFeature(feature) => {
                write!(f, "Unsupported feature: {}", feature)
            }
        }
    }
}

#[derive(Debug)]
pub struct BytecodeCompatibilityValidator {
    // Map from instruction names to their expected operand types
    #[allow(dead_code)]
    instruction_operands: HashMap<String, Vec<String>>,
    // Map from symbol names to their types
    symbol_types: HashMap<String, String>,
    // Map from entity names to their field types
    entity_fields: HashMap<String, HashMap<String, String>>,
    // Errors encountered during validation
    errors: Vec<BytecodeValidationError>,
}

impl BytecodeCompatibilityValidator {
    pub fn new() -> Self {
        let mut instruction_operands = HashMap::new();

        // Define expected operands for KERN bytecode instructions
        // Based on the specification in KERN_LANGUAGE.MD
        instruction_operands.insert("NOP".to_string(), vec![]);
        instruction_operands.insert("JMP".to_string(), vec!["instruction_index".to_string()]);
        instruction_operands.insert(
            "JMP_IF".to_string(),
            vec!["flag_register".to_string(), "instruction_index".to_string()],
        );
        instruction_operands.insert("HALT".to_string(), vec![]);
        instruction_operands.insert(
            "LOAD_SYM".to_string(),
            vec![
                "dest_register".to_string(),
                "symbol_table_index".to_string(),
            ],
        );
        instruction_operands.insert(
            "LOAD_NUM".to_string(),
            vec!["dest_register".to_string(), "immediate_value".to_string()],
        );
        instruction_operands.insert(
            "MOVE".to_string(),
            vec!["dest".to_string(), "src".to_string()],
        );
        instruction_operands.insert(
            "COMPARE".to_string(),
            vec![
                "reg_a".to_string(),
                "reg_b".to_string(),
                "comparator_enum".to_string(),
            ],
        );

        BytecodeCompatibilityValidator {
            instruction_operands,
            symbol_types: HashMap::new(),
            entity_fields: HashMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn validate_program(
        &mut self,
        program: &Program,
    ) -> Result<(), Vec<BytecodeValidationError>> {
        // First, register all symbols from the program
        self.register_symbols_from_program(program);

        // Then validate the program structure for bytecode compatibility
        for def in &program.definitions {
            self.validate_definition(def);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn register_symbols_from_program(&mut self, program: &Program) {
        for def in &program.definitions {
            match def {
                Definition::Entity(entity_def) => {
                    self.symbol_types
                        .insert(entity_def.name.clone(), "entity".to_string());

                    let mut fields = HashMap::new();
                    for field in &entity_def.fields {
                        // For now, we'll use a generic type; in a real implementation,
                        // we might have type annotations
                        fields.insert(field.name.clone(), "unknown".to_string());
                    }
                    self.entity_fields.insert(entity_def.name.clone(), fields);
                }
                Definition::Rule(rule_def) => {
                    self.symbol_types
                        .insert(rule_def.name.clone(), "rule".to_string());
                }
                Definition::Flow(flow_def) => {
                    self.symbol_types
                        .insert(flow_def.name.clone(), "flow".to_string());
                }
                Definition::Constraint(constraint_def) => {
                    self.symbol_types
                        .insert(constraint_def.name.clone(), "constraint".to_string());
                }
            }
        }
    }

    fn validate_definition(&mut self, def: &Definition) {
        match def {
            Definition::Entity(entity_def) => self.validate_entity_def(entity_def),
            Definition::Rule(rule_def) => self.validate_rule_def(rule_def),
            Definition::Flow(flow_def) => self.validate_flow_def(flow_def),
            Definition::Constraint(constraint_def) => self.validate_constraint_def(constraint_def),
        }
    }

    fn validate_entity_def(&mut self, entity_def: &EntityDef) {
        // Check for duplicate field names
        let mut field_names = std::collections::HashSet::new();
        for field in &entity_def.fields {
            if field_names.contains(&field.name) {
                self.errors
                    .push(BytecodeValidationError::InvalidOperand(format!(
                        "Duplicate field name '{}' in entity '{}'",
                        field.name, entity_def.name
                    )));
            } else {
                field_names.insert(&field.name);
            }
        }
    }

    fn validate_rule_def(&mut self, rule_def: &RuleDef) {
        // Validate the condition
        self.validate_condition(&rule_def.condition);

        // Validate each action
        for action in &rule_def.actions {
            self.validate_action(action);
        }
    }

    fn validate_flow_def(&mut self, flow_def: &FlowDef) {
        // Validate each action in the flow
        for action in &flow_def.actions {
            self.validate_action(action);
        }
    }

    fn validate_constraint_def(&mut self, constraint_def: &ConstraintDef) {
        // Validate the condition
        self.validate_condition(&constraint_def.condition);
    }

    fn validate_condition(&mut self, condition: &Condition) {
        match condition {
            Condition::Expression(expr) => self.validate_expression(expr),
            Condition::LogicalOp(left, _, right) => {
                self.validate_condition(left);
                self.validate_condition(right);
            }
        }
    }

    fn validate_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Comparison { left, right, .. } => {
                self.validate_term(left);
                self.validate_term(right);
            }
            Expression::Predicate(predicate) => {
                // Check if the predicate is defined
                if !self.symbol_types.contains_key(&predicate.name) {
                    self.errors.push(BytecodeValidationError::UndefinedSymbol(
                        predicate.name.clone(),
                    ));
                }

                // Validate predicate arguments
                for arg in &predicate.arguments {
                    self.validate_term(arg);
                }
            }
        }
    }

    fn validate_term(&mut self, term: &Term) {
        match term {
            Term::Identifier(name) => {
                // Check if the identifier is defined
                if !self.symbol_types.contains_key(name) {
                    self.errors
                        .push(BytecodeValidationError::UndefinedSymbol(name.clone()));
                }
            }
            Term::Number(_) => {
                // Numbers are always valid
            }
            Term::QualifiedRef(entity_name, field_name) => {
                // Check if the entity exists
                if !self.entity_fields.contains_key(entity_name) {
                    self.errors.push(BytecodeValidationError::UndefinedSymbol(
                        entity_name.clone(),
                    ));
                    return;
                }

                // Check if the field exists in the entity
                if let Some(fields) = self.entity_fields.get(entity_name) {
                    if !fields.contains_key(field_name) {
                        self.errors
                            .push(BytecodeValidationError::InvalidOperand(format!(
                                "Field '{}' does not exist in entity '{}'",
                                field_name, entity_name
                            )));
                    }
                }
            }
        }
    }

    fn validate_action(&mut self, action: &Action) {
        match action {
            Action::Predicate(predicate) => {
                // Check if the predicate is defined
                if !self.symbol_types.contains_key(&predicate.name) {
                    self.errors.push(BytecodeValidationError::UndefinedSymbol(
                        predicate.name.clone(),
                    ));
                }

                // Validate predicate arguments
                for arg in &predicate.arguments {
                    self.validate_term(arg);
                }
            }
            Action::Assignment(assignment) => {
                // Validate the value being assigned
                self.validate_term(&assignment.value);
            }
            Action::Control(control_action) => {
                self.validate_control_action(control_action);
            }
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

                // Validate else actions if present
                if let Some(else_actions) = &if_action.else_actions {
                    for action in else_actions {
                        self.validate_action(action);
                    }
                }
            }
            ControlAction::Loop(loop_action) => {
                // Validate loop actions
                for action in &loop_action.actions {
                    self.validate_action(action);
                }
            }
            ControlAction::Halt(_) => {
                // Halt action is always valid
            }
        }
    }

    pub fn get_errors(&self) -> &[BytecodeValidationError] {
        &self.errors
    }

    // Method to validate if a specific AST construct is compatible with bytecode generation
    pub fn is_ast_compatible(&self, node: &AstNode) -> bool {
        // This would check if the AST node can be translated to valid bytecode
        // For now, we'll return true for all nodes, but in a real implementation
        // this would check for unsupported constructs
        match node {
            // All current AST nodes are compatible with bytecode generation
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytecode_validator_basic() {
        let mut validator = BytecodeCompatibilityValidator::new();

        // Create a simple program
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

        // This should not cause any validation errors
        let result = validator.validate_program(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_bytecode_validator_with_error() {
        let mut validator = BytecodeCompatibilityValidator::new();

        // Create a program with an undefined symbol
        let program = Program {
            definitions: vec![Definition::Rule(RuleDef {
                name: "InvalidRule".to_string(),
                condition: Condition::Expression(Expression::Comparison {
                    left: Box::new(Term::Identifier("undefined_var".to_string())),
                    op: Comparator::Greater,
                    right: Box::new(Term::Number(0)),
                }),
                actions: vec![],
            })],
        };

        // This should cause a validation error
        let result = validator.validate_program(&program);
        assert!(result.is_err());

        let errors = result.err().unwrap();
        assert!(!errors.is_empty());
        assert!(matches!(
            errors[0],
            BytecodeValidationError::UndefinedSymbol(_)
        ));
    }
}
