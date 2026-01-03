# KERN Language Reference - Syntax Grammar

## 3.1 Formal Grammar (EBNF)

The complete KERN grammar in Extended Backus-Naur Form:

```
program = { definition } ;

definition = entity_def | rule_def | flow_def | constraint_def ;

entity_def = 'entity' identifier '{' { field_def } '}' ;
field_def = identifier ':' type [ '=' expression ] ;
type = 'sym' | 'num' | 'bool' | 'vec' | 'ref' | 'ctx' ;

rule_def = 'rule' identifier '{' 'if' condition 'then' action '}' ;
condition = expression ;
action = expression | '{' { expression ';' } '}' ;

flow_def = 'flow' identifier '{' { flow_step } '}' ;
flow_step = expression [ '->' expression ] ;

constraint_def = 'constraint' identifier '{' expression '}' ;

expression = comparison [ logical_op comparison ]* ;
comparison = term [ comparison_op term ]* ;
term = factor [ arithmetic_op factor ]* ;
factor = primary | '(' expression ')' | 'not' factor ;
primary = identifier | literal | function_call | '{' expression '}' ;

literal = number | boolean | symbol ;
number = [ '-' ] digit+ ;
boolean = 'true' | 'false' ;
symbol = '"' character* '"' ;

function_call = identifier '(' [ expression { ',' expression } ] ')' ;

condition = expression ;
action = expression | assignment | sequence ;
assignment = identifier '=' expression ;
sequence = '{' { expression ';' } '}' ;

logical_op = 'and' | 'or' ;
arithmetic_op = '+' | '-' | '*' | '/' | '%' ;
comparison_op = '==' | '!=' | '<' | '<=' | '>' | '>=' ;

identifier = letter ( letter | digit | '_' | '-' )* ;
letter = 'a' .. 'z' | 'A' .. 'Z' ;
digit = '0' .. '9' ;
character = ? any Unicode character except " and \ ?
```

## 3.2 Grammar Notes

### 3.2.1 Ambiguity Resolution
- The grammar is designed to be unambiguous with no left recursion
- Operator precedence follows: arithmetic > comparison > logical
- Function calls bind tighter than operators

### 3.2.2 Deterministic Parsing
- All constructs are designed for deterministic parsing
- No context-sensitive rules
- LR(1) parseable grammar

### 3.2.3 Error Recovery
- The grammar supports error recovery at definition boundaries
- Each definition can be parsed independently
- Syntax errors in one definition don't affect others

## 3.3 Grammar Extensions

The core grammar may be extended with implementation-specific features, but the base grammar remains fixed and deterministic.

## 3.4 PSI Observability

The grammar is designed to be easily parsed and represented as an execution graph for PSI analysis. Each syntactic construct maps directly to a graph node type.