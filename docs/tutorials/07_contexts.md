# KERN Tutorial 07: Contexts

## Overview

This tutorial explores KERN's context system, which provides variable scoping, state isolation, and execution environment management. Contexts enable complex multi-step processes with isolated state.

## Context Basics

A context in KERN provides isolated variable scoping:

```kern
context UserSession {
  user_id: num
  permissions: vec
  last_activity: num
}
```

## Context Definition

Contexts are defined with typed variables:

```kern
context ValidationContext {
  input_data: ref
  validation_result: bool
  error_messages: vec
  validation_timestamp: num
}

context ProcessingContext {
  raw_data: ref
  processed_data: ref
  processing_status: sym
  processing_time: num
}

context StorageContext {
  connection: ref
  transaction_active: bool
  stored_records: num
}
```

## Using Contexts

Contexts are used with the `with` construct:

```kern
entity User {
  id: num
  name: sym
  session_active: bool
}

rule ProcessUserWithSession {
  if user.id > 0 then
    with UserSession {
      user_id = user.id;
      permissions = get_user_permissions(user_id);
      last_activity = current_timestamp();
      user.session_active = true
    }
}
```

## Context Switching in Flows

Contexts are particularly useful in flows:

```kern
flow ComplexProcessingFlow {
  with validation_context {
    input_data = load_raw_data();
    validation_result = validate_data(input_data);
    if validation_result == false then {
      error_messages = get_validation_errors();
      log_errors(error_messages)
    }
  };
  
  if validation_context.validation_result == true then
    with processing_context {
      raw_data = validation_context.input_data;
      processed_data = transform_data(raw_data);
      processing_status = "completed";
      processing_time = measure_time()
    };
  
  with storage_context {
    connection = get_database_connection();
    store_processed_data(processing_context.processed_data);
    stored_records = 1
  }
}
```

## Context Isolation

Contexts provide complete variable isolation:

```kern
context ContextA {
  value: num = 10
}

context ContextB {
  value: num = 20  // Different variable, same name
}

rule IsolatedVariables {
  with ContextA {
    ContextA.value = ContextA.value + 5  // ContextA.value is now 15
  };
  with ContextB {
    ContextB.value = ContextB.value - 5  // ContextB.value is now 15
  };
  // Global scope has no access to ContextA.value or ContextB.value
}
```

## Nested Contexts

Contexts can be nested (though this should be used carefully):

```kern
flow NestedContextFlow {
  with outer_context {
    outer_value = initialize_outer();
    
    with inner_context {
      inner_value = derive_from(outer_context.outer_value);
      process_inner(inner_value)
    };
    
    process_outer(outer_context.outer_value)
  }
}
```

## Context with Conditional Logic

Contexts work with conditional execution:

```kern
rule ConditionalContext {
  if user.requires_validation then
    with validation_context {
      validation_context.input = user.data;
      validation_context.result = validate_user_data(user.data);
      
      if validation_context.result == true then
        user.status = "validated"
      else
        user.status = "invalid"
    }
  else
    user.status = "skipped"
}
```

## Context Lifecycle

Contexts follow a clear lifecycle:

```kern
flow ContextLifecycleExample {
  // Context is created and initialized
  with processing_context {
    // Context variables are accessible and modifiable
    processing_context.status = "initializing";
    processing_context.data = load_data();
    
    // Perform operations within context
    processing_context.result = process_data(processing_context.data);
    processing_context.status = "completed";
    
    // Context variables can be used for further logic
    if processing_context.result.success == true then
      log_success()
    else
      log_failure()
  }
  // Context is destroyed, variables are no longer accessible
}
```

## Context Inheritance (Conceptual)

While KERN doesn't have traditional inheritance, you can achieve similar results:

```kern
context BaseContext {
  created_at: num
  active: bool
}

context AdminContext {
  created_at: num      // Inherited conceptually
  active: bool         // Inherited conceptually
  admin_level: num
  permissions: vec
}

rule AdminSpecificLogic {
  if user.is_admin == true then
    with AdminContext {
      AdminContext.created_at = current_timestamp();
      AdminContext.active = true;
      AdminContext.admin_level = get_admin_level(user);
      AdminContext.permissions = get_admin_permissions(user)
    }
}
```

## Context Sharing Patterns

Contexts can pass data between each other:

```kern
flow ContextDataSharing {
  with source_context {
    source_context.data = prepare_data();
    source_context.ready = true
  };
  
  if source_context.ready == true then
    with target_context {
      target_context.input = source_context.data;  // Data passed between contexts
      target_context.result = process_data(target_context.input)
    }
}
```

## AST Representation

