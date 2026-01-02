//! Bytecode Verifier
//!
//! This module implements verification of KERN bytecode to ensure it's valid
//! and safe for execution by the VM. Verification includes structural, control flow,
//! register, context, and stack verification.

use crate::{Instruction, Opcode};

/// Verification error types
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationError {
    /// Instruction width is not 8 bytes
    InvalidInstructionWidth,
    /// Invalid opcode found
    InvalidOpcode(u8),
    /// Invalid operand format
    InvalidOperandFormat,
    /// Jump target is out of bounds
    JumpOutOfBounds(u16),
    /// Illegal fallthrough in control flow
    IllegalFallthrough,
    /// Use of register before definition
    UseBeforeDefinition(u16),
    /// Invalid register index
    InvalidRegisterIndex(u16),
    /// PUSH_CTX/POP_CTX imbalance
    ContextStackImbalance,
    /// Stack underflow detected
    StackUnderflow,
    /// Stack overflow detected
    StackOverflow,
    /// Invalid rule entry/exit
    InvalidRuleEntryExit,
}

/// Verification result
pub type VerificationResult = Result<(), VerificationError>;

/// Bytecode verifier
pub struct BytecodeVerifier {
    /// Maximum allowed stack depth
    max_stack_depth: u16,
}

impl BytecodeVerifier {
    pub fn new() -> Self {
        BytecodeVerifier {
            max_stack_depth: 1024, // Reasonable default
        }
    }

    /// Verify a sequence of bytecode instructions
    pub fn verify(&self, instructions: &[Instruction]) -> VerificationResult {
        // 1. Structural Verification
        self.verify_structure(instructions)?;
        
        // 2. Control Flow Verification
        self.verify_control_flow(instructions)?;
        
        // 3. Register Verification
        self.verify_registers(instructions)?;
        
        // 4. Context Verification
        self.verify_context(instructions)?;
        
        // 5. Stack Verification
        self.verify_stack(instructions)?;

        Ok(())
    }

    /// Verify structural properties of the bytecode
    fn verify_structure(&self, instructions: &[Instruction]) -> VerificationResult {
        for instr in instructions {
            // Verify opcode is valid
            match Opcode::from(instr.opcode) {
                Opcode::Nop | Opcode::Jmp | Opcode::JmpIf | Opcode::Halt |
                Opcode::LoadSym | Opcode::LoadNum | Opcode::LoadBool | Opcode::Move | Opcode::Compare |
                Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div | Opcode::Mod |
                Opcode::And | Opcode::Or | Opcode::Not |
                Opcode::CreateNode | Opcode::Connect | Opcode::Merge | Opcode::DeleteNode |
                Opcode::CallRule | Opcode::ReturnRule | Opcode::CheckCondition | Opcode::IncrementExecCount |
                Opcode::PushCtx | Opcode::PopCtx | Opcode::SetSymbol | Opcode::GetSymbol | Opcode::CopyCtx |
                Opcode::Throw | Opcode::Try | Opcode::Catch | Opcode::ClearErr |
                Opcode::CallExtern | Opcode::ReadIo | Opcode::WriteIo => {
                    // Valid opcode
                },
                _ => return Err(VerificationError::InvalidOpcode(instr.opcode)),
            }
        }
        Ok(())
    }

    /// Verify control flow properties
    fn verify_control_flow(&self, instructions: &[Instruction]) -> VerificationResult {
        let instr_count = instructions.len() as u16;
        
        for (idx, instr) in instructions.iter().enumerate() {
            match Opcode::from(instr.opcode) {
                Opcode::Jmp => {
                    // Verify jump target is in bounds
                    let target = instr.arg1;
                    if target >= instr_count {
                        return Err(VerificationError::JumpOutOfBounds(target));
                    }
                },
                Opcode::JmpIf => {
                    // Verify jump target is in bounds
                    let target = instr.arg2; // Using arg2 for target in our encoding
                    if target >= instr_count {
                        return Err(VerificationError::JumpOutOfBounds(target));
                    }
                },
                _ => {}
            }
        }
        
        Ok(())
    }

    /// Verify register usage
    fn verify_registers(&self, instructions: &[Instruction]) -> VerificationResult {
        // In a real implementation, we would track register definitions and uses
        // For now, we'll just verify that register indices are within bounds
        for instr in instructions {
            // Check if any register arguments exceed the maximum (R0-R15 = 0-15)
            if instr.arg1 > 15 && self.is_register_arg(instr, 1) {
                return Err(VerificationError::InvalidRegisterIndex(instr.arg1));
            }
            if instr.arg2 > 15 && self.is_register_arg(instr, 2) {
                return Err(VerificationError::InvalidRegisterIndex(instr.arg2));
            }
            if instr.arg3 > 15 && self.is_register_arg(instr, 3) {
                return Err(VerificationError::InvalidRegisterIndex(instr.arg3));
            }
        }
        
        Ok(())
    }

