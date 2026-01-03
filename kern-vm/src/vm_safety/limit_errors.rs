//! Limit Error Definitions for KERN VM
//! 
//! Defines the unified error codes for limit violations as specified in the safety layer.

/// Unified limit error codes as specified in the KERN VM safety specification
#[derive(Debug, Clone, PartialEq)]
pub enum LimitError {
    /// Memory limit exceeded
    MemoryLimitExceeded(MemoryLimitType),
    
    /// Instruction step limit exceeded
    StepLimitExceeded,
    
    /// Rule recursion/invocation limit exceeded
    RuleLimitExceeded,
    
    /// Loop iteration limit exceeded
    LoopLimitExceeded,
    
    /// Sandbox policy violation
    SandboxViolation,
    
    /// General security violation
    SecurityViolation,
}

impl std::fmt::Display for LimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LimitError::MemoryLimitExceeded(mem_type) => {
                write!(f, "Memory limit exceeded: {:?}", mem_type)
            }
            LimitError::StepLimitExceeded => write!(f, "Instruction step limit exceeded"),
            LimitError::RuleLimitExceeded => write!(f, "Rule invocation/recursion limit exceeded"),
            LimitError::LoopLimitExceeded => write!(f, "Loop iteration limit exceeded"),
            LimitError::SandboxViolation => write!(f, "Sandbox policy violation"),
            LimitError::SecurityViolation => write!(f, "Security violation"),
        }
    }
}

impl std::error::Error for LimitError {}

/// Types of memory limits that can be exceeded
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryLimitType {
    Code,
    Const,
    Stack,
    Heap,
    Meta,
}

impl std::fmt::Display for MemoryLimitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryLimitType::Code => write!(f, "Code"),
            MemoryLimitType::Const => write!(f, "Constant"),
            MemoryLimitType::Stack => write!(f, "Stack"),
            MemoryLimitType::Heap => write!(f, "Heap"),
            MemoryLimitType::Meta => write!(f, "Meta"),
        }
    }
}

/// Result type for operations that can fail with limit errors
pub type LimitResult<T> = Result<T, LimitError>;

/// Trait for types that can be checked for limit violations
pub trait LimitCheck {
    /// Check if this object violates any limits
    fn check_limits(&self) -> LimitResult<()>;
}

/// Helper function to create a memory limit error
pub fn memory_limit_error(mem_type: MemoryLimitType) -> LimitError {
    LimitError::MemoryLimitExceeded(mem_type)
}

/// Helper function to create a step limit error
pub fn step_limit_error() -> LimitError {
    LimitError::StepLimitExceeded
}

/// Helper function to create a rule limit error
pub fn rule_limit_error() -> LimitError {
    LimitError::RuleLimitExceeded
}

/// Helper function to create a loop limit error
pub fn loop_limit_error() -> LimitError {
    LimitError::LoopLimitExceeded
}

/// Helper function to create a sandbox violation error
pub fn sandbox_violation_error() -> LimitError {
    LimitError::SandboxViolation
}

/// Helper function to create a security violation error
pub fn security_violation_error() -> LimitError {
    LimitError::SecurityViolation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_error_creation() {
        let mem_error = memory_limit_error(MemoryLimitType::Heap);
        assert_eq!(mem_error, LimitError::MemoryLimitExceeded(MemoryLimitType::Heap));
        
        let step_error = step_limit_error();
        assert_eq!(step_error, LimitError::StepLimitExceeded);
        
        let rule_error = rule_limit_error();
        assert_eq!(rule_error, LimitError::RuleLimitExceeded);
        
        let loop_error = loop_limit_error();
        assert_eq!(loop_error, LimitError::LoopLimitExceeded);
        
        let sandbox_error = sandbox_violation_error();
        assert_eq!(sandbox_error, LimitError::SandboxViolation);
        
        let security_error = security_violation_error();
        assert_eq!(security_error, LimitError::SecurityViolation);
    }

    #[test]
    fn test_limit_error_display() {
        let mem_error = LimitError::MemoryLimitExceeded(MemoryLimitType::Stack);
        assert_eq!(format!("{}", mem_error), "Memory limit exceeded: Stack");
        
        let step_error = LimitError::StepLimitExceeded;
        assert_eq!(format!("{}", step_error), "Instruction step limit exceeded");
        
        let rule_error = LimitError::RuleLimitExceeded;
        assert_eq!(format!("{}", rule_error), "Rule invocation/recursion limit exceeded");
        
        let loop_error = LimitError::LoopLimitExceeded;
        assert_eq!(format!("{}", loop_error), "Loop iteration limit exceeded");
        
        let sandbox_error = LimitError::SandboxViolation;
        assert_eq!(format!("{}", sandbox_error), "Sandbox policy violation");
        
        let security_error = LimitError::SecurityViolation;
        assert_eq!(format!("{}", security_error), "Security violation");
    }

    #[test]
    fn test_memory_limit_type_display() {
        assert_eq!(format!("{}", MemoryLimitType::Code), "Code");
        assert_eq!(format!("{}", MemoryLimitType::Const), "Constant");
        assert_eq!(format!("{}", MemoryLimitType::Stack), "Stack");
        assert_eq!(format!("{}", MemoryLimitType::Heap), "Heap");
        assert_eq!(format!("{}", MemoryLimitType::Meta), "Meta");
    }

    #[test]
    fn test_limit_result_type() {
        let success: LimitResult<()> = Ok(());
        assert!(success.is_ok());
        
        let error: LimitResult<()> = Err(LimitError::StepLimitExceeded);
        assert!(error.is_err());
        assert_eq!(error.unwrap_err(), LimitError::StepLimitExceeded);
    }
}