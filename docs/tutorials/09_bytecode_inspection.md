# KERN Tutorial 09: Bytecode Inspection

## Overview

This tutorial explores KERN's bytecode representation and how to inspect the compiled form of KERN programs. Understanding bytecode is crucial for optimization, debugging, and low-level analysis.

## Bytecode Format

KERN uses a fixed-width 8-byte instruction format:

```
[OPCODE (1B)] [ARG1 (2B)] [ARG2 (2B)] [ARG3 (2B)] [FLAGS (1B)]
```

## Simple Example: Loading a Number

Let's start with a simple KERN program and its bytecode:

### Source Code
```kern
entity SimpleVar {
  value: num
}

rule SetSimpleValue {
  if true then
    simple_var.value = 42
}
```

### Bytecode Representation
```
Address | Instruction | Decoded
--------|-------------|--------
0x0000  | 11 2A 00 00 00 00 | LOAD_NUM 42 into R0
0x0008  | 62 00 00 00 00 00 | SET_SYMBOL (simple_var.value) from R0
0x0010  | 03 00 00 00 00 00 | HALT
```

## Instruction Breakdown

### LOAD_NUM (0x11)
- OPCODE: 0x11
- ARG1: 42 (value to load)
- ARG2: 0 (destination register, R0)
- ARG3: 0 (unused)
- FLAGS: 0

### SET_SYMBOL (0x62)
- OPCODE: 0x62
- ARG1: 0 (symbol reference)
- ARG2: 0 (source register, R0)
- ARG3: 0 (unused)
- FLAGS: 0

### HALT (0x03)
- OPCODE: 0x03
- All other fields: 0

## More Complex Example: Conditional Logic

### Source Code
```kern
entity ConditionalVar {
  input: num
  output: num
}

rule ConditionalLogic {
  if conditional_var.input > 10 then
    conditional_var.output = 100
  else
    conditional_var.output = 0
}
```

### Bytecode Representation
```
Address | Instruction | Decoded
--------|-------------|--------
0x0000  | 63 00 00 01 00 | GET_SYMBOL (conditional_var.input) to R1
0x0008  | 11 0A 00 02 00 | LOAD_NUM 10 into R2
0x0010  | 14 01 02 03 02 | COMPARE R1 > R2, result in R3 (flag set)
0x0018  | 02 28 00 00 00 | JMP_IF to address 0x28 if condition true
0x0020  | 11 00 00 04 00 | LOAD_NUM 0 into R4 (else branch)
0x0028  | 11 64 00 04 00 | LOAD_NUM 100 into R4 (then branch)
0x0030  | 62 01 00 04 00 | SET_SYMBOL (conditional_var.output) from R4
0x0038  | 03 00 00 00 00 | HALT
```

## Register Usage

KERN uses 16 general-purpose registers (R0-R15):
- R0-R15: General purpose symbolic registers
- Special registers are managed by the VM

## Control Flow Instructions

### JMP (0x01) - Unconditional Jump
```
JMP 0x20 00 00 00 00 00
```
Jumps to address 0x20 regardless of conditions.

### JMP_IF (0x02) - Conditional Jump
```
JMP_IF 0x28 00 00 00 00
```
Jumps to address 0x28 if the condition flag is set.

## Arithmetic Operations

### ADD (0x20)
```
ADD 01 02 03 00 00
```
R3 = R1 + R2

### SUB (0x21)
```
SUB 01 02 03 00 00
```
R3 = R1 - R2

### MUL (0x22)
```
MUL 01 02 03 00 00
```
R3 = R1 * R2

### DIV (0x23)
```
DIV 01 02 03 00 00
```
R3 = R1 / R2

## Logical Operations

### AND (0x30)
```
AND 01 02 03 00 00
```
R3 = R1 AND R2

### OR (0x31)
```
OR 01 02 03 00 00
```
R3 = R1 OR R2

### NOT (0x32)
```
NOT 01 02 00 00 00
```
R2 = NOT R1

## Context Management Instructions

### PUSH_CTX (0x60)
```
PUSH_CTX 00 00 00 00 00
```
Creates a new execution context.

### POP_CTX (0x61)
```
POP_CTX 00 00 00 00 00
```
Restores the previous execution context.

## Error Handling Instructions

### THROW (0x70)
```
THROW 01 00 00 00 00
```
Raises error with code 1.

### CLEAR_ERR (0x73)
```
CLEAR_ERR 00 00 00 00 00
```
Clears the error register.

## External Interface Instructions

### CALL_EXTERN (0x80)
```
CALL_EXTERN 01 00 00 00 00
```
Calls external function with ID 1.

### WRITE_IO (0x82)
```
WRITE_IO 01 00 00 00 00
```
Writes value in R1 to output.

## Bytecode Analysis Tools

### Disassembler Output
A KERN bytecode disassembler might show:
```
0x0000: LOAD_NUM R0, 42
0x0008: SET_SYMBOL sym_0, R0
0x0010: HALT
```

