//! KERN Symbol Table
//!
//! Manages symbols and their properties in the KERN language.

use crate::types::TypeDescriptor;
use std::collections::HashMap;

/// Different kinds of symbols in KERN
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Entity,
    Attribute,
    Rule,
    Flow,
    Constraint,
    Parameter,
    Variable,
}

/// A source location in the KERN source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl SourceLocation {
    pub fn new(file: String, line: usize, column: usize) -> Self {
        SourceLocation { file, line, column }
    }
}

/// A symbol in the KERN language
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name_id: String,
    pub kind: SymbolKind,
    pub ty: TypeDescriptor,
    pub scope_id: u32,
    pub location: SourceLocation,
    pub is_mutable: bool, // Whether the symbol can be reassigned
    pub is_builtin: bool, // Whether this is a builtin symbol
}

impl Symbol {
    pub fn new(
        name_id: String,
        kind: SymbolKind,
        ty: TypeDescriptor,
        scope_id: u32,
        location: SourceLocation,
    ) -> Self {
        Symbol {
            name_id,
            kind,
            ty,
            scope_id,
            location,
            is_mutable: true, // By default, symbols are mutable
            is_builtin: false,
        }
    }

    pub fn new_builtin(
        name_id: String,
        kind: SymbolKind,
        ty: TypeDescriptor,
        scope_id: u32,
        location: SourceLocation,
    ) -> Self {
        Symbol {
            name_id,
            kind,
            ty,
            scope_id,
            location,
            is_mutable: false, // Builtins are immutable
            is_builtin: true,
        }
    }
}

/// A collection of symbols
#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: HashMap<String, Symbol>,
    next_id: u32,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            next_id: 0,
        }
    }

    /// Registers a new symbol in the table
    pub fn register_symbol(&mut self, symbol: Symbol) -> Result<u32, String> {
        let name = symbol.name_id.clone();

        if self.symbols.contains_key(&name) {
            return Err(format!("Symbol '{}' already declared", name));
        }

        let id = self.next_id;
        self.symbols.insert(name, symbol);
        self.next_id += 1;

        Ok(id)
    }

    /// Looks up a symbol by name
    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// Gets all symbols in the table
    pub fn get_all_symbols(&self) -> Vec<&Symbol> {
        self.symbols.values().collect()
    }

    /// Checks if a symbol exists
    pub fn has_symbol(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// Updates a symbol's type (for type inference or refinement)
    pub fn update_symbol_type(
        &mut self,
        name: &str,
        new_type: TypeDescriptor,
    ) -> Result<(), String> {
        if let Some(symbol) = self.symbols.get_mut(name) {
            symbol.ty = new_type;
            Ok(())
        } else {
            Err(format!("Symbol '{}' not found", name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{TypeDescriptor, TypeKind};

    #[test]
    fn test_symbol_creation() {
        let location = SourceLocation::new("test.kern".to_string(), 1, 1);
        let ty = TypeDescriptor::new(TypeKind::Int);
        let symbol = Symbol::new(
            "test_var".to_string(),
            SymbolKind::Variable,
            ty,
            0,
            location,
        );

        assert_eq!(symbol.name_id, "test_var");
        assert_eq!(symbol.kind, SymbolKind::Variable);
        assert_eq!(symbol.scope_id, 0);
    }

    #[test]
    fn test_symbol_table() {
        let mut table = SymbolTable::new();
        let location = SourceLocation::new("test.kern".to_string(), 1, 1);
        let ty = TypeDescriptor::new(TypeKind::Int);

        let symbol = Symbol::new(
            "test_var".to_string(),
            SymbolKind::Variable,
            ty,
            0,
            location,
        );

        let id = table.register_symbol(symbol).unwrap();
        assert_eq!(id, 0);

        assert!(table.has_symbol("test_var"));
        assert!(!table.has_symbol("nonexistent"));

        let found_symbol = table.lookup_symbol("test_var").unwrap();
        assert_eq!(found_symbol.name_id, "test_var");
    }

    #[test]
    fn test_duplicate_symbol_error() {
        let mut table = SymbolTable::new();
        let location = SourceLocation::new("test.kern".to_string(), 1, 1);
        let ty = TypeDescriptor::new(TypeKind::Int);

        let symbol1 = Symbol::new(
            "test_var".to_string(),
            SymbolKind::Variable,
            ty.clone(),
            0,
            location.clone(),
        );

        let symbol2 = Symbol::new(
            "test_var".to_string(),
            SymbolKind::Variable,
            ty,
            0,
            location,
        );

        table.register_symbol(symbol1).unwrap();
        let result = table.register_symbol(symbol2);
        assert!(result.is_err());
    }
}
