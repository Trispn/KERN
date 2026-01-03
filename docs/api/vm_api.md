# KERN Virtual Machine API Documentation

## Overview

The KERN Virtual Machine (VM) API provides programmatic access to the KERN execution environment, including bytecode execution, state management, and safety controls.

## Core VM Components

### VirtualMachine

The main VM interface:

```rust
pub struct VirtualMachine {
    pub registers: VmRegisters,
    pub contexts: Vec<VmContext>,
    pub current_context: usize,
    pub memory: MemoryRegions,
    pub program: Vec<Instruction>,
    pub running: bool,
    pub max_steps: u32,
    pub step_count: u32,
    pub external_functions: HashMap<String, fn(&mut VirtualMachine) -> Result<(), String>>,
    pub execution_trace: Vec<ExecutionTraceEntry>,
}

impl VirtualMachine {
    pub fn new() -> Self { ... }
    pub fn with_config(config: VMConfig) -> Self { ... }
    pub fn load_program(&mut self, program: Vec<Instruction>) { ... }
    pub fn execute(&mut self) -> Result<(), VmError> { ... }
    pub fn step(&mut self) -> Result<(), VmError> { ... }
    pub fn reset(&mut self) { ... }
    pub fn get_register(&self, reg: usize) -> Option<i64> { ... }
    pub fn set_register(&mut self, reg: usize, value: i64) -> Result<(), VmError> { ... }
}
```

### VM Configuration

Configure VM behavior with safety limits:

```rust
pub struct VMConfig {
    pub memory_limits: MemoryLimits,
    pub execution_limits: ExecutionLimits,
    pub sandbox_policy: SandboxPolicy,
    pub perf_flags: bool,
}

impl VMConfig {
    pub fn new() -> Self { ... }
    pub fn default() -> Self { ... }
}
```

## Execution Methods

### `execute() -> Result<(), VmError>`

Executes the loaded program until completion or error.

**Returns:**
- `Ok(())`: Program executed successfully
- `Err(VmError)`: Execution error occurred

**Example:**
```rust
use kern_vm::VirtualMachine;

let mut vm = VirtualMachine::new();
// Load program into VM
vm.load_program(program);

match vm.execute() {
    Ok(()) => println!("Program executed successfully"),
    Err(e) => eprintln!("Execution failed: {:?}", e),
}
```

### `step() -> Result<(), VmError>`

Executes exactly one instruction.

**Returns:**
- `Ok(())`: Single instruction executed successfully
- `Err(VmError)`: Execution error occurred

**Example:**
```rust
use kern_vm::VirtualMachine;

let mut vm = VirtualMachine::new();
vm.load_program(program);

// Execute step by step
for _ in 0..5 {
    match vm.step() {
        Ok(()) => println!("Step executed, PC: {}", vm.registers.pc),
        Err(e) => {
            eprintln!("Step failed: {:?}", e);
            break;
        }
    }
}
```

### `load_program(program: Vec<Instruction>)`

Loads a program into the VM for execution.

**Parameters:**
- `program`: Vector of instructions to load

**Example:**
```rust
use kern_vm::{VirtualMachine, Instruction};

let mut vm = VirtualMachine::new();
let program = vec![
    Instruction::new(0x11, 42, 0, 0, 0), // LOAD_NUM R0, 42
    Instruction::new(0x03, 0, 0, 0, 0),  // HALT
];

vm.load_program(program);
```

## Register Management

### `get_register(reg: usize) -> Option<i64>`

Gets the value of a register.

**Parameters:**
- `reg`: Register index (0-15)

**Returns:**
- `Some(i64)`: Register value
- `None`: Invalid register index

### `set_register(reg: usize, value: i64) -> Result<(), VmError>`

Sets the value of a register.

**Parameters:**
- `reg`: Register index (0-15)
- `value`: Value to set

**Returns:**
- `Ok(())`: Register set successfully
- `Err(VmError)`: Invalid register index

**Example:**
```rust
use kern_vm::VirtualMachine;

let mut vm = VirtualMachine::new();

// Set register R0 to 100
vm.set_register(0, 100).unwrap();

// Get register R0 value
let value = vm.get_register(0).unwrap();
assert_eq!(value, 100);
```

## Memory Management

### Memory Regions

The VM has several memory regions:

```rust
pub struct MemoryRegions {
    pub code: Vec<u8>,      // Read-only bytecode
    pub constants: Vec<u8>, // Read-only constants
    pub stack: Vec<u8>,     // Operand + call stack
    pub heap: Vec<u8>,      // Graph nodes, symbols, contexts
    pub meta: Vec<u8>,      // PSI introspection & metadata
}
```

### Memory Limits

Configure memory limits for safety:

```rust
pub struct MemoryLimits {
    pub max_code_bytes: usize,
    pub max_constants_bytes: usize,
    pub max_stack_bytes: usize,
    pub max_heap_bytes: usize,
    pub max_meta_bytes: usize,
}

impl MemoryLimits {
    pub fn new(code: usize, constants: usize, stack: usize, heap: usize, meta: usize) -> Self {
        MemoryLimits {
            max_code_bytes: code,
            max_constants_bytes: constants,
            max_stack_bytes: stack,
            max_heap_bytes: heap,
            max_meta_bytes: meta,
        }
    }
}
```

