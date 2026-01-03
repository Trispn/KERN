# KERN Compiler API Documentation

## Overview

The KERN Compiler API provides programmatic access to the KERN compilation pipeline, including parsing, semantic analysis, bytecode generation, and optimization.

## Core Compiler Components

### Compiler Interface

```rust
pub struct Compiler {
    pub lexer: Lexer,
    pub parser: Parser,
    pub semantic_analyzer: SemanticAnalyzer,
    pub graph_builder: GraphBuilder,
    pub bytecode_generator: BytecodeGenerator,
    pub optimizer: Optimizer,
}

impl Compiler {
    pub fn new() -> Self { ... }
    pub fn compile(&mut self, source: &str) -> Result<BytecodeModule, CompileError> { ... }
    pub fn compile_file(&mut self, file_path: &str) -> Result<BytecodeModule, CompileError> { ... }
    pub fn get_ast(&self) -> Option<&Program> { ... }
    pub fn get_execution_graph(&self) -> Option<&ExecutionGraph> { ... }
    pub fn get_symbol_table(&self) -> &SymbolTable { ... }
}
```

### Compilation Process

The compilation process follows these stages:
1. **Lexical Analysis**: Source code → Tokens
2. **Parsing**: Tokens → AST
3. **Semantic Analysis**: AST → Validated AST
4. **Graph Building**: AST → Execution Graph
5. **Bytecode Generation**: Execution Graph → Bytecode
6. **Optimization**: Bytecode → Optimized Bytecode

### Compilation Methods

#### `compile(source: &str) -> Result<BytecodeModule, CompileError>`

Compiles KERN source code from a string into bytecode.

**Parameters:**
- `source`: The KERN source code as a string slice

**Returns:**
- `Ok(BytecodeModule)`: Successfully compiled bytecode module
- `Err(CompileError)`: Compilation error with details

**Example:**
```rust
let mut compiler = Compiler::new();
let source = r#"
entity User {
  id: num
  name: sym
}

rule CreateUser {
  if user.id > 0 then
    user.name = "New User"
}
"#;

match compiler.compile(source) {
    Ok(bytecode) => println!("Compilation successful"),
    Err(e) => eprintln!("Compilation failed: {:?}", e),
}
```

#### `compile_file(file_path: &str) -> Result<BytecodeModule, CompileError>`

Compiles KERN source code from a file into bytecode.

**Parameters:**
- `file_path`: Path to the KERN source file

**Returns:**
- `Ok(BytecodeModule)`: Successfully compiled bytecode module
- `Err(CompileError)`: Compilation error with details

**Example:**
```rust
let mut compiler = Compiler::new();
match compiler.compile_file("example.kern") {
    Ok(bytecode) => println!("File compiled successfully"),
    Err(e) => eprintln!("Compilation failed: {:?}", e),
}
```

## Error Handling

### CompileError

The compiler returns detailed error information:

```rust
pub enum CompileError {
    LexError { message: String, location: SourceLocation },
    ParseError { message: String, location: SourceLocation },
    SemanticError { message: String, location: SourceLocation },
    TypeError { message: String, location: SourceLocation },
    ValidationError { message: String, location: SourceLocation },
}
```

### SourceLocation

Provides location information for errors:

```rust
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}
```

## AST Access

### `get_ast() -> Option<&Program>`

Returns a reference to the parsed AST if available.

**Returns:**
- `Some(&Program)`: Reference to the AST
- `None`: AST not available (compilation not completed)

### `get_execution_graph() -> Option<&ExecutionGraph>`

Returns a reference to the execution graph if available.

**Returns:**
- `Some(&ExecutionGraph)`: Reference to the execution graph
- `None`: Graph not available (compilation not completed)

### `get_symbol_table() -> &SymbolTable`

Returns a reference to the symbol table.

**Returns:**
- `&SymbolTable`: Reference to the symbol table

## Configuration

### CompilerOptions

Configure the compiler behavior:

```rust
pub struct CompilerOptions {
    pub optimize: bool,
    pub debug_info: bool,
    pub strict_mode: bool,
    pub max_errors: usize,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        CompilerOptions {
            optimize: true,
            debug_info: false,
            strict_mode: false,
            max_errors: 10,
        }
    }
}
```

### Example Configuration

```rust
let mut compiler = Compiler::new();
compiler.set_option(CompilerOption::Optimize(true));
compiler.set_option(CompilerOption::DebugInfo(true));
compiler.set_option(CompilerOption::StrictMode(true));
```

## Advanced Features

