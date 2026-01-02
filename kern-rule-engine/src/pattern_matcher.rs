use crate::types::{Pattern, Value};
use std::collections::HashMap;

pub struct PatternMatcher {
    // Stateless for now
}

impl PatternMatcher {
    pub fn new() -> Self {
        PatternMatcher {}
    }

    /// Matches a pattern against a value and returns bindings if successful
    pub fn match_pattern(
        &self,
        pattern: &Pattern,
        value: &Value,
    ) -> Option<HashMap<String, Value>> {
        let mut bindings = HashMap::new();
        if self.match_pattern_with_bindings(pattern, value, &mut bindings) {
            Some(bindings)
        } else {
            None
        }
    }

    /// Internal function to match a pattern with variable bindings
    pub fn match_pattern_with_bindings(
        &self,
        pattern: &Pattern,
        value: &Value,
        bindings: &mut HashMap<String, Value>,
    ) -> bool {
        match (pattern, value) {
            (Pattern::Value(expected), actual) => expected == actual,
            (Pattern::Variable(name), val) => {
                if let Some(existing) = bindings.get(name) {
                    existing == val
                } else {
                    bindings.insert(name.clone(), val.clone());
                    true
                }
            }
            (Pattern::Composite(_head, patterns), Value::Vec(values)) => {
                if patterns.len() != values.len() {
                    return false;
                }
                for (p, v) in patterns.iter().zip(values.iter()) {
                    if !self.match_pattern_with_bindings(p, v, bindings) {
                        return false;
                    }
                }
                true
            }
            // Fallback for string-like matching for backward compatibility in tests
            // If we have a Pattern::Value(Value::Sym(s)) and a Value::Sym(v), it matches if s == v
            _ => false,
        }
    }

    /// Matches complex patterns (for now equivalent to match_pattern)
    pub fn match_complex_pattern(
        &self,
        pattern: &Pattern,
        value: &Value,
    ) -> Option<HashMap<String, Value>> {
        self.match_pattern(pattern, value)
    }

    /// Internal function to match complex patterns with variable bindings
    pub fn match_complex_pattern_with_bindings(
        &self,
        pattern: &Pattern,
        value: &Value,
        bindings: &mut HashMap<String, Value>,
    ) -> bool {
        self.match_pattern_with_bindings(pattern, value, bindings)
    }
}
