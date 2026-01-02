//! KERN Diagnostic System
//!
//! Provides diagnostic reporting for the KERN semantic analysis system.

use std::fmt;

/// Diagnostic severity levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Fatal,
}

/// Diagnostic codes for different types of issues
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticCode {
    // Type-related diagnostics
    TYPE_MISMATCH,
    INVALID_TYPE,
    UNKNOWN_TYPE,

    // Scope-related diagnostics
    UNDECLARED_SYMBOL,
    DUPLICATE_DECLARATION,
    ILLEGAL_SHADOWING,

    // Dependency-related diagnostics
    CYCLIC_DEPENDENCY,
    SELF_DEPENDENCY,

    // Rule-related diagnostics
    RULE_CONFLICT,
    OVERLAPPING_CONDITIONS,
    MUTUALLY_EXCLUSIVE_ACTIONS,

    // Bytecode-related diagnostics
    UNSUPPORTED_TYPE_FOR_BYTECODE,
    DYNAMIC_TYPE_REQUIRED,
    STACK_UNDERFLOW_RISK,
    INVALIDopCODE,

    // General diagnostics
    SYNTAX_ERROR,
    SEMANTIC_ERROR,
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticCode::TYPE_MISMATCH => write!(f, "TYPE_MISMATCH"),
            DiagnosticCode::INVALID_TYPE => write!(f, "INVALID_TYPE"),
            DiagnosticCode::UNKNOWN_TYPE => write!(f, "UNKNOWN_TYPE"),
            DiagnosticCode::UNDECLARED_SYMBOL => write!(f, "UNDECLARED_SYMBOL"),
            DiagnosticCode::DUPLICATE_DECLARATION => write!(f, "DUPLICATE_DECLARATION"),
            DiagnosticCode::ILLEGAL_SHADOWING => write!(f, "ILLEGAL_SHADOWING"),
            DiagnosticCode::CYCLIC_DEPENDENCY => write!(f, "CYCLIC_DEPENDENCY"),
            DiagnosticCode::SELF_DEPENDENCY => write!(f, "SELF_DEPENDENCY"),
            DiagnosticCode::RULE_CONFLICT => write!(f, "RULE_CONFLICT"),
            DiagnosticCode::OVERLAPPING_CONDITIONS => write!(f, "OVERLAPPING_CONDITIONS"),
            DiagnosticCode::MUTUALLY_EXCLUSIVE_ACTIONS => write!(f, "MUTUALLY_EXCLUSIVE_ACTIONS"),
            DiagnosticCode::UNSUPPORTED_TYPE_FOR_BYTECODE => {
                write!(f, "UNSUPPORTED_TYPE_FOR_BYTECODE")
            }
            DiagnosticCode::DYNAMIC_TYPE_REQUIRED => write!(f, "DYNAMIC_TYPE_REQUIRED"),
            DiagnosticCode::STACK_UNDERFLOW_RISK => write!(f, "STACK_UNDERFLOW_RISK"),
            DiagnosticCode::INVALIDopCODE => write!(f, "INVALIDopCODE"),
            DiagnosticCode::SYNTAX_ERROR => write!(f, "SYNTAX_ERROR"),
            DiagnosticCode::SEMANTIC_ERROR => write!(f, "SEMANTIC_ERROR"),
        }
    }
}

/// Source location for diagnostics
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub length: usize, // Length of the problematic text
}

impl SourceLocation {
    pub fn new(file: String, line: usize, column: usize) -> Self {
        SourceLocation {
            file,
            line,
            column,
            length: 0,
        }
    }

    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }
}

/// A diagnostic message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: DiagnosticCode,
    pub message: String,
    pub location: SourceLocation,
    pub notes: Vec<String>,   // Additional notes about the diagnostic
    pub help: Option<String>, // Suggested fix or help text
}

impl Diagnostic {
    pub fn new(
        severity: Severity,
        code: DiagnosticCode,
        message: String,
        location: SourceLocation,
    ) -> Self {
        Diagnostic {
            severity,
            code,
            message,
            location,
            notes: Vec::new(),
            help: None,
        }
    }

    pub fn with_note(mut self, note: String) -> Self {
        self.notes.push(note);
        self
    }

    pub fn with_help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }

    /// Formats the diagnostic in a human-readable way
    pub fn format(&self) -> String {
        let severity_str = match self.severity {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
            Severity::Fatal => "fatal",
        };

        let mut result = format!(
            "{}:{}:{}: {} {}: {}",
            self.location.file,
            self.location.line,
            self.location.column,
            severity_str,
            self.code,
            self.message
        );

        if !self.notes.is_empty() {
            for note in &self.notes {
                result.push_str(&format!("\n    = note: {}", note));
            }
        }

        if let Some(help) = &self.help {
            result.push_str(&format!("\n    = help: {}", help));
        }

        result
    }
}

/// Diagnostic reporter that collects and manages diagnostics
#[derive(Debug)]
pub struct DiagnosticReporter {
    diagnostics: Vec<Diagnostic>,
    max_errors: usize,   // Maximum number of errors before stopping
    max_warnings: usize, // Maximum number of warnings before stopping
    has_errors: bool,
}

impl DiagnosticReporter {
    pub fn new() -> Self {
        DiagnosticReporter {
            diagnostics: Vec::new(),
            max_errors: 100,   // Default to 100 errors before stopping
            max_warnings: 100, // Default to 100 warnings before stopping
            has_errors: false,
        }
    }

    pub fn with_max_errors(mut self, max: usize) -> Self {
        self.max_errors = max;
        self
    }

    pub fn with_max_warnings(mut self, max: usize) -> Self {
        self.max_warnings = max;
        self
    }

