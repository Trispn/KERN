# KERN Language Reference - Bytecode Mapping

## 13.1 Definition

The bytecode mapping defines how KERN source constructs are translated to the 8-byte fixed-width instruction format for the virtual machine.

## 13.2 Bytecode Format

Each instruction is exactly 8 bytes with the format:
```
[OPCODE (1B)] [ARG1 (2B)] [ARG2 (2B)] [ARG3 (2B)] [FLAGS (1B)]
```

## 13.3 Instruction Format Details

- **OPCODE**: 1 byte, instruction type (0x00-0xFF)
- **ARG1**: 2 bytes, first operand (0x0000-0xFFFF)
- **ARG2**: 2 bytes, second operand (0x0000-0xFFFF)
- **ARG3**: 2 bytes, third operand (0x0000-0xFFFF)
- **FLAGS**: 1 byte, instruction flags (0x00-0xFF)

## 13.4 Opcode Mapping

### 13.4.1 Control Flow Instructions
| Opcode | Name | ARG1 | ARG2 | ARG3 | Description |
|--------|------|------|------|------|-------------|
| 0x00 | NOP | - | - | - | No operation |
| 0x01 | JMP | target_addr | - | - | Unconditional jump |
| 0x02 | JMP_IF | target_addr | - | - | Conditional jump |
| 0x03 | HALT | - | - | - | Stop execution |

### 13.4.2 Data & Symbol Instructions
| Opcode | Name | ARG1 | ARG2 | ARG3 | Description |
|--------|------|------|------|------|-------------|
| 0x10 | LOAD_SYM | sym_id_low | sym_id_high | dest_reg | Load symbol value |
| 0x11 | LOAD_NUM | value_low | value_high | dest_reg | Load numeric literal |
| 0x12 | LOAD_BOOL | bool_val | - | dest_reg | Load boolean literal |
| 0x13 | MOVE | src_reg | dest_reg | - | Move between registers |
| 0x14 | COMPARE | reg_a | reg_b | result_reg | Compare values |

### 13.4.3 Arithmetic Instructions
| Opcode | Name | ARG1 | ARG2 | ARG3 | Description |
|--------|------|------|------|------|-------------|
| 0x20 | ADD | src1_reg | src2_reg | dest_reg | Add two registers |
| 0x21 | SUB | src1_reg | src2_reg | dest_reg | Subtract two registers |
| 0x22 | MUL | src1_reg | src2_reg | dest_reg | Multiply two registers |
| 0x23 | DIV | src1_reg | src2_reg | dest_reg | Divide two registers |
| 0x24 | MOD | src1_reg | src2_reg | dest_reg | Modulo operation |

### 13.4.4 Logical Instructions
| Opcode | Name | ARG1 | ARG2 | ARG3 | Description |
|--------|------|------|------|------|-------------|
| 0x30 | AND | src1_reg | src2_reg | dest_reg | Logical AND |
| 0x31 | OR | src1_reg | src2_reg | dest_reg | Logical OR |
| 0x32 | NOT | src_reg | dest_reg | - | Logical NOT |

### 13.4.5 Graph Operation Instructions
| Opcode | Name | ARG1 | ARG2 | ARG3 | Description |
|--------|------|------|------|------|-------------|
| 0x40 | CREATE_NODE | node_type | - | result_reg | Create graph node |
| 0x41 | CONNECT | node1 | node2 | edge_type | Create edge |
| 0x42 | MERGE | node1 | node2 | result_reg | Merge nodes |
| 0x43 | DELETE_NODE | node_id | - | - | Remove node |

### 13.4.6 Rule Execution Instructions
| Opcode | Name | ARG1 | ARG2 | ARG3 | Description |
|--------|------|------|------|------|-------------|
| 0x50 | CALL_RULE | rule_id | - | - | Invoke rule subgraph |
| 0x51 | RETURN_RULE | - | - | - | Return from rule |
| 0x52 | CHECK_CONDITION | cond_reg | - | result_reg | Evaluate condition |
| 0x53 | INCREMENT_EXEC_COUNT | counter_id | - | - | Track execution |

### 13.4.7 Context & State Instructions
| Opcode | Name | ARG1 | ARG2 | ARG3 | Description |
|--------|------|------|------|------|-------------|
| 0x60 | PUSH_CTX | - | - | result_reg | Push context frame |
| 0x61 | POP_CTX | - | - | - | Pop context frame |
| 0x62 | SET_SYMBOL | sym_id | value_reg | - | Update symbol |
| 0x63 | GET_SYMBOL | sym_id | - | result_reg | Read symbol |
| 0x64 | COPY_CTX | src_ctx | - | result_reg | Duplicate context |

### 13.4.8 Error Handling Instructions
| Opcode | Name | ARG1 | ARG2 | ARG3 | Description |
|--------|------|------|------|------|-------------|
| 0x70 | THROW | error_code | - | - | Raise error |
| 0x71 | TRY | handler_addr | - | - | Start try block |
| 0x72 | CATCH | - | - | - | Jump to catch block |
| 0x73 | CLEAR_ERR | - | - | - | Reset error state |

### 13.4.9 External Interface Instructions
| Opcode | Name | ARG1 | ARG2 | ARG3 | Description |
|--------|------|------|------|------|-------------|
| 0x80 | CALL_EXTERN | func_id | - | result_reg | Call external function |
| 0x81 | READ_IO | channel_id | - | result_reg | Read from input |
| 0x82 | WRITE_IO | value_reg | channel_id | - | Write to output |

## 13.5 Source-to-Bytecode Mapping Examples

### 13.5.1 Variable Assignment
Source: `x = 42`
Bytecode:
```
LOAD_NUM(0x11) 42(0x002A) 0x0000 x_reg 0x00
```

### 13.5.2 Arithmetic Operation
Source: `z = x + y`
Bytecode:
```
ADD(0x20) x_reg y_reg z_reg 0x00
```

### 13.5.3 Conditional Jump
Source: `if x > y then goto label`
Bytecode:
```
COMPARE(0x14) x_reg y_reg temp_reg 0x02  // 0x02 = Greater flag
JMP_IF(0x02) label_addr 0x0000 0x0000 0x00
```

## 13.6 Compilation Process

### 13.6.1 AST to LIR
- Abstract Syntax Tree (AST) converted to Low-level Intermediate Representation (LIR)
- Symbol resolution and type checking
- Optimization passes

### 13.6.2 LIR to Bytecode
- Register allocation
- Instruction selection
- Final bytecode generation

## 13.7 Error Handling in Bytecode

### 13.7.1 Runtime Error Detection
- Bounds checking for array access
- Division by zero detection
- Type safety enforcement
- Memory limit enforcement

### 13.7.2 Error Reporting
- Error codes stored in ERR register
- PC preserved at error location
- Context information maintained

## 13.8 PSI Observability

Bytecode is observable as:
- Instruction sequences for analysis
- Register usage patterns
- Memory access patterns
- Execution flow paths
- Performance characteristics
- Security validation points