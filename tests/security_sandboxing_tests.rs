//! Security Sandboxing Tests for KERN VM
//!
//! This test suite validates the security sandboxing mechanisms in the KERN VM,
//! ensuring that external calls and IO operations are properly restricted.

use kern_vm::{VirtualMachine, VMConfig, VmError};
use kern_vm::vm_safety::sandbox::{SandboxPolicy, SandboxEnvironment, SandboxError};
use kern_bytecode::Instruction;

#[test]
fn test_default_sandbox_policy() {
    // Test that the default sandbox policy is restrictive
    let config = VMConfig::new();
    
    // By default, the sandbox should have no allowed functions or IO channels
    assert_eq!(config.sandbox_policy.allowed_functions.len(), 0);
    assert_eq!(config.sandbox_policy.allowed_io_channels.len(), 0);
    assert_eq!(config.sandbox_policy.max_calls_per_function.len(), 0);
}

#[test]
fn test_sandbox_policy_configuration() {
    // Test configuring a sandbox policy
    let mut policy = SandboxPolicy::new();
    
    // Add allowed functions
    policy.allow_function("print");
    policy.allow_function("read_file");
    assert!(policy.is_function_allowed("print"));
    assert!(policy.is_function_allowed("read_file"));
    assert!(!policy.is_function_allowed("system"));
    
    // Add allowed IO channels
    policy.allow_io_channel("stdout");
    policy.allow_io_channel("stdin");
    assert!(policy.is_io_channel_allowed("stdout"));
    assert!(policy.is_io_channel_allowed("stdin"));
    assert!(!policy.is_io_channel_allowed("network"));
    
    // Set call limits
    policy.set_max_calls_for_function("print", 5);
    assert!(policy.would_exceed_call_limit("print", 5));
    assert!(!policy.would_exceed_call_limit("print", 4));
    
    // Verify configuration
    assert_eq!(policy.allowed_functions.len(), 2);
    assert_eq!(policy.allowed_io_channels.len(), 2);
    assert_eq!(policy.max_calls_per_function.get("print"), Some(&5));
}

#[test]
fn test_vm_with_restrictive_sandbox() {
    // Test VM with a restrictive sandbox policy
    let mut config = VMConfig::new();
    config.sandbox_policy = SandboxPolicy::new(); // Default restrictive policy
    
    let mut vm = VirtualMachine::with_config(config);
    
    // Try to execute an external call that should be blocked
    let result = vm.security_context.sandbox.execute_external_call("print");
    assert!(matches!(result, Err(SandboxError::FunctionNotAllowed(_))));
    
    // Try to execute an IO operation that should be blocked
    let result = vm.security_context.sandbox.execute_io_operation("stdout");
    assert!(matches!(result, Err(SandboxError::IoChannelNotAllowed(_))));
}

#[test]
fn test_vm_with_permissive_sandbox() {
    // Test VM with a permissive sandbox policy
    let mut policy = SandboxPolicy::new();
    policy.allow_function("print");
    policy.allow_io_channel("stdout");
    
    let mut config = VMConfig::new();
    config.sandbox_policy = policy;
    
    let mut vm = VirtualMachine::with_config(config);
    
    // External call should be allowed
    let result = vm.security_context.sandbox.execute_external_call("print");
    assert!(result.is_ok());
    
    // IO operation should be allowed
    let result = vm.security_context.sandbox.execute_io_operation("stdout");
    assert!(result.is_ok());
    
    // But other functions should still be blocked
    let result = vm.security_context.sandbox.execute_external_call("system");
    assert!(matches!(result, Err(SandboxError::FunctionNotAllowed(_))));
    
    // And other IO channels should still be blocked
    let result = vm.security_context.sandbox.execute_io_operation("network");
    assert!(matches!(result, Err(SandboxError::IoChannelNotAllowed(_))));
}

#[test]
fn test_sandbox_call_limiting() {
    // Test that sandbox enforces call limits
    let mut policy = SandboxPolicy::new();
    policy.allow_function("print");
    policy.set_max_calls_for_function("print", 3);
    
    let mut config = VMConfig::new();
    config.sandbox_policy = policy;
    
    let mut vm = VirtualMachine::with_config(config);
    
    // First 3 calls should succeed
    for _ in 0..3 {
        let result = vm.security_context.sandbox.execute_external_call("print");
        assert!(result.is_ok());
    }
    
    // 4th call should fail due to limit
    let result = vm.security_context.sandbox.execute_external_call("print");
    assert!(matches!(result, Err(SandboxError::CallLimitExceeded(_))));
    
    // Verify call count
    assert_eq!(vm.security_context.sandbox.get_function_call_count("print"), 4); // Includes the failed call
}