### Incremental Compilation

The compiler supports incremental compilation for better performance:

```rust
pub struct IncrementalCompiler {
    base_compiler: Compiler,
    cache: CompilationCache,
}

impl IncrementalCompiler {
    pub fn new() -> Self { ... }
    pub fn compile_with_cache(&mut self, source: &str, cache_key: &str) -> Result<BytecodeModule, CompileError> { ... }
    pub fn invalidate_cache(&mut self, cache_key: &str) { ... }
}
```

### Plugin System

The compiler supports plugins for extending functionality:

```rust
pub trait CompilerPlugin {
    fn pre_parse(&mut self, source: &str) -> Result<String, PluginError>;
    fn post_parse(&mut self, ast: &mut Program) -> Result<(), PluginError>;
    fn pre_codegen(&mut self, graph: &mut ExecutionGraph) -> Result<(), PluginError>;
    fn post_codegen(&mut self, bytecode: &mut BytecodeModule) -> Result<(), PluginError>;
}
```

## Performance Considerations

### Compilation Caching

For repeated compilations, consider using caching:

```rust
use std::collections::HashMap;

pub struct CachedCompiler {
    compiler: Compiler,
    cache: HashMap<String, BytecodeModule>,
}

impl CachedCompiler {
    pub fn compile_with_cache(&mut self, source: &str) -> Result<BytecodeModule, CompileError> {
        let hash = calculate_hash(source);
        if let Some(cached) = self.cache.get(&hash) {
            return Ok(cached.clone());
        }
        
        let result = self.compiler.compile(source)?;
        self.cache.insert(hash, result.clone());
        Ok(result)
    }
}
```

### Memory Management

The compiler manages memory efficiently:

- AST nodes are reference-counted
- Symbol tables use efficient hash maps
- Bytecode is compact and optimized

## Integration Examples

### Basic Integration

```rust
use kern_compiler::{Compiler, BytecodeModule};

fn compile_kern_code(source: &str) -> Result<BytecodeModule, String> {
    let mut compiler = Compiler::new();
    compiler.compile(source)
        .map_err(|e| format!("Compilation error: {:?}", e))
}
```

### Advanced Integration with Error Handling

```rust
use kern_compiler::{Compiler, CompileError, SourceLocation};

fn compile_with_detailed_errors(source: &str) -> Result<BytecodeModule, Vec<String>> {
    let mut compiler = Compiler::new();
    match compiler.compile(source) {
        Ok(bytecode) => Ok(bytecode),
        Err(CompileError::LexError { message, location }) => {
            Err(vec![format!("Lexical error at {}:{}: {}", 
                            location.line, location.column, message)])
        },
        Err(CompileError::ParseError { message, location }) => {
            Err(vec![format!("Parse error at {}:{}: {}", 
                            location.line, location.column, message)])
        },
        // Handle other error types similarly
        _ => Err(vec!["Compilation failed".to_string()]),
    }
}
```

## Testing and Validation

### Unit Testing

The compiler API supports testing:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_compilation() {
        let mut compiler = Compiler::new();
        let source = r#"
        entity Test {
            value: num
        }
        "#;
        
        let result = compiler.compile(source);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_compilation_error() {
        let mut compiler = Compiler::new();
        let source = r#"invalid syntax here"#;
        
        let result = compiler.compile(source);
        assert!(result.is_err());
    }
}
```

## Security Considerations

### Input Validation

The compiler validates all inputs:

- Source code is validated for proper UTF-8
- File paths are validated to prevent directory traversal
- Maximum source size limits prevent resource exhaustion

### Sandboxing

Compilation occurs in a safe environment:

- No external system calls during compilation
- Memory limits prevent allocation attacks
- Time limits prevent infinite compilation loops

## Performance Metrics

The compiler provides performance metrics:

```rust
pub struct CompilationStats {
    pub lex_time: Duration,
    pub parse_time: Duration,
    pub semantic_time: Duration,
    pub codegen_time: Duration,
    pub total_time: Duration,
    pub bytecode_size: usize,
    pub symbol_count: usize,
}
```

## Troubleshooting

### Common Issues

1. **Syntax Errors**: Check the error location and message
2. **Semantic Errors**: Verify entity/field references exist
3. **Type Errors**: Ensure type compatibility in expressions
4. **Memory Issues**: Consider compilation limits

### Debugging Tips

- Enable debug info for detailed error locations
- Use incremental compilation for faster iteration
- Check the symbol table for reference issues
- Validate the execution graph for correctness