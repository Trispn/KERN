//! VM Instruction Execution Tests
//!
//! This test suite validates the execution of individual VM instructions,
//! ensuring each opcode behaves as expected according to the KERN specification.

use kern_vm::{VirtualMachine, VmError, VmRegisters};
use kern_bytecode::{Instruction, Opcode};

#[test]
fn test_nop_instruction() {
    // Test NOP (No Operation) instruction
    let program = vec![
        Instruction::new(0x00, 0, 0, 0, 0), // NOP
        Instruction::new(0x03, 0, 0, 0, 0), // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    // PC should have advanced by 1 (from 0 to 1, then to 2 where HALT is)
    assert_eq!(vm.registers.pc, 2);
}

#[test]
fn test_load_num_instruction() {
    // Test LOAD_NUM instruction
    let program = vec![
        Instruction::new(0x11, 0, 42, 0, 0), // LOAD_NUM R1, 42 (arg1=dest_reg, arg2=value)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(1), Some(42));
}

#[test]
fn test_load_sym_instruction() {
    // Test LOAD_SYM instruction
    let program = vec![
        Instruction::new(0x10, 0, 0, 2, 0), // LOAD_SYM R2, sym_id=0 (encoded as arg1=0, arg2=0, arg3=dest_reg)
        Instruction::new(0x03, 0, 0, 0, 0), // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    // Symbol ID should be loaded into register R2
    assert_eq!(vm.get_register(2), Some(0)); // Symbol ID 0 as i64
}

#[test]
fn test_move_instruction() {
    // Test MOVE instruction
    let program = vec![
        Instruction::new(0x11, 0, 99, 0, 0), // LOAD_NUM R0, 99
        Instruction::new(0x13, 0, 1, 0, 0),  // MOVE R0 to R1 (src=0, dest=1)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(0), Some(99));
    assert_eq!(vm.get_register(1), Some(99));
}

#[test]
fn test_compare_instruction() {
    // Test COMPARE instruction with different comparison types
    let program = vec![
        Instruction::new(0x11, 0, 10, 0, 0), // LOAD_NUM R0, 10
        Instruction::new(0x11, 1, 5, 0, 0),  // LOAD_NUM R1, 5
        Instruction::new(0x14, 0, 1, 2, 0),  // COMPARE R0, R1, R2 (Equal? - should be 0)
        Instruction::new(0x14, 0, 1, 3, 2),  // COMPARE R0, R1, R3 (Greater? - should be 1)
        Instruction::new(0x14, 1, 0, 4, 3),  // COMPARE R1, R0, R4 (Less? - should be 1)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(2), Some(0)); // 10 == 5 is false
    assert_eq!(vm.get_register(3), Some(1)); // 10 > 5 is true
    assert_eq!(vm.get_register(4), Some(1)); // 5 < 10 is true
}

#[test]
fn test_arithmetic_instructions() {
    // Test ADD, SUB, MUL, DIV, MOD instructions
    let program = vec![
        Instruction::new(0x11, 0, 15, 0, 0), // LOAD_NUM R0, 15
        Instruction::new(0x11, 1, 3, 0, 0),  // LOAD_NUM R1, 3
        Instruction::new(0x20, 0, 1, 2, 0),  // ADD R0, R1 -> R2 (15 + 3 = 18)
        Instruction::new(0x21, 0, 1, 3, 0),  // SUB R0, R1 -> R3 (15 - 3 = 12)
        Instruction::new(0x22, 0, 1, 4, 0),  // MUL R0, R1 -> R4 (15 * 3 = 45)
        Instruction::new(0x23, 0, 1, 5, 0),  // DIV R0, R1 -> R5 (15 / 3 = 5)
        Instruction::new(0x24, 0, 1, 6, 0),  // MOD R0, R1 -> R6 (15 % 3 = 0)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(2), Some(18)); // ADD
    assert_eq!(vm.get_register(3), Some(12)); // SUB
    assert_eq!(vm.get_register(4), Some(45)); // MUL
    assert_eq!(vm.get_register(5), Some(5));  // DIV
    assert_eq!(vm.get_register(6), Some(0));  // MOD
}

#[test]
fn test_logical_instructions() {
    // Test AND, OR, NOT instructions
    let program = vec![
        Instruction::new(0x11, 0, 12, 0, 0), // LOAD_NUM R0, 12 (binary: 1100)
        Instruction::new(0x11, 1, 10, 0, 0), // LOAD_NUM R1, 10 (binary: 1010)
        Instruction::new(0x30, 0, 1, 2, 0),  // AND R0, R1 -> R2 (1100 & 1010 = 1000 = 8)
        Instruction::new(0x31, 0, 1, 3, 0),  // OR R0, R1 -> R3 (1100 | 1010 = 1110 = 14)
        Instruction::new(0x32, 0, 0, 4, 0),  // NOT R0 -> R4 (bitwise NOT of 12)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(2), Some(8));  // AND: 12 & 10 = 8
    assert_eq!(vm.get_register(3), Some(14)); // OR: 12 | 10 = 14
    // NOT result depends on the representation, but should be the bitwise NOT of 12
}

#[test]
fn test_jump_instructions() {
    // Test JMP and JMP_IF instructions
    let program = vec![
        Instruction::new(0x11, 0, 1, 0, 0),  // LOAD_NUM R0, 1 (condition true)
        Instruction::new(0x01, 4, 0, 0, 0),  // JMP to PC=4 (unconditional jump)
        Instruction::new(0x11, 1, 99, 0, 0), // LOAD_NUM R1, 99 (should be skipped)
        Instruction::new(0x11, 2, 88, 0, 0), // LOAD_NUM R2, 88 (should be skipped)
        Instruction::new(0x11, 3, 42, 0, 0), // LOAD_NUM R3, 42 (jump target)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(1), None); // Should not be set (skipped)
    assert_eq!(vm.get_register(2), None); // Should not be set (skipped)
    assert_eq!(vm.get_register(3), Some(42)); // Should be set (jump target)
}

#[test]
fn test_conditional_jump_instruction() {
    // Test JMP_IF instruction with true condition
    let program = vec![
        Instruction::new(0x11, 0, 1, 0, 0),  // LOAD_NUM R0, 1 (condition)
        Instruction::new(0x14, 0, 3, 4, 0),  // COMPARE R0, R3 (R3 is 0 by default), result in R4
        Instruction::new(0x02, 5, 0, 0, 0),  // JMP_IF to PC=5 (should jump since R0 != 0)
        Instruction::new(0x11, 1, 99, 0, 0), // LOAD_NUM R1, 99 (should be skipped)
        Instruction::new(0x11, 2, 88, 0, 0), // LOAD_NUM R2, 88 (should be skipped)
        Instruction::new(0x11, 3, 42, 0, 0), // LOAD_NUM R3, 42 (jump target)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(1), None); // Should not be set (skipped)
    assert_eq!(vm.get_register(2), None); // Should not be set (skipped)
    assert_eq!(vm.get_register(3), Some(42)); // Should be set (jump target)
}

#[test]
fn test_context_instructions() {
    // Test context management instructions
    let program = vec![
        Instruction::new(0x40, 1, 0, 0, 0),  // CTX_CREATE, store new ctx ID in R1
        Instruction::new(0x41, 1, 0, 0, 0),  // CTX_SWITCH, switch to context in R1
        Instruction::new(0x11, 0, 123, 0, 0), // LOAD_NUM R0, 123 in new context
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    // Context operations should execute without errors
    assert!(vm.contexts.len() >= 2); // Should have initial context + new context
}

#[test]
fn test_error_instructions() {
    // Test error handling instructions
    let program = vec![
        Instruction::new(0x50, 42, 0, 0, 0), // ERR_SET with error code 42
        Instruction::new(0x52, 4, 0, 0, 0),  // ERR_CHECK, jump to PC=4 if error exists
        Instruction::new(0x11, 1, 99, 0, 0), // LOAD_NUM R1, 99 (should be skipped)
        Instruction::new(0x11, 2, 88, 0, 0), // LOAD_NUM R2, 88 (should be skipped)
        Instruction::new(0x51, 0, 0, 0, 0),  // ERR_CLEAR
        Instruction::new(0x11, 3, 42, 0, 0), // LOAD_NUM R3, 42 (error handler)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(1), None); // Should not be set (skipped due to error)
    assert_eq!(vm.get_register(2), None); // Should not be set (skipped due to error)
    assert_eq!(vm.get_register(3), Some(42)); // Should be set (error handler executed)
    assert_eq!(vm.registers.err, 0); // Error should be cleared
}

#[test]
fn test_external_call_instruction() {
    // Test external function call instruction
    let program = vec![
        Instruction::new(0x60, 1, 0, 0, 0), // EXT_CALL function ID 1
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    // Should execute without crashing (with proper sandbox validation)
    assert!(result.is_ok() || matches!(result, Err(VmError::SecurityError(_))));
}

#[test]
fn test_output_instruction() {
    // Test output instruction
    let program = vec![
        Instruction::new(0x11, 0, 123, 0, 0), // LOAD_NUM R0, 123
        Instruction::new(0x71, 0, 0, 0, 0),   // OUTPUT R0
        Instruction::new(0x03, 0, 0, 0, 0),   // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    // Should execute without crashing (with proper sandbox validation)
    assert!(result.is_ok());
    assert_eq!(vm.get_register(0), Some(123));
}

#[test]
fn test_halt_instruction() {
    // Test HALT instruction
    let program = vec![
        Instruction::new(0x11, 0, 42, 0, 0), // LOAD_NUM R0, 42
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
        Instruction::new(0x11, 1, 99, 0, 0), // LOAD_NUM R1, 99 (should not execute)
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(0), Some(42));
    assert_eq!(vm.get_register(1), None); // Should not be set
    assert!(vm.registers.is_halt_requested());
}

#[test]
fn test_instruction_step_by_step() {
    // Test executing instructions one by one
    let program = vec![
        Instruction::new(0x11, 0, 10, 0, 0), // LOAD_NUM R0, 10
        Instruction::new(0x11, 1, 20, 0, 0), // LOAD_NUM R1, 20
        Instruction::new(0x20, 0, 1, 2, 0),  // ADD R0, R1 -> R2
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    
    // Execute step by step
    assert!(vm.step().is_ok()); // LOAD_NUM R0, 10
    assert_eq!(vm.get_register(0), Some(10));
    
    assert!(vm.step().is_ok()); // LOAD_NUM R1, 20
    assert_eq!(vm.get_register(1), Some(20));
    
    assert!(vm.step().is_ok()); // ADD R0, R1 -> R2
    assert_eq!(vm.get_register(2), Some(30));
    
    assert!(vm.step().is_ok()); // HALT
    assert!(vm.registers.is_halt_requested());
}

#[test]
fn test_invalid_instruction_handling() {
    // Test handling of invalid opcodes
    let program = vec![
        Instruction::new(0xFF, 0, 0, 0, 0), // Invalid opcode
        Instruction::new(0x03, 0, 0, 0, 0), // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    // Should return an error for invalid opcode
    assert!(result.is_err());
    match result {
        Err(VmError::InvalidOpcode(opcode)) => assert_eq!(opcode, 0xFF),
        _ => panic!("Expected InvalidOpcode error"),
    }
}

#[test]
fn test_register_bounds_checking() {
    // Test that register access is properly bounds-checked
    let program = vec![
        Instruction::new(0x11, 16, 42, 0, 0), // LOAD_NUM R16, 42 (invalid register)
        Instruction::new(0x03, 0, 0, 0, 0),   // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    // Should return an error for invalid register
    assert!(result.is_err());
    match result {
        Err(VmError::InvalidRegister(reg)) => assert_eq!(reg, 16),
        _ => panic!("Expected InvalidRegister error"),
    }
}

#[test]
fn test_program_counter_bounds() {
    // Test that program counter doesn't go out of bounds
    let program = vec![
        Instruction::new(0x01, 100, 0, 0, 0), // JMP to PC=100 (out of bounds)
        Instruction::new(0x03, 0, 0, 0, 0),   // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    // Should return an error for invalid PC
    assert!(result.is_err());
    match result {
        Err(VmError::InvalidPc) => {}, // Expected
        _ => panic!("Expected InvalidPc error"),
    }
}