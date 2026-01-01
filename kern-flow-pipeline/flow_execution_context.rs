use std::collections::HashMap;

// Import the shared types
use crate::types::{Value, SymbolTable};

/// FlowExecutionContext tracks step execution and nested flow contexts
#[derive(Debug, Clone)]
pub struct FlowExecutionContext {
    pub flow_id: u32,
    pub step_index: u32,
    pub parent_context: Option<Box<FlowExecutionContext>>,
    pub local_symbols: SymbolTable,
    pub global_symbols: SymbolTable,
    pub active_rule_stack: Vec<u32>,
    pub halted: bool,
    pub break_requested: bool,
    pub continue_requested: bool,
}

impl FlowExecutionContext {
    pub fn new(flow_id: u32) -> Self {
        FlowExecutionContext {
            flow_id,
            step_index: 0,
            parent_context: None,
            local_symbols: SymbolTable::new(),
            global_symbols: SymbolTable::new(),
            active_rule_stack: Vec::new(),
            halted: false,
            break_requested: false,
            continue_requested: false,
        }
    }

    pub fn with_parent(flow_id: u32, parent: FlowExecutionContext) -> Self {
        FlowExecutionContext {
            flow_id,
            step_index: 0,
            parent_context: Some(Box::new(parent)),
            local_symbols: SymbolTable::new(),
            global_symbols: SymbolTable::new(),
            active_rule_stack: Vec::new(),
            halted: false,
            break_requested: false,
            continue_requested: false,
        }
    }

    pub fn get_symbol(&self, name: &str) -> Option<&Value> {
        // First check local symbols
        if let Some(value) = self.local_symbols.get(name) {
            return Some(value);
        }

        // Then check global symbols
        if let Some(value) = self.global_symbols.get(name) {
            return Some(value);
        }

        // Then check parent context if available
        if let Some(parent) = &self.parent_context {
            return parent.get_symbol(name);
        }

        None
    }

    pub fn set_symbol(&mut self, name: &str, value: Value) {
        self.local_symbols.insert(name.to_string(), value);
    }

    pub fn set_global_symbol(&mut self, name: &str, value: Value) {
        self.global_symbols.insert(name.to_string(), value);
    }

    pub fn increment_step(&mut self) {
        self.step_index += 1;
    }

    pub fn push_rule(&mut self, rule_id: u32) {
        self.active_rule_stack.push(rule_id);
    }

    pub fn pop_rule(&mut self) {
        self.active_rule_stack.pop();
    }
}