use std::collections::HashMap;

// Define the KERN bytecode instruction format
// Each instruction is 8 bytes: OPCODE (1B) | FLAGS (1B) | OPERAND (6B / 48 bits)
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: u8,
    pub flags: u8,
    pub operand: u64, // Using u64 but only using 48 bits
}

impl Instruction {
    pub fn new(opcode: u8, flags: u8, operand: u64) -> Self {
        Instruction {
            opcode,
            flags,
            operand: operand & 0xFFFFFFFFFFFF, // Mask to 48 bits
        }
    }

    // Serialize the instruction to bytes (8 bytes total)
    pub fn to_bytes(&self) -> [u8; 8] {
        [
            self.opcode,
            self.flags,
            (self.operand & 0xFF) as u8,
            ((self.operand >> 8) & 0xFF) as u8,
            ((self.operand >> 16) & 0xFF) as u8,
            ((self.operand >> 24) & 0xFF) as u8,
            ((self.operand >> 32) & 0xFF) as u8,
            ((self.operand >> 40) & 0xFF) as u8,
        ]
    }

    // Deserialize an instruction from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }

        let operand = (bytes[2] as u64) |
                     ((bytes[3] as u64) << 8) |
                     ((bytes[4] as u64) << 16) |
                     ((bytes[5] as u64) << 24) |
                     ((bytes[6] as u64) << 32) |
                     ((bytes[7] as u64) << 40);

        Some(Instruction {
            opcode: bytes[0],
            flags: bytes[1],
            operand,
        })
    }
}

// Define the KERN opcodes according to the specification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    // Control Flow Instructions
    Nop = 0x00,      // Do nothing
    Jmp = 0x01,      // Jump unconditionally to address
    JmpIf = 0x02,    // Jump if top-of-stack Bool is true
    Halt = 0x03,     // Stop execution

    // Data & Symbol Instructions
    LoadSym = 0x10,  // Load symbol value onto stack
    LoadNum = 0x11,  // Push literal number onto stack
    Move = 0x12,     // Copy top of stack to dst symbol
    Compare = 0x13,  // Compare top 2 stack values, result pushed as Bool

    // Graph Operation Instructions
    CreateNode = 0x20,    // Create graph node in execution graph
    Connect = 0x21,       // Create edge (data/control)
    Merge = 0x22,         // Merge nodes into single logical node
    DeleteNode = 0x23,    // Remove node from graph

    // Rule Execution Instructions
    CallRule = 0x30,           // Invoke rule subgraph
    ReturnRule = 0x31,         // Return from rule execution
    CheckCondition = 0x32,     // Evaluate condition node; push Bool
    IncrementExecCount = 0x33, // Track recursion / iteration

    // Context & State Instructions
    PushCtx = 0x40,  // Push new context frame onto stack
    PopCtx = 0x41,   // Pop context frame
    SetSymbol = 0x42, // Update symbol in current context
    GetSymbol = 0x43, // Read symbol value onto stack
    CopyCtx = 0x44,   // Duplicate context for subflow / rule

    // Error Handling Instructions
    Throw = 0x50,     // Raise error with deterministic code
    Try = 0x51,       // Start try block at addr
    Catch = 0x52,     // Jump to catch block if exception thrown
    ClearErr = 0x53,  // Reset error state

    // External Interface Instructions
    CallExtern = 0x60, // Call external function in host environment
    ReadIo = 0x61,     // Read from external input
    WriteIo = 0x62,    // Write value from stack to external
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
            0x12 => Opcode::Move,
            0x13 => Opcode::Compare,
            0x20 => Opcode::CreateNode,
            0x21 => Opcode::Connect,
            0x22 => Opcode::Merge,
            0x23 => Opcode::DeleteNode,
            0x30 => Opcode::CallRule,
            0x31 => Opcode::ReturnRule,
            0x32 => Opcode::CheckCondition,
            0x33 => Opcode::IncrementExecCount,
            0x40 => Opcode::PushCtx,
            0x41 => Opcode::PopCtx,
            0x42 => Opcode::SetSymbol,
            0x43 => Opcode::GetSymbol,
            0x44 => Opcode::CopyCtx,
            0x50 => Opcode::Throw,
            0x51 => Opcode::Try,
            0x52 => Opcode::Catch,
            0x53 => Opcode::ClearErr,
            0x60 => Opcode::CallExtern,
            0x61 => Opcode::ReadIo,
            0x62 => Opcode::WriteIo,
            _ => Opcode::Nop, // Default to NOP for unknown opcodes
        }
    }
}

