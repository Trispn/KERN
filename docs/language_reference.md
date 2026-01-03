# KERN Language Reference

## Formal Grammar Specification (EBNF)

This section defines the authoritative KERN grammar. All parsers, compilers, and PSI tooling must conform exactly to this grammar.

### Lexical Elements

```
letter = "A".."Z" | "a".."z" | "_" ;
digit = "0".."9" ;
identifier = letter , { letter | digit } ;
number = [ "-" ] , digit , { digit } ;
newline = "\n" ;
whitespace = " " | "\t" | newline ;
```

**Keywords** are reserved and cannot be used as identifiers:
- `entity`, `rule`, `flow`, `constraint`
- `if`, `then`, `else`, `and`, `or`
- `loop`, `break`, `halt`
- `true`, `false`

### Program Structure

```
program = { definition } ;
definition = entity_def | rule_def | flow_def | constraint_def ;
```

### Entity Definition

```
entity_def = "entity" , identifier , "{" , { field_def } , "}" ;
field_def = identifier ;
```

Entities define pure data structures. No methods, no behavior.

### Rule Definition

```
rule_def = "rule" , identifier , ":" , "if" , condition , "then" , action_list ;
```

Rules are declarative and side-effect explicit.

### Conditions

```
condition = expression | expression , logical_op , condition ;
logical_op = "and" | "or" ;
```

### Expressions

```
expression = term , comparator , term | predicate ;
comparator = "==" | "!=" | ">" | "<" | ">=" | "<=" ;
term = identifier | number | qualified_ref ;
qualified_ref = identifier , "." , identifier ;
```

### Predicates

```
predicate = identifier , "(" , [ argument_list ] , ")" ;
argument_list = term , { "," , term } ;
```

### Actions

```
action_list = action , { "," , action } ;
action = predicate | assignment | control_action ;
assignment = identifier , "=" , term ;
```

### Control Actions

```
control_action = if_action | loop_action | halt_action ;
if_action = "if" , condition , "then" , action_list , [ "else" , action_list ] ;
loop_action = "loop" , "{" , action_list , "}" ;
halt_action = "halt" ;
```

### Flow Definition

```
flow_def = "flow" , identifier , "{" , action_list , "}" ;
```

Flows define explicit execution pipelines.

### Constraint Definition

```
constraint_def = "constraint" , identifier , ":" , condition ;
```

Constraints validate state and must never mutate data.

## Core Language Constructs

### Entities

Entities define the data structures that rules operate on. They are pure data containers with no methods or behavior.

#### Syntax
```
entity <Identifier> {
    <field_name>
    <field_name>
    ...
}
```

#### Example
```kern
entity Farmer {
    id
    name
    location
    produce
    certification
}

entity Crop {
    id
    type
    season
    quality
    farmer_id
}
```

#### Rules
- Field names must be valid identifiers
- No methods or functions allowed
- No inheritance or relationships defined
- Fields are symbolic identifiers by default
- Entities are immutable data containers

### Rules

Rules are the core logic construct in KERN, following an explicit if/then structure.

#### Syntax
```
rule <Identifier>:
    if <condition>
    then <action_list>
```

#### Example
```kern
rule ValidateFarmer:
    if farmer.id != 0 and farmer.location != ""
    then mark_valid(farmer)

rule ApproveCertified:
    if farmer.certification == "organic" and farmer.id > 0
    then approve_farmer(farmer)

rule ProcessMultiple:
    if condition1() and condition2()
    then action1(), action2(), action3()
```

#### Rule Components
- **Name**: Unique identifier for the rule
- **Condition (Guard)**: Boolean expression that determines if the rule fires
- **Action**: One or more operations executed when the condition is true

#### Condition Expressions
Conditions can include:
- Comparison operations: `==`, `!=`, `>`, `<`, `>=`, `<=`
- Logical operations: `and`, `or`
- Function calls that return boolean values
- Field access: `entity.field`

#### Actions
Actions can be:
- Function calls: `function_name(arguments)`
- Assignments: `variable = value`
- Control flow: `if`, `loop`, `halt`

### Flows

Flows define the explicit execution order of rules and actions.

#### Syntax
```
flow <Identifier> {
    <action>
    <action>
    ...
}
```

#### Example
```kern
flow FarmerApprovalProcess {
    ValidateFarmer
    ApproveCertified
    GenerateReport
}

flow ComplexFlow {
    ValidateFarmer
    if farmer.valid == true then ProcessApproved()
    GenerateReport
}
```

#### Rules
- Flows execute actions in the specified order
- Actions can be rule names or direct function calls
- Control flow constructs are allowed within flows
- No implicit parallelization - execution is sequential unless explicitly specified

### Constraints

Constraints provide validation logic that must be satisfied.

#### Syntax
```
constraint <Identifier>: <condition>
```

#### Example
```kern
constraint ValidId: farmer.id > 0
constraint ValidLocation: farmer.location != ""
constraint UniqueId: count_matching(farmers, id) == 1
```

#### Rules
- Constraints must evaluate to boolean values
- Constraints never mutate data
- Constraints are used for validation and verification
- Multiple constraints can apply to the same data

## Data Types and Values

### Primitive Types

KERN supports the following primitive types:

#### Symbol (sym)
- Represents symbolic identifiers
- Used for entity fields and variable names
- Immutable by default
- Example: `farmer`, `id`, `location`

#### Number (num)
- Integer values (no floats by default)
- Signed integers: positive, negative, and zero
- Example: `42`, `-5`, `0`

#### Boolean (bool)
- Two values: `true` and `false`
- Used in conditions and logical operations
- Example: `true`, `false`

#### Vector (vec)
- Small fixed-size collections
- Homogeneous elements of the same type
- Example: `[1, 2, 3]`, `["a", "b", "c"]`

