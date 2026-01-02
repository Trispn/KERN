use kern_bytecode::{BytecodeModule, Instruction, Opcode};
use std::collections::HashMap;

// Define the KERN VM registers
#[derive(Debug, Clone)]
pub struct VmRegisters {
    pub r: [i64; 16], // General purpose registers R0-R15
    pub pc: u32,      // Program Counter (instruction index, not byte offset)
    pub flag: u64,    // Condition flags (ZERO, NEG, ERR, HALT, CMP bits)
    pub ctx: u64,     // Current Context ID
    pub err: u64,     // Error Register (holds error code or 0)
}

impl VmRegisters {
    pub fn new() -> Self {
        VmRegisters {
            r: [0; 16],
            pc: 0,
            flag: 0,
            ctx: 0,
            err: 0,
        }
    }

    // Helper methods for flag register
    pub fn set_zero_flag(&mut self, value: bool) {
        if value {
            self.flag |= 1; // Bit 0: ZERO
        } else {
            self.flag &= !1;
        }
    }

    pub fn set_negative_flag(&mut self, value: bool) {
        if value {
            self.flag |= 1 << 1; // Bit 1: NEGATIVE
        } else {
            self.flag &= !(1 << 1);
        }
    }

    pub fn set_compare_true_flag(&mut self, value: bool) {
        if value {
            self.flag |= 1 << 2; // Bit 2: COMPARE_TRUE
        } else {
            self.flag &= !(1 << 2);
        }
    }

    pub fn set_error_flag(&mut self, value: bool) {
        if value {
            self.flag |= 1 << 3; // Bit 3: ERROR_PRESENT
        } else {
            self.flag &= !(1 << 3);
        }
    }

    pub fn set_halt_flag(&mut self, value: bool) {
        if value {
            self.flag |= 1 << 4; // Bit 4: HALT_REQUESTED
        } else {
            self.flag &= !(1 << 4);
        }
    }

    pub fn is_zero(&self) -> bool {
        (self.flag & 1) != 0
    }

    pub fn is_negative(&self) -> bool {
        (self.flag & (1 << 1)) != 0
    }

    pub fn is_compare_true(&self) -> bool {
        (self.flag & (1 << 2)) != 0
    }

    pub fn has_error(&self) -> bool {
        (self.flag & (1 << 3)) != 0
    }

