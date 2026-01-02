//! KERN Bytecode Compiler
//!
//! This module implements the bytecode compiler that transforms KERN AST into
//! deterministic, verifiable bytecode for the KERN VM.

pub mod ast_lowering;
pub mod lir;
pub mod lir_builder;
pub mod register_allocator;
pub mod emitter;
pub mod optimizer;
pub mod verifier;
pub mod serializer;
pub mod compiler_driver;

// Define the KERN bytecode instruction format
// Each instruction is 8 bytes: OPCODE (1B) | ARG1 (2B) | ARG2 (2B) | ARG3 (2B) | FLAGS (1B)
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: u8,
    pub arg1: u16,  // First argument (register, immediate, etc.)
    pub arg2: u16,  // Second argument
    pub arg3: u16,  // Third argument
    pub flags: u8,  // Flags
}

impl Instruction {
    pub fn new(opcode: u8, arg1: u16, arg2: u16, arg3: u16, flags: u8) -> Self {
        Instruction {
            opcode,
            arg1,
            arg2,
            arg3,
            flags,
        }
    }

    // Serialize the instruction to bytes (8 bytes total)
    pub fn to_bytes(&self) -> [u8; 8] {
        [
            self.opcode,
            (self.arg1 & 0xFF) as u8,
            ((self.arg1 >> 8) & 0xFF) as u8,
            (self.arg2 & 0xFF) as u8,
            ((self.arg2 >> 8) & 0xFF) as u8,
            (self.arg3 & 0xFF) as u8,
            ((self.arg3 >> 8) & 0xFF) as u8,
            self.flags,
        ]
    }

    // Deserialize an instruction from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }

        Some(Instruction {
            opcode: bytes[0],
            arg1: (bytes[1] as u16) | ((bytes[2] as u16) << 8),
            arg2: (bytes[3] as u16) | ((bytes[4] as u16) << 8),
            arg3: (bytes[5] as u16) | ((bytes[6] as u16) << 8),
            flags: bytes[7],
        })
    }
}

// Define the KERN opcodes according to the specification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    // Control Flow Instructions
    Nop = 0x00,   // Do nothing
    Jmp = 0x01,   // Jump unconditionally to address
    JmpIf = 0x02, // Jump if condition is true
    Halt = 0x03,  // Stop execution

    // Data & Symbol Instructions
    LoadSym = 0x10, // Load symbol value into register
    LoadNum = 0x11, // Load numeric literal into register
    LoadBool = 0x12, // Load boolean literal into register
    Move = 0x13,    // Move value between registers
    Compare = 0x14, // Compare values, result in register

    // Arithmetic Instructions
    Add = 0x20,     // Add two registers
    Sub = 0x21,     // Subtract two registers
    Mul = 0x22,     // Multiply two registers
    Div = 0x23,     // Divide two registers
    Mod = 0x24,     // Modulo operation

    // Logical Instructions
    And = 0x30,     // Logical AND
    Or = 0x31,      // Logical OR
    Not = 0x32,     // Logical NOT

    // Graph Operation Instructions
    CreateNode = 0x40, // Create graph node in execution graph
    Connect = 0x41,    // Create edge (data/control)
    Merge = 0x42,      // Merge nodes into single logical node
    DeleteNode = 0x43, // Remove node from graph

    // Rule Execution Instructions
    CallRule = 0x50,           // Invoke rule subgraph
    ReturnRule = 0x51,         // Return from rule execution
    CheckCondition = 0x52,     // Evaluate condition node
    IncrementExecCount = 0x53, // Track recursion / iteration

    // Context & State Instructions
    PushCtx = 0x60,   // Push new context frame
    PopCtx = 0x61,    // Pop context frame
    SetSymbol = 0x62, // Update symbol in current context
    GetSymbol = 0x63, // Read symbol value from context
    CopyCtx = 0x64,   // Duplicate context

    // Error Handling Instructions
    Throw = 0x70,    // Raise error with deterministic code
    Try = 0x71,      // Start try block
    Catch = 0x72,    // Jump to catch block if exception thrown
    ClearErr = 0x73, // Reset error state

    // External Interface Instructions
    CallExtern = 0x80, // Call external function in host environment
    ReadIo = 0x81,     // Read from external input
    WriteIo = 0x82,    // Write value to external output
}

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        match opcode {
            0x00 => Opcode::Nop,
            0x01 => Opcode::Jmp,
            0x02 => Opcode::JmpIf,
            0x03 => Opcode::Halt,
            0x10 => Opcode::LoadSym,
            0x11 => Opcode::LoadNum,
            0x12 => Opcode::LoadBool,
            0x13 => Opcode::Move,
            0x14 => Opcode::Compare,
            0x20 => Opcode::Add,
            0x21 => Opcode::Sub,
            0x22 => Opcode::Mul,
            0x23 => Opcode::Div,
            0x24 => Opcode::Mod,
            0x30 => Opcode::And,
            0x31 => Opcode::Or,
            0x32 => Opcode::Not,
            0x40 => Opcode::CreateNode,
            0x41 => Opcode::Connect,
            0x42 => Opcode::Merge,
            0x43 => Opcode::DeleteNode,
            0x50 => Opcode::CallRule,
            0x51 => Opcode::ReturnRule,
            0x52 => Opcode::CheckCondition,
            0x53 => Opcode::IncrementExecCount,
            0x60 => Opcode::PushCtx,
            0x61 => Opcode::PopCtx,
            0x62 => Opcode::SetSymbol,
            0x63 => Opcode::GetSymbol,
            0x64 => Opcode::CopyCtx,
            0x70 => Opcode::Throw,
            0x71 => Opcode::Try,
            0x72 => Opcode::Catch,
            0x73 => Opcode::ClearErr,
            0x80 => Opcode::CallExtern,
            0x81 => Opcode::ReadIo,
            0x82 => Opcode::WriteIo,
            _ => Opcode::Nop, // Default to NOP for unknown opcodes
        }
    }
}

