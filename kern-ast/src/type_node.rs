use crate::{IdentifierNode, SourceLocation};

/// TypeNode represents a type in the KERN language.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeNode {
    /// Name of the type
    pub name: IdentifierNode,
    
    /// Whether the type is nullable
    pub nullable: bool,
    
    /// Source location of the type
    pub location: SourceLocation,
}

impl TypeNode {
    /// Creates a new type node
    pub fn new(name: IdentifierNode, nullable: bool, location: SourceLocation) -> Self {
        TypeNode { name, nullable, location }
    }
    
    /// Creates a new type node with default location
    pub fn new_with_default_location(name: IdentifierNode, nullable: bool) -> Self {
        TypeNode {
            name,
            nullable,
            location: SourceLocation::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_node_creation() {
        let name = IdentifierNode::new_with_default_location("string".to_string());
        let loc = SourceLocation::new(1, 10, 5, 6);
        let type_node = TypeNode::new(name, true, loc);
        
        assert_eq!(type_node.name.text(), "string");
        assert!(type_node.nullable);
        assert_eq!(type_node.location.line, 10);
    }
}