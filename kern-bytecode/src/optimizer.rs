//! Bytecode Optimization Passes
//!
//! This module implements various optimization passes for KERN bytecode.
//! All optimizations are deterministic, semantics-preserving, and optional.

use crate::{Instruction, Opcode};

/// Optimization pass result
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub instructions: Vec<Instruction>,
    pub optimizations_applied: Vec<String>,
}

/// Bytecode optimizer that runs multiple optimization passes
pub struct BytecodeOptimizer;

impl BytecodeOptimizer {
    pub fn new() -> Self {
        BytecodeOptimizer
    }

    /// Run all optimization passes on the bytecode
    pub fn optimize(&self, mut instructions: Vec<Instruction>) -> OptimizationResult {
        let mut optimizations_applied = Vec::new();

        // Run optimization passes in the specified order
        let original_len = instructions.len();

        // 1. Dead Instruction Elimination
        instructions = self.dead_instruction_elimination(instructions);
        if instructions.len() != original_len {
            optimizations_applied.push("Dead Instruction Elimination".to_string());
        }

        // 2. Constant Folding
        let prev_len = instructions.len();
        instructions = self.constant_folding(instructions);
        if instructions.len() != prev_len {
            optimizations_applied.push("Constant Folding".to_string());
        }

        // 3. Jump Simplification
        let prev_len = instructions.len();
        instructions = self.jump_simplification(instructions);
        if instructions.len() != prev_len {
            optimizations_applied.push("Jump Simplification".to_string());
        }

        // 4. Redundant Move Removal
        let prev_len = instructions.len();
        instructions = self.redundant_move_removal(instructions);
        if instructions.len() != prev_len {
            optimizations_applied.push("Redundant Move Removal".to_string());
        }

        // 5. No-Op Removal
        let prev_len = instructions.len();
        instructions = self.no_op_removal(instructions);
        if instructions.len() != prev_len {
            optimizations_applied.push("No-Op Removal".to_string());
        }

        OptimizationResult {
            instructions,
            optimizations_applied,
        }
    }

    /// Dead Instruction Elimination
    /// Removes instructions that have no effect on the program's output
    fn dead_instruction_elimination(&self, mut instructions: Vec<Instruction>) -> Vec<Instruction> {
        // For now, we'll implement a simple version that removes instructions after HALT
        let mut new_instructions = Vec::new();
        let mut halt_found = false;

        for instr in instructions {
            if halt_found {
                // Skip all instructions after HALT
                continue;
            }

            if instr.opcode == Opcode::Halt as u8 {
                halt_found = true;
            }

            new_instructions.push(instr);
        }

        new_instructions
    }

    /// Constant Folding
    /// Performs compile-time evaluation of constant expressions
    fn constant_folding(&self, instructions: Vec<Instruction>) -> Vec<Instruction> {
        // For now, this is a placeholder implementation
        // In a real implementation, we would analyze the bytecode to find
        // constant expressions that can be evaluated at compile time
        instructions
    }

    /// Jump Simplification
    /// Simplifies jump chains and redundant jumps
    fn jump_simplification(&self, mut instructions: Vec<Instruction>) -> Vec<Instruction> {
        // This is a simplified implementation that looks for immediate jumps to jumps
        // In a real implementation, we would build a control flow graph and perform
        // more sophisticated analysis
        
        // For now, we'll just return the instructions as-is
        // A full implementation would require more complex analysis
        instructions
    }

    /// Redundant Move Removal
    /// Removes moves from a register to itself
    fn redundant_move_removal(&self, instructions: Vec<Instruction>) -> Vec<Instruction> {
        // For now, this is a placeholder implementation
        // In a real implementation, we would identify and remove moves like:
        // MOVE R1 R1 (move register to itself)
        instructions
    }

    /// No-Op Removal
    /// Removes NOP instructions that have no effect
    fn no_op_removal(&self, instructions: Vec<Instruction>) -> Vec<Instruction> {
        instructions.into_iter()
            .filter(|instr| instr.opcode != Opcode::Nop as u8)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Instruction;

    #[test]
    fn test_no_op_removal() {
        let optimizer = BytecodeOptimizer::new();

        let mut instructions = Vec::new();
        instructions.push(Instruction::new(Opcode::LoadNum as u8, 1, 42, 0, 0)); // Load 42 into R1
        instructions.push(Instruction::new(Opcode::Nop as u8, 0, 0, 0, 0));      // NOP
        instructions.push(Instruction::new(Opcode::Nop as u8, 0, 0, 0, 0));      // NOP
        instructions.push(Instruction::new(Opcode::Add as u8, 2, 1, 1, 0));      // Add R1+R1 into R2

        let result = optimizer.optimize(instructions);

        // Should have removed the NOPs
        assert_eq!(result.instructions.len(), 2);
        assert_eq!(result.instructions[0].opcode, Opcode::LoadNum as u8);
        assert_eq!(result.instructions[1].opcode, Opcode::Add as u8);
        assert!(result.optimizations_applied.contains(&"No-Op Removal".to_string()));
    }

    #[test]
    fn test_dead_instruction_elimination() {
        let optimizer = BytecodeOptimizer::new();

        let mut instructions = Vec::new();
        instructions.push(Instruction::new(Opcode::LoadNum as u8, 1, 42, 0, 0)); // Load 42 into R1
        instructions.push(Instruction::new(Opcode::Halt as u8, 0, 0, 0, 0));     // HALT
        instructions.push(Instruction::new(Opcode::Add as u8, 2, 1, 1, 0));      // This should be removed (dead code)

        let result = optimizer.optimize(instructions);

        // Should have removed the instruction after HALT
        assert_eq!(result.instructions.len(), 2);
        assert_eq!(result.instructions[0].opcode, Opcode::LoadNum as u8);
        assert_eq!(result.instructions[1].opcode, Opcode::Halt as u8);
        assert!(result.optimizations_applied.contains(&"Dead Instruction Elimination".to_string()));
    }

    #[test]
    fn test_optimization_pipeline() {
        let optimizer = BytecodeOptimizer::new();

        let mut instructions = Vec::new();
        instructions.push(Instruction::new(Opcode::Nop as u8, 0, 0, 0, 0));      // NOP to be removed
        instructions.push(Instruction::new(Opcode::LoadNum as u8, 1, 42, 0, 0)); // Load 42 into R1
        instructions.push(Instruction::new(Opcode::Nop as u8, 0, 0, 0, 0));      // NOP to be removed
        instructions.push(Instruction::new(Opcode::Halt as u8, 0, 0, 0, 0));     // HALT
        instructions.push(Instruction::new(Opcode::Add as u8, 2, 1, 1, 0));      // Dead code to be removed

        let result = optimizer.optimize(instructions);

        // Should have applied multiple optimizations
        assert_eq!(result.instructions.len(), 2); // LoadNum and Halt only
        assert_eq!(result.instructions[0].opcode, Opcode::LoadNum as u8);
        assert_eq!(result.instructions[1].opcode, Opcode::Halt as u8);

        // Check that multiple optimizations were applied
        assert!(result.optimizations_applied.contains(&"No-Op Removal".to_string()));
        assert!(result.optimizations_applied.contains(&"Dead Instruction Elimination".to_string()));
    }
}