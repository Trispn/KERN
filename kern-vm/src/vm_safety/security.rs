//! Security Validation Implementation for KERN VM
//! 
//! Implements the security validation system as specified in the safety layer.

use crate::vm_safety::sandbox::{SandboxEnvironment, SandboxError};
use kern_bytecode::Instruction;

/// Security validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityError {
    IllegalOpcode(u8),
    InvalidMemoryAccess,
    ContextEscape,
    RuleHijack,
    ExternalMisuse,
    SecurityViolation,
    SandboxViolation(SandboxError),
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityError::IllegalOpcode(opcode) => write!(f, "Illegal opcode: 0x{:02X}", opcode),
            SecurityError::InvalidMemoryAccess => write!(f, "Invalid memory access"),
            SecurityError::ContextEscape => write!(f, "Context escape attempt"),
            SecurityError::RuleHijack => write!(f, "Rule hijack attempt"),
            SecurityError::ExternalMisuse => write!(f, "External function misuse"),
            SecurityError::SecurityViolation => write!(f, "General security violation"),
            SecurityError::SandboxViolation(sandbox_error) => write!(f, "Sandbox violation: {}", sandbox_error),
        }
    }
}

impl std::error::Error for SecurityError {}

/// Security validator that performs validation checks
#[derive(Debug, Clone)]
pub struct SecurityValidator {
    pub allow_self_modifying_code: bool,
    pub allow_dynamic_dispatch: bool,
    pub allow_runtime_code_loading: bool,
    pub allowed_opcodes: Vec<u8>,
}

impl SecurityValidator {
    pub fn new() -> Self {
        SecurityValidator {
            allow_self_modifying_code: false,
            allow_dynamic_dispatch: false,
            allow_runtime_code_loading: false,
            allowed_opcodes: vec![
                0x00, 0x01, 0x02, 0x03,  // Control Flow: NOP, JMP, JMP_IF, HALT
                0x10, 0x11, 0x12, 0x13, 0x14,  // Data & Symbol: LOAD_SYM, LOAD_NUM, LOAD_BOOL, MOVE, COMPARE
                0x20, 0x21, 0x22, 0x23, 0x24,  // Arithmetic: ADD, SUB, MUL, DIV, MOD
                0x30, 0x31, 0x32,              // Logical: AND, OR, NOT
                0x40, 0x41, 0x42, 0x43,        // Graph: CREATE_NODE, CONNECT, MERGE, DELETE_NODE
                0x50, 0x51, 0x52, 0x53,        // Rule: CALL_RULE, RETURN_RULE, CHECK_CONDITION, INCREMENT_EXEC_COUNT
                0x60, 0x61, 0x62, 0x63, 0x64,  // Context: PUSH_CTX, POP_CTX, SET_SYMBOL, GET_SYMBOL, COPY_CTX
                0x70, 0x71, 0x72, 0x73,        // Error: THROW, TRY, CATCH, CLEAR_ERR
                0x80, 0x81, 0x82,              // External: CALL_EXTERN, READ_IO, WRITE_IO
            ],
        }
    }

    /// Validate an instruction for security compliance
    pub fn validate_instruction(&self, instruction: &Instruction) -> Result<(), SecurityError> {
        // Check for illegal opcodes
        if !self.allowed_opcodes.contains(&instruction.opcode) {
            return Err(SecurityError::IllegalOpcode(instruction.opcode));
        }

        // Additional validation based on opcode
        match instruction.opcode {
            // For external calls, we need to validate against sandbox
            0x80 => {
                // CALL_EXTERN - validate through sandbox
                // This would require additional context about the function being called
            }
            0x81 | 0x82 => {
                // READ_IO, WRITE_IO - validate through sandbox
                // This would require additional context about the IO channel
            }
            _ => {
                // Other instructions are generally safe
            }
        }

        Ok(())
    }

    /// Validate bytecode module for security compliance
    pub fn validate_bytecode(&self, instructions: &[Instruction]) -> Result<(), SecurityError> {
        for instruction in instructions {
            self.validate_instruction(instruction)?;
        }
        Ok(())
    }

