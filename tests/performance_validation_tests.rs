//! Performance Validation Tests for KERN VM
//!
//! This test suite validates that the KERN VM meets performance targets,
//! particularly startup time <10ms as specified.

use kern_vm::VirtualMachine;
use std::time::Instant;

#[test]
fn test_vm_startup_performance() {
    // Test that VM startup time is less than 10ms
    let start_time = Instant::now();
    let vm = VirtualMachine::new();
    let startup_time = start_time.elapsed();
    
    println!("VM startup time: {:?}", startup_time);
    
    // Validate that startup time is under 10ms
    assert!(startup_time.as_millis() < 10, 
            "VM startup time ({:?}) exceeded 10ms target", startup_time);
    
    // Verify VM was properly initialized
    assert_eq!(vm.registers.pc, 0);
    assert_eq!(vm.registers.flag, 0);
    assert_eq!(vm.registers.ctx, 0);
    assert_eq!(vm.registers.err, 0);
    assert_eq!(vm.registers.r, [0; 16]);
}

#[test]
fn test_vm_with_config_startup_performance() {
    // Test that VM with custom configuration still meets startup time requirements
    use kern_vm::{VMConfig, vm_safety::memory_limits::MemoryLimits};
    
    let start_time = Instant::now();
    let config = VMConfig::new();
    let vm = VirtualMachine::with_config(config);
    let startup_time = start_time.elapsed();
    
    println!("VM with config startup time: {:?}", startup_time);
    
    // Validate that startup time is under 10ms
    assert!(startup_time.as_millis() < 10, 
            "VM with config startup time ({:?}) exceeded 10ms target", startup_time);
    
    // Verify VM was properly initialized
    assert_eq!(vm.registers.pc, 0);
    assert!(vm.contexts.len() >= 1);
}

#[test]
fn test_multiple_vm_instantiation_performance() {
    // Test performance when creating multiple VM instances
    let start_time = Instant::now();
    
    // Create 10 VM instances
    for _ in 0..10 {
        let vm = VirtualMachine::new();
        // Use the VM briefly to ensure it's fully initialized
        assert_eq!(vm.registers.pc, 0);
    }
    
    let total_time = start_time.elapsed();
    let avg_time_per_vm = total_time / 10;
    
    println!("Average VM startup time over 10 instances: {:?}", avg_time_per_vm);
    println!("Total time for 10 VMs: {:?}", total_time);
    
    // Validate that average startup time is under 10ms
    assert!(avg_time_per_vm.as_millis() < 10, 
            "Average VM startup time ({:?}) exceeded 10ms target", avg_time_per_vm);
}

#[test]
fn test_vm_execution_performance_simple_program() {
    // Test execution performance of a simple program
    use kern_bytecode::Instruction;
    
    let mut vm = VirtualMachine::new();
    
    // Create a simple program
    let program = vec![
        Instruction::new(0x11, 0, 42, 0, 0), // LOAD_NUM R0, 42
        Instruction::new(0x11, 1, 24, 0, 0), // LOAD_NUM R1, 24
        Instruction::new(0x20, 0, 1, 2, 0),  // ADD R0, R1 -> R2
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];
    
    vm.load_program(program);
    
    let start_time = Instant::now();
    let result = vm.execute();
    let execution_time = start_time.elapsed();
    
    assert!(result.is_ok());
    assert_eq!(vm.get_register(2), Some(66)); // 42 + 24 = 66
    
    println!("Simple program execution time: {:?}", execution_time);
    
    // Simple programs should execute very quickly
    assert!(execution_time.as_millis() < 10, 
            "Simple program execution time ({:?}) exceeded 10ms target", execution_time);
}

#[test]
fn test_vm_execution_performance_medium_program() {
    // Test execution performance of a medium-sized program
    use kern_bytecode::Instruction;
    
    let mut vm = VirtualMachine::new();
    
    // Create a program with 100 instructions
    let mut program = Vec::new();
    for i in 0..100 {
        if i % 4 == 0 {
            program.push(Instruction::new(0x11, 0, (i % 50) as u16, 0, 0)); // LOAD_NUM R0
        } else if i % 4 == 1 {
            program.push(Instruction::new(0x11, 1, ((i + 5) % 50) as u16, 0, 0)); // LOAD_NUM R1
        } else if i % 4 == 2 {
            program.push(Instruction::new(0x20, 0, 1, 2, 0)); // ADD R0, R1 -> R2
        } else {
            program.push(Instruction::new(0x13, 2, 0, 0, 0)); // MOVE R2 to R0
        }
    }
    program.push(Instruction::new(0x03, 0, 0, 0, 0)); // HALT
    
    vm.load_program(program);
    
    let start_time = Instant::now();
    let result = vm.execute();
    let execution_time = start_time.elapsed();
    
    assert!(result.is_ok());
    
    println!("Medium program (100 instructions) execution time: {:?}", execution_time);
    
    // Medium programs should still execute quickly (under 100ms)
    assert!(execution_time.as_millis() < 100, 
            "Medium program execution time ({:?}) exceeded 100ms target", execution_time);
}

