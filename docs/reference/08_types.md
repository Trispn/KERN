# KERN Language Reference - Types

## 8.1 Definition

KERN uses a minimal type system with primitive types that map directly to the execution model. Types are checked at compile time and enforced at runtime where necessary.

## 8.2 Primitive Types

KERN supports the following primitive types:

### 8.2.1 sym (Symbol)
- Represents symbolic identifiers
- Maps to string values in the symbol table
- Used for named references
- Immutable once defined

### 8.2.2 num (Number)
- Represents integer values
- 64-bit signed integers
- Used for arithmetic operations
- Range: -2^63 to 2^63-1

### 8.2.3 bool (Boolean)
- Represents true/false values
- Used for conditional logic
- Result of comparison operations
- Values: true, false

### 8.2.4 vec (Vector)
- Represents small fixed-size arrays
- Maximum size defined by implementation
- Homogeneous element types
- Indexed access only

### 8.2.5 ref (Reference)
- Represents external references
- Points to resources outside KERN
- Used for I/O operations
- Implementation-defined behavior

### 8.2.6 ctx (Context)
- Represents execution contexts
- Manages variable scoping
- Enables context switching
- Implementation-defined structure

## 8.3 Type System Rules

### 8.3.1 Type Safety
- All type errors are caught at compile time
- No implicit type conversions
- Explicit casting required where allowed
- Static type checking for all expressions

### 8.3.2 Type Inference
- Limited type inference for variable declarations
- Explicit types required for function signatures
- Context-dependent inference where unambiguous

## 8.4 Examples

### 8.4.1 Type Declarations
```
entity User {
  id: num          // User ID as number
  name: sym        // User name as symbol
  active: bool     // Active status as boolean
  tags: vec        // Tags as vector
  profile: ref     // Profile as external reference
  context: ctx     // Execution context
}
```

### 8.4.2 Type Operations
```
rule TypeOperations {
  if user.active == true then {
    user.id = user.id + 1;      // num arithmetic
    user.name = "updated";      // sym assignment
    user.active = not user.active; // bool operation
  }
}
```

## 8.5 Error Conditions

| Error Code | Condition | Recoverable |
|------------|-----------|-------------|
| TYP001 | Type mismatch in assignment | No |
| TYP002 | Invalid operation for type | No |
| TYP003 | Out of bounds vector access | No |
| TYP004 | Invalid type conversion | No |

## 8.6 Bytecode Mapping

Types map to bytecode as follows:
- `sym`: Symbol table references
- `num`: 64-bit integer operations
- `bool`: Flag register operations
- `vec`: Memory array operations
- `ref`: External function calls
- `ctx`: Context switching instructions

## 8.7 PSI Observability

Types are observable as:
- Type information in the PSI knowledge base
- Type safety guarantees for reasoning
- Runtime type information for debugging
- Type relationships for analysis