## Execution Limits

### Execution Limits Configuration

```rust
pub struct ExecutionLimits {
    pub max_steps: u64,
    pub max_rule_invocations: u64,
    pub max_loop_iterations: u64,
}

impl ExecutionLimits {
    pub fn new(max_steps: u64, max_rules: u64, max_loops: u64) -> Self {
        ExecutionLimits {
            max_steps,
            max_rule_invocations: max_rules,
            max_loop_iterations: max_loops,
        }
    }
}
```

## Error Handling

### VmError

The VM returns detailed error information:

```rust
pub enum VmError {
    InvalidOpcode(u8),
    InvalidRegister(u16),
    InvalidAddress(u32),
    ExecutionLimitExceeded,
    DivisionByZero,
    StackOverflow,
    StackUnderflow,
    InvalidPc,
    InvalidInstruction,
    UndefinedSymbol,
    MemoryLimitExceeded,
    SecurityError(SecurityError),
    SandboxViolation,
    LimitError(LimitError),
}
```

## Safety and Security

### Sandbox Policy

Configure sandbox restrictions:

```rust
pub struct SandboxPolicy {
    pub allowed_functions: Vec<String>,
    pub allowed_io_channels: Vec<String>,
    pub max_calls_per_function: HashMap<String, u64>,
}

impl SandboxPolicy {
    pub fn new() -> Self { ... }
    pub fn allow_function(&mut self, function_name: &str) { ... }
    pub fn allow_io_channel(&mut self, channel_name: &str) { ... }
    pub fn set_max_calls_for_function(&mut self, function_name: &str, max_calls: u64) { ... }
}
```

### Security Validation

The VM validates all operations:

```rust
pub struct SecurityValidator {
    pub allowed_opcodes: HashSet<u8>,
    pub instruction_validator: Box<dyn InstructionValidator>,
}

pub trait InstructionValidator {
    fn validate_instruction(&self, instruction: &Instruction) -> Result<(), SecurityError>;
    fn validate_program(&self, program: &[Instruction]) -> Result<(), SecurityError>;
}
```

## External Function Integration

### Adding External Functions

```rust
impl VirtualMachine {
    pub fn add_external_function(
        &mut self,
        name: &str,
        func: fn(&mut VirtualMachine) -> Result<(), String>,
    ) {
        self.external_functions.insert(name.to_string(), func);
    }
}

// Example external function
fn print_external(vm: &mut VirtualMachine) -> Result<(), String> {
    let value = vm.get_register(0).unwrap_or(0);
    println!("External print: {}", value);
    Ok(())
}

// Usage
let mut vm = VirtualMachine::new();
vm.add_external_function("print", print_external);
```

## Performance Monitoring

### Performance Metrics

The VM provides performance metrics:

```rust
pub struct PerformanceMetrics {
    pub instruction_count: u64,
    pub execution_time: Duration,
    pub memory_usage: MemoryUsage,
    pub rule_invocations: u64,
    pub context_switches: u64,
}

impl VirtualMachine {
    pub fn get_performance_metrics(&self) -> PerformanceMetrics { ... }
    pub fn generate_performance_report(&self) -> String { ... }
    pub fn reset_performance_metrics(&mut self) { ... }
}
```

### Execution Tracing

The VM maintains execution traces for debugging:

```rust
pub struct ExecutionTraceEntry {
    pub pc_before: u32,
    pub opcode: u8,
    pub operands: u64,
    pub register_diff: [i64; 16],
    pub memory_diff: Vec<u8>,
}

impl VirtualMachine {
    pub fn get_execution_trace(&self) -> &[ExecutionTraceEntry] { ... }
    pub fn clear_execution_trace(&mut self) { ... }
}
```

## Context Management

### VmContext

Execution contexts manage state isolation:

```rust
pub struct VmContext {
    pub id: u64,
    pub registers: VmRegisters,
    pub memory: Vec<u8>,
    pub variables: HashMap<String, i64>,
}

impl VirtualMachine {
    pub fn create_context(&mut self) -> u64 { ... }
    pub fn switch_context(&mut self, ctx_id: u64) -> Result<(), VmError> { ... }
    pub fn destroy_context(&mut self, ctx_id: u64) -> Result<(), VmError> { ... }
}
```

## Configuration Examples

### Basic VM Setup

```rust
use kern_vm::{VirtualMachine, VMConfig};

let vm = VirtualMachine::new();
// Uses default configuration
```

### Custom VM Configuration

```rust
use kern_vm::{VirtualMachine, VMConfig, vm_safety::memory_limits::MemoryLimits};

let mut config = VMConfig::new();
config.memory_limits = MemoryLimits::new(
    1024 * 100,   // 100KB code
    1024 * 50,    // 50KB constants
    1024 * 200,   // 200KB stack
    1024 * 1000,  // 1MB heap
    1024 * 10,    // 10KB meta
);

let vm = VirtualMachine::with_config(config);
```

