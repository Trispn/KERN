# KERN Usage Guide

## Writing KERN Programs

This guide walks you through the complete process of writing, compiling, and running KERN programs. We'll cover everything from basic syntax to advanced usage patterns.

### Basic Program Structure

A KERN program consists of four main structural constructs: entities, rules, flows, and constraints. Let's examine each component:

#### Entities - Data Definition

Entities define the data structures that your rules will operate on:

```kern
entity Farmer {
    id
    name
    location
    produce
    certification
}

entity Crop {
    id
    type
    season
    quality
}
```

Entities are pure data containers with no methods or behavior. They serve as the foundation for your business logic.

#### Rules - Logic Definition

Rules define the core logic of your system using an explicit if/then structure:

```kern
rule ValidateFarmer:
    if farmer.id != 0 and farmer.location != ""
    then mark_valid(farmer)

rule ApproveCertified:
    if farmer.certification == "organic" and farmer.id > 0
    then approve_farmer(farmer)
```

Rules are declarative and side-effect explicit. Each rule has:
- A unique name for identification
- A condition (guard) that evaluates to true/false
- An action that executes when the condition is met

#### Flows - Execution Orchestration

Flows define the explicit execution order of your rules:

```kern
flow ProcessFarmers {
    ValidateFarmer
    ApproveCertified
    GenerateReport
}
```

Flows provide deterministic execution order while maintaining the declarative nature of individual rules.

#### Constraints - Validation Logic

Constraints provide validation that must be satisfied:

```kern
constraint ValidId: farmer.id > 0
constraint ValidLocation: farmer.location != ""
```

Constraints are pure validation functions that never mutate data.

### Complete Example Program

Let's create a complete example that demonstrates all constructs:

```kern
# Define our data structures
entity Farmer {
    id
    name
    location
    produce
    certification
    approved
}

entity Report {
    farmer_id
    status
    timestamp
}

# Define validation rules
rule ValidateFarmer:
    if farmer.id != 0 and farmer.name != "" and farmer.location != ""
    then mark_valid(farmer)

rule ApproveCertified:
    if farmer.certification == "organic" and farmer.valid == true
    then set_approved(farmer, true)

rule GenerateReport:
    if farmer.approved == true
    then create_report(farmer.id, "APPROVED", current_time())

# Define validation constraints
constraint ValidId: farmer.id > 0
constraint ValidName: farmer.name != ""

# Define execution flow
flow FarmerApprovalProcess {
    ValidateFarmer
    ApproveCertified
    GenerateReport
}
```

## Compiling KERN Programs

KERN follows a multi-phase compilation pipeline that transforms source code into executable bytecode.

### Compilation Process

The compilation process involves several stages:

1. **Lexical Analysis**: Source code → Tokens
2. **Syntax Analysis**: Tokens → Abstract Syntax Tree (AST)
3. **Semantic Analysis**: AST → Validated AST with type information
4. **Graph Building**: Validated AST → Execution Graph
5. **Bytecode Generation**: Execution Graph → Bytecode
6. **Optimization**: Bytecode → Optimized Bytecode

### Compilation Commands

Currently, KERN compilation is handled programmatically through the Rust API. Here's how to compile a program:

```rust
use kern_parser::Parser;
use kern_graph_builder::GraphBuilder;
use kern_bytecode::BytecodeCompiler;

fn compile_kern_program(source_code: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Step 1: Parse the source code
    let mut parser = Parser::new(source_code);
    let program = parser.parse_program()?;

    // Step 2: Build execution graph
    let mut graph_builder = GraphBuilder::new();
    let graph = graph_builder.build_execution_graph(&program);

    // Step 3: Compile to bytecode
    let mut compiler = BytecodeCompiler::new();
    let bytecode = compiler.compile_graph(&graph);

    // Step 4: Serialize bytecode to bytes
    Ok(bytecode.serialize()?)
}
```

### Compilation Options and Flags

The KERN compiler supports various options for different use cases:

#### Debug Compilation
```bash
# Compile with debug information and tracing
cargo run --bin kern-compiler -- --debug --source my_program.kern
```

#### Release Compilation
```bash
# Compile with optimizations for production
cargo run --bin kern-compiler -- --release --source my_program.kern
```

#### Verbose Output
```bash
# Show detailed compilation steps
cargo run --bin kern-compiler -- --verbose --source my_program.kern
```