// Define the bytecode compiler that translates execution graphs to bytecode
pub struct BytecodeCompiler {
    instructions: Vec<Instruction>,
    symbol_table: HashMap<String, u64>,
    next_symbol_id: u64,
    register_map: HashMap<String, u64>, // Maps variable names to register IDs
}

impl BytecodeCompiler {
    pub fn new() -> Self {
        BytecodeCompiler {
            instructions: Vec::new(),
            symbol_table: HashMap::new(),
            next_symbol_id: 0,
            register_map: HashMap::new(),
        }
    }

    // Compile an execution graph to bytecode
    pub fn compile_graph(&mut self, graph: &kern_graph_builder::ExecutionGraph) -> Vec<Instruction> {
        // Reset the compiler state
        self.instructions.clear();
        self.symbol_table.clear();
        self.next_symbol_id = 0;
        self.register_map.clear();

        // Process each node in the execution graph
        for node in &graph.nodes {
            if let Err(e) = self.compile_node(node) {
                eprintln!("Error compiling node: {}", e);
            }
        }

        self.instructions.clone()
    }

    fn compile_node(&mut self, node: &kern_graph_builder::GraphNode) -> Result<(), &'static str> {
        match node.node_type {
            kern_graph_builder::GraphNodeType::Op => self.compile_op_node(node),
            kern_graph_builder::GraphNodeType::Rule => self.compile_rule_node(node),
            kern_graph_builder::GraphNodeType::Control => self.compile_control_node(node),
            kern_graph_builder::GraphNodeType::Graph => self.compile_graph_node(node),
            kern_graph_builder::GraphNodeType::Io => self.compile_io_node(node),
        }
    }

    fn compile_op_node(&mut self, node: &kern_graph_builder::GraphNode) -> Result<(), &'static str> {
        match node.opcode {
            0x10 => { // LOAD_SYM
                // operand: symbol ID
                let symbol_id = self.get_or_create_symbol("temp_symbol"); // In real impl, get from metadata
                self.instructions.push(Instruction::new(Opcode::LoadSym as u8, node.flags as u8, symbol_id));
            },
            0x11 => { // LOAD_NUM
                // operand: number value
                let value = node.id as u64; // In real impl, get actual value
                self.instructions.push(Instruction::new(Opcode::LoadNum as u8, node.flags as u8, value));
            },
            0x12 => { // MOVE
                // operand: encoded as (src << 32) | dst
                let src_reg = node.input_regs[0] as u64;
                let dst_reg = node.output_regs[0] as u64;
                let operand = (src_reg << 32) | dst_reg;
                self.instructions.push(Instruction::new(Opcode::Move as u8, node.flags as u8, operand));
            },
            0x13 => { // COMPARE
                // operand: encoded as (reg_a << 32) | reg_b
                let reg_a = node.input_regs[0] as u64;
                let reg_b = node.input_regs[1] as u64;
                let operand = (reg_a << 32) | reg_b;
                self.instructions.push(Instruction::new(Opcode::Compare as u8, node.flags as u8, operand));
            },
            _ => {
                // For other opcodes, we'll add implementations as needed
                println!("Compiling operation node with opcode: {}", node.opcode);
            }
        }
        Ok(())
    }

    fn compile_rule_node(&mut self, node: &kern_graph_builder::GraphNode) -> Result<(), &'static str> {
        // Compile rule evaluation
        self.instructions.push(Instruction::new(Opcode::CallRule as u8, node.flags as u8, node.id as u64));

        Ok(())
    }

    fn compile_control_node(&mut self, node: &kern_graph_builder::GraphNode) -> Result<(), &'static str> {
        match node.opcode {
            0x00 => { // NOP
                self.instructions.push(Instruction::new(Opcode::Nop as u8, node.flags as u8, 0));
            },
            0x01 => { // JMP
                // operand: target instruction index
                let target = node.input_regs[0] as u64; // In real impl, get actual target
                self.instructions.push(Instruction::new(Opcode::Jmp as u8, node.flags as u8, target));
            },
            0x02 => { // JMP_IF
                // operand: encoded as (flag_reg << 32) | target
                let flag_reg = node.input_regs[0] as u64;
                let target = node.input_regs[1] as u64; // In real impl, get actual target
                let operand = (flag_reg << 32) | target;
                self.instructions.push(Instruction::new(Opcode::JmpIf as u8, node.flags as u8, operand));
            },
            0x03 => { // HALT
                self.instructions.push(Instruction::new(Opcode::Halt as u8, node.flags as u8, 0));
            },
            _ => {
                println!("Compiling control node with opcode: {}", node.opcode);
            }
        }
        Ok(())
    }

    fn compile_graph_node(&mut self, node: &kern_graph_builder::GraphNode) -> Result<(), &'static str> {
        // Compile graph operations
        match node.opcode {
            0x20 => { // CREATE_NODE
                // operand: node_id
                self.instructions.push(Instruction::new(Opcode::CreateNode as u8, node.flags as u8, node.id as u64));
            },
            0x21 => { // CONNECT
                // operand: encoded as (from_id << 32) | to_id
                if node.input_regs.len() >= 2 {
                    let from_id = node.input_regs[0] as u64;
                    let to_id = node.input_regs[1] as u64;
                    let operand = (from_id << 32) | to_id;
                    self.instructions.push(Instruction::new(Opcode::Connect as u8, node.flags as u8, operand));
                }
            },
            _ => {
                println!("Compiling graph node with opcode: {}", node.opcode);
            }
        }
        Ok(())
    }

    fn compile_io_node(&mut self, node: &kern_graph_builder::GraphNode) -> Result<(), &'static str> {
        // Compile external interface calls
        match node.opcode {
            0x60 => { // CALL_EXTERN
                // operand: function ID
                self.instructions.push(Instruction::new(Opcode::CallExtern as u8, node.flags as u8, node.id as u64));
            },
            0x61 => { // READ_IO
                // operand: IO ID
                self.instructions.push(Instruction::new(Opcode::ReadIo as u8, node.flags as u8, node.id as u64));
            },
            0x62 => { // WRITE_IO
                // operand: IO ID
                self.instructions.push(Instruction::new(Opcode::WriteIo as u8, node.flags as u8, node.id as u64));
            },
            _ => {
                println!("Compiling IO node with opcode: {}", node.opcode);
            }
        }
        Ok(())
    }

    fn get_or_create_symbol(&mut self, symbol: &str) -> u64 {
        if let Some(&id) = self.symbol_table.get(symbol) {
            id
        } else {
            let id = self.next_symbol_id;
            self.symbol_table.insert(symbol.to_string(), id);
            self.next_symbol_id += 1;
            id
        }
    }

    // Helper function to get bytecode as bytes
    pub fn get_bytecode_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for instruction in &self.instructions {
            bytes.extend_from_slice(&instruction.to_bytes());
        }
        bytes
    }

    // Helper function to load bytecode from bytes
    pub fn load_from_bytes(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        if bytes.len() % 8 != 0 {
            return Err("Invalid bytecode length: must be multiple of 8 bytes per instruction");
        }

        self.instructions.clear();
        for chunk in bytes.chunks(8) {
            if let Some(instruction) = Instruction::from_bytes(chunk) {
                self.instructions.push(instruction);
            } else {
                return Err("Failed to parse instruction from bytes");
            }
        }

        Ok(())
    }
}

