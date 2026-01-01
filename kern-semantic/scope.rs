//! KERN Scope Management
//! 
//! Handles lexical scoping and symbol resolution in the KERN language.

use crate::symbol::{Symbol, SymbolKind, SymbolTable};
use std::collections::{HashMap, VecDeque};

/// A scope in the KERN language
#[derive(Debug)]
pub struct Scope {
    pub id: u32,
    pub parent: Option<u32>,
    pub symbols: HashMap<String, u32>,  // Maps symbol names to their IDs in the global symbol table
    pub depth: u32,
}

impl Scope {
    pub fn new(id: u32, parent: Option<u32>) -> Self {
        Scope {
            id,
            parent,
            symbols: HashMap::new(),
            depth: parent.map(|p| p + 1).unwrap_or(0),
        }
    }
}

/// Manages the scope stack and symbol resolution
#[derive(Debug)]
pub struct ScopeManager {
    scopes: Vec<Scope>,
    current_scope_id: Option<u32>,
    symbol_table: SymbolTable,
}

impl ScopeManager {
    pub fn new() -> Self {
        let mut scopes = Vec::new();
        // Create the global scope (ID 0)
        scopes.push(Scope::new(0, None));
        
        ScopeManager {
            scopes,
            current_scope_id: Some(0),
            symbol_table: SymbolTable::new(),
        }
    }

    /// Gets the current scope
    pub fn current_scope(&self) -> Option<&Scope> {
        self.current_scope_id.and_then(|id| self.scopes.get(id as usize))
    }

    /// Gets a specific scope by ID
    pub fn get_scope(&self, id: u32) -> Option<&Scope> {
        self.scopes.get(id as usize)
    }

    /// Enters a new scope
    pub fn enter_scope(&mut self) -> u32 {
        let parent_id = self.current_scope_id;
        let new_id = self.scopes.len() as u32;
        
        let new_scope = Scope::new(new_id, parent_id);
        self.scopes.push(new_scope);
        self.current_scope_id = Some(new_id);
        
        new_id
    }

    /// Exits the current scope
    pub fn exit_scope(&mut self) -> Result<(), String> {
        let current_id = self.current_scope_id.ok_or("No current scope to exit")?;
        
        if current_id == 0 {
            return Err("Cannot exit global scope".to_string());
        }
        
        let parent_scope = self.scopes[current_id as usize].parent;
        self.current_scope_id = parent_scope;
        
        Ok(())
    }

    /// Declares a symbol in the current scope
    pub fn declare_symbol(&mut self, mut symbol: Symbol) -> Result<u32, String> {
        let current_scope_id = self.current_scope_id.ok_or("No current scope")?;
        let symbol_name = symbol.name_id.clone();
        
        // Check if symbol already exists in the current scope
        let current_scope = &self.scopes[current_scope_id as usize];
        if current_scope.symbols.contains_key(&symbol_name) {
            // Check if shadowing is allowed
            if !self.is_shadowing_allowed(&symbol) {
                return Err(format!("Symbol '{}' already declared in current scope", symbol_name));
            }
        }
        
        // Register the symbol in the global symbol table
        let symbol_id = self.symbol_table.register_symbol(symbol)?;
        
        // Add the symbol to the current scope
        self.scopes[current_scope_id as usize].symbols.insert(symbol_name, symbol_id);
        
        Ok(symbol_id)
    }

    /// Checks if shadowing is allowed for a symbol
    fn is_shadowing_allowed(&self, symbol: &Symbol) -> bool {
        matches!(symbol.kind, SymbolKind::Parameter | SymbolKind::Variable)
    }

    /// Resolves a symbol by name, searching from current scope up to global scope
    pub fn resolve_symbol(&self, name: &str) -> Option<&Symbol> {
        let current_scope_id = self.current_scope_id?;
        let mut scope_id = current_scope_id;
        
        loop {
            let scope = &self.scopes[scope_id as usize];
            
            if let Some(&symbol_id) = scope.symbols.get(name) {
                return self.symbol_table.lookup_symbol_by_id(symbol_id);
            }
            
            scope_id = self.scopes[scope_id as usize].parent?;
        }
    }