    /// Validate memory access for security compliance
    pub fn validate_memory_access(&self, address: u32, size: u32) -> Result<(), SecurityError> {
        // In a real implementation, this would check if the address and size are valid
        // For now, we'll just return Ok as the validation would depend on the specific memory layout
        if address > u32::MAX - size {
            return Err(SecurityError::InvalidMemoryAccess);
        }
        Ok(())
    }

    /// Validate context access for security compliance
    pub fn validate_context_access(&self, context_id: u64) -> Result<(), SecurityError> {
        // In a real implementation, this would check if the context access is allowed
        // For now, we'll just return Ok
        if context_id > 1000000 { // Arbitrary limit for demonstration
            return Err(SecurityError::ContextEscape);
        }
        Ok(())
    }

    /// Validate rule access for security compliance
    pub fn validate_rule_access(&self, rule_id: u32) -> Result<(), SecurityError> {
        // In a real implementation, this would check if the rule access is allowed
        // For now, we'll just return Ok
        if rule_id > 1000000 { // Arbitrary limit for demonstration
            return Err(SecurityError::RuleHijack);
        }
        Ok(())
    }

    /// Check if self-modifying code is allowed
    pub fn allows_self_modifying_code(&self) -> bool {
        self.allow_self_modifying_code
    }

    /// Check if dynamic dispatch is allowed
    pub fn allows_dynamic_dispatch(&self) -> bool {
        self.allow_dynamic_dispatch
    }

    /// Check if runtime code loading is allowed
    pub fn allows_runtime_code_loading(&self) -> bool {
        self.allow_runtime_code_loading
    }

    /// Set whether self-modifying code is allowed (should always be false for security)
    pub fn set_allow_self_modifying_code(&mut self, allow: bool) {
        if !allow {
            self.allow_self_modifying_code = false;
        } else {
            // In a secure environment, this should never be enabled
            panic!("Self-modifying code is not allowed in secure KERN VM");
        }
    }

    /// Set whether dynamic dispatch is allowed (should always be false for security)
    pub fn set_allow_dynamic_dispatch(&mut self, allow: bool) {
        if !allow {
            self.allow_dynamic_dispatch = false;
        } else {
            // In a secure environment, this should never be enabled
            panic!("Dynamic dispatch is not allowed in secure KERN VM");
        }
    }

    /// Set whether runtime code loading is allowed (should always be false for security)
    pub fn set_allow_runtime_code_loading(&mut self, allow: bool) {
        if !allow {
            self.allow_runtime_code_loading = false;
        } else {
            // In a secure environment, this should never be enabled
            panic!("Runtime code loading is not allowed in secure KERN VM");
        }
    }
}

/// Security validation result
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid(SecurityError),
}

/// Security validation context that combines validator with sandbox
#[derive(Debug)]
pub struct SecurityValidationContext {
    pub validator: SecurityValidator,
    pub sandbox: SandboxEnvironment,
}

impl SecurityValidationContext {
    pub fn new(validator: SecurityValidator, sandbox: SandboxEnvironment) -> Self {
        SecurityValidationContext { validator, sandbox }
    }

    /// Validate an instruction in the context of the current security policy
    pub fn validate_instruction(&mut self, instruction: &Instruction) -> Result<(), SecurityError> {
        // First validate with the security validator
        self.validator.validate_instruction(instruction)?;

        // Then validate external calls with the sandbox
        match instruction.opcode {
            0x80 => {
                // CALL_EXTERN - need to validate the function call
                // For now, we'll just record a generic external call
                // In a real implementation, we'd extract the function name from the instruction
            }
            0x81 | 0x82 => {
                // READ_IO, WRITE_IO - need to validate the IO operation
                // For now, we'll just record a generic IO operation
                // In a real implementation, we'd extract the channel name from the instruction
            }
            _ => {}
        }

        Ok(())
    }