#[test]
fn test_vm_execution_with_sandboxed_external_calls() {
    // Test executing bytecode that makes external calls with sandbox validation
    let mut policy = SandboxPolicy::new();
    policy.allow_function("extern_fn_0"); // This matches what the VM uses for function ID 0
    policy.set_max_calls_for_function("extern_fn_0", 2);
    
    let mut config = VMConfig::new();
    config.sandbox_policy = policy;
    
    let mut vm = VirtualMachine::with_config(config);
    
    // Create a program that calls external functions
    let program = vec![
        Instruction::new(0x60, 0, 0, 0, 0), // EXT_CALL with function ID 0
        Instruction::new(0x60, 0, 0, 0, 0), // EXT_CALL with function ID 0 again
        Instruction::new(0x03, 0, 0, 0, 0), // HALT
    ];
    
    vm.load_program(program);
    let result = vm.execute();
    
    // Should execute successfully since we allowed 2 calls and made 2 calls
    assert!(result.is_ok());
}

#[test]
fn test_vm_execution_with_sandbox_violation() {
    // Test that VM properly handles sandbox violations
    let mut policy = SandboxPolicy::new();
    // Don't allow any external functions
    
    let mut config = VMConfig::new();
    config.sandbox_policy = policy;
    
    let mut vm = VirtualMachine::with_config(config);
    
    // Create a program that tries to call an external function
    let program = vec![
        Instruction::new(0x60, 0, 0, 0, 0), // EXT_CALL with function ID 0 (should be blocked)
        Instruction::new(0x03, 0, 0, 0, 0), // HALT
    ];
    
    vm.load_program(program);
    let result = vm.execute();
    
    // Should fail due to sandbox violation
    assert!(result.is_err());
    match result {
        Err(VmError::SecurityError(_)) => {}, // Expected sandbox violation
        _ => panic!("Expected SecurityError due to sandbox violation"),
    }
}

