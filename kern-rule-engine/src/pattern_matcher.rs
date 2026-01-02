use std::collections::HashMap;
use crate::{RuleExecutionInfo, RuleMatch};

// Pattern types for matching
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Value(String),
    Variable(String),  // A variable that can match any value
    Composite(String, Vec<Pattern>),  // A composite pattern like (entity.field value)
}

// Pattern matching engine
pub struct PatternMatcher {
    pub patterns: Vec<Pattern>,
}

impl PatternMatcher {
    pub fn new() -> Self {
        PatternMatcher {
            patterns: Vec::new(),
        }
    }

    /// Matches a pattern against the current program state
    pub fn match_pattern(&self, pattern: &Pattern, value: &str) -> Option<HashMap<String, String>> {
        let mut bindings = HashMap::new();
        if self.match_pattern_with_bindings(pattern, value, &mut bindings) {
            Some(bindings)
        } else {
            None
        }
    }

    /// Internal function to match a pattern with variable bindings
    fn match_pattern_with_bindings(&self, pattern: &Pattern, value: &str, bindings: &mut HashMap<String, String>) -> bool {
        match pattern {
            Pattern::Value(expected) => {
                // Direct value comparison
                expected == value
            },
            Pattern::Variable(var_name) => {
                // Check if this variable is already bound
                if let Some(bound_value) = bindings.get(var_name) {
                    // If already bound, the value must match
                    bound_value == value
                } else {
                    // If not bound, bind it to the current value
                    bindings.insert(var_name.clone(), value.to_string());
                    true
                }
            },
            Pattern::Composite(pattern_name, pattern_parts) => {
                // Match structured data patterns
                match (pattern_name.as_str(), value) {
                    ("entity.field", field_value) => {
                        // Match entity.field pattern against field value
                        if let Some(expected_field) = pattern_parts.get(0) {
                            if let Pattern::Value(expected) = expected_field {
                                return field_value == expected;
                            }
                        }
                        false
                    },
                    ("entity", entity_value) => {
                        // Match entity pattern against entity value
                        if let Some(expected_entity) = pattern_parts.get(0) {
                            if let Pattern::Value(expected) = expected_entity {
                                return entity_value == expected;
                            }
                        }
                        false
                    },
                    ("entity.fields", field_values_str) => {
                        // Match entity fields pattern against comma-separated field values
                        let field_values: Vec<&str> = field_values_str.split(',').collect();
                        if pattern_parts.len() == field_values.len() {
                            for (pattern_part, field_value) in pattern_parts.iter().zip(field_values.iter()) {
                                if !self.match_pattern_with_bindings(pattern_part, field_value, bindings) {
                                    return false;
                                }
                            }
                            true
                        } else {
                            false
                        }
                    },
                    _ => false,
                }
            }
        }
    }

    /// Matches a pattern against the execution graph nodes
    pub fn match_graph_pattern(&self, pattern: &Pattern, rule_info: &RuleExecutionInfo) -> Option<RuleMatch> {
        // This is a simplified implementation
        // In a real system, this would match against the execution graph nodes
        let mut bindings = HashMap::new();
        
        // For now, we'll just return a dummy match
        Some(RuleMatch {
            rule_id: rule_info.rule_id,
            bindings,
        })
    }

    /// Enhanced pattern matching engine that supports complex pattern matching
    pub fn match_complex_pattern(&self, pattern: &Pattern, value: &str) -> Option<HashMap<String, String>> {
        let mut bindings = HashMap::new();
        if self.match_complex_pattern_with_bindings(pattern, value, &mut bindings) {
            Some(bindings)
        } else {
            None
        }
    }

    /// Internal function to match complex patterns with variable bindings
    fn match_complex_pattern_with_bindings(&self, pattern: &Pattern, value: &str, bindings: &mut HashMap<String, String>) -> bool {
        match pattern {
            Pattern::Value(expected) => {
                // Direct value comparison
                expected == value
            },
            Pattern::Variable(var_name) => {
                // Check if this variable is already bound
                if let Some(bound_value) = bindings.get(var_name) {
                    // If already bound, the value must match
                    bound_value == value
                } else {
                    // If not bound, bind it to the current value
                    bindings.insert(var_name.clone(), value.to_string());
                    true
                }
            },
            Pattern::Composite(pattern_name, pattern_parts) => {
                match (pattern_name.as_str(), value) {
                    // Match entity.field pattern
                    ("entity.field", field_value) => {
                        if let Some(expected_field) = pattern_parts.get(0) {
                            if let Pattern::Value(expected) = expected_field {
                                return field_value == expected;
                            }
                        }
                        false
                    },
                    // Match entity pattern
                    ("entity", entity_value) => {
                        if let Some(expected_entity) = pattern_parts.get(0) {
                            if let Pattern::Value(expected) = expected_entity {
                                return entity_value == expected;
                            }
                        }
                        false
                    },
                    // Match entity fields pattern
                    ("entity.fields", field_values_str) => {
                        let field_values: Vec<&str> = field_values_str.split(',').collect();
                        if pattern_parts.len() == field_values.len() {
                            for (pattern_part, field_value) in pattern_parts.iter().zip(field_values.iter()) {
                                if !self.match_complex_pattern_with_bindings(pattern_part, field_value, bindings) {
                                    return false;
                                }
                            }
                            true
                        } else {
                            false
                        }
                    },
                    // Match any pattern (wildcard)
                    ("any", _) => true,
                    // Match type pattern
                    ("type.str", _) => true,  // All values are strings in this simplified implementation
                    _ => false,
                }
            }
        }
    }

    /// Matches multiple patterns against a set of values (for rule conditions)
    pub fn match_multiple_patterns(&self, patterns: &[Pattern], values: &[String]) -> Option<Vec<HashMap<String, String>>> {
        if patterns.len() != values.len() {
            return None;
        }

        let mut all_bindings = Vec::new();
        let mut global_bindings = HashMap::new();

        for (pattern, value) in patterns.iter().zip(values.iter()) {
            let mut local_bindings = global_bindings.clone();
            if self.match_pattern_with_bindings(pattern, value, &mut local_bindings) {
                // Update global bindings with new bindings
                global_bindings = local_bindings;
                all_bindings.push(global_bindings.clone());
            } else {
                return None; // Pattern didn't match
            }
        }

        Some(all_bindings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_matching() {
        let matcher = PatternMatcher::new();
        
        // Test simple value matching
        let value_pattern = Pattern::Value("test_value".to_string());
        let result = matcher.match_pattern(&value_pattern, "test_value");
        assert!(result.is_some());
        
        // Test variable binding
        let var_pattern = Pattern::Variable("x".to_string());
        let result = matcher.match_pattern(&var_pattern, "bound_value");
        assert!(result.is_some());
        if let Some(bindings) = result {
            assert_eq!(bindings.get("x"), Some(&"bound_value".to_string()));
        }
        
        // Test composite pattern
        let composite_pattern = Pattern::Composite(
            "entity.field".to_string(),
            vec![Pattern::Value("location".to_string())]
        );
        let result = matcher.match_pattern(&composite_pattern, "location");
        assert!(result.is_some());
    }
}