    pub fn is_halt_requested(&self) -> bool {
        (self.flag & (1 << 4)) != 0
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

// Memory regions for the KERN VM
#[derive(Debug, Clone)]
pub struct MemoryRegions {
    pub code: Vec<u8>,    // Read-only bytecode
    pub constants: Vec<u8>, // Read-only constants
    pub stack: Vec<u8>,   // Operand + call stack
    pub heap: Vec<u8>,    // Graph nodes, symbols, contexts
    pub meta: Vec<u8>,    // PSI introspection & metadata
}

impl MemoryRegions {
    pub fn new() -> Self {
        MemoryRegions {
            code: Vec::new(),
            constants: Vec::new(),
            stack: vec![0; 4096], // 4KB stack with hard limit
            heap: vec![0; 1024 * 100], // 100KB heap
            meta: vec![0; 1024], // 1KB metadata
        }
    }
}

// Define the KERN Virtual Machine
pub struct VirtualMachine {
    pub registers: VmRegisters,
    pub contexts: Vec<VmContext>,
    pub current_context: usize,
    pub memory: MemoryRegions,
    pub program: Vec<Instruction>,
    pub running: bool,
    pub max_steps: u32, // Maximum execution steps to prevent infinite loops
    pub step_count: u32,
    pub external_functions: HashMap<String, fn(&mut VirtualMachine) -> Result<(), String>>,
    pub execution_trace: Vec<ExecutionTraceEntry>, // For PSI introspection
    jumped: bool, // Track if the last instruction was a jump
}

// Execution trace entry for PSI introspection
#[derive(Debug, Clone)]
pub struct ExecutionTraceEntry {
    pub pc_before: u32,
    pub opcode: u8,
    pub operands: u64,
    pub register_diff: [i64; 16], // Difference in general purpose registers
    pub memory_diff: Vec<u8>,     // Memory changes
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
    InvalidPc,           // PC out of range
    InvalidInstruction,  // Invalid instruction format
    UndefinedSymbol,     // Symbol not found in context
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            registers: VmRegisters::new(),
            contexts: vec![VmContext::new(0)], // Initialize with one context
            current_context: 0,
            memory: MemoryRegions::new(),
            program: Vec::new(),
            running: false,
            max_steps: 100000, // 100k max steps
            step_count: 0,
            external_functions: HashMap::new(),
            execution_trace: Vec::new(),
            jumped: false,
        }
    }

    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
        self.registers.pc = 0;
    }

    /// Execute the program using the canonical fetch-decode-execute cycle
    pub fn execute(&mut self) -> Result<(), VmError> {
        self.running = true;
        self.step_count = 0;

        while !self.registers.is_halt_requested()
            && self.registers.pc < self.program.len() as u32
            && self.step_count < self.max_steps
        {
            self.step()?;
            self.step_count += 1;
        }

        if self.step_count >= self.max_steps {
            return Err(VmError::ExecutionLimitExceeded);
        }

        // Set running to false when execution completes
        self.running = false;

        Ok(())
    }

    /// Execute exactly one instruction (for step-by-step execution)
    pub fn step(&mut self) -> Result<(), VmError> {
        // Reset the jump flag at the beginning of each step
        self.jumped = false;

        // Fetch instruction
        let instruction = self.fetch()?;

        // Save state before execution for trace
        let pc_before = self.registers.pc;
        let mut register_diff = [0i64; 16];
        for i in 0..16 {
            register_diff[i] = self.registers.r[i]; // Save original values
        }

        // Execute instruction
        self.execute_instruction(&instruction)?;

        // Calculate register differences for trace
        for i in 0..16 {
            register_diff[i] = self.registers.r[i] - register_diff[i];
        }

        // Add to execution trace for PSI introspection
        let trace_entry = ExecutionTraceEntry {
            pc_before,
            opcode: instruction.opcode as u8,
            operands: (instruction.arg1 as u64) | ((instruction.arg2 as u64) << 16) | ((instruction.arg3 as u64) << 32) | ((instruction.flags as u64) << 48),
            register_diff,
            memory_diff: Vec::new(), // Simplified for now
        };
        self.execution_trace.push(trace_entry);

        // Increment PC if no jump occurred in the instruction
        if !self.jumped {
            self.registers.pc += 1;
        }

        Ok(())
    }

    /// Fetch instruction from program memory
    fn fetch(&self) -> Result<Instruction, VmError> {
        if self.registers.pc >= self.program.len() as u32 {
            return Err(VmError::InvalidPc);
        }
        Ok(self.program[self.registers.pc as usize].clone())
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

    // Context management methods
    fn push_context(&mut self, new_context: VmContext) {
        self.contexts.push(new_context);
    }

    fn pop_context(&mut self) -> Option<VmContext> {
        if self.contexts.len() > 1 { // Keep at least one context
            self.contexts.pop()
        } else {
            None
        }
    }

    fn copy_context(&mut self, ctx_id: u64) -> Option<VmContext> {
        self.contexts.get(ctx_id as usize).cloned().map(|mut ctx| {
            ctx.id = self.contexts.len() as u64; // Assign new ID
            ctx
        })
    }

    // Control Flow Instructions
    fn op_nop(&mut self) {
        // No operation - just continue to next instruction
    }

    fn op_jmp(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Unconditional jump to target instruction
        let target = instruction.arg1 as u32;
        if target < self.program.len() as u32 {
            self.registers.pc = target;
            self.jumped = true;
            return Ok(());
        }
        Err(VmError::InvalidPc)
    }

    fn op_jmp_if(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Conditional jump based on flag register
        // operand: target address
        let target = instruction.arg1 as u32;

        if self.registers.is_compare_true() {
            if target < self.program.len() as u32 {
                self.registers.pc = target;
                self.jumped = true;
                return Ok(());
            }
        }

        Ok(())
    }

    fn op_halt(&mut self) {
        self.registers.set_halt_flag(true);
    }

    // Data & Symbol Instructions
    fn op_load_sym(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Load a symbol into a register
        // operand: (symbol_id << 32) | dest_reg
        let symbol_id = (instruction.arg1 as u32) | ((instruction.arg2 as u32) << 16);
        let dest_reg = instruction.arg3 as usize;

        if dest_reg >= self.registers.r.len() {
            return Err(VmError::InvalidRegister(dest_reg as u16));
        }

        // In a real implementation, this would load a symbol from the symbol table
        self.registers.r[dest_reg] = symbol_id as i64;
        Ok(())
    }

    fn op_load_num(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Load a number into a register
        // operand: (dest_reg << 32) | value (where value is in lower 32 bits)
        let dest_reg = instruction.arg1 as usize;
        let value = instruction.arg2 as i64; // Extract lower 32 bits as unsigned value

        if dest_reg >= self.registers.r.len() {
            return Err(VmError::InvalidRegister(dest_reg as u16));
        }

        self.registers.r[dest_reg] = value;
        Ok(())
    }

    fn op_move(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Move value from one register to another
        // operand: (src_reg << 32) | dest_reg
        let src_reg = instruction.arg1 as usize;
        let dest_reg = instruction.arg2 as usize;

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
        let reg_a = instruction.arg1 as usize;
        let reg_b = instruction.arg2 as usize;
        let result_reg = instruction.arg3 as usize;

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

        // Update flags
        self.registers.set_zero_flag(val_a == val_b);
        self.registers.set_negative_flag(val_a < val_b);
        self.registers.set_compare_true_flag(result);

        if result_reg < self.registers.r.len() {
            self.registers.r[result_reg] = if result { 1 } else { 0 };
        }

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
        let dest_reg = instruction.arg1 as usize;
        if dest_reg < self.registers.r.len() {
            self.registers.r[dest_reg] = new_ctx_id as i64;
        }

        Ok(())
    }

    fn op_ctx_switch(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Switch to a different execution context
        // operand: reg containing ctx_id
        let reg = instruction.arg1 as usize;
        if reg < self.registers.r.len() {
            let ctx_id = self.registers.r[reg] as usize;
            if ctx_id < self.contexts.len() {
                self.current_context = ctx_id;
            }
        }
        Ok(())
    }

    fn op_ctx_clone(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Clone an execution context
        // operand: source context ID
        let src_ctx_id = instruction.arg1 as usize;
        if src_ctx_id < self.contexts.len() {
            if let Some(cloned_ctx) = self.copy_context(src_ctx_id as u64) {
                self.contexts.push(cloned_ctx);
                Ok(())
            } else {
                Err(VmError::InvalidAddress(src_ctx_id as u32))
            }
        } else {
            Err(VmError::InvalidAddress(src_ctx_id as u32))
        }
    }

    fn op_ctx_destroy(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Destroy an execution context
        // operand: context ID to destroy
        let ctx_id = instruction.arg1 as usize;
        if ctx_id < self.contexts.len() && ctx_id != 0 { // Don't destroy the root context
            self.contexts.remove(ctx_id);
            Ok(())
        } else {
            Err(VmError::InvalidAddress(ctx_id as u32))
        }
    }

    // Error Handling Instructions
    fn op_err_set(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Set error register
        // operand: error code
        self.registers.err = instruction.arg1 as u64;
        self.registers.set_error_flag(true);
        Ok(())
    }

    fn op_err_clear(&mut self) {
        // Clear error register
        self.registers.err = 0;
        self.registers.set_error_flag(false);
    }

    fn op_err_check(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Check for error and jump if error exists
        // operand: target address
        if self.registers.err != 0 {
            let target = instruction.arg1 as u32;
            if target < self.program.len() as u32 {
                self.registers.pc = target;
                self.jumped = true;
                return Ok(());
            }
        }
        Ok(())
    }

    // External Interface Instructions
    fn op_ext_call(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        // Call an external function
        // operand: function ID
        let fn_id = instruction.arg1 as u64;
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
        let reg = instruction.arg1 as usize;
        if reg < self.registers.r.len() {
            println!("Output: {}", self.registers.r[reg]);
        }
        Ok(())
    }

    // Introspection hooks for PSI
    pub fn trace_state(&self) -> String {
        format!(
            "PC: {}, FLAG: 0x{:X}, CTX: {}, ERR: {}, R0-R3: [{}, {}, {}, {}]",
            self.registers.pc,
            self.registers.flag,
            self.registers.ctx,
            self.registers.err,
            self.registers.r[0],
            self.registers.r[1],
            self.registers.r[2],
            self.registers.r[3]
        )
    }

    pub fn trace_registers(&self) -> [i64; 16] {
        self.registers.r
    }

    pub fn trace_context(&self) -> Vec<VmContext> {
        self.contexts.clone()
    }

    pub fn trace_graph(&self) -> String {
        // In a real implementation, this would return the execution graph state
        format!("Graph trace not implemented in this version")
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

    #[test]
    fn test_register_model() {
        let mut registers = VmRegisters::new();

        // Test initial state
        assert_eq!(registers.pc, 0);
        assert_eq!(registers.flag, 0);
        assert_eq!(registers.ctx, 0);
        assert_eq!(registers.err, 0);
        assert_eq!(registers.r, [0; 16]);

        // Test flag operations
        registers.set_zero_flag(true);
        assert!(registers.is_zero());

        registers.set_compare_true_flag(true);
        assert!(registers.is_compare_true());

        registers.set_error_flag(true);
        assert!(registers.has_error());

        registers.set_halt_flag(true);
        assert!(registers.is_halt_requested());
    }

    #[test]
    fn test_step_execution() {
        let mut vm = VirtualMachine::new();

        // Create a simple program: load 42 into R0, load 24 into R1, compare them
        let program = vec![
            Instruction::new(0x11, 0, (0u64 << 32) | 42), // LOAD_NUM R0, 42
            Instruction::new(0x11, 0, (1u64 << 32) | 24), // LOAD_NUM R1, 24
            Instruction::new(0x13, 0, (0u64 << 32) | (1u64 << 16) | 2), // COMPARE R0, R1, R2 (Equal?)
        ];

        vm.load_program(program);

        // Execute step by step
        assert!(vm.step().is_ok()); // Load 42 into R0
        assert_eq!(vm.get_register(0), Some(42));

        assert!(vm.step().is_ok()); // Load 24 into R1
        assert_eq!(vm.get_register(1), Some(24));

        assert!(vm.step().is_ok()); // Compare R0 and R1
        assert_eq!(vm.get_register(2), Some(0)); // Should be 0 since 42 != 24

        // Check that we have execution traces
        assert_eq!(vm.execution_trace.len(), 3);
    }

    #[test]
    fn test_introspection_hooks() {
        let mut vm = VirtualMachine::new();

        // Test introspection hooks
        let state_trace = vm.trace_state();
        assert!(state_trace.contains("PC: 0"));

        let registers_trace = vm.trace_registers();
        assert_eq!(registers_trace, [0; 16]);

        let context_trace = vm.trace_context();
        assert_eq!(context_trace.len(), 1); // Should have at least one context
    }
}
