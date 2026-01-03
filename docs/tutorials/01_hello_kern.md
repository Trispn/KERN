# KERN Tutorial 01: Hello KERN

## Overview

This tutorial introduces the basic concepts of KERN through a simple example. We'll create a basic program that demonstrates entities, rules, and execution.

## The Hello World Example

Let's start with a simple KERN program that represents a basic user system:

```kern
entity User {
  id: num
  name: sym
  active: bool
}

rule CreateUser {
  if user.id > 0 and user.name != "" then
    user.active = true
}

constraint ValidUser {
  user.id > 0 and user.name != ""
}
```

## Step-by-Step Explanation

### 1. Entity Definition
```kern
entity User {
  id: num
  name: sym
  active: bool
}
```
This defines a `User` entity with three fields:
- `id`: A numeric identifier
- `name`: A symbolic name
- `active`: A boolean status

### 2. Rule Definition
```kern
rule CreateUser {
  if user.id > 0 and user.name != "" then
    user.active = true
}
```
This rule states: "If a user has a positive ID and a non-empty name, then set their active status to true."

### 3. Constraint Definition
```kern
constraint ValidUser {
  user.id > 0 and user.name != ""
}
```
This constraint ensures that all users must have a positive ID and non-empty name.

## AST Representation

The AST for this program would look like:
```
Program
├── EntityDef: User
│   ├── Field: id (num)
│   ├── Field: name (sym)
│   └── Field: active (bool)
├── RuleDef: CreateUser
│   ├── Condition: user.id > 0 and user.name != ""
│   └── Action: user.active = true
└── ConstraintDef: ValidUser
    └── Expression: user.id > 0 and user.name != ""
```

## Execution Graph

The execution graph would contain nodes for:
- User entity definition
- CreateUser rule with condition and action
- ValidUser constraint validation

## Bytecode Snippet

The compiled bytecode would include instructions for:
- Loading values for comparison
- Performing logical operations
- Setting field values
- Validating constraints

## Execution Trace

When executed with a user having id=1 and name="Alice":
1. Validate constraint: user.id > 0 and user.name != "" (true)
2. Evaluate rule condition: user.id > 0 and user.name != "" (true)
3. Execute rule action: user.active = true
4. Result: User is active

## PSI Observation Snapshot

The PSI system would observe:
- Entity schema for User
- Rule logic for activation
- Constraint validation requirements
- Execution trace with state changes

## Running the Example

To run this example in a KERN environment:
1. Define the entities, rules, and constraints
2. Provide input data (a user with id=1 and name="Alice")
3. Execute the program
4. Observe the deterministic result (user.active = true)

## Key Takeaways

- KERN programs are composed of entities, rules, flows, and constraints
- All operations are deterministic
- Rules define conditional logic
- Constraints define validation requirements
- The execution is predictable and reproducible

This simple example demonstrates the core concepts of KERN. In the next tutorial, we'll explore more complex entity relationships and type systems.