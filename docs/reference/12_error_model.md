# KERN Language Reference - Error Model

## 12.1 Definition

The KERN error model defines how errors are detected, classified, reported, and handled in a deterministic manner.

## 12.2 Error Classification

KERN errors are classified into categories:

### 12.2.1 Syntax Errors (SYN)
- Detected during parsing
- Prevent compilation
- Non-recoverable

### 12.2.2 Semantic Errors (SEM)
- Detected during semantic analysis
- Prevent compilation
- Non-recoverable

### 12.2.3 Type Errors (TYP)
- Detected during type checking
- Prevent compilation
- Non-recoverable

### 12.2.4 Validation Errors (VAL)
- Detected during validation
- May occur at runtime
- Potentially recoverable

### 12.2.5 Runtime Errors (VM)
- Detected during execution
- May be recoverable
- Include resource limits

### 12.2.6 Control Errors (CTL)
- Related to control flow
- May be recoverable
- Include loop detection

### 12.2.7 Context Errors (CTX)
- Related to context management
- May be recoverable
- Include context switching

## 12.3 Error Handling

### 12.3.1 Compile-time Errors
- Detected during compilation phases
- Prevent bytecode generation
- Reported with source location

### 12.3.2 Runtime Errors
- Detected during execution
- May trigger error handling
- May cause execution halt

## 12.4 Error Codes

### 12.4.1 Syntax Error Codes
| Code | Meaning | Recoverable |
|------|---------|-------------|
| SYN001 | Unexpected token | No |
| SYN002 | Missing required element | No |
| SYN003 | Invalid syntax structure | No |
| SYN004 | Mismatched delimiters | No |
| SYN005 | Invalid identifier | No |

### 12.4.2 Semantic Error Codes
| Code | Meaning | Recoverable |
|------|---------|-------------|
| SEM001 | Duplicate definition | No |
| SEM002 | Undefined reference | No |
| SEM003 | Circular dependency | Yes |
| SEM004 | Invalid expression | No |
| SEM005 | Rule conflict | Yes |
| SEM006 | Circular rule dependency | No |
| SEM007 | Undefined symbol in expression | No |
| SEM008 | Constraint references undefined symbol | No |

### 12.4.3 Type Error Codes
| Code | Meaning | Recoverable |
|------|---------|-------------|
| TYP001 | Type mismatch | No |
| TYP002 | Invalid operation for type | No |
| TYP003 | Out of bounds access | No |
| TYP004 | Invalid type conversion | No |

### 12.4.4 Validation Error Codes
| Code | Meaning | Recoverable |
|------|---------|-------------|
| VAL001 | Constraint validation failed | No |
| VAL002 | Validation timeout | No |

### 12.4.5 Runtime Error Codes
| Code | Meaning | Recoverable |
|------|---------|-------------|
| VM001 | Invalid opcode | No |
| VM002 | Invalid register access | No |
| VM003 | Invalid memory access | No |
| VM004 | Execution limit exceeded | No |
| VM005 | Division by zero | Yes |
| VM006 | Stack overflow | No |
| VM007 | Stack underflow | No |
| VM008 | Memory limit exceeded | No |
| VM009 | Invalid PC value | No |
| VM010 | Undefined symbol | No |
| VM011 | Security violation | No |
| VM012 | Sandbox violation | No |

### 12.4.6 Control Error Codes
| Code | Meaning | Recoverable |
|------|---------|-------------|
| CTL001 | Invalid conditional expression | No |
| CTL002 | Undefined context reference | No |
| CTL003 | Infinite loop detected | No |
| CTL004 | Context switch failure | Yes |

### 12.4.7 Context Error Codes
| Code | Meaning | Recoverable |
|------|---------|-------------|
| CTX001 | Invalid context reference | No |
| CTX002 | Context variable not found | No |
| CTX003 | Context inheritance cycle | No |
| CTX004 | Context memory limit exceeded | No |
| CTX005 | Invalid context switch | Yes |

## 12.5 Error Reporting

### 12.5.1 Error Format
All errors follow the format:
```
ERROR_CODE: Description
  Location: file:line:column
  Context: surrounding code
  Suggestion: possible fix
```

### 12.5.2 Source Location
Errors include:
- File name
- Line number
- Column number
- Code snippet

## 12.6 Error Recovery

### 12.6.1 Compile-time Recovery
- Syntax errors: Parser recovery to detect additional errors
- Semantic errors: Continue analysis where possible
- Type errors: Report all type mismatches

### 12.6.2 Runtime Recovery
- Some errors allow execution to continue
- Error state is preserved
- Explicit error handling possible

## 12.7 Error Prevention

KERN's design prevents many errors:
- Strong typing prevents type errors
- Deterministic execution prevents race conditions
- Explicit control flow prevents goto-related errors
- Memory safety prevents buffer overflows

## 12.8 PSI Observability

Errors are observable as:
- Error codes and classifications
- Error locations in source
- Error contexts and causes
- Recovery attempts and outcomes
- Error patterns for analysis