#### Output Formats
```bash
# Generate different output formats
cargo run --bin kern-compiler -- --format bytecode --source my_program.kern  # Default
cargo run --bin kern-compiler -- --format graph --source my_program.kern    # Execution graph
cargo run --bin kern-compiler -- --format ast --source my_program.kern      # Abstract syntax tree
```

### Error Handling During Compilation

KERN provides comprehensive error reporting at all compilation stages:

#### Syntax Errors
```
Error: Expected 'entity', 'rule', 'flow', or 'constraint' at line 15, column 5
  |
15 |     invalid_syntax_here
  |     ^^^^^^^^^^^^^^^^^^^
```

#### Semantic Errors
```
Error: Undefined symbol 'undefined_var' at line 22, column 10
  |
22 |     if undefined_var == 5
  |          ^^^^^^^^^^^^^
```

#### Type Errors
```
Error: Type mismatch: expected boolean, found number at line 8, column 12
  |
8  |     if farmer.id
  |            ^^^^^^
```

## Running KERN Programs

### Runtime Environment

KERN programs execute in a register-based virtual machine that provides:

- Deterministic execution with no hidden state
- Explicit context management
- Memory-safe operations
- Bounded resource usage

### Execution Process

1. **Program Loading**: Bytecode is loaded into the VM
2. **Context Initialization**: Execution context is set up with initial data
3. **Graph Execution**: Execution graph is traversed according to defined flows
4. **Rule Evaluation**: Rules are evaluated based on conditions
5. **Action Execution**: Actions are executed when conditions are met
6. **Result Output**: Execution results are returned or stored

### Running Programs Programmatically

Here's how to execute a KERN program using the Rust API:

```rust
use kern_vm::VirtualMachine;
use kern_bytecode::Bytecode;

fn execute_kern_program(bytecode: Bytecode) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new virtual machine instance
    let mut vm = VirtualMachine::new();
    
    // Load the bytecode program
    vm.load_program(bytecode)?;
    
    // Set up initial execution context
    vm.set_context("default", create_initial_context())?;
    
    // Execute the program
    vm.execute()?;
    
    // Retrieve results
    let results = vm.get_results()?;
    
    // Process results
    for result in results {
        println!("Execution result: {:?}", result);
    }
    
    Ok(())
}

fn create_initial_context() -> Context {
    let mut context = Context::new();
    
    // Add initial data to the context
    context.set("farmers", vec![
        Farmer { id: 1, name: "John Doe", location: "California", produce: "Apples", certification: "organic", approved: false },
        Farmer { id: 2, name: "Jane Smith", location: "Oregon", produce: "Wheat", certification: "conventional", approved: false }
    ]);
    
    context
}
```

### Command Line Execution

If a command-line runner is available, you can execute programs directly:

```bash
# Run a KERN program
cargo run --bin kern-runner -- my_program.kern

# Run with input data
cargo run --bin kern-runner -- --input data.json my_program.kern

# Run with specific flow
cargo run --bin kern-runner -- --flow FarmerApprovalProcess my_program.kern

# Run with tracing enabled
cargo run --bin kern-runner -- --trace my_program.kern
```

### Runtime Options

#### Execution Tracing
```bash
# Enable execution tracing for debugging
cargo run --bin kern-runner -- --trace --source my_program.kern
```

#### Memory Limits
```bash
# Set memory limits for the execution
cargo run --bin kern-runner -- --memory-limit 100MB my_program.kern
```

#### Execution Timeouts
```bash
# Set execution timeout
cargo run --bin kern-runner -- --timeout 30s my_program.kern
```

#### Input/Output Options
```bash
# Specify input and output files
cargo run --bin kern-runner -- --input input.json --output output.json my_program.kern
```

## Advanced Usage Patterns

### Working with Contexts

Contexts isolate execution state and allow for complex workflows:

```rust
use kern_vm::Context;

// Create multiple contexts for different scenarios
let mut vm = VirtualMachine::new();

// Context for validation
let mut validation_ctx = Context::new();
validation_ctx.set("data", validation_data);
vm.set_context("validation", validation_ctx)?;

// Context for processing
let mut processing_ctx = Context::new();
processing_ctx.set("data", processing_data);
vm.set_context("processing", processing_ctx)?;

// Execute different flows in different contexts
vm.execute_flow("validation", "ValidationFlow")?;
vm.execute_flow("processing", "ProcessingFlow")?;
```

