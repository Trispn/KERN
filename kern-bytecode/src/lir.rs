//! Linear IR (LIR) Data Structures
//!
//! This module defines the Linear IR (LIR) data structures used in the bytecode compilation pipeline.

use std::collections::HashMap;

/// A register in the virtual machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register(pub u16);

impl Register {
    pub fn new(id: u16) -> Self {
        Register(id)
    }
    
    pub fn id(&self) -> u16 {
        self.0
    }
}

/// LIR Operation
#[derive(Debug, Clone, PartialEq)]
pub enum LirOp {
    // Control Flow Operations
    Nop,
    Jmp(u32),                    // Jump to label
    JmpIf(Register, u32),        // Jump to label if register is true
    JmpIfNot(Register, u32),     // Jump to label if register is false
    Halt,                        // Halt execution
    
    // Data & Symbol Operations
    LoadSym(String),             // Load symbol value into register
    LoadNum(i64),                // Load numeric literal into register
    LoadBool(bool),              // Load boolean literal into register
    Move(Register, Register),    // Move value from one register to another
    CmpEq(Register, Register),   // Compare equality
    CmpNe(Register, Register),   // Compare inequality
    CmpLt(Register, Register),   // Compare less than
    CmpLe(Register, Register),   // Compare less than or equal
    CmpGt(Register, Register),   // Compare greater than
    CmpGe(Register, Register),   // Compare greater than or equal
    
    // Arithmetic Operations
    Add(Register, Register),     // Add two registers
    Sub(Register, Register),     // Subtract two registers
    Mul(Register, Register),     // Multiply two registers
    Div(Register, Register),     // Divide two registers
    Mod(Register, Register),     // Modulo operation
    Neg(Register),               // Negate register value
    
    // Logical Operations
    And(Register, Register),     // Logical AND
    Or(Register, Register),      // Logical OR
    Not(Register),               // Logical NOT
    
    // Graph Operations
    CreateNode(String),          // Create a graph node
    Connect(Register, Register), // Connect two nodes
    Merge(Register, Register),   // Merge two nodes
    DeleteNode(Register),        // Delete a node
    
    // Rule Execution Operations
    RuleEntry(u32, String),      // Rule entry point (label, name)
    FlowEntry(u32, String),      // Flow entry point (label, name)
    ConstraintEntry(u32, String), // Constraint entry point (label, name)
    CallRule(String),            // Call a rule by name
    ReturnRule,                  // Return from rule
    CheckCondition(Register),    // Check a condition
    ConstraintFailure(String),   // Constraint violation
    
    // Context & State Operations
    PushCtx,                     // Push new context frame
    PopCtx,                      // Pop context frame
    SetSymbol(String, Register), // Set symbol value in current context
    GetSymbol(String),           // Get symbol value from context
    CopyCtx,                     // Copy context
    
    // External Interface Operations
    Call(String, Vec<Register>), // Call external function
    ReadIo(String),              // Read from external input
    WriteIo(String, Register),   // Write to external output
    
    // Special Operations
    Label(u32),                  // Label for jumps
    Phi(Register, Vec<Register>), // Phi node for SSA
}

/// LIR Instruction
#[derive(Debug, Clone, PartialEq)]
pub struct LirInstruction {
    pub op: LirOp,
    pub dst: Option<Register>,    // Destination register (if any)
    pub src1: Option<Register>,   // First source register (if any)
    pub src2: Option<Register>,   // Second source register (if any)
    pub immediate: Option<i64>,   // Immediate value (if any)
    pub label: Option<u32>,       // Associated label (if any)
}

/// LIR Program
#[derive(Debug, Clone)]
pub struct LirProgram {
    pub instructions: Vec<LirInstruction>,
    pub symbol_table: HashMap<String, Register>,
    pub rule_table: HashMap<String, u32>,
    pub next_register: u16,
    pub next_label: u32,
}

impl LirProgram {
    pub fn new() -> Self {
        LirProgram {
            instructions: Vec::new(),
            symbol_table: HashMap::new(),
            rule_table: HashMap::new(),
            next_register: 0,
            next_label: 0,
        }
    }
    
    pub fn add_instruction(&mut self, instr: LirInstruction) {
        self.instructions.push(instr);
    }
    
    pub fn alloc_register(&mut self) -> Register {
        let reg = Register(self.next_register);
        self.next_register += 1;
        reg
    }
    
    pub fn alloc_label(&mut self) -> u32 {
        let label = self.next_label;
        self.next_label += 1;
        label
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_creation() {
        let reg1 = Register::new(0);
        let reg2 = Register::new(1);
        
        assert_eq!(reg1.id(), 0);
        assert_eq!(reg2.id(), 1);
        assert_ne!(reg1, reg2);
    }

    #[test]
    fn test_lir_instruction_creation() {
        let reg1 = Register(0);
        let reg2 = Register(1);
        
        let instr = LirInstruction {
            op: LirOp::Add(reg1, reg2),
            dst: Some(Register(2)),
            src1: Some(reg1),
            src2: Some(reg2),
            immediate: None,
            label: None,
        };
        
        match instr.op {
            LirOp::Add(r1, r2) => {
                assert_eq!(r1, reg1);
                assert_eq!(r2, reg2);
            }
            _ => panic!("Expected Add operation"),
        }
        
        assert_eq!(instr.dst, Some(Register(2)));
        assert_eq!(instr.src1, Some(reg1));
        assert_eq!(instr.src2, Some(reg2));
    }

    #[test]
    fn test_lir_program() {
        let mut program = LirProgram::new();
        
        assert_eq!(program.instructions.len(), 0);
        assert_eq!(program.next_register, 0);
        assert_eq!(program.next_label, 0);
        
        let reg1 = program.alloc_register();
        let reg2 = program.alloc_register();
        
        assert_eq!(reg1, Register(0));
        assert_eq!(reg2, Register(1));
        assert_eq!(program.next_register, 2);
        
        let label1 = program.alloc_label();
        let label2 = program.alloc_label();
        
        assert_eq!(label1, 0);
        assert_eq!(label2, 1);
        assert_eq!(program.next_label, 2);
    }
}