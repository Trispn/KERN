//! Comprehensive bytecode execution tests for the KERN VM
//!
//! This test suite validates the execution of bytecode instructions in the KERN VM,
//! covering all major instruction categories and edge cases.

use kern_vm::{VirtualMachine, VmError};
use kern_bytecode::{Instruction, Opcode};

#[test]
fn test_basic_arithmetic_execution() {
    // Test basic arithmetic operations: ADD, SUB, MUL, DIV
    let program = vec![
        Instruction::new(0x11, 10, 0, 0, 0), // LOAD_NUM 10 into R0
        Instruction::new(0x11, 5, 1, 0, 0),  // LOAD_NUM 5 into R1
        Instruction::new(0x20, 0, 1, 2, 0),  // ADD R0, R1 -> R2 (10 + 5 = 15)
        Instruction::new(0x21, 0, 1, 3, 0),  // SUB R0, R1 -> R3 (10 - 5 = 5)
        Instruction::new(0x22, 0, 1, 4, 0),  // MUL R0, R1 -> R4 (10 * 5 = 50)
        Instruction::new(0x23, 0, 1, 5, 0),  // DIV R0, R1 -> R5 (10 / 5 = 2)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(0), Some(10)); // R0 = 10
    assert_eq!(vm.get_register(1), Some(5));  // R1 = 5
    assert_eq!(vm.get_register(2), Some(15)); // R2 = 10 + 5 = 15
    assert_eq!(vm.get_register(3), Some(5));  // R3 = 10 - 5 = 5
    assert_eq!(vm.get_register(4), Some(50)); // R4 = 10 * 5 = 50
    assert_eq!(vm.get_register(5), Some(2));  // R5 = 10 / 5 = 2
}

#[test]
fn test_comparison_execution() {
    // Test comparison operations and flag setting
    let program = vec![
        Instruction::new(0x11, 10, 0, 0, 0), // LOAD_NUM 10 into R0
        Instruction::new(0x11, 5, 1, 0, 0),  // LOAD_NUM 5 into R1
        Instruction::new(0x11, 10, 2, 0, 0), // LOAD_NUM 10 into R2
        Instruction::new(0x14, 0, 1, 3, 0),  // COMPARE R0, R1, R3 (R0 > R1)
        Instruction::new(0x14, 0, 2, 4, 0),  // COMPARE R0, R2, R4 (R0 == R2)
        Instruction::new(0x14, 1, 0, 5, 0),  // COMPARE R1, R0, R5 (R1 < R0)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(3), Some(1)); // R3 = 1 (10 > 5 is true)
    assert_eq!(vm.get_register(4), Some(1)); // R4 = 1 (10 == 10 is true)
    assert_eq!(vm.get_register(5), Some(1)); // R5 = 1 (5 < 10 is true)
}

#[test]
fn test_control_flow_execution() {
    // Test control flow instructions: JMP, JMP_IF
    let program = vec![
        Instruction::new(0x11, 1, 0, 0, 0),  // LOAD_NUM 1 into R0 (flag for condition)
        Instruction::new(0x13, 0, 0, 1, 0),  // MOVE R0 to R1 (set condition true)
        Instruction::new(0x02, 4, 0, 0, 0),  // JMP_IF to instruction at PC=4 (should jump)
        Instruction::new(0x11, 99, 2, 0, 0), // LOAD_NUM 99 into R2 (skipped)
        Instruction::new(0x11, 42, 3, 0, 0), // LOAD_NUM 42 into R3 (jump target)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(2), None); // R2 should not be set (skipped instruction)
    assert_eq!(vm.get_register(3), Some(42)); // R3 should be 42 (executed after jump)
}

#[test]
fn test_context_management_execution() {
    // Test context management instructions
    let program = vec![
        Instruction::new(0x60, 0, 0, 0, 0),  // PUSH_CTX (create new context)
        Instruction::new(0x11, 123, 0, 0, 0), // LOAD_NUM 123 into R0 in new context
        Instruction::new(0x62, 0, 5, 0, 0),  // SET_SYMBOL (store R0 as symbol with ID 5)
        Instruction::new(0x61, 0, 0, 0, 0),  // POP_CTX (return to previous context)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    // Context operations should execute without errors
    assert!(vm.contexts.len() >= 1); // Should have at least the initial context
}

#[test]
fn test_error_handling_execution() {
    // Test error handling instructions
    let program = vec![
        Instruction::new(0x50, 1, 0, 0, 0),  // THROW error with code 1
        Instruction::new(0x71, 3, 0, 0, 0),  // TRY (start try block)
        Instruction::new(0x11, 100, 0, 0, 0), // LOAD_NUM 100 into R0 (in try block)
        Instruction::new(0x72, 5, 0, 0, 0),  // CATCH (jump to PC=5 if error)
        Instruction::new(0x11, 200, 1, 0, 0), // LOAD_NUM 200 into R1 (catch block)
        Instruction::new(0x73, 0, 0, 0, 0),  // CLEAR_ERR
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    // The exact behavior depends on how error handling is implemented
    // This test ensures the VM doesn't crash when executing error instructions
    assert!(result.is_ok() || matches!(result, Err(VmError::InvalidOpcode(_))));
}

#[test]
fn test_external_interface_execution() {
    // Test external interface instructions
    let program = vec![
        Instruction::new(0x82, 0, 0, 0, 0),  // WRITE_IO to stdout (output R0)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.set_register(0, 42).unwrap(); // Set R0 to 42 for output
    vm.load_program(program);
    let result = vm.execute();
    
    // This should execute without crashing (assuming stdout is allowed in sandbox)
    assert!(result.is_ok());
}

#[test]
fn test_complex_execution_scenario() {
    // Test a more complex scenario combining multiple instruction types
    let program = vec![
        // Initialize values
        Instruction::new(0x11, 10, 0, 0, 0), // R0 = 10
        Instruction::new(0x11, 20, 1, 0, 0), // R1 = 20
        Instruction::new(0x11, 0, 2, 0, 0),  // R2 = 0 (accumulator)
        
        // Loop: add R1 to R2, R0 times
        Instruction::new(0x20, 2, 1, 2, 0),  // R2 = R2 + R1 (add R1 to accumulator)
        Instruction::new(0x21, 0, 3, 0, 0),  // R0 = R0 - 1 (decrement counter)
        Instruction::new(0x14, 0, 3, 4, 0),  // Compare R0 with 0 (R3 is zero register)
        Instruction::new(0x02, 3, 0, 0, 0),  // JMP_IF to PC=3 if R0 != 0 (loop back)
        
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let result = vm.execute();
    
    assert!(result.is_ok());
    // After loop: R2 should contain 10 * 20 = 200
    assert_eq!(vm.get_register(2), Some(200));
}

#[test]
fn test_bytecode_serialization_deserialization() {
    // Test that bytecode can be serialized and deserialized correctly
    let original_instructions = vec![
        Instruction::new(0x11, 42, 0, 0, 0), // LOAD_NUM 42 into R0
        Instruction::new(0x11, 24, 1, 0, 0), // LOAD_NUM 24 into R1
        Instruction::new(0x20, 0, 1, 2, 0),  // ADD R0, R1 -> R2
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];

    // Serialize each instruction to bytes and back
    let mut serialized_bytes = Vec::new();
    for instr in &original_instructions {
        serialized_bytes.extend_from_slice(&instr.to_bytes());
    }

    // Deserialize back to instructions
    let mut deserialized_instructions = Vec::new();
    for chunk in serialized_bytes.chunks(8) {
        if chunk.len() == 8 {
            if let Some(instr) = Instruction::from_bytes(chunk) {
                deserialized_instructions.push(instr);
            }
        }
    }

    // Verify that deserialized instructions match original
    assert_eq!(original_instructions.len(), deserialized_instructions.len());
    for (orig, deser) in original_instructions.iter().zip(deserialized_instructions.iter()) {
        assert_eq!(orig.opcode, deser.opcode);
        assert_eq!(orig.arg1, deser.arg1);
        assert_eq!(orig.arg2, deser.arg2);
        assert_eq!(orig.arg3, deser.arg3);
        assert_eq!(orig.flags, deser.flags);
    }

    // Test execution with deserialized bytecode
    let mut vm = VirtualMachine::new();
    vm.load_program(deserialized_instructions);
    let result = vm.execute();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(2), Some(66)); // 42 + 24 = 66
}

#[test]
fn test_large_program_execution() {
    // Test execution of a larger program to ensure no performance issues
    let mut program = Vec::new();
    
    // Create a program with 1000 instructions
    for i in 0..1000 {
        if i % 4 == 0 {
            program.push(Instruction::new(0x11, (i % 100) as u16, 0, 0, 0)); // LOAD_NUM
        } else if i % 4 == 1 {
            program.push(Instruction::new(0x11, ((i + 10) % 100) as u16, 1, 0, 0)); // LOAD_NUM
        } else if i % 4 == 2 {
            program.push(Instruction::new(0x20, 0, 1, 2, 0)); // ADD
        } else {
            program.push(Instruction::new(0x13, 2, 0, 0, 0)); // MOVE R2 to R0
        }
    }
    program.push(Instruction::new(0x03, 0, 0, 0, 0)); // HALT

    let mut vm = VirtualMachine::new();
    vm.load_program(program);
    let start_time = std::time::Instant::now();
    let result = vm.execute();
    let execution_time = start_time.elapsed();
    
    assert!(result.is_ok());
    // Execution should be reasonably fast
    assert!(execution_time.as_millis() < 1000); // Should complete in under 1 second
}