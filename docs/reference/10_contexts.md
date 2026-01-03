# KERN Language Reference - Contexts

## 10.1 Definition

Contexts in KERN provide variable scoping, state isolation, and execution environment management. They enable complex multi-step processes with isolated state.

## 10.2 Syntax

```
context_def = 'context' identifier '{' { variable_def } '}' ;
variable_def = identifier ':' type [ '=' expression ] ;
```

Context switching:
```
'with' identifier '{' { expression } '}' ;
```

## 10.3 Semantics

Contexts provide:
- Variable scoping and isolation
- State management for complex processes
- Execution environment switching
- Data encapsulation
- Context inheritance and cloning

## 10.4 Examples

### 10.4.1 Context Definition
```
context UserSession {
  user_id: num;
  permissions: vec;
  last_activity: num = 0
}
```

### 10.4.2 Context Usage
```
rule ProcessRequest {
  if request.authenticated == true then
    with UserSession {
      user_id = request.user_id;
      permissions = get_user_permissions(user_id);
      last_activity = current_time()
    }
}
```

### 10.4.3 Context Inheritance
```
context AdminSession extends UserSession {
  admin_level: num;
  can_modify_system: bool = true
}
```

## 10.5 Execution Guarantees

- Context variables are isolated between contexts
- Context switching is explicit and deterministic
- Context state is preserved across rule executions
- Context inheritance follows explicit hierarchy
- Context cloning creates independent state

## 10.6 Error Conditions

| Error Code | Condition | Recoverable |
|------------|-----------|-------------|
| CTX001 | Invalid context reference | No |
| CTX002 | Context variable not found | No |
| CTX003 | Context inheritance cycle | No |
| CTX004 | Context memory limit exceeded | No |
| CTX005 | Invalid context switch | Yes |

## 10.7 Bytecode Mapping

Contexts map to bytecode as:
- CTX_CREATE for new contexts
- CTX_SWITCH for context switching
- CTX_CLONE for context duplication
- CTX_DESTROY for context cleanup
- Memory management for context data

## 10.8 Context Operations

### 10.8.1 Context Creation
```
create_context(context_name) -> context_id
```

### 10.8.2 Context Switching
```
switch_to_context(context_id)
```

### 10.8.3 Context Cloning
```
clone_context(source_id) -> new_context_id
```

### 10.8.4 Context Destruction
```
destroy_context(context_id)
```

## 10.9 Context Lifecycle

Contexts follow a defined lifecycle:
1. Creation with initial state
2. Activation for execution
3. Modification during execution
4. Deactivation after execution
5. Cleanup when no longer needed

## 10.10 PSI Observability

Contexts are observable as:
- State containers in the PSI knowledge base
- Variable scoping for analysis
- Execution environment information
- Context relationships and dependencies
- State transition patterns