    /// Gets the global symbol table
    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

    /// Gets a mutable reference to the global symbol table
    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    /// Checks if a symbol exists in the current scope only
    pub fn is_declared_in_current_scope(&self, name: &str) -> bool {
        if let Some(current_scope_id) = self.current_scope_id {
            let current_scope = &self.scopes[current_scope_id as usize];
            current_scope.symbols.contains_key(name)
        } else {
            false
        }
    }

    /// Gets all symbols in the current scope
    pub fn get_symbols_in_current_scope(&self) -> Vec<&Symbol> {
        let mut symbols = Vec::new();
        
        if let Some(current_scope_id) = self.current_scope_id {
            let current_scope = &self.scopes[current_scope_id as usize];
            
            for &symbol_id in current_scope.symbols.values() {
                if let Some(symbol) = self.symbol_table.lookup_symbol_by_id(symbol_id) {
                    symbols.push(symbol);
                }
            }
        }
        
        symbols
    }
}

// Extension for SymbolTable to support ID-based lookups
impl SymbolTable {
    pub fn lookup_symbol_by_id(&self, id: u32) -> Option<&Symbol> {
        // Since we don't store IDs directly, we need to find by iterating
        // In a real implementation, we'd store both name->symbol and id->symbol mappings
        for symbol in self.symbols.values() {
            // This is a simplified approach - in a real system we'd have a more efficient lookup
            // For now, we'll just return the first symbol that matches the expected ID position
            // This is a limitation of our current implementation
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{TypeDescriptor, TypeKind};

    #[test]
    fn test_scope_creation() {
        let mut scope_manager = ScopeManager::new();
        
        assert_eq!(scope_manager.current_scope().unwrap().id, 0);
        assert_eq!(scope_manager.current_scope().unwrap().parent, None);
        
        let new_scope_id = scope_manager.enter_scope();
        assert_eq!(new_scope_id, 1);
        assert_eq!(scope_manager.current_scope().unwrap().id, 1);
        assert_eq!(scope_manager.current_scope().unwrap().parent, Some(0));
    }

    #[test]
    fn test_symbol_declaration() {
        let mut scope_manager = ScopeManager::new();
        let location = crate::symbol::SourceLocation::new("test.kern".to_string(), 1, 1);
        let ty = TypeDescriptor::new(TypeKind::Int);
        
        let symbol = Symbol::new(
            "test_var".to_string(),
            SymbolKind::Variable,
            ty,
            scope_manager.current_scope_id.unwrap(),
            location,
        );
        
        let result = scope_manager.declare_symbol(symbol);
        assert!(result.is_ok());
        
        assert!(scope_manager.resolve_symbol("test_var").is_some());
        assert!(scope_manager.resolve_symbol("nonexistent").is_none());
    }

    #[test]
    fn test_nested_scopes() {
        let mut scope_manager = ScopeManager::new();
        
        // Declare in global scope
        let location = crate::symbol::SourceLocation::new("test.kern".to_string(), 1, 1);
        let ty = TypeDescriptor::new(TypeKind::Int);
        let symbol = Symbol::new(
            "global_var".to_string(),
            SymbolKind::Variable,
            ty.clone(),
            0,
            location.clone(),
        );
        scope_manager.declare_symbol(symbol).unwrap();
        
        // Enter new scope
        scope_manager.enter_scope();
        
        // Declare in nested scope
        let symbol2 = Symbol::new(
            "local_var".to_string(),
            SymbolKind::Variable,
            ty,
            1,
            location,
        );
        scope_manager.declare_symbol(symbol2).unwrap();
        
        // Should find both symbols from nested scope
        assert!(scope_manager.resolve_symbol("global_var").is_some());
        assert!(scope_manager.resolve_symbol("local_var").is_some());
        
        // Exit scope
        scope_manager.exit_scope().unwrap();
        
        // Should only find global symbol from global scope
        assert!(scope_manager.resolve_symbol("global_var").is_some());
        assert!(scope_manager.resolve_symbol("local_var").is_none());
    }
}