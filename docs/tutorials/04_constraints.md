# KERN Tutorial 04: Constraints

## Overview

This tutorial explores KERN's constraint system, which is used to enforce data integrity, business rules, and validation requirements. Constraints define conditions that must be satisfied for a state to be valid.

## Constraint Structure

A constraint in KERN has the basic structure:

```kern
constraint ConstraintName {
  expression
}
```

The expression must evaluate to true for the constraint to be satisfied.

## Simple Constraint Example

Let's start with a simple constraint that ensures positive prices:

```kern
entity Product {
  id: num
  name: sym
  price: num
}

constraint PositivePrice {
  product.price > 0
}
```

This constraint ensures that all products must have a positive price.

## Complex Constraints

Constraints can have complex expressions:

```kern
constraint ValidUserAge {
  user.age >= 0 and user.age <= 150
}

constraint ValidOrder {
  order.quantity > 0 and order.total > 0 and order.customer_id > 0
}

constraint PasswordStrength {
  length(user.password) >= 8 and 
  contains_uppercase(user.password) and
  contains_digit(user.password)
}
```

## Multiple Constraints

An entity can have multiple constraints:

```kern
entity User {
  id: num
  name: sym
  age: num
  email: sym
  password: sym
}

constraint ValidId {
  user.id > 0
}

constraint ValidName {
  user.name != ""
}

constraint ValidAge {
  user.age >= 0 and user.age <= 150
}

constraint ValidEmail {
  user.email != "" and contains(user.email, "@")
}

constraint ValidPassword {
  length(user.password) >= 8
}
```

## Constraints with Calculations

Constraints can include calculations:

```kern
entity Order {
  items: vec
  total: num
  tax: num
  total_with_tax: num
}

constraint ValidTotal {
  order.total_with_tax == order.total + order.tax
}

constraint NonEmptyOrder {
  length(order.items) > 0
}
```

## Cross-Entity Constraints

Constraints can reference multiple entities:

```kern
entity Customer {
  id: num
  name: sym
  credit_limit: num
  current_balance: num
}

entity Order {
  customer_id: num
  amount: num
}

constraint CreditLimit {
  customer.current_balance + order.amount <= customer.credit_limit
}
```

## AST Representation

The AST for a constraint example would look like:
```
ConstraintDef: PositivePrice
└── BinaryOp: product.price > 0
    ├── FieldAccess: product.price
    └── Literal: 0
```

## Execution Graph

The execution graph for constraints includes:
- Validation nodes
- Expression evaluation nodes
- Error handling nodes
- Dependency tracking for validation order

## Bytecode Snippet

The bytecode would include instructions for:
- Loading field values
- Comparing values
- Performing logical operations
- Handling validation failures
- Managing validation state

## Execution Trace Example

With a product having price=25:
1. Constraint expression evaluated: product.price > 0 (true)
2. Result: Constraint satisfied, product is valid

With a product having price=-5:
1. Constraint expression evaluated: product.price > 0 (false)
2. Result: Constraint violated, validation error

## Constraint Timing

Constraints can be evaluated at different times:

### 1. Entry Validation
```kern
constraint EntryValidation {
  // Validated before state transition
  input.value > 0
}
```

### 2. Exit Validation
```kern
constraint ExitValidation {
  // Validated after state transition
  output.result != ""
}
```

### 3. Invariant Validation
```kern
constraint Invariant {
  // Always maintained
  balance >= 0
}
```

## Constraint Composition

Multiple constraints can be combined:

```kern
constraint ComplexRule {
  (user.age >= 18 and user.has_license) or 
  (user.age >= 16 and user.has_permit and user.supervised)
}
```

## Error Handling with Constraints

Constraints work with error handling:

```kern
entity Transaction {
  amount: num
  account_balance: num
  status: sym
}

constraint SufficientFunds {
  transaction.account_balance >= transaction.amount
}

rule ProcessTransaction {
  if transaction.amount > 0 and constraint.SufficientFunds then {
    transaction.account_balance = transaction.account_balance - transaction.amount;
    transaction.status = "completed"
  }
  else
    transaction.status = "insufficient_funds"
}
```

## Constraints vs Rules

Constraints and rules serve different purposes:

### Constraints (Validation)
```kern
constraint PositiveBalance {
  account.balance >= 0
}
```
- Define what is valid
- Must be true for state to be valid
- Checked automatically
- Prevent invalid states

### Rules (Transformation)
```kern
rule CalculateInterest {
  if account.balance > 0 then
    account.balance = account.balance * 1.01
}
```
- Define what to do
- Transform state based on conditions
- Execute when conditions are met
- Change state

## Constraint Patterns

### 1. Range Validation Pattern
```kern
constraint ValidAgeRange {
  user.age >= 0 and user.age <= 150
}
```

### 2. Format Validation Pattern
```kern
constraint ValidEmailFormat {
  contains(user.email, "@") and contains(user.email, ".")
}
```

### 3. Relationship Validation Pattern
```kern
constraint ValidOrderCustomer {
  order.customer_id == customer.id
}
```

### 4. Business Rule Pattern
```kern
constraint BusinessHours {
  current_time >= 9 and current_time <= 17
}
```

## Constraint Interactions

Multiple constraints can interact:

```kern
entity Account {
  balance: num
  daily_limit: num
  daily_spent: num
}

constraint PositiveBalance {
  account.balance >= 0
}

constraint WithinDailyLimit {
  account.daily_spent <= account.daily_limit
}

constraint SufficientFunds {
  account.balance >= transaction.amount
}
```

## Performance Considerations

Constraints should be efficient:

```kern
// Efficient constraint
constraint SimpleCheck {
  value > 0
}

// Potentially expensive constraint
constraint ComplexCheck {
  expensive_calculation(value) == expected_result
}
```

## PSI Observation

The PSI system observes:
- Constraint definitions and their expressions
- Validation results and outcomes
- Error conditions when constraints are violated
- Constraint dependencies and relationships
- Validation timing and execution patterns

## Best Practices

1. Keep constraints simple and efficient
2. Use constraints for validation, not transformation
3. Ensure constraints are always testable
4. Use descriptive names for constraints
5. Group related constraints logically
6. Consider performance when writing complex constraints
7. Validate that constraints don't conflict with each other

## Summary

In this tutorial, we learned:
- How to define constraints with validation expressions
- How to create complex validation logic
- How to apply multiple constraints to entities
- How constraints differ from rules
- How to handle constraint violations
- Best practices for constraint design

In the next tutorial, we'll explore flows in detail, learning how to create execution pipelines and manage complex multi-step processes.