// Decoder algorithm implementation
pub fn decode(instr_bytes: &[u8]) -> Option<Instruction> {
    Instruction::from_bytes(instr_bytes)
}

// Operand encoding and decoding utilities
pub mod operand_utils {
    use super::Instruction;

    // Encode two 24-bit values into a 48-bit operand
    pub fn encode_two_24bit(val1: u32, val2: u32) -> u64 {
        let masked_val1 = (val1 & 0xFFFFFF) as u64;
        let masked_val2 = (val2 & 0xFFFFFF) as u64;
        (masked_val1 << 24) | masked_val2
    }

    // Decode two 24-bit values from a 48-bit operand
    pub fn decode_two_24bit(operand: u64) -> (u32, u32) {
        let val1 = ((operand >> 24) & 0xFFFFFF) as u32;
        let val2 = (operand & 0xFFFFFF) as u32;
        (val1, val2)
    }

    // Encode three 16-bit values into a 48-bit operand
    pub fn encode_three_16bit(val1: u16, val2: u16, val3: u16) -> u64 {
        let val1 = val1 as u64;
        let val2 = val2 as u64;
        let val3 = val3 as u64;
        (val1 << 32) | (val2 << 16) | val3
    }

    // Decode three 16-bit values from a 48-bit operand
    pub fn decode_three_16bit(operand: u64) -> (u16, u16, u16) {
        let val1 = ((operand >> 32) & 0xFFFF) as u16;
        let val2 = ((operand >> 16) & 0xFFFF) as u16;
        let val3 = (operand & 0xFFFF) as u16;
        (val1, val2, val3)
    }

    // Encode a 48-bit address or ID
    pub fn encode_address(addr: u64) -> u64 {
        addr & 0xFFFFFFFFFFFF
    }

    // Encode symbol ID with additional flags
    pub fn encode_symbol_with_flags(symbol_id: u32, flags: u8) -> u64 {
        let id_part = (symbol_id as u64) << 16;
        let flags_part = (flags as u64) & 0xFFFF;
        id_part | flags_part
    }

