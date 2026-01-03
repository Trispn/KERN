# KERN APIs & Integration

## Integration Overview

KERN is designed for seamless integration with other systems, languages, and platforms. The integration architecture follows the principle of explicit interfaces and capability-based security, ensuring that KERN programs can safely interact with external systems while maintaining their deterministic properties.

## Rust API Integration

### Core Integration Components

The primary integration points are provided through Rust crates in the KERN workspace:

- **kern-vm**: Virtual machine for executing KERN bytecode
- **kern-bytecode**: Bytecode compilation and serialization
- **kern-parser**: Source code parsing and AST generation
- **kern-graph-builder**: Execution graph construction

### Basic Integration Example

Here's a complete example of integrating KERN into a Rust application:

```rust
use kern_vm::{VirtualMachine, Context};
use kern_parser::Parser;
use kern_graph_builder::GraphBuilder;
use kern_bytecode::BytecodeCompiler;

// Function to execute a KERN program within your application
pub fn execute_kern_program(
    source_code: &str,
    input_data: serde_json::Value
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Step 1: Parse the source code
    let mut parser = Parser::new(source_code);
    let program = parser.parse_program()?;
    
    // Step 2: Build execution graph
    let mut graph_builder = GraphBuilder::new();
    let graph = graph_builder.build_execution_graph(&program);
    
    // Step 3: Compile to bytecode
    let mut compiler = BytecodeCompiler::new();
    let bytecode = compiler.compile_graph(&graph);
    
    // Step 4: Execute in VM
    let mut vm = VirtualMachine::new();
    vm.load_program(bytecode)?;
    
    // Step 5: Set up execution context with input data
    let mut context = Context::new();
    context.set_data("input", input_data)?;
    vm.set_context("main", context)?;
    
    // Step 6: Execute the program
    vm.execute()?;
    
    // Step 7: Retrieve results
    let results = vm.get_results()?;
    Ok(serde_json::to_value(results)?)
}

// Example usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kern_code = r#"
        entity Data {
            value
            processed
        }
        
        rule ProcessData:
            if data.value > 10
            then set_processed(data, true)
            else set_processed(data, false)
        
        flow ProcessFlow {
            ProcessData
        }
    "#;
    
    let input = serde_json::json!({
        "value": 15
    });
    
    let result = execute_kern_program(kern_code, input)?;
    println!("Execution result: {}", result);
    
    Ok(())
}
```

### Virtual Machine API

The Virtual Machine provides the core execution interface:

```rust
use kern_vm::{VirtualMachine, Context, ExecutionResult};

// Create and configure a VM instance
let mut vm = VirtualMachine::new();

// Load a compiled program
vm.load_program(bytecode)?;

// Create execution contexts
let mut context = Context::new();
context.set_data("key", "value")?;
vm.set_context("default", context)?;

// Execute the program
vm.execute()?;

// Retrieve execution results
let results = vm.get_results()?;

// Access specific context data
let output = vm.get_context_data("default", "output")?;
```

### Context Management

Contexts provide isolated execution environments:

```rust
use kern_vm::Context;

// Create a new context
let mut ctx = Context::new();

// Set data in the context
ctx.set_data("user_id", 12345)?;
ctx.set_data("permissions", vec!["read", "write"])?;

// Get data from the context
let user_id: i32 = ctx.get_data("user_id")?;

// Clone a context
let cloned_ctx = ctx.clone();

// Merge contexts
ctx.merge(&other_context)?;
```

## External Function Integration

KERN programs can call external functions through adapters, enabling integration with host systems.

### Registering External Functions

```rust
use kern_vm::VirtualMachine;

// Register a simple external function
vm.register_external_function("log", |args| {
    if let Some(message) = args.get(0) {
        println!("KERN Log: {}", message);
    }
    Ok(()) // Return success
})?;

// Register a function with return value
vm.register_external_function("get_timestamp", |_args| {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    Ok(timestamp)
})?;

// Register a database function
vm.register_external_function("query_database", |args| {
    if let Some(query) = args.get(0).and_then(|v| v.as_str()) {
        // Execute database query
        let result = execute_db_query(query)?;
        Ok(result)
    } else {
        Err("Invalid query argument".into())
    }
})?;
```

### Using External Functions in KERN

Once registered, external functions can be called from KERN programs:

```kern
rule LogActivity:
    if user.action == "login"
    then log("User logged in: " + user.id)

rule GetCurrentTime:
    if condition_met()
    then current_time = get_timestamp()

rule QueryData:
    if need_data()
    then result = query_database("SELECT * FROM users WHERE active = true")
```

### Complex External Function Example

Here's a more complex example with error handling:

```rust
use kern_vm::{VirtualMachine, ExternalFunction, Value};
use serde_json::Value as JsonValue;

// Define a complex external function
struct DatabaseAdapter {
    connection: DatabaseConnection, // Your database connection type
}

impl DatabaseAdapter {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize database connection
        Ok(DatabaseAdapter {
            connection: DatabaseConnection::new()?,
        })
    }
    
    fn query(&self, sql: &str) -> Result<JsonValue, Box<dyn std::error::Error>> {
        // Execute the query and return results as JSON
        self.connection.query(sql)
    }
}

// Create the adapter instance
let db_adapter = DatabaseAdapter::new()?;

// Register the function with closure capturing the adapter
vm.register_external_function("db_query", move |args| -> Result<Value, Box<dyn std::error::Error>> {
    if let Some(sql) = args.get(0).and_then(|v| v.as_str()) {
        match db_adapter.query(sql) {
            Ok(results) => Ok(Value::from_json(&results)?),
            Err(e) => Err(format!("Database error: {}", e).into()),
        }
    } else {
        Err("Expected string argument for SQL query".into())
    }
})?;
```

## Language Bindings

### HTTP API

KERN can be exposed as an HTTP service for integration with any language that supports HTTP:

```rust
use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ExecuteRequest {
    program: String,
    input: serde_json::Value,
}

#[derive(Serialize)]
struct ExecuteResponse {
    success: bool,
    result: Option<serde_json::Value>,
    error: Option<String>,
}

// HTTP endpoint for executing KERN programs
async fn execute_kern_handler(
    request: ExecuteRequest
) -> Result<impl warp::Reply, warp::Rejection> {
    match execute_kern_program(&request.program, request.input) {
        Ok(result) => Ok(warp::reply::json(&ExecuteResponse {
            success: true,
            result: Some(result),
            error: None,
        })),
        Err(e) => Ok(warp::reply::json(&ExecuteResponse {
            success: false,
            result: None,
            error: Some(e.to_string()),
        })),
    }
}

// Set up the route
let execute_route = warp::post()
    .and(warp::path("execute"))
    .and(warp::body::json())
    .and_then(execute_kern_handler);

// Run the server
warp::serve(execute_route).run(([127, 0, 0, 1], 3030)).await;
```

### Example HTTP Usage

```bash
# Execute a KERN program via HTTP
curl -X POST http://localhost:3030/execute \
  -H "Content-Type: application/json" \
  -d '{
    "program": "rule Hello: if 1 == 1 then log(\"Hello, World!\")",
    "input": {}
  }'
```

### REST API Design

A complete REST API for KERN integration:

```rust
use warp::Filter;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// In-memory storage for compiled programs
type ProgramStore = Arc<Mutex<HashMap<String, Vec<u8>>>>;

#[derive(Deserialize)]
struct CompileRequest {
    source: String,
    name: String,
}

#[derive(Deserialize)]
struct ExecuteRequest {
    program_name: String,
    context: serde_json::Value,
}

// Compile endpoint
async fn compile_handler(
    store: ProgramStore,
    request: CompileRequest
) -> Result<impl warp::Reply, warp::Rejection> {
    match compile_kern_program(&request.source) {
        Ok(bytecode) => {
            {
                let mut programs = store.lock().unwrap();
                programs.insert(request.name.clone(), bytecode);
            }
            Ok(warp::reply::json(&serde_json::json!({
                "success": true,
                "program_name": request.name
            })))
        }
        Err(e) => Ok(warp::reply::json(&serde_json::json!({
            "success": false,
            "error": e.to_string()
        }))),
    }
}

// Execute endpoint
async fn execute_handler(
    store: ProgramStore,
    request: ExecuteRequest
) -> Result<impl warp::Reply, warp::Rejection> {
    let programs = store.lock().unwrap();
    if let Some(bytecode) = programs.get(&request.program_name) {
        drop(programs); // Release lock before execution
        
        match execute_bytecode_with_context(bytecode, request.context) {
            Ok(result) => Ok(warp::reply::json(&serde_json::json!({
                "success": true,
                "result": result
            }))),
            Err(e) => Ok(warp::reply::json(&serde_json::json!({
                "success": false,
                "error": e.to_string()
            }))),
        }
    } else {
        Ok(warp::reply::json(&serde_json::json!({
            "success": false,
            "error": "Program not found"
        })))
    }
}

// Set up routes with shared state
let store: ProgramStore = Arc::new(Mutex::new(HashMap::new()));

let compile_route = warp::post()
    .and(warp::path("compile"))
    .and(warp::body::json())
    .and(with_store(store.clone()))
    .and_then(compile_handler);

let execute_route = warp::post()
    .and(warp::path("execute"))
    .and(warp::body::json())
    .and(with_store(store))
    .and_then(execute_handler);

let routes = compile_route.or(execute_route);

warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

// Helper to inject shared state
fn with_store(
    store: ProgramStore,
) -> impl Filter<Extract = (ProgramStore,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || store.clone())
}
```