// Define the bytecode module structure as specified
#[derive(Debug, Clone)]
pub struct BytecodeModule {
    pub header: ModuleHeader,
    pub instruction_stream: Vec<Instruction>,
    pub constant_pool: Vec<Constant>,
    pub symbol_table: Vec<Symbol>,
    pub rule_table: Vec<RuleEntry>,
    pub graph_table: Vec<GraphEntry>,
    pub metadata: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ModuleHeader {
    pub magic: [u8; 4],           // "KERN"
    pub version: u32,
    pub instruction_count: u32,
    pub section_offsets: SectionOffsets,
    pub checksum: u64,
}

#[derive(Debug, Clone)]
pub struct SectionOffsets {
    pub instruction_offset: u32,
    pub constant_pool_offset: u32,
    pub symbol_table_offset: u32,
    pub rule_table_offset: u32,
    pub graph_table_offset: u32,
    pub metadata_offset: u32,
}

#[derive(Debug, Clone)]
pub enum Constant {
    Num(i64),
    Bool(bool),
    Sym(String),
    Vec(Vec<Constant>),
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct RuleEntry {
    pub id: u32,
    pub entry_pc: u32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct GraphEntry {
    pub id: u32,
    pub node_count: u32,
    pub edge_count: u32,
}

// Decoder algorithm implementation
pub fn decode(instr_bytes: &[u8]) -> Option<Instruction> {
    Instruction::from_bytes(instr_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_serialization() {
        let instruction = Instruction::new(0x10, 0xFF, 0x1234, 0x5678, 0xAB);
        let bytes = instruction.to_bytes();
        let parsed_instruction = Instruction::from_bytes(&bytes).unwrap();

        assert_eq!(instruction.opcode, parsed_instruction.opcode);
        assert_eq!(instruction.arg1, parsed_instruction.arg1);
        assert_eq!(instruction.arg2, parsed_instruction.arg2);
        assert_eq!(instruction.arg3, parsed_instruction.arg3);
        assert_eq!(instruction.flags, parsed_instruction.flags);
    }

    #[test]
    fn test_opcode_conversion() {
        assert_eq!(Opcode::from(0x00), Opcode::Nop);
        assert_eq!(Opcode::from(0x01), Opcode::Jmp);
        assert_eq!(Opcode::from(0x10), Opcode::LoadSym);
        assert_eq!(Opcode::from(0x11), Opcode::LoadNum);
        assert_eq!(Opcode::from(0x20), Opcode::Add);
        assert_eq!(Opcode::from(0x30), Opcode::And);
        assert_eq!(Opcode::from(0x40), Opcode::CreateNode);
        assert_eq!(Opcode::from(0x50), Opcode::CallRule);
        assert_eq!(Opcode::from(0x60), Opcode::PushCtx);
        assert_eq!(Opcode::from(0x70), Opcode::Throw);
        assert_eq!(Opcode::from(0x80), Opcode::CallExtern);
    }

    #[test]
    fn test_instruction_creation() {
        let instr = Instruction::new(0x11, 42, 1, 0, 0); // LoadNum 42 into register 1
        assert_eq!(instr.opcode, 0x11);
        assert_eq!(instr.arg1, 42);   // The value to load
        assert_eq!(instr.arg2, 1);    // The destination register
        assert_eq!(instr.arg3, 0);    // Unused in this case
        assert_eq!(instr.flags, 0);   // No special flags
    }

    #[test]
    fn test_decode_function() {
        let instruction = Instruction::new(0x20, 1, 2, 3, 0); // Add R1, R2 -> R3
        let bytes = instruction.to_bytes();

        // Decode from bytes
        if let Some(decoded) = decode(&bytes) {
            assert_eq!(decoded.opcode, instruction.opcode);
            assert_eq!(decoded.arg1, instruction.arg1);
            assert_eq!(decoded.arg2, instruction.arg2);
            assert_eq!(decoded.arg3, instruction.arg3);
            assert_eq!(decoded.flags, instruction.flags);
        } else {
            panic!("Failed to decode instruction from bytes");
        }
    }
}
