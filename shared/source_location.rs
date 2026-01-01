#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: SourceLocation,
    pub end: SourceLocation,
}