    // Decode symbol ID and flags from operand
    pub fn decode_symbol_with_flags(operand: u64) -> (u32, u8) {
        let symbol_id = ((operand >> 16) & 0xFFFFFFFF) as u32;
        let flags = (operand & 0xFFFF) as u8;
        (symbol_id, flags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kern_parser::Parser;
    use kern_graph_builder::{GraphBuilder, ExecutionGraph};

    #[test]
    fn test_instruction_serialization() {
        let instruction = Instruction::new(0x10, 0xFF, 0x12345);
        let bytes = instruction.to_bytes();
        let parsed_instruction = Instruction::from_bytes(&bytes).unwrap();

        assert_eq!(instruction.opcode, parsed_instruction.opcode);
        assert_eq!(instruction.flags, parsed_instruction.flags);
        assert_eq!(instruction.operand, parsed_instruction.operand);
    }

    #[test]
    fn test_bytecode_compiler() {
        let input = r#"
        entity Farmer {
            id
            location
        }

        rule CheckLocation:
            if farmer.location == "valid"
            then approve_farmer(farmer)
        "#;

        let mut parser = Parser::new(input);
        let result = parser.parse_program();
        assert!(result.is_ok());
        let program = result.unwrap();

        let mut builder = GraphBuilder::new();
        let graph = builder.build_execution_graph(&program);
        println!("Generated execution graph with {} nodes", graph.nodes.len());

        let mut compiler = BytecodeCompiler::new();
        let bytecode = compiler.compile_graph(&graph);
        println!("Compiled {} bytecode instructions", bytecode.len());

        assert!(!bytecode.is_empty());
    }

    #[test]
    fn test_bytecode_serialization() {
        let input = r#"
        entity TestEntity {
            field1
        }

        rule TestRule:
            if field1 == 42
            then action()
        "#;

        let mut parser = Parser::new(input);
        let result = parser.parse_program();
        assert!(result.is_ok());
        let program = result.unwrap();

        let mut builder = GraphBuilder::new();
        let graph = builder.build_execution_graph(&program);

        let mut compiler = BytecodeCompiler::new();
        let _bytecode = compiler.compile_graph(&graph);

        // Test serialization
        let bytecode_bytes = compiler.get_bytecode_bytes();
        println!("Serialized {} bytes of bytecode", bytecode_bytes.len());

        // Test deserialization
        let mut new_compiler = BytecodeCompiler::new();
        let load_result = new_compiler.load_from_bytes(&bytecode_bytes);
        assert!(load_result.is_ok());
        assert_eq!(compiler.instructions.len(), new_compiler.instructions.len());
    }
}

#[cfg(test)]
mod decoder_tests {
    use super::*;

    #[test]
    fn test_decoder_algorithm() {
        let instruction = Instruction::new(0x10, 0xFF, 0x12345);
        let bytes = instruction.to_bytes();
        let decoded_instruction = decode(&bytes).unwrap();

        assert_eq!(instruction.opcode, decoded_instruction.opcode);
        assert_eq!(instruction.flags, decoded_instruction.flags);
        assert_eq!(instruction.operand, decoded_instruction.operand);
    }

    #[test]
    fn test_operand_encoding_decoding() {
        // Test two 24-bit encoding/decoding
        let val1 = 0xABCDEFu32;
        let val2 = 0x123456u32;
        let encoded = operand_utils::encode_two_24bit(val1, val2);
        let (decoded_val1, decoded_val2) = operand_utils::decode_two_24bit(encoded);
        assert_eq!(val1, decoded_val1);
        assert_eq!(val2, decoded_val2);

        // Test three 16-bit encoding/decoding
        let val1 = 0xABCDu16;
        let val2 = 0x1234u16;
        let val3 = 0x5678u16;
        let encoded = operand_utils::encode_three_16bit(val1, val2, val3);
        let (decoded_val1, decoded_val2, decoded_val3) = operand_utils::decode_three_16bit(encoded);
        assert_eq!(val1, decoded_val1);
        assert_eq!(val2, decoded_val2);
        assert_eq!(val3, decoded_val3);

        // Test symbol with flags encoding/decoding
        let symbol_id = 0x12345u32;
        let flags = 0xABu8;
        let encoded = operand_utils::encode_symbol_with_flags(symbol_id, flags);
        let (decoded_id, decoded_flags) = operand_utils::decode_symbol_with_flags(encoded);
        assert_eq!(symbol_id, decoded_id);
        assert_eq!(flags, decoded_flags);
    }
}