    /// Helper to determine if an argument is a register
    fn is_register_arg(&self, instr: &Instruction, arg_num: u8) -> bool {
        match Opcode::from(instr.opcode) {
            Opcode::LoadSym | Opcode::LoadNum | Opcode::LoadBool => {
                // arg1 is destination register
                arg_num == 1
            },
            Opcode::Move => {
                // arg1 is destination, arg2 is source
                arg_num == 1 || arg_num == 2
            },
            Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div | Opcode::Mod |
            Opcode::And | Opcode::Or | Opcode::Compare => {
                // arg1 is destination, arg2 and arg3 are sources
                arg_num == 1 || arg_num == 2 || arg_num == 3
            },
            Opcode::Not => {
                // arg1 is destination, arg2 is source
                arg_num == 1 || arg_num == 2
            },
            Opcode::JmpIf => {
                // arg1 is condition register
                arg_num == 1
            },
            _ => false,
        }
    }

    /// Verify context operations
    fn verify_context(&self, instructions: &[Instruction]) -> VerificationResult {
        let mut ctx_stack_depth = 0i32;
        
        for instr in instructions {
            match Opcode::from(instr.opcode) {
                Opcode::PushCtx => {
                    ctx_stack_depth += 1;
                },
                Opcode::PopCtx => {
                    ctx_stack_depth -= 1;
                    if ctx_stack_depth < 0 {
                        return Err(VerificationError::ContextStackImbalance);
                    }
                },
                _ => {}
            }
        }
        
        // Context stack should be balanced at the end
        if ctx_stack_depth != 0 {
            return Err(VerificationError::ContextStackImbalance);
        }
        
        Ok(())
    }

    /// Verify stack operations
    fn verify_stack(&self, instructions: &[Instruction]) -> VerificationResult {
        let mut stack_depth = 0i32;
        
        for instr in instructions {
            match Opcode::from(instr.opcode) {
                // Operations that might affect stack (in our model)
                Opcode::CallRule | Opcode::CallExtern => {
                    stack_depth += 1;
                    if stack_depth > self.max_stack_depth as i32 {
                        return Err(VerificationError::StackOverflow);
                    }
                },
                Opcode::ReturnRule => {
                    stack_depth -= 1;
                    if stack_depth < 0 {
                        return Err(VerificationError::StackUnderflow);
                    }
                },
                _ => {}
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bytecode() {
        let verifier = BytecodeVerifier::new();
        
        // Create a simple valid program: load num, halt
        let instructions = vec![
            Instruction::new(Opcode::LoadNum as u8, 1, 42, 0, 0), // Load 42 into R1
            Instruction::new(Opcode::Halt as u8, 0, 0, 0, 0),     // Halt
        ];
        
        let result = verifier.verify(&instructions);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_opcode() {
        let verifier = BytecodeVerifier::new();
        
        // Create bytecode with invalid opcode
        let instructions = vec![
            Instruction::new(0xFF, 0, 0, 0, 0), // Invalid opcode
            Instruction::new(Opcode::Halt as u8, 0, 0, 0, 0),
        ];
        
        let result = verifier.verify(&instructions);
        assert_eq!(result, Err(VerificationError::InvalidOpcode(0xFF)));
    }

    #[test]
    fn test_jump_out_of_bounds() {
        let verifier = BytecodeVerifier::new();
        
        // Create bytecode with jump to out-of-bounds target
        let instructions = vec![
            Instruction::new(Opcode::Jmp as u8, 5, 0, 0, 0), // Jump to index 5, but only 2 instructions
            Instruction::new(Opcode::Halt as u8, 0, 0, 0, 0),
        ];
        
        let result = verifier.verify(&instructions);
        assert_eq!(result, Err(VerificationError::JumpOutOfBounds(5)));
    }

    #[test]
    fn test_context_imbalance() {
        let verifier = BytecodeVerifier::new();
        
        // Create bytecode with unbalanced context operations
        let instructions = vec![
            Instruction::new(Opcode::PushCtx as u8, 0, 0, 0, 0), // Push context
            Instruction::new(Opcode::Halt as u8, 0, 0, 0, 0),    // Halt without pop
        ];
        
        let result = verifier.verify(&instructions);
        assert_eq!(result, Err(VerificationError::ContextStackImbalance));
    }

    #[test]
    fn test_context_balance() {
        let verifier = BytecodeVerifier::new();
        
        // Create bytecode with balanced context operations
        let instructions = vec![
            Instruction::new(Opcode::PushCtx as u8, 0, 0, 0, 0), // Push context
            Instruction::new(Opcode::PopCtx as u8, 0, 0, 0, 0),  // Pop context
            Instruction::new(Opcode::Halt as u8, 0, 0, 0, 0),    // Halt
        ];
        
        let result = verifier.verify(&instructions);
        assert!(result.is_ok());
    }
}