The AST for contexts would look like:
```
ContextDef: UserSession
├── VariableDef: user_id (num)
├── VariableDef: permissions (vec)
└── VariableDef: last_activity (num)
ContextAction: UserSession
├── Assignment: user_id = user.id
├── Assignment: permissions = get_user_permissions(user_id)
├── Assignment: last_activity = current_timestamp()
└── Assignment: user.session_active = true
```

## Execution Graph

The execution graph for contexts includes:
- Context definition nodes
- Context activation nodes
- Variable access edges
- Data flow between contexts
- Context lifecycle management

## Bytecode Snippet

The bytecode would include instructions for:
- Context creation (CTX_CREATE)
- Context switching (CTX_SWITCH)
- Variable access within contexts
- Context destruction (CTX_DESTROY)
- Data flow management

## Execution Trace Example

With a context usage:
1. Context UserSession created with initial state
2. Variables user_id, permissions, last_activity set within context
3. Context operations completed
4. Context destroyed, state preserved as needed
5. Result: User session established

## Context Patterns

### 1. Isolation Pattern
```kern
context IsolatedCalculation {
  temp_value: num
  intermediate_result: num
}

rule SafeCalculation {
  with IsolatedCalculation {
    IsolatedCalculation.temp_value = input.value * 2;
    IsolatedCalculation.intermediate_result = complex_calculation(IsolatedCalculation.temp_value);
    output.result = IsolatedCalculation.intermediate_result
  }
}
```

### 2. Resource Management Pattern
```kern
context ResourceManager {
  resource_handle: ref
  resource_active: bool
}

flow ResourceIntensiveTask {
  with ResourceManager {
    ResourceManager.resource_handle = acquire_resource();
    ResourceManager.resource_active = true;
    
    perform_task_with(ResourceManager.resource_handle);
    
    release_resource(ResourceManager.resource_handle);
    ResourceManager.resource_active = false
  }
}
```

### 3. Transaction Pattern
```kern
context TransactionContext {
  transaction_id: num
  transaction_active: bool
  rollback_required: bool
}

flow TransactionalOperation {
  with TransactionContext {
    TransactionContext.transaction_id = begin_transaction();
    TransactionContext.transaction_active = true;
    
    try {
      perform_operation();
      commit_transaction(TransactionContext.transaction_id)
    }
    catch error {
      TransactionContext.rollback_required = true;
      rollback_transaction(TransactionContext.transaction_id)
    };
    
    TransactionContext.transaction_active = false
  }
}
```

## Context Best Practices

### 1. Keep Contexts Focused
```kern
// Good: Focused context
context ValidationContext {
  input: ref
  result: bool
  errors: vec
}

// Avoid: Overly broad context
context EverythingContext {
  // Too many unrelated variables
}
```

### 2. Use Descriptive Names
```kern
context UserAuthenticationContext {
  user_credentials: ref
  auth_result: bool
  auth_token: ref
  auth_timestamp: num
}
```

### 3. Initialize Context Variables
```kern
context ProcessingContext {
  input_data: ref
  result: ref
  status: sym = "pending"  // Default value
  start_time: num = 0      // Default value
}
```

## Context Limitations and Considerations

### 1. Memory Usage
Contexts consume memory, so use them appropriately:

```kern
// Consider memory implications
context LargeDataContext {
  large_dataset: ref  // Be aware of memory requirements
}
```

### 2. Complexity Management
Avoid overly complex context nesting:

```kern
// Better: Flatter context structure
flow SimpleContexts {
  with validation_context { validate() };
  with processing_context { process() };
  with storage_context { store() }
}
```

## PSI Observation

The PSI system observes:
- Context definitions and their variables
- Context activation and deactivation
- Variable access patterns within contexts
- Data flow between contexts
- Context lifecycle events
- State isolation properties

## Error Handling in Contexts

Contexts should include proper error handling:

```kern
context RobustContext {
  data: ref
  status: sym
  error_code: num
}

rule ContextWithErrorHandling {
  with RobustContext {
    try {
      RobustContext.data = load_data();
      RobustContext.status = "loaded";
      process_data(RobustContext.data);
      RobustContext.status = "processed"
    }
    catch error {
      RobustContext.error_code = error.code;
      RobustContext.status = "error";
      log_context_error(error)
    }
  }
}
```

## Summary

In this tutorial, we learned:
- How to define contexts with typed variables
- How to use contexts with the `with` construct
- How contexts provide variable isolation
- How to manage context lifecycles
- How to use contexts in flows and rules
- Best practices for context design
- How contexts interact with other KERN constructs

In the next tutorial, we'll explore error handling in depth, learning how to create robust KERN programs that handle failures gracefully.