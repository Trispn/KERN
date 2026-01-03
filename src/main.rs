use kern_vm::{VirtualMachine, VmRegisters};
use kern_bytecode::Instruction;

fn main() {
    println!("Testing KERN VM implementation...");
    
    // Test register model
    let mut registers = VmRegisters::new();
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
    
    println!("✓ Register model tests passed");
    
    // Test VM creation
    let mut vm = VirtualMachine::new();
    println!("✓ VM creation passed");
    
    // Create a simple program: load 42 into R0, load 24 into R1, compare them
    let program = vec![
        Instruction::new(0x11, 0, 42, 0, 0), // LOAD_NUM R0, 42
        Instruction::new(0x11, 1, 24, 0, 0), // LOAD_NUM R1, 24
        Instruction::new(0x13, 0, 1, 2, 0), // COMPARE R0, R1, R2 (Equal?)
    ];

    vm.load_program(program);
    
    // Execute step by step
    assert!(vm.step().is_ok()); // Load 42 into R0
    assert_eq!(vm.get_register(0), Some(42));
    println!("✓ Step 1 (LOAD_NUM R0, 42) passed");
    
    assert!(vm.step().is_ok()); // Load 24 into R1
    assert_eq!(vm.get_register(1), Some(24));
    println!("✓ Step 2 (LOAD_NUM R1, 24) passed");
    
    assert!(vm.step().is_ok()); // Compare R0 and R1
    assert_eq!(vm.get_register(2), Some(0)); // Should be 0 since 42 != 24
    println!("✓ Step 3 (COMPARE R0, R1, R2) passed");
    
    // Check that we have execution traces
    assert_eq!(vm.execution_trace.len(), 3);
    println!("✓ Execution trace test passed");
    
    // Test introspection hooks
    let state_trace = vm.trace_state();
    assert!(state_trace.contains("PC:"));
    println!("✓ Introspection hooks test passed");
    
    println!("\nAll VM tests passed! ✓");
    println!("KERN VM Core implementation is complete and functional.");
}