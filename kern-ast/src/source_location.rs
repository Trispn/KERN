/// SourceLocation tracks the position of an AST node in the source code.
/// 
/// This is essential for debugging, error reporting, and maintaining
/// a connection between the AST and the original source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    /// ID of the source file (resolved externally in a string table)
    pub file_id: u32,
    
    /// 1-based line number
    pub line: u32,
    
    /// 1-based column number
    pub column: u32,
    
    /// Length of the token/construct in characters
    pub length: u32,
}

impl SourceLocation {
    /// Creates a new source location
    pub fn new(file_id: u32, line: u32, column: u32, length: u32) -> Self {
        SourceLocation {
            file_id,
            line,
            column,
            length,
        }
    }
    
    /// Creates a default source location (useful for synthetic nodes)
    pub fn default() -> Self {
        SourceLocation {
            file_id: 0,
            line: 0,
            column: 0,
            length: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_location_creation() {
        let loc = SourceLocation::new(1, 10, 5, 3);
        assert_eq!(loc.file_id, 1);
        assert_eq!(loc.line, 10);
        assert_eq!(loc.column, 5);
        assert_eq!(loc.length, 3);
    }
}