use kern_bytecode::{BytecodeCompiler, Instruction, Opcode};
use std::collections::HashMap;

// Define the KERN VM registers
#[derive(Debug, Clone)]
pub struct VmRegisters {
    pub r: [i64; 16], // General purpose registers R0-R15
    pub ctx: u64,     // Current execution context
    pub err: u8,      // Error register
    pub pc: u32,      // Program counter
    pub flag: u8,     // Condition flags
}

impl VmRegisters {
    pub fn new() -> Self {
        VmRegisters {
            r: [0; 16],
            ctx: 0,
            err: 0,
            pc: 0,
            flag: 0,
        }
    }
}

// Define the KERN VM execution context
#[derive(Debug, Clone)]
pub struct VmContext {
    pub id: u64,
    pub registers: VmRegisters,
    pub memory: Vec<u8>,
    pub variables: HashMap<String, i64>,
}

impl VmContext {
    pub fn new(id: u64) -> Self {
        VmContext {
            id,
            registers: VmRegisters::new(),
            memory: vec![0; 1024], // 1KB initial memory
            variables: HashMap::new(),
        }
    }
}

// Define the KERN Virtual Machine
pub struct VirtualMachine {
    pub registers: VmRegisters,
    pub contexts: Vec<VmContext>,
    pub current_context: usize,
    pub memory: Vec<u8>,
    pub program: Vec<Instruction>,
    pub pc: u32, // Program counter
    pub running: bool,
    pub max_steps: u32, // Maximum execution steps to prevent infinite loops
    pub step_count: u32,
    pub external_functions: HashMap<String, fn(&mut VirtualMachine) -> Result<(), String>>,
}

