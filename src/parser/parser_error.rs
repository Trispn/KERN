use crate::shared::source_location::SourceLocation;
use crate::shared::diagnostics::ErrorCode;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub code: ErrorCode,
    pub message: String,
    pub location: SourceLocation,
    pub file: String,
}

impl ParserError {
    pub fn new(code: ErrorCode, message: String, location: SourceLocation, file: String) -> Self {
        Self {
            code,
            message,
            location,
            file,
        }
    }
}