# KERN Language Reference - Entities

## 4.1 Definition

An entity defines a data structure with typed fields. Entities serve as the data model in KERN and form the basis for all other constructs.

## 4.2 Syntax

```
entity_def = 'entity' identifier '{' { field_def } '}' ;
field_def = identifier ':' type [ '=' expression ] ;
type = 'sym' | 'num' | 'bool' | 'vec' | 'ref' | 'ctx' ;
```

## 4.3 Semantics

Entities define:
- A named data structure
- Typed fields with optional default values
- The schema for data instances in the execution graph
- Constraints on data shape (through associated constraints)

## 4.4 Examples

### 4.4.1 Basic Entity
```
entity User {
  id: num
  name: sym
  active: bool
}
```

### 4.4.2 Entity with Default Values
```
entity Account {
  balance: num = 0
  account_type: sym = "checking"
  is_active: bool = true
}
```

## 4.5 Execution Guarantees

- Entity definitions are processed at compile time
- Field types are validated statically
- Default values are evaluated at instance creation
- No runtime type checking is needed after validation

## 4.6 Error Conditions

| Error Code | Condition | Recoverable |
|------------|-----------|-------------|
| SYN001 | Invalid identifier syntax | No |
| SYN002 | Missing field type | No |
| SEM001 | Duplicate field names | No |
| SEM002 | Invalid default value type | No |

## 4.7 Bytecode Mapping

Entity definitions are compiled to:
- Symbol table entries for the entity name
- Type information in the metadata section
- Validation code for instance creation
- No runtime bytecode for the definition itself

## 4.8 PSI Observability

Entities are observable as:
- Type definitions in the PSI knowledge base
- Schema information for data instances
- Validation rules for data integrity
- Graph node types in the execution graph