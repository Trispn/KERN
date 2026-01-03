# KERN Language Reference - Execution Model

## 11.1 Definition

The KERN execution model defines how programs execute deterministically using a graph-based approach with register-based bytecode.

## 11.2 Core Principles

The execution model adheres to these principles:
- **Deterministic**: Same input always produces same output
- **Graph-based**: Execution represented as nodes and edges
- **Register-based**: Uses 16 general-purpose registers (R0-R15)
- **No call stack**: Execution without traditional call/return mechanism
- **Explicit control flow**: All control flow is explicitly defined

## 11.3 Virtual Machine Architecture

### 11.3.1 Registers
- **R0-R15**: General-purpose symbolic registers
- **CTX**: Current execution context register
- **ERR**: Error register (holds error code or 0)
- **PC**: Program counter (instruction index)
- **FLAG**: Condition flags register

### 11.3.2 Memory Model
- **Code**: Read-only bytecode section
- **Constants**: Read-only constants section
- **Stack**: Operand and call stack (4KB limit)
- **Heap**: Graph nodes, symbols, contexts (100KB default)
- **Meta**: PSI introspection and metadata (1KB default)

## 11.4 Bytecode Format

Each instruction is 8 bytes with the format:
```
[OPCODE (1B)] [ARG1 (2B)] [ARG2 (2B)] [ARG3 (2B)] [FLAGS (1B)]
```

## 11.5 Instruction Categories

### 11.5.1 Control Flow Instructions
- `NOP` (0x00): No operation
- `JMP` (0x01): Unconditional jump
- `JMP_IF` (0x02): Conditional jump
- `HALT` (0x03): Stop execution

### 11.5.2 Data & Symbol Instructions
- `LOAD_SYM` (0x10): Load symbol value
- `LOAD_NUM` (0x11): Load numeric literal
- `MOVE` (0x13): Move between registers
- `COMPARE` (0x14): Compare values

### 11.5.3 Arithmetic Instructions
- `ADD` (0x20): Add two registers
- `SUB` (0x21): Subtract two registers
- `MUL` (0x22): Multiply two registers
- `DIV` (0x23): Divide two registers
- `MOD` (0x24): Modulo operation

### 11.5.4 Logical Instructions
- `AND` (0x30): Logical AND
- `OR` (0x31): Logical OR
- `NOT` (0x32): Logical NOT

### 11.5.5 Graph Operation Instructions
- `CREATE_NODE` (0x40): Create graph node
- `CONNECT` (0x41): Create edge
- `MERGE` (0x42): Merge nodes
- `DELETE_NODE` (0x43): Remove node

### 11.5.6 Rule Execution Instructions
- `CALL_RULE` (0x50): Invoke rule subgraph
- `RETURN_RULE` (0x51): Return from rule
- `CHECK_CONDITION` (0x52): Evaluate condition
- `INCREMENT_EXEC_COUNT` (0x53): Track execution

### 11.5.7 Context & State Instructions
- `PUSH_CTX` (0x60): Push context frame
- `POP_CTX` (0x61): Pop context frame
- `SET_SYMBOL` (0x62): Update symbol
- `GET_SYMBOL` (0x63): Read symbol
- `COPY_CTX` (0x64): Duplicate context

### 11.5.8 Error Handling Instructions
- `THROW` (0x70): Raise error
- `TRY` (0x71): Start try block
- `CATCH` (0x72): Catch block
- `CLEAR_ERR` (0x73): Reset error state

### 11.5.9 External Interface Instructions
- `CALL_EXTERN` (0x80): Call external function
- `READ_IO` (0x81): Read from input
- `WRITE_IO` (0x82): Write to output

## 11.6 Execution Process

### 11.6.1 Fetch-Decode-Execute Cycle
1. Fetch instruction from program counter
2. Decode instruction opcode and operands
3. Execute instruction with safety checks
4. Update program counter
5. Repeat until HALT or error

### 11.6.2 Safety Layer
- Memory limits enforcement
- Step count limits
- Sandbox policy validation
- Security validation

## 11.7 Execution Guarantees

- Programs execute deterministically
- Memory limits prevent resource exhaustion
- Step limits prevent infinite loops
- Sandbox policies prevent unauthorized access
- Error handling is explicit and controlled

## 11.8 Error Conditions

| Error Code | Condition | Recoverable |
|------------|-----------|-------------|
| VM001 | Invalid opcode | No |
| VM002 | Invalid register access | No |
| VM003 | Invalid memory access | No |
| VM004 | Execution limit exceeded | No |
| VM005 | Division by zero | Yes |
| VM006 | Stack overflow | No |
| VM007 | Stack underflow | No |
| VM008 | Memory limit exceeded | No |

## 11.9 PSI Observability

Execution is observable as:
- Complete execution traces
- Register state changes
- Memory usage patterns
- Context switches
- Rule firing sequences
- Graph transformations
- Performance metrics