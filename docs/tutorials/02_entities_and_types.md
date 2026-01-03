# KERN Tutorial 02: Entities and Types

## Overview

This tutorial explores KERN's entity system and type system in detail. We'll learn how to define complex data structures and work with different types.

## Entity Basics

In KERN, entities define the data model. Let's start with a simple example:

```kern
entity Person {
  id: num
  name: sym
  age: num
  is_adult: bool
}
```

This defines a `Person` entity with four fields of different types.

## Type System Deep Dive

KERN supports six primitive types:

### 1. sym (Symbol)
Used for named references and string-like values:
```kern
entity Account {
  username: sym = "guest"  // Default value
  status: sym = "active"
}
```

### 2. num (Number)
Used for integer values:
```kern
entity Product {
  price: num = 0
  quantity: num = 10
  discount: num = 0
}
```

### 3. bool (Boolean)
Used for true/false values:
```kern
entity Permission {
  read: bool = true
  write: bool = false
  execute: bool = false
}
```

### 4. vec (Vector)
Used for small fixed-size arrays:
```kern
entity ShoppingCart {
  items: vec
  is_empty: bool
}
```

### 5. ref (Reference)
Used for external references:
```kern
entity Document {
  content: ref
  owner: ref
}
```

### 6. ctx (Context)
Used for execution contexts:
```kern
entity Process {
  context: ctx
  status: sym
}
```

## Complex Entity Relationships

Entities can reference other entities:

```kern
entity Address {
  street: sym
  city: sym
  zip_code: num
}

entity Customer {
  id: num
  name: sym
  address: ref  // Reference to Address entity
  active: bool
}
```

## Default Values

Entities can have default values:

```kern
entity Settings {
  theme: sym = "light"
  notifications: bool = true
  max_items: num = 100
  tags: vec = []
}
```

## Rules with Entities

Rules can operate on entities to transform their state:

```kern
rule ActivateCustomer {
  if customer.id > 0 and customer.name != "" then
    customer.active = true
}

rule CalculateAdult {
  if person.age >= 18 then
    person.is_adult = true
  else
    person.is_adult = false
}

rule ValidateProduct {
  if product.price > 0 and product.quantity >= 0 then {
    product.status = "available";
    product.in_stock = true
  }
  else {
    product.status = "unavailable";
    product.in_stock = false
  }
}
```

## Constraints on Entities

Constraints ensure data integrity:

```kern
constraint ValidAge {
  person.age >= 0 and person.age <= 150
}

constraint PositivePrice {
  product.price > 0
}

constraint NonEmptyName {
  customer.name != ""
}
```

## AST Representation

The AST for a complex entity example would look like:
```
EntityDef: Customer
├── Field: id (num)
├── Field: name (sym)
├── Field: address (ref)
└── Field: active (bool)
RuleDef: ActivateCustomer
├── Condition: customer.id > 0 and customer.name != ""
└── Action: customer.active = true
ConstraintDef: NonEmptyName
└── Expression: customer.name != ""
```

## Execution Graph

The execution graph would include:
- Entity definition nodes
- Rule condition and action nodes
- Constraint validation nodes
- Data flow edges between related operations

## Bytecode Snippet

The bytecode would include instructions for:
- Creating entity instances
- Loading field values
- Comparing values
- Setting field values
- Validating constraints

## Execution Trace Example

With a customer having id=1 and name="John":
1. Rule condition evaluated: customer.id > 0 and customer.name != "" (true)
2. Rule action executed: customer.active = true
3. Constraint validated: customer.name != "" (true)
4. Result: Customer is active with valid name

## Advanced Entity Features

### Entity Inheritance (Conceptual)
While KERN doesn't have traditional inheritance, you can achieve similar results:

```kern
entity BaseUser {
  id: num
  created_at: num
  active: bool
}

entity AdminUser {
  id: num
  created_at: num
  active: bool
  permissions: vec
  admin_level: num
}
```

### Entity Composition

Combine entities through references:

```kern
entity Order {
  id: num
  customer: ref    // Reference to Customer entity
  items: vec       // Vector of references to Product entities
  total: num
  status: sym
}

entity OrderItem {
  product: ref     // Reference to Product entity
  quantity: num
  price: num
}
```

## Best Practices

1. Use descriptive field names
2. Set appropriate default values
3. Validate entities with constraints
4. Keep entities focused on a single responsibility
5. Use references for relationships between entities

## PSI Observation

The PSI system observes:
- Entity schemas and their relationships
- Type information for each field
- Default values and their implications
- Rule logic operating on entities
- Constraint validation patterns

## Summary

In this tutorial, we learned:
- How to define entities with different field types
- How to use default values
- How to create rules that operate on entities
- How to validate entities with constraints
- How entities relate to each other

In the next tutorial, we'll explore rules in more depth, learning how to create complex conditional logic and inference patterns.