#### Reference (ref)
- External references to data or functions
- Used for integration with external systems
- Example: `@database.users`, `@api.get_data`

#### Context (ctx)
- Execution context containing state
- Isolated execution environments
- Example: `ctx.current_user`, `ctx.session`

### Type System Rules
- No implicit type conversion
- Type checking occurs at compile time
- Variables must be declared before use
- Type safety is enforced by the compiler

## Expressions and Operators

### Comparison Operators
- `==` : Equal to
- `!=` : Not equal to
- `>` : Greater than
- `<` : Less than
- `>=` : Greater than or equal to
- `<=` : Less than or equal to

### Logical Operators
- `and` : Logical AND (short-circuit evaluation)
- `or` : Logical OR (short-circuit evaluation)

### Arithmetic Operators
Currently not supported as KERN focuses on logic rather than computation. Arithmetic operations should be performed in external functions.

### Operator Precedence
1. Parentheses `()`
2. Comparison operators
3. `and`
4. `or`

## Control Flow

### If-Then-Else
Conditional execution based on boolean conditions:

```
if condition then action
if condition then action else alternative_action
```

#### Example
```kern
rule ConditionalAction:
    if farmer.certification == "organic"
    then approve_farmer(farmer)
    else flag_for_review(farmer)
```

### Loops
Iterative execution with explicit bounds:

```
loop { action_list }
```

#### Example
```kern
rule ProcessAllFarmers:
    if has_more_farmers()
    then loop { 
        process_next_farmer(), 
        check_termination_condition() 
    }
```

### Halt
Immediate termination of execution:

```
halt
```

#### Example
```kern
rule EmergencyStop:
    if system_error_detected()
    then halt
```

## Built-in Functions

KERN provides several built-in functions for common operations:

### Validation Functions
- `count_matching(collection, condition)` - Count items matching a condition
- `exists_in(collection, item)` - Check if item exists in collection
- `all_match(collection, condition)` - Check if all items match condition
- `any_match(collection, condition)` - Check if any item matches condition

### Utility Functions
- `current_time()` - Get current timestamp
- `generate_id()` - Generate a unique identifier
- `log(message)` - Log a message for debugging
- `print(message)` - Output a message

### Collection Functions
- `filter(collection, condition)` - Filter collection by condition
- `map(collection, function)` - Transform collection with function
- `reduce(collection, function, initial)` - Reduce collection to single value

## Error Handling

KERN uses a data-based error handling model rather than exceptions:

### Error Representation
Errors are represented as data structures rather than control flow:

```kern
entity Result {
    success
    value
    error_message
}

rule SafeOperation:
    if operation_result.success == true
    then handle_success(operation_result.value)
    else handle_error(operation_result.error_message)
```

### Validation with Constraints
Use constraints to prevent invalid states:

```kern
constraint NoErrors: result.success == true
constraint ValidId: farmer.id > 0
```

## Comments and Documentation

KERN supports single-line comments:

```
# This is a comment
rule Example:  # Inline comment
    if condition  # Another comment
    then action
```

## Naming Conventions

### Entities and Rules
- Use PascalCase: `Farmer`, `ValidateFarmer`, `FarmerApprovalProcess`

### Fields and Variables
- Use camelCase: `farmerId`, `location`, `certification`

### Constants
- Use SCREAMING_SNAKE_CASE: `MAX_FARMERS`, `DEFAULT_LOCATION`

## Scoping and Visibility

### Global Scope
- All entities, rules, flows, and constraints are globally visible
- Names must be unique across the entire program
- No nested scoping within constructs

### Context Scope
- Variables within contexts are isolated
- Contexts provide execution isolation
- Data sharing between contexts must be explicit

## Compilation and Validation Rules

### Static Validation
All KERN programs must pass static validation:

1. **Syntax Validation**: Must conform to the formal grammar
2. **Semantic Validation**: All symbols must be defined
3. **Type Validation**: All operations must be type-safe
4. **Constraint Validation**: All constraints must be satisfied
5. **Flow Validation**: All flows must have valid execution paths

### Determinism Requirements
- Same input must always produce same output
- No randomness or hidden state
- All control flow must be explicit
- No implicit operations or side effects

### Minimalism Requirements
- Every feature must justify its byte cost
- No syntactic sugar without clear benefit
- Prefer explicit over implicit behavior
- Maintain small instruction set

## Examples of Complete Programs

### Simple Validation Program
```kern
entity User {
    id
    name
    email
    age
}

rule ValidateUser:
    if user.id > 0 and user.email != "" and user.age >= 18
    then mark_valid(user)

constraint ValidId: user.id > 0
constraint ValidEmail: user.email != ""
constraint ValidAge: user.age >= 18

flow ValidateUserFlow {
    ValidateUser
}
```

### Complex Business Logic
```kern
entity Order {
    id
    customer_id
    items
    total
    status
}

entity Customer {
    id
    credit_limit
    account_status
}

rule CheckCredit:
    if order.total <= customer.credit_limit and customer.account_status == "active"
    then approve_order(order)

rule FlagLargeOrder:
    if order.total > 10000
    then flag_for_review(order)

rule UpdateStatus:
    if order.approved == true
    then set_status(order, "approved")
    else set_status(order, "pending_review")

constraint ValidOrder: order.total > 0
constraint ValidCustomer: customer.id > 0

flow ProcessOrder {
    CheckCredit
    FlagLargeOrder
    UpdateStatus
}
```

This language reference provides a comprehensive specification of KERN's syntax, semantics, and built-in constructs. The formal grammar ensures deterministic parsing, while the design principles maintain the language's focus on determinism, minimalism, and explicit logic.