use std::collections::{HashMap, HashSet};
use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Entity,
    Field,
    Variable,
    Rule,
    Flow,
    Predicate,
    Parameter,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub definition: Option<AstNode>, // Optional reference to the AST node that defines this symbol
    pub scope_level: usize,         // For scope resolution
    pub r#type: Option<String>,     // Type information if available
}

impl Symbol {
    pub fn new(name: String, kind: SymbolKind, definition: Option<AstNode>, scope_level: usize, r#type: Option<String>) -> Self {
        Symbol {
            name,
            kind,
            definition,
            scope_level,
            r#type,
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    // Global symbol table
    symbols: HashMap<String, Vec<Symbol>>,
    // Track which symbols are defined at each scope level
    scope_symbols: HashMap<usize, HashSet<String>>,
    // Current scope level
    current_scope: usize,
    // All defined entities
    entities: HashMap<String, EntityDef>,
    // All defined rules
    rules: HashMap<String, RuleDef>,
    // All defined flows
    flows: HashMap<String, FlowDef>,
    // All defined constraints
    constraints: HashMap<String, ConstraintDef>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            scope_symbols: HashMap::new(),
            current_scope: 0,
            entities: HashMap::new(),
            rules: HashMap::new(),
            flows: HashMap::new(),
            constraints: HashMap::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.current_scope += 1;
    }

    pub fn exit_scope(&mut self) {
        // Remove all symbols from the current scope
        if let Some(symbols_to_remove) = self.scope_symbols.remove(&self.current_scope) {
            for symbol_name in symbols_to_remove {
                if let Some(symbol_list) = self.symbols.get_mut(&symbol_name) {
                    symbol_list.retain(|symbol| symbol.scope_level != self.current_scope);
                    if symbol_list.is_empty() {
                        self.symbols.remove(&symbol_name);
                    }
                }
            }
        }
        
        if self.current_scope > 0 {
            self.current_scope -= 1;
        }
    }

    pub fn insert(&mut self, name: String, symbol: Symbol) -> Result<(), String> {
        // Check if symbol already exists in current scope (for some symbol kinds)
        if self.lookup_in_current_scope(&name).is_some() {
            match symbol.kind {
                SymbolKind::Variable | SymbolKind::Parameter => {
                    return Err(format!("Symbol '{}' already exists in current scope", name));
                }
                _ => {} // Other symbol kinds might be allowed to be redefined
            }
        }

        self.symbols.entry(name.clone()).or_insert_with(Vec::new).push(symbol);
        self.scope_symbols.entry(self.current_scope).or_insert_with(HashSet::new).insert(name);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        // Look through scopes from innermost to outermost
        for scope_level in (0..=self.current_scope).rev() {
            if let Some(symbols) = self.symbols.get(name) {
                // Find the symbol with the highest scope level that's <= current scope
                for symbol in symbols.iter().rev() {
                    if symbol.scope_level <= scope_level {
                        return Some(symbol);
                    }
                }
            }
        }
        None
    }

    pub fn lookup_in_current_scope(&self, name: &str) -> Option<&Symbol> {
        if let Some(symbols) = self.symbols.get(name) {
            for symbol in symbols.iter().rev() {
                if symbol.scope_level == self.current_scope {
                    return Some(symbol);
                }
            }
        }
        None
    }

    pub fn get_current_scope(&self) -> usize {
        self.current_scope
    }

    pub fn get_all_symbols_in_scope(&self, scope_level: usize) -> Vec<&Symbol> {
        let mut result = Vec::new();
        if let Some(symbol_names) = self.scope_symbols.get(&scope_level) {
            for name in symbol_names {
                if let Some(symbols) = self.symbols.get(name) {
                    for symbol in symbols {
                        if symbol.scope_level == scope_level {
                            result.push(symbol);
                        }
                    }
                }
            }
        }
        result
    }

    // Methods to register definitions
    pub fn register_entity(&mut self, entity: &EntityDef) -> Result<(), String> {
        let entity_symbol = Symbol::new(
            entity.name.clone(),
            SymbolKind::Entity,
            Some(AstNode::EntityDef(entity.clone())),
            self.current_scope,
            Some("entity".to_string()),
        );

        self.insert(entity.name.clone(), entity_symbol)?;
        self.entities.insert(entity.name.clone(), entity.clone());
        
        // Register fields of the entity
        for field in &entity.fields {
            let field_symbol = Symbol::new(
                format!("{}.{}", entity.name, field.name),
                SymbolKind::Field,
                Some(AstNode::FieldDef(field.clone())),
                self.current_scope,
                None, // Field types would be determined based on context
            );
            
            self.insert(field.name.clone(), field_symbol)?;
        }
        
        Ok(())
    }

    pub fn register_rule(&mut self, rule: &RuleDef) -> Result<(), String> {
        let rule_symbol = Symbol::new(
            rule.name.clone(),
            SymbolKind::Rule,
            Some(AstNode::RuleDef(rule.clone())),
            self.current_scope,
            Some("rule".to_string()),
        );

        self.insert(rule.name.clone(), rule_symbol)?;
        self.rules.insert(rule.name.clone(), rule.clone());
        
        Ok(())
    }

    pub fn register_flow(&mut self, flow: &FlowDef) -> Result<(), String> {
        let flow_symbol = Symbol::new(
            flow.name.clone(),
            SymbolKind::Flow,
            Some(AstNode::FlowDef(flow.clone())),
            self.current_scope,
            Some("flow".to_string()),
        );

        self.insert(flow.name.clone(), flow_symbol)?;
        self.flows.insert(flow.name.clone(), flow.clone());
        
        Ok(())
    }

    pub fn register_constraint(&mut self, constraint: &ConstraintDef) -> Result<(), String> {
        let constraint_symbol = Symbol::new(
            constraint.name.clone(),
            SymbolKind::Variable, // Using Variable for now, could be its own kind
            Some(AstNode::ConstraintDef(constraint.clone())),
            self.current_scope,
            Some("constraint".to_string()),
        );

        self.insert(constraint.name.clone(), constraint_symbol)?;
        self.constraints.insert(constraint.name.clone(), constraint.clone());
        
        Ok(())
    }

    // Check if an entity exists
    pub fn entity_exists(&self, name: &str) -> bool {
        self.entities.contains_key(name)
    }

    // Get an entity definition
    pub fn get_entity(&self, name: &str) -> Option<&EntityDef> {
        self.entities.get(name)
    }

    // Check if a rule exists
    pub fn rule_exists(&self, name: &str) -> bool {
        self.rules.contains_key(name)
    }

    // Get a rule definition
    pub fn get_rule(&self, name: &str) -> Option<&RuleDef> {
        self.rules.get(name)
    }

    // Check if a flow exists
    pub fn flow_exists(&self, name: &str) -> bool {
        self.flows.contains_key(name)
    }

    // Get a flow definition
    pub fn get_flow(&self, name: &str) -> Option<&FlowDef> {
        self.flows.get(name)
    }

    // Check if a constraint exists
    pub fn constraint_exists(&self, name: &str) -> bool {
        self.constraints.contains_key(name)
    }

    // Get a constraint definition
    pub fn get_constraint(&self, name: &str) -> Option<&ConstraintDef> {
        self.constraints.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table_basic() {
        let mut symbol_table = SymbolTable::new();
        
        // Insert a symbol
        let symbol = Symbol::new(
            "test_var".to_string(),
            SymbolKind::Variable,
            None,
            0,
            Some("num".to_string()),
        );
        
        assert!(symbol_table.insert("test_var".to_string(), symbol).is_ok());
        
        // Look up the symbol
        let found_symbol = symbol_table.lookup("test_var");
        assert!(found_symbol.is_some());
        assert_eq!(found_symbol.unwrap().name, "test_var");
    }

    #[test]
    fn test_symbol_table_scoping() {
        let mut symbol_table = SymbolTable::new();
        
        // Insert a symbol in global scope
        let global_symbol = Symbol::new(
            "scoped_var".to_string(),
            SymbolKind::Variable,
            None,
            0,
            Some("num".to_string()),
        );
        symbol_table.insert("scoped_var".to_string(), global_symbol).unwrap();
        
        // Enter a new scope
        symbol_table.enter_scope();
        assert_eq!(symbol_table.get_current_scope(), 1);
        
        // Insert a symbol with the same name in the new scope
        let local_symbol = Symbol::new(
            "scoped_var".to_string(),
            SymbolKind::Variable,
            None,
            1,
            Some("bool".to_string()),
        );
        symbol_table.insert("scoped_var".to_string(), local_symbol).unwrap();
        
        // Look up should return the local symbol
        let found_symbol = symbol_table.lookup("scoped_var");
        assert!(found_symbol.is_some());
        assert_eq!(found_symbol.unwrap().scope_level, 1);
        assert_eq!(found_symbol.unwrap().r#type, Some("bool".to_string()));
        
        // Exit the scope
        symbol_table.exit_scope();
        assert_eq!(symbol_table.get_current_scope(), 0);
        
        // Now lookup should return the global symbol
        let found_symbol = symbol_table.lookup("scoped_var");
        assert!(found_symbol.is_some());
        assert_eq!(found_symbol.unwrap().scope_level, 0);
        assert_eq!(found_symbol.unwrap().r#type, Some("num".to_string()));
    }

    #[test]
    fn test_entity_registration() {
        let mut symbol_table = SymbolTable::new();
        
        // Create an entity
        let entity = EntityDef {
            name: "Farmer".to_string(),
            fields: vec![
                FieldDef { name: "id".to_string() },
                FieldDef { name: "location".to_string() },
            ],
        };
        
        // Register the entity
        assert!(symbol_table.register_entity(&entity).is_ok());
        
        // Check that the entity exists
        assert!(symbol_table.entity_exists("Farmer"));
        assert!(symbol_table.lookup("Farmer").is_some());
        
        // Check that the fields are accessible
        assert!(symbol_table.lookup("id").is_some());
        assert!(symbol_table.lookup("location").is_some());
    }
}