## Integration Patterns

### Event-Driven Integration

KERN can be integrated into event-driven architectures:

```rust
use tokio::sync::mpsc;
use serde_json::Value;

// Event processor that executes KERN rules
pub struct KERNEventProcessor {
    vm: VirtualMachine,
    rule_contexts: HashMap<String, Context>,
}

impl KERNEventProcessor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut vm = VirtualMachine::new();
        
        // Register common external functions
        vm.register_external_function("emit_event", |args| {
            if let Some(event) = args.get(0) {
                // Emit event to event bus
                emit_to_bus(event)?;
            }
            Ok(())
        })?;
        
        Ok(KERNEventProcessor {
            vm,
            rule_contexts: HashMap::new(),
        })
    }
    
    pub async fn process_event(&mut self, event: Value) -> Result<Value, Box<dyn std::error::Error>> {
        // Set up context with event data
        let mut context = Context::new();
        context.set_data("event", event)?;
        self.vm.set_context("event_processing", context)?;
        
        // Execute the event processing rules
        self.vm.execute_flow("event_processing", "ProcessEventFlow")?;
        
        // Get results
        let result = self.vm.get_context_data("event_processing", "result")?;
        Ok(result)
    }
}

// Usage in event system
let mut processor = KERNEventProcessor::new()?;
let result = processor.process_event(event_data).await?;
```

### Database Integration

Integrating KERN with database systems:

```rust
use sqlx::{PgPool, Row};

// Database adapter for KERN
pub struct DatabaseAdapter {
    pool: PgPool,
}

impl DatabaseAdapter {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(DatabaseAdapter { pool })
    }
    
    pub async fn register_with_vm(&self, vm: &mut VirtualMachine) -> Result<(), Box<dyn std::error::Error>> {
        let pool = self.pool.clone();
        
        vm.register_external_function("db_select", move |args| {
            let pool = pool.clone();
            Box::pin(async move {
                if let Some(query) = args.get(0).and_then(|v| v.as_str()) {
                    let rows = sqlx::query(query)
                        .fetch_all(&pool)
                        .await
                        .map_err(|e| format!("Database error: {}", e))?;
                    
                    // Convert rows to KERN values
                    let result: Vec<serde_json::Value> = rows
                        .iter()
                        .map(|row| {
                            let mut obj = serde_json::Map::new();
                            for column in row.columns() {
                                if let Ok(value) = row.try_get::<serde_json::Value, _>(column.name()) {
                                    obj.insert(column.name().to_string(), value);
                                }
                            }
                            serde_json::Value::Object(obj)
                        })
                        .collect();
                    
                    Ok(serde_json::Value::Array(result).into())
                } else {
                    Err("Expected string query".into())
                }
            })
        })?;
        
        Ok(())
    }
}
```

### API Gateway Integration

Using KERN as a business logic layer in API gateways:

```rust
use warp::Filter;
use std::sync::Arc;

// KERN-based API gateway
pub struct KERNGateway {
    vm: Arc<Mutex<VirtualMachine>>,
    routing_rules: String,
}

impl KERNGateway {
    pub fn new(rules: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut vm = VirtualMachine::new();
        
        // Compile routing rules
        let bytecode = compile_kern_program(rules)?;
        vm.load_program(bytecode)?;
        
        Ok(KERNGateway {
            vm: Arc::new(Mutex::new(vm)),
            routing_rules: rules.to_string(),
        })
    }
    
    pub async fn route_request(&self, request: warp::http::Request<impl std::any::Any>) -> Result<warp::http::Response<impl std::any::Any>, warp::Rejection> {
        let mut vm = self.vm.lock().unwrap();
        
        // Set up context with request data
        let mut context = Context::new();
        context.set_data("request_method", request.method().as_str())?;
        context.set_data("request_path", request.uri().path())?;
        context.set_data("request_headers", extract_headers(&request))?;
        
        vm.set_context("routing", context)?;
        vm.execute_flow("routing", "RouteRequest")?;
        
        // Get routing decision
        let route_decision: serde_json::Value = vm.get_context_data("routing", "route")?;
        
        // Execute the routed request
        let response = execute_routed_request(route_decision).await?;
        Ok(response)
    }
}
```

## Security and Capability Management

### Capability-Based Security

KERN uses capability-based security for safe integration:

