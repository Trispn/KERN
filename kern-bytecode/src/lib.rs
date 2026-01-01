use std::collections::HashMap;

// Define the KERN bytecode instruction format
// Each instruction is 8 bytes: OPCODE (1B) | ARG1 (2B) | ARG2 (2B) | ARG3 (2B) | FLAGS (1B)
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: u8,
    pub arg1: u16,
    pub arg2: u16,
    pub arg3: u16,
    pub flags: u8,
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
            (self.arg1 >> 8) as u8,
            (self.arg2 & 0xFF) as u8,
            (self.arg2 >> 8) as u8,
            (self.arg3 & 0xFF) as u8,
            (self.arg3 >> 8) as u8,
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

// Define the KERN opcodes as specified in the language documentation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    // Control Flow Instructions
    Nop = 0x00,
    Jmp = 0x01,
    JmpIf = 0x02,
    Halt = 0x03,

    // Data & Symbol Instructions
    LoadSym = 0x10,
    LoadNum = 0x11,
    Move = 0x12,
    Compare = 0x13,

    // Graph Instructions
    GraphNodeCreate = 0x20,
    GraphEdgeCreate = 0x21,
    GraphMatch = 0x22,
    GraphTraverse = 0x23,

    // Rule Execution Instructions
    RuleLoad = 0x30,
    RuleEval = 0x31,
    RuleFire = 0x32,
    RulePrioritySet = 0x33,

    // Context & State Instructions
    CtxCreate = 0x40,
    CtxSwitch = 0x41,
    CtxClone = 0x42,
    CtxDestroy = 0x43,

    // Error Handling Instructions
    ErrSet = 0x50,
    ErrClear = 0x51,
    ErrCheck = 0x52,

    // External Interface Instructions
    ExtCall = 0x60,
    ExtBind = 0x61,

    // Termination & Output
    Return = 0x70,
    Output = 0x71,
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
            0x20 => Opcode::GraphNodeCreate,
            0x21 => Opcode::GraphEdgeCreate,
            0x22 => Opcode::GraphMatch,
            0x23 => Opcode::GraphTraverse,
            0x30 => Opcode::RuleLoad,
            0x31 => Opcode::RuleEval,
            0x32 => Opcode::RuleFire,
            0x33 => Opcode::RulePrioritySet,
            0x40 => Opcode::CtxCreate,
            0x41 => Opcode::CtxSwitch,
            0x42 => Opcode::CtxClone,
            0x43 => Opcode::CtxDestroy,
            0x50 => Opcode::ErrSet,
            0x51 => Opcode::ErrClear,
            0x52 => Opcode::ErrCheck,
            0x60 => Opcode::ExtCall,
            0x61 => Opcode::ExtBind,
            0x70 => Opcode::Return,
            0x71 => Opcode::Output,
            _ => Opcode::Nop, // Default to NOP for unknown opcodes
        }
    }
}

// Define the bytecode compiler that translates execution graphs to bytecode
pub struct BytecodeCompiler {
    instructions: Vec<Instruction>,
    symbol_table: HashMap<String, u16>,
    next_symbol_id: u16,
    register_map: HashMap<String, u16>, // Maps variable names to register IDs
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
            self.compile_node(node)?;
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
                // arg1: destination register, arg2: symbol ID
                let dest_reg = node.output_regs[0];
                let symbol_id = self.get_or_create_symbol("temp_symbol"); // In real impl, get from metadata
                self.instructions.push(Instruction::new(Opcode::LoadSym as u8, dest_reg, symbol_id, 0, node.flags as u8));
            },
            0x11 => { // LOAD_NUM
                // arg1: destination register, arg2: number value
                let dest_reg = node.output_regs[0];
                let value = node.id as u16; // In real impl, get actual value
                self.instructions.push(Instruction::new(Opcode::LoadNum as u8, dest_reg, value, 0, node.flags as u8));
            },
            0x12 => { // MOVE
                // arg1: destination register, arg2: source register
                let dest_reg = node.output_regs[0];
                let src_reg = node.input_regs[0];
                self.instructions.push(Instruction::new(Opcode::Move as u8, dest_reg, src_reg, 0, node.flags as u8));
            },
            0x13 => { // COMPARE
                // arg1: register A, arg2: register B, arg3: result register
                let reg_a = node.input_regs[0];
                let reg_b = node.input_regs[1];
                let result_reg = node.output_regs[0];
                self.instructions.push(Instruction::new(Opcode::Compare as u8, reg_a, reg_b, result_reg, node.flags as u8));
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
        self.instructions.push(Instruction::new(Opcode::RuleLoad as u8, node.id as u16, 0, 0, node.flags as u8));
        self.instructions.push(Instruction::new(Opcode::RuleEval as u8, node.id as u16, 0, 0, node.flags as u8));
        
        // If rule evaluates to true, fire it
        self.instructions.push(Instruction::new(Opcode::RuleFire as u8, node.id as u16, 0, 0, node.flags as u8));
        
        Ok(())
    }

    fn compile_control_node(&mut self, node: &kern_graph_builder::GraphNode) -> Result<(), &'static str> {
        match node.opcode {
            0x00 => { // NOP
                self.instructions.push(Instruction::new(Opcode::Nop as u8, 0, 0, 0, node.flags as u8));
            },
            0x01 => { // JMP
                // arg1: target instruction index
                let target = node.input_regs[0]; // In real impl, get actual target
                self.instructions.push(Instruction::new(Opcode::Jmp as u8, target, 0, 0, node.flags as u8));
            },
            0x02 => { // JMP_IF
                // arg1: flag register, arg2: target instruction index
                let flag_reg = node.input_regs[0];
                let target = node.input_regs[1]; // In real impl, get actual target
                self.instructions.push(Instruction::new(Opcode::JmpIf as u8, flag_reg, target, 0, node.flags as u8));
            },
            0x03 => { // HALT
                self.instructions.push(Instruction::new(Opcode::Halt as u8, 0, 0, 0, node.flags as u8));
            },
            _ => {
                println!("Compiling control node with opcode: {}", node.opcode);
            }
        }
        Ok(())
    }

    fn compile_graph_node(&mut self, _node: &kern_graph_builder::GraphNode) -> Result<(), &'static str> {
        // Compile graph operations
        println!("Compiling graph node");
        Ok(())
    }

    fn compile_io_node(&mut self, _node: &kern_graph_builder::GraphNode) -> Result<(), &'static str> {
        // Compile external interface calls
        println!("Compiling IO node");
        Ok(())
    }

    fn get_or_create_symbol(&mut self, symbol: &str) -> u16 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use kern_parser::Parser;
    use kern_graph_builder::{GraphBuilder, ExecutionGraph};

    #[test]
    fn test_instruction_serialization() {
        let instruction = Instruction::new(0x10, 1, 2, 3, 0xFF);
        let bytes = instruction.to_bytes();
        let parsed_instruction = Instruction::from_bytes(&bytes).unwrap();

        assert_eq!(instruction.opcode, parsed_instruction.opcode);
        assert_eq!(instruction.arg1, parsed_instruction.arg1);
        assert_eq!(instruction.arg2, parsed_instruction.arg2);
        assert_eq!(instruction.arg3, parsed_instruction.arg3);
        assert_eq!(instruction.flags, parsed_instruction.flags);
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