### Execution Trace
The VM can provide execution traces:
```
Step 0: PC=0x0000, Instruction=LOAD_NUM R0, 42
  R0 = 42, PC = 0x0008
Step 1: PC=0x0008, Instruction=SET_SYMBOL sym_0, R0
  Symbol sym_0 = 42, PC = 0x0010
Step 2: PC=0x0010, Instruction=HALT
  Execution halted
```

## Performance Analysis

Bytecode inspection reveals performance characteristics:

### Instruction Count
- Count of total instructions
- Distribution of instruction types
- Hot paths in execution

### Register Usage
- Which registers are used most
- Register allocation efficiency
- Potential for optimization

### Memory Access Patterns
- Frequency of symbol access
- Context switching overhead
- Data locality

## Debugging with Bytecode

### Setting Breakpoints
Breakpoints can be set at specific bytecode addresses:
```
BREAKPOINT 0x0028  // Before setting output value
```

### Variable Inspection
At any point, register values can be inspected:
```
REGISTERS:
  R0: 42
  R1: 15 (input value)
  R2: 10 (comparison value)
  R3: 1 (comparison result)
  R4: 100 (output value)
```

## Optimization Opportunities

Bytecode inspection reveals optimization opportunities:

### Dead Code Elimination
```
// Before optimization
0x0000: LOAD_NUM R0, 42
0x0008: LOAD_NUM R1, 99  // Never used
0x0010: SET_SYMBOL sym_0, R0
0x0018: HALT

// After optimization
0x0000: LOAD_NUM R0, 42
0x0008: SET_SYMBOL sym_0, R0
0x0010: HALT
```

### Constant Folding
```
// Before optimization
0x0000: LOAD_NUM R0, 10
0x0008: LOAD_NUM R1, 20
0x0010: ADD R0, R1, R2
0x0018: SET_SYMBOL sym_0, R2

// After optimization
0x0000: LOAD_NUM R0, 30
0x0008: SET_SYMBOL sym_0, R0
```

## Security Analysis

Bytecode inspection can verify security properties:

### Sandbox Compliance
- Verify external calls are properly validated
- Check for unauthorized memory access
- Ensure instruction limits are respected

### Resource Limits
- Monitor memory allocation instructions
- Track context creation limits
- Verify step count limits

## PSI Observability

The PSI system can observe bytecode for:

### Pattern Recognition
- Common instruction sequences
- Execution patterns
- Optimization opportunities

### Anomaly Detection
- Unexpected instruction sequences
- Resource usage anomalies
- Performance deviations

## Bytecode Verification

Bytecode can be verified for correctness:

### Type Safety
- Verify register usage matches expected types
- Check for invalid memory accesses
- Validate symbol references

### Determinism
- Ensure no random or time-dependent instructions
- Verify consistent execution paths
- Check for hidden state dependencies

## Example: Complete Program Analysis

### Source
```kern
entity Calculator {
  a: num
  b: num
  result: num
}

rule AddNumbers {
  if calculator.a >= 0 and calculator.b >= 0 then
    calculator.result = calculator.a + calculator.b
  else
    calculator.result = 0
}
```

### Bytecode Analysis
```
Address | Instruction | Description
--------|-------------|------------
0x0000  | GET_SYMBOL  | Load calculator.a to R0
0x0008  | LOAD_NUM    | Load 0 to R1 (for comparison)
0x0010  | COMPARE     | Compare R0 >= R1, result in R2
0x0018  | GET_SYMBOL  | Load calculator.b to R3
0x0020  | COMPARE     | Compare R3 >= R1, result in R4
0x0028  | AND         | R2 AND R4, result in R5
0x0030  | JMP_IF      | Jump if condition true (to 0x0040)
0x0038  | LOAD_NUM    | Load 0 to R6 (else branch)
0x0040  | GET_SYMBOL  | Load calculator.a to R7 (then branch)
0x0048  | GET_SYMBOL  | Load calculator.b to R8
0x0050  | ADD         | R7 + R8, result in R6
0x0058  | SET_SYMBOL  | Store R6 to calculator.result
0x0060  | HALT        | Stop execution
```

## Best Practices for Bytecode Analysis

### 1. Understand the Instruction Set
Familiarize yourself with all KERN opcodes and their effects.

### 2. Use Disassemblers
Use tools to convert bytecode back to readable form.

### 3. Monitor Performance
Track instruction counts and execution patterns.

### 4. Verify Security
Ensure bytecode complies with security policies.

### 5. Check Determinism
Verify that bytecode produces consistent results.

## Summary

In this tutorial, we learned:
- The 8-byte fixed-width instruction format
- How different KERN constructs compile to bytecode
- How to analyze bytecode for performance and security
- How to use bytecode for debugging and optimization
- How PSI systems can observe bytecode patterns
- Best practices for bytecode analysis

In the next tutorial, we'll explore PSI integration, learning how KERN programs interact with the PSI reasoning system.