### External Function Integration

KERN can call external functions through adapters:

```kern
# In your KERN program
rule CallExternal:
    if condition_met()
    then external_function(data)
```

```rust
// In your Rust code
vm.register_external_function("external_function", |args| {
    // Your external function implementation
    // This could be a database call, API request, etc.
    Ok(result)
})?;
```

### Error Handling in Programs

KERN uses a data-based error handling model:

```kern
rule ProcessWithErrors:
    if operation_result.error == false
    then handle_success(operation_result.value)
    else handle_error(operation_result.error_message)

# Constraints for validation
constraint NoErrors: result.error == false
```

## Debugging KERN Programs

### Debugging Tools

KERN provides several debugging capabilities:

#### Execution Tracing
Enable tracing to see the execution flow:

```bash
cargo run --bin kern-runner -- --trace my_program.kern
```

#### Step-by-step Execution
Execute programs one step at a time:

```rust
// In your Rust code
vm.enable_single_step_mode();
while !vm.is_complete() {
    let step_result = vm.execute_single_step()?;
    println!("Step result: {:?}", step_result);
}
```

#### Graph Visualization
Visualize the execution graph:

```bash
cargo run --bin kern-compiler -- --format graph --visualize my_program.kern
```

### Common Debugging Scenarios

#### Infinite Loops
KERN prevents infinite loops by design, but if you encounter execution issues:

1. Check for recursive rule dependencies
2. Verify flow definitions don't create cycles
3. Use execution tracing to identify problematic rules

#### Rule Not Firing
If a rule isn't executing as expected:

1. Verify the condition evaluates to true with your input data
2. Check that the rule is included in an active flow
3. Ensure all dependencies are satisfied

#### Performance Issues
For slow execution:

1. Profile rule evaluation times
2. Check for expensive operations in conditions
3. Optimize data structures and access patterns

## Best Practices

### Program Organization

1. **Separate Data and Logic**: Keep entity definitions separate from rule logic
2. **Modular Flows**: Break complex flows into smaller, reusable components
3. **Clear Naming**: Use descriptive names for entities, rules, and flows
4. **Validation First**: Validate data before processing it

### Performance Considerations

1. **Minimize Condition Complexity**: Keep rule conditions as simple as possible
2. **Efficient Data Access**: Structure data for efficient access patterns
3. **Batch Operations**: Group similar operations when possible
4. **Avoid Redundant Rules**: Eliminate duplicate or overlapping rule logic

### Maintainability

1. **Document Complex Rules**: Add comments for complex rule logic
2. **Use Constraints**: Validate data with constraints rather than complex conditions
3. **Test Thoroughly**: Create comprehensive test cases for all rules
4. **Version Control**: Track changes to KERN programs with version control

## Integration with Other Systems

### API Integration

KERN programs can be integrated into larger systems through APIs:

```rust
use warp::Filter;

// Create an API endpoint that executes KERN programs
async fn execute_kern_endpoint(
    program: String,
    input_data: serde_json::Value
) -> Result<impl warp::Reply, warp::Rejection> {
    let bytecode = compile_kern_program(&program)?;
    let mut vm = VirtualMachine::new();
    vm.load_program(bytecode)?;
    vm.set_context("input", input_data)?;
    vm.execute()?;
    
    let result = vm.get_results()?;
    Ok(warp::reply::json(&result))
}

// Set up the route
let execute_route = warp::post()
    .and(warp::path("execute"))
    .and(warp::body::json())
    .and_then(execute_kern_endpoint);
```

### Data Integration

KERN can work with various data sources:

```rust
// Load data from JSON
let data: serde_json::Value = serde_json::from_str(&json_string)?;
context.set("input_data", data);

// Load data from database
let db_data = database.query("SELECT * FROM farmers")?;
context.set("db_data", db_data);

// Load data from API
let api_response = reqwest::get("https://api.example.com/data").await?;
let api_data: serde_json::Value = api_response.json().await?;
context.set("api_data", api_data);
```

This usage guide provides a comprehensive overview of how to write, compile, and run KERN programs. The next sections will dive deeper into the language specification, compiler internals, and integration patterns.