#[test]
fn test_vm_execution_performance_large_program() {
    // Test execution performance of a larger program
    use kern_bytecode::Instruction;
    
    let mut vm = VirtualMachine::new();
    
    // Create a program with 1000 instructions
    let mut program = Vec::new();
    for i in 0..1000 {
        if i % 5 == 0 {
            program.push(Instruction::new(0x11, 0, (i % 100) as u16, 0, 0)); // LOAD_NUM R0
        } else if i % 5 == 1 {
            program.push(Instruction::new(0x11, 1, ((i + 10) % 100) as u16, 0, 0)); // LOAD_NUM R1
        } else if i % 5 == 2 {
            program.push(Instruction::new(0x20, 0, 1, 2, 0)); // ADD R0, R1 -> R2
        } else if i % 5 == 3 {
            program.push(Instruction::new(0x13, 2, 0, 0, 0)); // MOVE R2 to R0
        } else {
            program.push(Instruction::new(0x14, 0, 3, 4, 0)); // COMPARE R0, R3 -> R4
        }
    }
    program.push(Instruction::new(0x03, 0, 0, 0, 0)); // HALT
    
    vm.load_program(program);
    
    let start_time = Instant::now();
    let result = vm.execute();
    let execution_time = start_time.elapsed();
    
    assert!(result.is_ok());
    
    println!("Large program (1000 instructions) execution time: {:?}", execution_time);
    
    // Large programs should execute efficiently (under 1 second for 1000 instructions)
    assert!(execution_time.as_millis() < 1000, 
            "Large program execution time ({:?}) exceeded 1000ms target", execution_time);
}

#[test]
fn test_vm_memory_allocation_performance() {
    // Test that memory allocation doesn't significantly impact performance
    use kern_vm::{VMConfig, vm_safety::memory_limits::MemoryLimits};
    
    // Create VM with specific memory limits
    let mut config = VMConfig::new();
    config.memory_limits = MemoryLimits::new(1024 * 100, 1024 * 50, 1024 * 200, 1024 * 10, 1024 * 10); // 100KB heap, etc.
    
    let start_time = Instant::now();
    let vm = VirtualMachine::with_config(config);
    let startup_time = start_time.elapsed();
    
    println!("VM with memory limits startup time: {:?}", startup_time);
    
    // Validate that startup time is still under 10ms even with custom memory limits
    assert!(startup_time.as_millis() < 10, 
            "VM with memory limits startup time ({:?}) exceeded 10ms target", startup_time);
    
    // Verify memory limits were applied
    assert_eq!(vm.get_memory_limits().max_heap_bytes, 1024 * 100);
}

#[test]
fn test_vm_step_execution_performance() {
    // Test performance of step-by-step execution
    use kern_bytecode::Instruction;
    
    let mut vm = VirtualMachine::new();
    
    // Create a simple program
    let program = vec![
        Instruction::new(0x11, 0, 42, 0, 0), // LOAD_NUM R0, 42
        Instruction::new(0x11, 1, 24, 0, 0), // LOAD_NUM R1, 24
        Instruction::new(0x20, 0, 1, 2, 0),  // ADD R0, R1 -> R2
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];
    
    vm.load_program(program);
    
    let start_time = Instant::now();
    
    // Execute step by step
    for _ in 0..4 {
        assert!(vm.step().is_ok());
    }
    
    let step_execution_time = start_time.elapsed();
    
    println!("Step execution time for 4 instructions: {:?}", step_execution_time);
    
    // Step execution should be efficient
    assert!(step_execution_time.as_millis() < 10, 
            "Step execution time ({:?}) exceeded 10ms target", step_execution_time);
}

#[test]
fn test_vm_with_performance_monitoring_startup() {
    // Test VM startup with performance monitoring enabled
    use kern_vm::VMConfig;
    
    let mut config = VMConfig::new();
    config.perf_flags = true; // Enable performance monitoring
    
    let start_time = Instant::now();
    let vm = VirtualMachine::with_config(config);
    let startup_time = start_time.elapsed();
    
    println!("VM with perf monitoring startup time: {:?}", startup_time);
    
    // Even with performance monitoring, startup should be under 10ms
    assert!(startup_time.as_millis() < 10, 
            "VM with perf monitoring startup time ({:?}) exceeded 10ms target", startup_time);
    
    // Verify performance monitoring is enabled
    assert!(vm.config.perf_flags);
}

#[test]
fn test_concurrent_vm_performance() {
    // Test performance when multiple VMs exist simultaneously
    use std::thread;
    
    let start_time = Instant::now();
    
    // Create multiple threads each with a VM
    let handles: Vec<_> = (0..4).map(|_| {
        thread::spawn(|| {
            let vm = VirtualMachine::new();
            // Execute a simple program
            use kern_bytecode::Instruction;
            let program = vec![
                Instruction::new(0x11, 0, 10, 0, 0),
                Instruction::new(0x11, 1, 20, 0, 0),
                Instruction::new(0x20, 0, 1, 2, 0),
                Instruction::new(0x03, 0, 0, 0, 0),
            ];
            let mut vm = vm;
            vm.load_program(program);
            vm.execute().unwrap();
            vm.get_register(2).unwrap() // Return the result
        })
    }).collect();
    
    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    let total_time = start_time.elapsed();
    
    // Verify all threads completed successfully with correct results
    for result in &results {
        assert_eq!(*result, 30); // 10 + 20 = 30
    }
    
    println!("Concurrent VM execution time for 4 VMs: {:?}", total_time);
    
    // Concurrent execution should still be reasonable
    assert!(total_time.as_millis() < 50, 
            "Concurrent VM execution time ({:?}) exceeded 50ms target", total_time);
}