#[derive(Debug)]
pub enum VmError {
    InvalidOpcode(u8),
    InvalidRegister(u16),
    InvalidAddress(u32),
    ExecutionLimitExceeded,
    DivisionByZero,
    StackOverflow,
    StackUnderflow,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            registers: VmRegisters::new(),
            contexts: vec![VmContext::new(0)], // Initialize with one context
            current_context: 0,
            memory: vec![0; 1024 * 100], // 100KB memory
            program: Vec::new(),
            pc: 0,
            running: false,
            max_steps: 100000, // 100k max steps
            step_count: 0,
            external_functions: HashMap::new(),
        }
    }

    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
        self.pc = 0;
    }

    pub fn execute(&mut self) -> Result<(), VmError> {
        self.running = true;
        self.step_count = 0;

        while self.running
            && self.pc < self.program.len() as u32
            && self.step_count < self.max_steps
        {
            let instruction = self.program[self.pc as usize].clone();
            self.execute_instruction(&instruction)?;
            self.pc += 1;
            self.step_count += 1;
        }

        if self.step_count >= self.max_steps {
            return Err(VmError::ExecutionLimitExceeded);
        }

        Ok(())
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        match instruction.opcode as u8 {
            // Control Flow Instructions
            0x00 => self.op_nop(),                // NOP
            0x01 => self.op_jmp(instruction)?,    // JMP
            0x02 => self.op_jmp_if(instruction)?, // JMP_IF
            0x03 => self.op_halt(),               // HALT

            // Data & Symbol Instructions
            0x10 => self.op_load_sym(instruction)?, // LOAD_SYM
            0x11 => self.op_load_num(instruction)?, // LOAD_NUM
            0x12 => self.op_move(instruction)?,     // MOVE
            0x13 => self.op_compare(instruction)?,  // COMPARE

            // Graph Instructions
            0x20 => self.op_graph_node_create(instruction)?, // GRAPH_NODE_CREATE
            0x21 => self.op_graph_edge_create(instruction)?, // GRAPH_EDGE_CREATE
            0x22 => self.op_graph_match(instruction)?,       // GRAPH_MATCH
            0x23 => self.op_graph_traverse(instruction)?,    // GRAPH_TRAVERSE

            // Rule Execution Instructions
            0x30 => self.op_rule_load(instruction)?, // RULE_LOAD
            0x31 => self.op_rule_eval(instruction)?, // RULE_EVAL
            0x32 => self.op_rule_fire(instruction)?, // RULE_FIRE
            0x33 => self.op_rule_priority_set(instruction)?, // RULE_PRIORITY_SET

            // Context & State Instructions
            0x40 => self.op_ctx_create(instruction)?, // CTX_CREATE
            0x41 => self.op_ctx_switch(instruction)?, // CTX_SWITCH
            0x42 => self.op_ctx_clone(instruction)?,  // CTX_CLONE
            0x43 => self.op_ctx_destroy(instruction)?, // CTX_DESTROY

            // Error Handling Instructions
            0x50 => self.op_err_set(instruction)?,   // ERR_SET
            0x51 => self.op_err_clear(),             // ERR_CLEAR
            0x52 => self.op_err_check(instruction)?, // ERR_CHECK

            // External Interface Instructions
            0x60 => self.op_ext_call(instruction)?, // EXT_CALL
            0x61 => self.op_ext_bind(instruction)?, // EXT_BIND

            // Termination & Output
            0x70 => self.op_return(instruction)?, // RETURN
            0x71 => self.op_output(instruction)?, // OUTPUT

            _ => return Err(VmError::InvalidOpcode(instruction.opcode)),
        }

        Ok(())
    }

    // Control Flow Instructions
    fn op_nop(&mut self) {
        // No operation - just continue to next instruction
    }

    fn op_jmp(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Unconditional jump to target instruction
        let target = instruction.operand as u32;
        if target < self.program.len() as u32 {
            self.pc = target.saturating_sub(1); // -1 because the main loop will increment pc
        }
        Ok(())
    }

    fn op_jmp_if(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Conditional jump based on flag register
        // operand: (flag_reg << 32) | target
        let flag_reg = (instruction.operand >> 32) as usize;
        let target = (instruction.operand & 0xFFFFFFFF) as u32;

        if flag_reg >= self.registers.r.len() {
            return Err(VmError::InvalidRegister(flag_reg as u16));
        }

        if self.registers.r[flag_reg] != 0 {
            if target < self.program.len() as u32 {
                self.pc = target.saturating_sub(1); // -1 because the main loop will increment pc
            }
        }

        Ok(())
    }

    fn op_halt(&mut self) {
        self.running = false;
    }

    // Data & Symbol Instructions
    fn op_load_sym(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Load a symbol into a register
        // operand: (symbol_id << 32) | dest_reg
        let symbol_id = (instruction.operand >> 32) as u32;
        let dest_reg = (instruction.operand & 0xFFFFFFFF) as usize;

        if dest_reg >= self.registers.r.len() {
            return Err(VmError::InvalidRegister(dest_reg as u16));
        }

        // In a real implementation, this would load a symbol from the symbol table
        self.registers.r[dest_reg] = symbol_id as i64;
        Ok(())
    }

    fn op_load_num(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Load a number into a register
        // operand: (dest_reg << 32) | value
        let dest_reg = (instruction.operand >> 32) as usize;
        let value = (instruction.operand & 0xFFFFFFFF) as i64;

        if dest_reg >= self.registers.r.len() {
            return Err(VmError::InvalidRegister(dest_reg as u16));
        }

        self.registers.r[dest_reg] = value;
        Ok(())
    }

    fn op_move(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Move value from one register to another
        // operand: (src_reg << 32) | dest_reg
        let src_reg = (instruction.operand >> 32) as usize;
        let dest_reg = (instruction.operand & 0xFFFFFFFF) as usize;

        if dest_reg >= self.registers.r.len() || src_reg >= self.registers.r.len() {
            return Err(VmError::InvalidRegister(
                if dest_reg >= self.registers.r.len() {
                    dest_reg as u16
                } else {
                    src_reg as u16
                },
            ));
        }

        self.registers.r[dest_reg] = self.registers.r[src_reg];
        Ok(())
    }

    fn op_compare(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Compare two registers and set flags
        // operand: (reg_a << 32) | (reg_b << 16) | result_reg
        let reg_a = (instruction.operand >> 32) as usize;
        let reg_b = ((instruction.operand >> 16) & 0xFFFF) as usize;
        let result_reg = (instruction.operand & 0xFFFF) as usize;

        if reg_a >= self.registers.r.len()
            || reg_b >= self.registers.r.len()
            || result_reg >= self.registers.r.len()
        {
            return Err(VmError::InvalidRegister(reg_a as u16));
        }

        let val_a = self.registers.r[reg_a];
        let val_b = self.registers.r[reg_b];

        // Set flags based on comparison
        let result = match instruction.flags {
            0 => val_a == val_b, // Equal
            1 => val_a != val_b, // Not Equal
            2 => val_a > val_b,  // Greater
            3 => val_a < val_b,  // Less
            4 => val_a >= val_b, // Greater or Equal
            5 => val_a <= val_b, // Less or Equal
            _ => false,
        };

        if result_reg < self.registers.r.len() {
            self.registers.r[result_reg] = if result { 1 } else { 0 };
        }
        self.registers.flag = if result { 1 } else { 0 };

        Ok(())
    }

    // Graph Instructions (simplified for this implementation)
    fn op_graph_node_create(&mut self, _instruction: &Instruction) -> Result<(), VmError> {
        // Create a graph node
        println!("Creating graph node");
        Ok(())
    }

    fn op_graph_edge_create(&mut self, _instruction: &Instruction) -> Result<(), VmError> {
        // Create a graph edge
        println!("Creating graph edge");
        Ok(())
    }

    fn op_graph_match(&mut self, _instruction: &Instruction) -> Result<(), VmError> {
        // Perform graph pattern matching
        println!("Performing graph match");
        Ok(())
    }

    fn op_graph_traverse(&mut self, _instruction: &Instruction) -> Result<(), VmError> {
        // Traverse the graph
        println!("Traversing graph");
        Ok(())
    }

    // Rule Execution Instructions
    fn op_rule_load(&mut self, _instruction: &Instruction) -> Result<(), VmError> {
        // Load a rule for execution
        println!("Loading rule");
        Ok(())
    }

    fn op_rule_eval(&mut self, _instruction: &Instruction) -> Result<(), VmError> {
        // Evaluate a rule
        println!("Evaluating rule");
        Ok(())
    }

    fn op_rule_fire(&mut self, _instruction: &Instruction) -> Result<(), VmError> {
        // Fire a rule
        println!("Firing rule");
        Ok(())
    }

    fn op_rule_priority_set(&mut self, _instruction: &Instruction) -> Result<(), VmError> {
        // Set rule priority
        println!("Setting rule priority");
        Ok(())
    }

    // Context & State Instructions
    fn op_ctx_create(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Create a new execution context
        let new_ctx_id = self.contexts.len() as u64;
        let new_ctx = VmContext::new(new_ctx_id);
        self.contexts.push(new_ctx);

        // Store the context ID in the specified register
        let dest_reg = instruction.operand as usize;
        if dest_reg < self.registers.r.len() {
            self.registers.r[dest_reg] = new_ctx_id as i64;
        }

        Ok(())
    }

    fn op_ctx_switch(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Switch to a different execution context
        // operand: reg containing ctx_id
        let reg = instruction.operand as usize;
        if reg < self.registers.r.len() {
            let ctx_id = self.registers.r[reg] as usize;
            if ctx_id < self.contexts.len() {
                self.current_context = ctx_id;
            }
        }
        Ok(())
    }

    fn op_ctx_clone(&mut self, _instruction: &Instruction) -> Result<(), VmError> {
        // Clone an execution context
        println!("Cloning context");
        Ok(())
    }

    fn op_ctx_destroy(&mut self, _instruction: &Instruction) -> Result<(), VmError> {
        // Destroy an execution context
        println!("Destroying context");
        Ok(())
    }

    // Error Handling Instructions
    fn op_err_set(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Set error register
        // operand: error code
        self.registers.err = instruction.operand as u8;
        Ok(())
    }

    fn op_err_clear(&mut self) {
        // Clear error register
        self.registers.err = 0;
    }

    fn op_err_check(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Check for error and jump if error exists
        // operand: target address
        if self.registers.err != 0 {
            let target = instruction.operand as u32;
            if target < self.program.len() as u32 {
                self.pc = target.saturating_sub(1); // -1 because the main loop will increment pc
            }
        }
        Ok(())
    }

    // External Interface Instructions
    fn op_ext_call(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Call an external function
        // operand: function ID
        let fn_id = instruction.operand;
        println!("Calling external function with ID: {}", fn_id);
        Ok(())
    }

    fn op_ext_bind(&mut self, _instruction: &Instruction) -> Result<(), VmError> {
        // Bind a symbol to an external adapter
        println!("Binding external function");
        Ok(())
    }

    // Termination & Output
    fn op_return(&mut self, _instruction: &Instruction) -> Result<(), VmError> {
        // Return from current execution
        self.running = false;
        Ok(())
    }

    fn op_output(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Output a value
        // operand: register index
        let reg = instruction.operand as usize;
        if reg < self.registers.r.len() {
            println!("Output: {}", self.registers.r[reg]);
        }
        Ok(())
    }

    // Helper function to add an external function
    pub fn add_external_function(
        &mut self,
        name: &str,
        func: fn(&mut VirtualMachine) -> Result<(), String>,
    ) {
        self.external_functions.insert(name.to_string(), func);
    }

    // Helper function to get register value
    pub fn get_register(&self, reg: usize) -> Option<i64> {
        if reg < self.registers.r.len() {
            Some(self.registers.r[reg])
        } else {
            None
        }
    }

    // Helper function to set register value
    pub fn set_register(&mut self, reg: usize, value: i64) -> Result<(), VmError> {
        if reg >= self.registers.r.len() {
            return Err(VmError::InvalidRegister(reg as u16));
        }
        self.registers.r[reg] = value;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kern_bytecode::BytecodeCompiler;
    use kern_graph_builder::GraphBuilder;
    use kern_parser::Parser;

    #[test]
    fn test_vm_execution() {
        let input = r#"
        entity TestEntity {
            value
        }

        rule TestRule:
            if value == 42
            then output(value)
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

        let mut vm = VirtualMachine::new();
        vm.load_program(bytecode);

        let execution_result = vm.execute();
        assert!(execution_result.is_ok());

        println!("VM executed successfully with {} steps", vm.step_count);
    }

    #[test]
    fn test_basic_vm_operations() {
        let mut vm = VirtualMachine::new();

        // Create a simple program: load 42 into R0, load 24 into R1, compare them
        let program = vec![
            Instruction::new(0x11, 0, (0u64 << 32) | 42), // LOAD_NUM R0, 42
            Instruction::new(0x11, 0, (1u64 << 32) | 24), // LOAD_NUM R1, 24
            Instruction::new(0x13, 0, (0u64 << 32) | (1u64 << 16) | 2), // COMPARE R0, R1, R2 (Equal?)
        ];

        vm.load_program(program);
        let result = vm.execute();
        assert!(result.is_ok());

        // Check that R2 contains the comparison result (0 for false, since 42 != 24)
        assert_eq!(vm.get_register(2), Some(0));
    }
}