### Secure VM Configuration

```rust
use kern_vm::{VirtualMachine, VMConfig, vm_safety::sandbox::SandboxPolicy};

let mut config = VMConfig::new();

// Configure sandbox policy
let mut policy = SandboxPolicy::new();
policy.allow_function("safe_function");
policy.allow_io_channel("stdout");
policy.set_max_calls_for_function("safe_function", 10);

config.sandbox_policy = policy;

let vm = VirtualMachine::with_config(config);
```

## Advanced Features

### PSI Introspection Hooks

The VM provides hooks for PSI analysis:

```rust
impl VirtualMachine {
    pub fn trace_state(&self) -> String { ... }
    pub fn trace_registers(&self) -> [i64; 16] { ... }
    pub fn trace_context(&self) -> Vec<VmContext> { ... }
    pub fn trace_graph(&self) -> String { ... }
}
```

### Step Limiting

Control execution with step limits:

```rust
use kern_vm::{VirtualMachine, VMConfig, vm_safety::step_limits::ExecutionLimits};

let mut config = VMConfig::new();
config.execution_limits = ExecutionLimits::new(1000, 100, 50); // 1000 steps, 100 rules, 50 loops

let vm = VirtualMachine::with_config(config);
```

## Integration Examples

### Basic Execution

```rust
use kern_vm::{VirtualMachine, Instruction};

fn execute_simple_program() -> Result<(), String> {
    let mut vm = VirtualMachine::new();
    
    // Create a simple program: load 42 into R0, halt
    let program = vec![
        Instruction::new(0x11, 0, 42, 0, 0), // LOAD_NUM R0, 42
        Instruction::new(0x03, 0, 0, 0, 0),  // HALT
    ];
    
    vm.load_program(program);
    
    vm.execute()
        .map_err(|e| format!("Execution failed: {:?}", e))?;
    
    // Verify result
    assert_eq!(vm.get_register(0), Some(42));
    
    Ok(())
}
```

### Error Handling

```rust
use kern_vm::{VirtualMachine, VmError};

fn execute_with_error_handling() {
    let mut vm = VirtualMachine::new();
    
    // Load and execute with error handling
    match vm.execute() {
        Ok(()) => println!("Execution completed successfully"),
        Err(VmError::ExecutionLimitExceeded) => {
            eprintln!("Execution exceeded limits");
        }
        Err(VmError::MemoryLimitExceeded) => {
            eprintln!("Memory limit exceeded");
        }
        Err(VmError::InvalidOpcode(opcode)) => {
            eprintln!("Invalid opcode: 0x{:02X}", opcode);
        }
        Err(e) => {
            eprintln!("Execution error: {:?}", e);
        }
    }
}
```

## Testing and Validation

### Unit Testing

The VM API supports comprehensive testing:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_execution() {
        let mut vm = VirtualMachine::new();
        
        let program = vec![
            Instruction::new(0x11, 0, 42, 0, 0), // LOAD_NUM R0, 42
            Instruction::new(0x03, 0, 0, 0, 0),  // HALT
        ];
        
        vm.load_program(program);
        assert!(vm.execute().is_ok());
        assert_eq!(vm.get_register(0), Some(42));
    }
    
    #[test]
    fn test_step_execution() {
        let mut vm = VirtualMachine::new();
        
        let program = vec![
            Instruction::new(0x11, 0, 42, 0, 0), // LOAD_NUM R0, 42
            Instruction::new(0x03, 0, 0, 0, 0),  // HALT
        ];
        
        vm.load_program(program);
        
        // Execute first step
        assert!(vm.step().is_ok());
        assert_eq!(vm.get_register(0), Some(42));
        
        // Execute second step (HALT)
        assert!(vm.step().is_ok());
        assert!(vm.registers.is_halt_requested());
    }
}
```

## Security Considerations

### Sandboxing

All VM execution is sandboxed by default:

- Memory access is validated
- External function calls are restricted
- IO operations are controlled
- Execution limits prevent resource exhaustion

### Input Validation

The VM validates all inputs:

- Bytecode is validated before execution
- Register access is bounds-checked
- Memory access is validated
- External calls are policy-checked

## Performance Considerations

### Execution Speed

The VM is optimized for performance:

- Direct-threaded interpreter
- Efficient instruction dispatch
- Register-based operations
- Minimal memory allocations during execution

### Memory Usage

The VM manages memory efficiently:

- Fixed-size memory regions
- Pre-allocated execution contexts
- Efficient instruction representation
- Minimal runtime allocations

## Troubleshooting

### Common Issues

1. **Execution Limits**: Check step, rule, and loop limits
2. **Memory Limits**: Verify memory allocation limits
3. **Sandbox Violations**: Check allowed functions and IO channels
4. **Invalid Instructions**: Validate bytecode format

### Debugging Tips

- Use execution tracing to understand program flow
- Monitor performance metrics for bottlenecks
- Check security policies for external function access
- Validate register and memory access patterns