```rust
use kern_vm::{VirtualMachine, Capability, CapabilitySet};

// Define capabilities
let mut capabilities = CapabilitySet::new();

// File system capability
capabilities.add(Capability::new("read_file", |args| {
    if let Some(path) = args.get(0).and_then(|v| v.as_str()) {
        std::fs::read_to_string(path).map(Value::from)
    } else {
        Err("Invalid path".into())
    }
}));

// Network capability
capabilities.add(Capability::new("http_get", |args| {
    if let Some(url) = args.get(0).and_then(|v| v.as_str()) {
        reqwest::blocking::get(url)
            .map(|resp| Value::from(resp.text().unwrap_or_default()))
    } else {
        Err("Invalid URL".into())
    }
}));

// Apply capabilities to VM
let mut vm = VirtualMachine::new();
vm.set_capabilities(capabilities)?;
```

### Sandboxing

KERN provides execution sandboxing:

```rust
use kern_vm::{VirtualMachine, SandboxConfig};

// Configure sandbox limits
let config = SandboxConfig {
    max_memory: 100 * 1024 * 1024, // 100 MB
    max_execution_time: std::time::Duration::from_secs(10),
    allowed_syscalls: vec!["read", "write"],
    network_access: false,
    file_access: false,
};

let mut vm = VirtualMachine::new();
vm.set_sandbox_config(config);
```

## Performance Considerations

### Integration Performance

When integrating KERN into applications, consider these performance factors:

#### Compilation Caching
```rust
use std::collections::HashMap;
use std::time::SystemTime;

struct CompiledProgramCache {
    programs: HashMap<String, (Vec<u8>, SystemTime)>,
    max_age: std::time::Duration,
}

impl CompiledProgramCache {
    fn get_or_compile(&mut self, source: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let hash = calculate_hash(source);
        
        if let Some((bytecode, timestamp)) = self.programs.get(&hash) {
            if SystemTime::now().duration_since(*timestamp)? < self.max_age {
                return Ok(bytecode.clone());
            }
        }
        
        let bytecode = compile_kern_program(source)?;
        self.programs.insert(hash, (bytecode.clone(), SystemTime::now()));
        Ok(bytecode)
    }
}
```

#### Context Reuse
```rust
// Reuse contexts for similar operations
let mut reusable_context = Context::new();
reusable_context.set_data("constants", get_constants())?;

for input in inputs {
    reusable_context.set_data("input", input)?;
    vm.set_context("main", reusable_context.clone())?;
    vm.execute()?;
    let result = vm.get_results()?;
    // Process result
}
```

## Error Handling and Monitoring

### Comprehensive Error Handling

```rust
use kern_vm::{VirtualMachine, ExecutionError};

pub fn safe_execute_kern(
    vm: &mut VirtualMachine,
    program: &[u8],
    context: Context
) -> Result<ExecutionResult, IntegrationError> {
    // Load program safely
    vm.load_program(program)
        .map_err(|e| IntegrationError::ProgramLoad(e))?;
    
    // Set context safely
    vm.set_context("main", context)
        .map_err(|e| IntegrationError::ContextSetup(e))?;
    
    // Execute with timeout
    let result = vm.execute_with_timeout(std::time::Duration::from_secs(30))
        .map_err(|e| IntegrationError::Execution(e))?;
    
    Ok(result)
}

#[derive(Debug)]
pub enum IntegrationError {
    ProgramLoad(Box<dyn std::error::Error>),
    ContextSetup(Box<dyn std::error::Error>),
    Execution(ExecutionError),
    Timeout,
}
```

### Monitoring and Metrics

```rust
use metrics::{counter, histogram, gauge};

pub struct KERNMetrics {
    execution_count: metrics::Counter,
    execution_time: metrics::Histogram,
    active_contexts: metrics::Gauge,
}

impl KERNMetrics {
    pub fn new() -> Self {
        KERNMetrics {
            execution_count: counter!("kern_executions_total"),
            execution_time: histogram!("kern_execution_duration_seconds"),
            active_contexts: gauge!("kern_active_contexts"),
        }
    }
    
    pub async fn execute_with_metrics(
        &self,
        vm: &mut VirtualMachine,
        context: Context
    ) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        self.execution_count.increment(1);
        self.active_contexts.increment(1.0);
        
        let start = std::time::Instant::now();
        let result = vm.execute_with_context(context).await;
        let duration = start.elapsed();
        
        self.execution_time.record(duration.as_secs_f64());
        self.active_contexts.decrement(1.0);
        
        result
    }
}
```

This comprehensive integration guide provides all the information needed to integrate KERN into various systems, languages, and architectures while maintaining security and performance best practices.