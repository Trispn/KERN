use crate::shared::source_location::SourceLocation;

#[derive(Debug, Clone)]
pub enum ErrorCode {
    UnexpectedCharacter,
    UnterminatedString,
    InvalidNumber,
    ExpectedToken,
    UnexpectedToken,
    MissingSemicolon,
    MissingRightParen,
    MissingRightBrace,
    MissingRightBracket,
    ExpectedIdentifier,
    ExpectedExpression,
    ExpectedStatement,
    ExpectedDeclaration,
    ExpectedTypeName,
    ExpectedOperator,
    ExpectedDelimiter,
    ExpectedKeyword,
    InvalidAssignmentTarget,
    ExpectedFunctionName,
    ExpectedParameterList,
    ExpectedArgumentList,
    ExpectedVariableName,
    ExpectedPropertyName,
    ExpectedMemberAccess,
    ExpectedIndex,
    ExpectedCondition,
    ExpectedBody,
    ExpectedBlock,
    ExpectedIfClause,
    ExpectedElseClause,
    ExpectedLoopCondition,
    ExpectedLoopBody,
    ExpectedReturnExpression,
    ExpectedBreakStatement,
    ExpectedContinueStatement,
    ExpectedImportPath,
    ExpectedExportSpecifier,
    ExpectedModuleDeclaration,
    ExpectedEntityDeclaration,
    ExpectedRuleDeclaration,
    ExpectedFlowDeclaration,
    ExpectedConstraintDeclaration,
    ExpectedFieldDeclaration,
    ExpectedRuleCondition,
    ExpectedRuleContext,
    ExpectedRuleAction,
    ExpectedFlowStep,
    ExpectedConstraintCondition,
    ExpectedConstraintAction,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::UnexpectedCharacter => write!(f, "UNEXPECTED_CHARACTER"),
            ErrorCode::UnterminatedString => write!(f, "UNTERMINATED_STRING"),
            ErrorCode::InvalidNumber => write!(f, "INVALID_NUMBER"),
            ErrorCode::ExpectedToken => write!(f, "EXPECTED_TOKEN"),
            ErrorCode::UnexpectedToken => write!(f, "UNEXPECTED_TOKEN"),
            ErrorCode::MissingSemicolon => write!(f, "MISSING_SEMICOLON"),
            ErrorCode::MissingRightParen => write!(f, "MISSING_RIGHT_PAREN"),
            ErrorCode::MissingRightBrace => write!(f, "MISSING_RIGHT_BRACE"),
            ErrorCode::MissingRightBracket => write!(f, "MISSING_RIGHT_BRACKET"),
            ErrorCode::ExpectedIdentifier => write!(f, "EXPECTED_IDENTIFIER"),
            ErrorCode::ExpectedExpression => write!(f, "EXPECTED_EXPRESSION"),
            ErrorCode::ExpectedStatement => write!(f, "EXPECTED_STATEMENT"),
            ErrorCode::ExpectedDeclaration => write!(f, "EXPECTED_DECLARATION"),
            ErrorCode::ExpectedTypeName => write!(f, "EXPECTED_TYPE_NAME"),
            ErrorCode::ExpectedOperator => write!(f, "EXPECTED_OPERATOR"),
            ErrorCode::ExpectedDelimiter => write!(f, "EXPECTED_DELIMITER"),
            ErrorCode::ExpectedKeyword => write!(f, "EXPECTED_KEYWORD"),
            ErrorCode::InvalidAssignmentTarget => write!(f, "INVALID_ASSIGNMENT_TARGET"),
            ErrorCode::ExpectedFunctionName => write!(f, "EXPECTED_FUNCTION_NAME"),
            ErrorCode::ExpectedParameterList => write!(f, "EXPECTED_PARAMETER_LIST"),
            ErrorCode::ExpectedArgumentList => write!(f, "EXPECTED_ARGUMENT_LIST"),
            ErrorCode::ExpectedVariableName => write!(f, "EXPECTED_VARIABLE_NAME"),
            ErrorCode::ExpectedPropertyName => write!(f, "EXPECTED_PROPERTY_NAME"),
            ErrorCode::ExpectedMemberAccess => write!(f, "EXPECTED_MEMBER_ACCESS"),
            ErrorCode::ExpectedIndex => write!(f, "EXPECTED_INDEX"),
            ErrorCode::ExpectedCondition => write!(f, "EXPECTED_CONDITION"),
            ErrorCode::ExpectedBody => write!(f, "EXPECTED_BODY"),
            ErrorCode::ExpectedBlock => write!(f, "EXPECTED_BLOCK"),
            ErrorCode::ExpectedIfClause => write!(f, "EXPECTED_IF_CLAUSE"),
            ErrorCode::ExpectedElseClause => write!(f, "EXPECTED_ELSE_CLAUSE"),
            ErrorCode::ExpectedLoopCondition => write!(f, "EXPECTED_LOOP_CONDITION"),
            ErrorCode::ExpectedLoopBody => write!(f, "EXPECTED_LOOP_BODY"),
            ErrorCode::ExpectedReturnExpression => write!(f, "EXPECTED_RETURN_EXPRESSION"),
            ErrorCode::ExpectedBreakStatement => write!(f, "EXPECTED_BREAK_STATEMENT"),
            ErrorCode::ExpectedContinueStatement => write!(f, "EXPECTED_CONTINUE_STATEMENT"),
            ErrorCode::ExpectedImportPath => write!(f, "EXPECTED_IMPORT_PATH"),
            ErrorCode::ExpectedExportSpecifier => write!(f, "EXPECTED_EXPORT_SPECIFIER"),
            ErrorCode::ExpectedModuleDeclaration => write!(f, "EXPECTED_MODULE_DECLARATION"),
            ErrorCode::ExpectedEntityDeclaration => write!(f, "EXPECTED_ENTITY_DECLARATION"),
            ErrorCode::ExpectedRuleDeclaration => write!(f, "EXPECTED_RULE_DECLARATION"),
            ErrorCode::ExpectedFlowDeclaration => write!(f, "EXPECTED_FLOW_DECLARATION"),
            ErrorCode::ExpectedConstraintDeclaration => write!(f, "EXPECTED_CONSTRAINT_DECLARATION"),
            ErrorCode::ExpectedFieldDeclaration => write!(f, "EXPECTED_FIELD_DECLARATION"),
            ErrorCode::ExpectedRuleCondition => write!(f, "EXPECTED_RULE_CONDITION"),
            ErrorCode::ExpectedRuleContext => write!(f, "EXPECTED_RULE_CONTEXT"),
            ErrorCode::ExpectedRuleAction => write!(f, "EXPECTED_RULE_ACTION"),
            ErrorCode::ExpectedFlowStep => write!(f, "EXPECTED_FLOW_STEP"),
            ErrorCode::ExpectedConstraintCondition => write!(f, "EXPECTED_CONSTRAINT_CONDITION"),
            ErrorCode::ExpectedConstraintAction => write!(f, "EXPECTED_CONSTRAINT_ACTION"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: ErrorCode,
    pub message: String,
    pub location: SourceLocation,
    pub file: String,
}

impl Diagnostic {
    pub fn new(code: ErrorCode, message: String, location: SourceLocation, file: String) -> Self {
        Self {
            code,
            message,
            location,
            file,
        }
    }

    pub fn format(&self) -> String {
        format!(
            "{}:{}:{} {} {}",
            self.file,
            self.location.line,
            self.location.column,
            self.code,
            self.message
        )
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostics {
    pub diagnostics: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn add_error(&mut self, code: ErrorCode, message: String, location: SourceLocation, file: String) {
        self.diagnostics.push(Diagnostic::new(code, message, location, file));
    }

    pub fn has_errors(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    pub fn print_diagnostics(&self) {
        for diagnostic in &self.diagnostics {
            println!("{}", diagnostic.format());
        }
    }
}