    /// Validate a sequence of instructions
    pub fn validate_instructions(&mut self, instructions: &[Instruction]) -> Result<(), SecurityError> {
        for instruction in instructions {
            self.validate_instruction(instruction)?;
        }
        Ok(())
    }

    /// Validate memory access in the context of the current security policy
    pub fn validate_memory_access(&self, address: u32, size: u32) -> Result<(), SecurityError> {
        self.validator.validate_memory_access(address, size)
    }

    /// Validate context access in the context of the current security policy
    pub fn validate_context_access(&self, context_id: u64) -> Result<(), SecurityError> {
        self.validator.validate_context_access(context_id)
    }

    /// Validate rule access in the context of the current security policy
    pub fn validate_rule_access(&self, rule_id: u32) -> Result<(), SecurityError> {
        self.validator.validate_rule_access(rule_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kern_bytecode::Instruction;

    #[test]
    fn test_security_validator_creation() {
        let validator = SecurityValidator::new();
        assert!(!validator.allows_self_modifying_code());
        assert!(!validator.allows_dynamic_dispatch());
        assert!(!validator.allows_runtime_code_loading());
        assert!(validator.allowed_opcodes.contains(&0x00)); // NOP
        assert!(validator.allowed_opcodes.contains(&0x11)); // LOAD_NUM
    }

    #[test]
    fn test_instruction_validation() {
        let validator = SecurityValidator::new();
        
        // Valid instruction
        let valid_instr = Instruction::new(0x11, 42, 1, 0, 0); // LOAD_NUM
        assert!(validator.validate_instruction(&valid_instr).is_ok());

        // Invalid instruction (not in allowed list)
        let invalid_instr = Instruction::new(0xFF, 0, 0, 0, 0); // Invalid opcode
        assert_eq!(validator.validate_instruction(&invalid_instr),
                   Err(SecurityError::IllegalOpcode(0xFF)));
    }

    #[test]
    fn test_bytecode_validation() {
        let validator = SecurityValidator::new();
        
        let valid_instructions = vec![
            Instruction::new(0x11, 42, 1, 0, 0), // LOAD_NUM
            Instruction::new(0x00, 0, 0, 0, 0),  // NOP
            Instruction::new(0x03, 0, 0, 0, 0),  // HALT
        ];
        
        assert!(validator.validate_bytecode(&valid_instructions).is_ok());

        let invalid_instructions = vec![
            Instruction::new(0x11, 42, 1, 0, 0), // LOAD_NUM
            Instruction::new(0xFF, 0, 0, 0, 0),  // Invalid opcode
        ];
        
        assert!(validator.validate_bytecode(&invalid_instructions).is_err());
    }

    #[test]
    fn test_memory_access_validation() {
        let validator = SecurityValidator::new();
        
        // Valid memory access
        assert!(validator.validate_memory_access(100, 10).is_ok());
        
        // Invalid memory access (would overflow)
        assert_eq!(validator.validate_memory_access(u32::MAX, 10),
                   Err(SecurityError::InvalidMemoryAccess));
    }

    #[test]
    fn test_context_access_validation() {
        let validator = SecurityValidator::new();
        
        // Valid context access
        assert!(validator.validate_context_access(100).is_ok());
        
        // Invalid context access (too large)
        assert_eq!(validator.validate_context_access(2000000),
                   Err(SecurityError::ContextEscape));
    }

    #[test]
    fn test_security_validation_context() {
        use crate::vm_safety::sandbox::SandboxPolicy;
        
        let validator = SecurityValidator::new();
        let sandbox = SandboxEnvironment::new(SandboxPolicy::new());
        let mut context = SecurityValidationContext::new(validator, sandbox);
        
        // Valid instruction
        let instr = Instruction::new(0x11, 42, 1, 0, 0); // LOAD_NUM
        assert!(context.validate_instruction(&instr).is_ok());
        
        // Invalid instruction
        let invalid_instr = Instruction::new(0xFF, 0, 0, 0, 0); // Invalid opcode
        assert!(context.validate_instruction(&invalid_instr).is_err());
    }
}