#[test]
fn test_vm_execution_with_io_sandboxing() {
    // Test executing bytecode that performs IO with sandbox validation
    let mut policy = SandboxPolicy::new();
    policy.allow_io_channel("stdout");
    
    let mut config = VMConfig::new();
    config.sandbox_policy = policy;
    
    let mut vm = VirtualMachine::with_config(config);
    
    // Create a program that outputs to stdout (allowed)
    let program = vec![
        Instruction::new(0x11, 0, 42, 0, 0), // LOAD_NUM R0, 42
        Instruction::new(0x71, 0, 0, 0, 0),  // OUTPUT R0 (to stdout)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];
    
    vm.load_program(program);
    let result = vm.execute();
    
    // Should execute successfully since stdout is allowed
    assert!(result.is_ok());
}

#[test]
fn test_vm_execution_with_io_sandbox_violation() {
    // Test that VM properly handles IO sandbox violations
    let mut policy = SandboxPolicy::new();
    // Don't allow any IO channels
    
    let mut config = VMConfig::new();
    config.sandbox_policy = policy;
    
    let mut vm = VirtualMachine::with_config(config);
    
    // Create a program that tries to output (should be blocked)
    let program = vec![
        Instruction::new(0x11, 0, 42, 0, 0), // LOAD_NUM R0, 42
        Instruction::new(0x71, 0, 0, 0, 0),  // OUTPUT R0 (to stdout - should be blocked)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];
    
    vm.load_program(program);
    let result = vm.execute();
    
    // Should fail due to sandbox violation
    assert!(result.is_err());
    match result {
        Err(VmError::SecurityError(_)) => {}, // Expected sandbox violation
        _ => panic!("Expected SecurityError due to IO sandbox violation"),
    }
}

#[test]
fn test_sandbox_environment_creation() {
    // Test creating and using sandbox environment directly
    let mut policy = SandboxPolicy::new();
    policy.allow_function("test_func");
    policy.allow_io_channel("test_io");
    policy.set_max_calls_for_function("test_func", 1);
    
    let sandbox_env = SandboxEnvironment::new(policy);
    
    // Verify the environment was created with the policy
    assert!(sandbox_env.policy.is_function_allowed("test_func"));
    assert!(sandbox_env.policy.is_io_channel_allowed("test_io"));
    assert!(sandbox_env.is_sandboxed()); // Should be true since it has restrictions
}

#[test]
fn test_sandbox_with_multiple_functions() {
    // Test sandbox with multiple functions and different limits
    let mut policy = SandboxPolicy::new();
    policy.allow_function("func_a");
    policy.allow_function("func_b");
    policy.set_max_calls_for_function("func_a", 3);
    policy.set_max_calls_for_function("func_b", 1);
    
    let mut config = VMConfig::new();
    config.sandbox_policy = policy;
    
    let mut vm = VirtualMachine::with_config(config);
    
    // Call func_a twice (within limit)
    assert!(vm.security_context.sandbox.execute_external_call("func_a").is_ok());
    assert!(vm.security_context.sandbox.execute_external_call("func_a").is_ok());
    
    // Call func_b once (at limit)
    assert!(vm.security_context.sandbox.execute_external_call("func_b").is_ok());
    
    // Call func_a once more (still within limit)
    assert!(vm.security_context.sandbox.execute_external_call("func_a").is_ok());
    
    // Now func_a should be at its limit
    assert!(matches!(
        vm.security_context.sandbox.execute_external_call("func_a"),
        Err(SandboxError::CallLimitExceeded(_))
    ));
    
    // func_b should also be at its limit
    assert!(matches!(
        vm.security_context.sandbox.execute_external_call("func_b"),
        Err(SandboxError::CallLimitExceeded(_))
    ));
    
    // Verify call counts
    assert_eq!(vm.security_context.sandbox.get_function_call_count("func_a"), 3);
    assert_eq!(vm.security_context.sandbox.get_function_call_count("func_b"), 1);
}

#[test]
fn test_security_validation_of_instructions() {
    // Test that security validation works for instructions
    use kern_vm::vm_safety::security::{SecurityValidator, SecurityError};
    
    let mut vm = VirtualMachine::new();
    
    // Valid instruction should pass validation
    let valid_instruction = Instruction::new(0x11, 42, 0, 0, 0); // LOAD_NUM
    let result = vm.security_context.validate_instruction(&valid_instruction);
    assert!(result.is_ok());
    
    // Invalid opcode should fail validation
    let invalid_instruction = Instruction::new(0xFF, 0, 0, 0, 0); // Invalid opcode
    let result = vm.security_context.validate_instruction(&invalid_instruction);
    assert!(result.is_err());
    match result {
        Err(SecurityError::IllegalOpcode(opcode)) => assert_eq!(opcode, 0xFF),
        _ => panic!("Expected IllegalOpcode error"),
    }
}

#[test]
fn test_program_security_validation() {
    // Test that entire programs are validated for security before execution
    let mut policy = SandboxPolicy::new();
    // Don't allow any external functions
    
    let mut config = VMConfig::new();
    config.sandbox_policy = policy;
    
    let mut vm = VirtualMachine::with_config(config);
    
    // Create a program with an external call (should be blocked)
    let program = vec![
        Instruction::new(0x60, 0, 0, 0, 0), // EXT_CALL (should be blocked by security validation)
        Instruction::new(0x03, 0, 0, 0, 0), // HALT
    ];
    
    // Security validation should fail for the entire program
    let result = vm.security_context.validate_instructions(&program);
    assert!(result.is_err());
    
    // Execution should also fail
    vm.load_program(program);
    let result = vm.execute();
    assert!(result.is_err());
    match result {
        Err(VmError::SecurityError(_)) => {}, // Expected security error
        _ => panic!("Expected SecurityError"),
    }
}

#[test]
fn test_sandbox_with_real_vm_execution() {
    // Test a more realistic scenario with proper sandbox configuration
    let mut policy = SandboxPolicy::new();
    policy.allow_function("extern_fn_0");
    policy.allow_function("extern_fn_1");
    policy.allow_io_channel("stdout");
    policy.set_max_calls_for_function("extern_fn_0", 5);
    policy.set_max_calls_for_function("extern_fn_1", 2);
    
    let mut config = VMConfig::new();
    config.sandbox_policy = policy;
    
    let mut vm = VirtualMachine::with_config(config);
    
    // Create a program that uses allowed functions and IO
    let program = vec![
        Instruction::new(0x11, 0, 10, 0, 0), // LOAD_NUM R0, 10
        Instruction::new(0x11, 1, 20, 0, 0), // LOAD_NUM R1, 20
        Instruction::new(0x60, 0, 0, 0, 0),  // EXT_CALL function 0
        Instruction::new(0x60, 1, 0, 0, 0),  // EXT_CALL function 1
        Instruction::new(0x20, 0, 1, 2, 0),  // ADD R0, R1 -> R2
        Instruction::new(0x71, 2, 0, 0, 0),  // OUTPUT R2 (to stdout)
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];
    
    vm.load_program(program);
    let result = vm.execute();
    
    // Should execute successfully with all allowed operations
    assert!(result.is_ok());
    assert_eq!(vm.get_register(2), Some(30)); // 10 + 20 = 30
}

#[test]
fn test_sandbox_prevents_unauthorized_access() {
    // Test that the sandbox prevents unauthorized access to system resources
    let mut policy = SandboxPolicy::new();
    // Only allow safe operations, not system-level functions
    
    let mut config = VMConfig::new();
    config.sandbox_policy = policy;
    
    let mut vm = VirtualMachine::with_config(config);
    
    // Try to call a system-level function that should be blocked
    let result = vm.security_context.sandbox.execute_external_call("system");
    assert!(matches!(result, Err(SandboxError::FunctionNotAllowed(_))));
    
    let result = vm.security_context.sandbox.execute_external_call("exec");
    assert!(matches!(result, Err(SandboxError::FunctionNotAllowed(_))));
    
    let result = vm.security_context.sandbox.execute_external_call("network_connect");
    assert!(matches!(result, Err(SandboxError::FunctionNotAllowed(_))));
    
    // IO operations should also be blocked
    let result = vm.security_context.sandbox.execute_io_operation("network");
    assert!(matches!(result, Err(SandboxError::IoChannelNotAllowed(_))));
    
    let result = vm.security_context.sandbox.execute_io_operation("file_write");
    assert!(matches!(result, Err(SandboxError::IoChannelNotAllowed(_))));
}