    /// Reports a diagnostic
    pub fn report(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic.clone());

        if matches!(diagnostic.severity, Severity::Error | Severity::Fatal) {
            self.has_errors = true;
        }
    }

    /// Reports an error diagnostic
    pub fn error(&mut self, code: DiagnosticCode, message: String, location: SourceLocation) {
        self.report(Diagnostic::new(Severity::Error, code, message, location));
    }

    /// Reports a warning diagnostic
    pub fn warning(&mut self, code: DiagnosticCode, message: String, location: SourceLocation) {
        self.report(Diagnostic::new(Severity::Warning, code, message, location));
    }

    /// Reports an info diagnostic
    pub fn info(&mut self, code: DiagnosticCode, message: String, location: SourceLocation) {
        self.report(Diagnostic::new(Severity::Info, code, message, location));
    }

    /// Reports a fatal diagnostic
    pub fn fatal(&mut self, code: DiagnosticCode, message: String, location: SourceLocation) {
        self.report(Diagnostic::new(Severity::Fatal, code, message, location));
    }

    /// Checks if there are any errors
    pub fn has_errors(&self) -> bool {
        self.has_errors
    }

    /// Checks if there are any diagnostics of the specified severity
    pub fn has_diagnostics_of_severity(&self, severity: Severity) -> bool {
        self.diagnostics.iter().any(|d| d.severity == severity)
    }

    /// Gets all diagnostics
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Gets all errors
    pub fn errors(&self) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal))
            .collect()
    }

    /// Gets all warnings
    pub fn warnings(&self) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .collect()
    }

    /// Gets all info messages
    pub fn info_messages(&self) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Info)
            .collect()
    }

    /// Prints all diagnostics to standard output
    pub fn print_diagnostics(&self) {
        for diagnostic in &self.diagnostics {
            println!("{}", diagnostic.format());
        }
    }

    /// Checks if the diagnostic limits have been reached
    pub fn limits_reached(&self) -> bool {
        self.errors().len() >= self.max_errors || self.warnings().len() >= self.max_warnings
    }

    /// Gets the count of diagnostics by severity
    pub fn diagnostic_counts(&self) -> (usize, usize, usize, usize) {
        let mut errors = 0;
        let mut warnings = 0;
        let mut infos = 0;
        let mut fatals = 0;

        for diagnostic in &self.diagnostics {
            match diagnostic.severity {
                Severity::Error => errors += 1,
                Severity::Warning => warnings += 1,
                Severity::Info => infos += 1,
                Severity::Fatal => fatals += 1,
            }
        }

        (errors, warnings, infos, fatals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_creation() {
        let location = SourceLocation::new("test.kern".to_string(), 10, 5);
        let diagnostic = Diagnostic::new(
            Severity::Error,
            DiagnosticCode::TYPE_MISMATCH,
            "Type mismatch: expected Int, found Bool".to_string(),
            location,
        );

        assert_eq!(diagnostic.severity, Severity::Error);
        assert_eq!(diagnostic.code, DiagnosticCode::TYPE_MISMATCH);
        assert_eq!(
            diagnostic.message,
            "Type mismatch: expected Int, found Bool"
        );
        assert_eq!(diagnostic.location.file, "test.kern");
        assert_eq!(diagnostic.location.line, 10);
        assert_eq!(diagnostic.location.column, 5);
    }

    #[test]
    fn test_diagnostic_formatting() {
        let location = SourceLocation::new("test.kern".to_string(), 10, 5);
        let diagnostic = Diagnostic::new(
            Severity::Error,
            DiagnosticCode::TYPE_MISMATCH,
            "Type mismatch: expected Int, found Bool".to_string(),
            location,
        )
        .with_note("The variable was declared here".to_string())
        .with_help("Try converting the value to Int".to_string());

        let formatted = diagnostic.format();
        assert!(formatted.contains("error"));
        assert!(formatted.contains("TYPE_MISMATCH"));
        assert!(formatted.contains("Type mismatch: expected Int, found Bool"));
        assert!(formatted.contains("note: The variable was declared here"));
        assert!(formatted.contains("help: Try converting the value to Int"));
    }

    #[test]
    fn test_diagnostic_reporter() {
        let mut reporter = DiagnosticReporter::new();

        let location = SourceLocation::new("test.kern".to_string(), 10, 5);
        reporter.error(
            DiagnosticCode::TYPE_MISMATCH,
            "Type mismatch: expected Int, found Bool".to_string(),
            location.clone(),
        );

        reporter.warning(
            DiagnosticCode::UNDECLARED_SYMBOL,
            "Undeclared symbol 'x'".to_string(),
            location.clone(),
        );

        reporter.info(
            DiagnosticCode::SEMANTIC_ERROR,
            "Semantic analysis completed".to_string(),
            location,
        );

        assert_eq!(reporter.diagnostics().len(), 3);
        assert_eq!(reporter.errors().len(), 1);
        assert_eq!(reporter.warnings().len(), 1);
        assert_eq!(reporter.info_messages().len(), 1);
        assert!(reporter.has_errors());
    }

    #[test]
    fn test_diagnostic_codes_display() {
        assert_eq!(
            format!("{}", DiagnosticCode::TYPE_MISMATCH),
            "TYPE_MISMATCH"
        );
        assert_eq!(
            format!("{}", DiagnosticCode::UNDECLARED_SYMBOL),
            "UNDECLARED_SYMBOL"
        );
        assert_eq!(
            format!("{}", DiagnosticCode::CYCLIC_DEPENDENCY),
            "CYCLIC_DEPENDENCY"
        );
    }
}
