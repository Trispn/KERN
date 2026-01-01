use crate::SourceLocation;

/// IdentifierNode represents an identifier in the KERN language.
/// 
/// Identifiers are interned strings with source location tracking.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentifierNode {
    /// Text of the identifier (interned string ID)
    pub text: String,  // In a real implementation, this would be an interned_string_id
    
    /// Source location of the identifier
    pub location: SourceLocation,
}

impl IdentifierNode {
    /// Creates a new identifier node
    pub fn new(text: String, location: SourceLocation) -> Self {
        IdentifierNode { text, location }
    }
    
    /// Creates a new identifier node with default location
    pub fn new_with_default_location(text: String) -> Self {
        IdentifierNode {
            text,
            location: SourceLocation::default(),
        }
    }
    
    /// Returns the text of the identifier
    pub fn text(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_node_creation() {
        let loc = SourceLocation::new(1, 10, 5, 3);
        let ident = IdentifierNode::new("test".to_string(), loc);
        
        assert_eq!(ident.text(), "test");
        assert_eq!(ident.location.line, 10);
    }
}