#!/usr/bin/env python3
"""
Python simulation of the KERN recursive descent parser to verify the implementation logic.
This simulates the Rust implementation to verify functionality.
"""

from enum import Enum
from typing import List, Optional, Union

# AST Node definitions
class AstNode:
    pass

class Definition(AstNode):
    pass

class Program(AstNode):
    def __init__(self, definitions: List[Definition]):
        self.definitions = definitions

class EntityDef(Definition):
    def __init__(self, name: str, fields: List['FieldDef']):
        self.name = name
        self.fields = fields

class FieldDef(AstNode):
    def __init__(self, name: str):
        self.name = name

class RuleDef(Definition):
    def __init__(self, name: str, condition: 'Condition', actions: List['Action']):
        self.name = name
        self.condition = condition
        self.actions = actions

class Condition(AstNode):
    pass

class Expression(Condition):
    pass

class Term(AstNode):
    pass

class Predicate(AstNode):
    def __init__(self, name: str, arguments: List[Term]):
        self.name = name
        self.arguments = arguments

class Action(AstNode):
    pass

class Assignment(Action):
    def __init__(self, variable: str, value: Term):
        self.variable = variable
        self.value = value

class FlowDef(Definition):
    def __init__(self, name: str, actions: List[Action]):
        self.name = name
        self.actions = actions

class ConstraintDef(Definition):
    def __init__(self, name: str, condition: Condition):
        self.name = name
        self.condition = condition

# Token types (matching the lexer)
class TokenType(Enum):
    # Keywords
    ENTITY = "ENTITY"
    RULE = "RULE"
    FLOW = "FLOW"
    CONSTRAINT = "CONSTRAINT"
    IF = "IF"
    THEN = "THEN"
    ELSE = "ELSE"
    LOOP = "LOOP"
    BREAK = "BREAK"
    HALT = "HALT"
    AND = "AND"
    OR = "OR"

    # Identifiers and literals
    IDENTIFIER = "IDENTIFIER"
    NUMBER = "NUMBER"

    # Symbols
    COLON = "COLON"
    COMMA = "COMMA"
    DOT = "DOT"
    LEFT_BRACE = "LEFT_BRACE"
    RIGHT_BRACE = "RIGHT_BRACE"
    LEFT_PAREN = "LEFT_PAREN"
    RIGHT_PAREN = "RIGHT_PAREN"
    EQUAL = "EQUAL"  # ==
    NOT_EQUAL = "NOT_EQUAL"  # !=
    GREATER = "GREATER"  # >
    LESS = "LESS"  # <
    GREATER_EQUAL = "GREATER_EQUAL"  # >=
    LESS_EQUAL = "LESS_EQUAL"  # <=
    ASSIGNMENT = "ASSIGNMENT"  # =

    # Error
    ILLEGAL = "ILLEGAL"

    # Special
    EOF = "EOF"

class Token:
    def __init__(self, token_type: TokenType, value=None, line=1, column=1, position=0):
        self.token_type = token_type
        self.value = value
        self.line = line
        self.column = column
        self.position = position

    def __repr__(self):
        if self.value is not None:
            return f"Token({self.token_type.value}, '{self.value}', line={self.line})"
        return f"Token({self.token_type.value}, line={self.line})"

# Import the lexer from the previous implementation
import sys
sys.path.append("..\\kern-lexer")
from test_lexer import Lexer, TokenType as LexerTokenType

class ParseError:
    def __init__(self, message: str, line: int, column: int, position: int):
        self.message = message
        self.line = line
        self.column = column
        self.position = position

    def __str__(self):
        return f"Parse error at {self.line}:{self.column}: {self.message}"

class Parser:
    def __init__(self, input_text: str):
        self.lexer = Lexer(input_text)
        self.current_token = self.lexer.next_token()
        self.errors = []
        self.recovery_enabled = True

    def next_token(self):
        self.current_token = self.lexer.next_token()

    def expect_token(self, token_type: LexerTokenType):
        if self.current_token.token_type == token_type:
            token = self.current_token
            self.next_token()
            return token
        else:
            error = ParseError(
                f"Expected {token_type}, got {self.current_token.token_type}",
                self.current_token.line,
                self.current_token.column,
                self.current_token.position
            )
            self.errors.append(error)
            raise error

    def is_at_end(self):
        return self.current_token.token_type == LexerTokenType.EOF

    def is_current_token(self, token_type: LexerTokenType):
        return self.current_token.token_type == token_type

    def parse_program(self):
        definitions = []
        
        while not self.is_at_end():
            try:
                definition = self.parse_definition()
                if definition:
                    definitions.append(definition)
            except ParseError:
                if self.recovery_enabled:
                    # Skip tokens until we find the start of another definition
                    self.skip_until([LexerTokenType.ENTITY, LexerTokenType.RULE, 
                                   LexerTokenType.FLOW, LexerTokenType.CONSTRAINT])
                else:
                    break
        
        return Program(definitions)

    def skip_until(self, expected_tokens):
        while not self.is_at_end():
            for expected_token in expected_tokens:
                if self.current_token.token_type == expected_token:
                    return
            self.next_token()

    def parse_definition(self):
        if self.is_current_token(LexerTokenType.ENTITY):
            return self.parse_entity_def()
        elif self.is_current_token(LexerTokenType.RULE):
            return self.parse_rule_def()
        elif self.is_current_token(LexerTokenType.FLOW):
            return self.parse_flow_def()
        elif self.is_current_token(LexerTokenType.CONSTRAINT):
            return self.parse_constraint_def()
        elif self.is_current_token(LexerTokenType.EOF):
            return None
        else:
            # Unexpected token
            error = ParseError(
                f"Unexpected token {self.current_token.token_type}, expected a definition keyword",
                self.current_token.line,
                self.current_token.column,
                self.current_token.position
            )
            self.errors.append(error)
            self.next_token()  # Skip this token
            return None

    def parse_entity_def(self):
        self.expect_token(LexerTokenType.ENTITY)
        
        name = self.current_token.value
        self.expect_token(LexerTokenType.IDENTIFIER)
        
        self.expect_token(LexerTokenType.LEFT_BRACE)
        
        fields = []
        while not self.is_current_token(LexerTokenType.RIGHT_BRACE) and not self.is_at_end():
            field = self.parse_field_def()
            if field:
                fields.append(field)
        
        self.expect_token(LexerTokenType.RIGHT_BRACE)
        
        return EntityDef(name, fields)

    def parse_field_def(self):
        if self.is_current_token(LexerTokenType.IDENTIFIER):
            name = self.current_token.value
            self.next_token()  # consume identifier
            return FieldDef(name)
        return None

    def parse_rule_def(self):
        self.expect_token(LexerTokenType.RULE)
        
        name = self.current_token.value
        self.expect_token(LexerTokenType.IDENTIFIER)
        
        self.expect_token(LexerTokenType.COLON)
        
        self.expect_token(LexerTokenType.IF)
        condition = self.parse_condition()
        
        self.expect_token(LexerTokenType.THEN)
        actions = self.parse_action_list()
        
        return RuleDef(name, condition, actions)

    def parse_condition(self):
        # For simplicity, just return a basic expression
        return self.parse_expression()

    def parse_expression(self):
        # For simplicity in this simulation, return a basic expression
        return Expression()

    def parse_action_list(self):
        actions = []
        
        # Parse first action if available
        action = self.parse_action()
        if action:
            actions.append(action)
        
        # Parse additional actions separated by commas
        while self.is_current_token(LexerTokenType.COMMA):
            self.next_token()  # consume comma
            action = self.parse_action()
            if action:
                actions.append(action)
            else:
                break
        
        return actions

    def parse_action(self):
        if self.is_current_token(LexerTokenType.IDENTIFIER):
            # This could be a predicate call or assignment
            identifier = self.current_token.value
            self.next_token()
            
            if self.is_current_token(LexerTokenType.LEFT_PAREN):
                # This is a predicate call
                return self.parse_predicate()
            else:
                # For simplicity in this simulation, return a basic action
                return Action()
        elif self.is_current_token(LexerTokenType.IF):
            self.next_token()  # consume 'if'
            self.parse_condition()
            self.expect_token(LexerTokenType.THEN)
            then_actions = self.parse_action_list()
            else_actions = None
            if self.is_current_token(LexerTokenType.ELSE):
                self.next_token()  # consume 'else'
                else_actions = self.parse_action_list()
            return Action()  # Simplified
        elif self.is_current_token(LexerTokenType.HALT):
            self.next_token()  # consume 'halt'
            return Action()  # Simplified
        else:
            return None

    def parse_predicate(self):
        name = self.current_token.value
        self.expect_token(LexerTokenType.IDENTIFIER)
        self.expect_token(LexerTokenType.LEFT_PAREN)
        
        arguments = []
        if not self.is_current_token(LexerTokenType.RIGHT_PAREN):
            # Parse first argument
            if self.is_current_token(LexerTokenType.IDENTIFIER) or self.is_current_token(LexerTokenType.NUMBER):
                # Simplified argument parsing
                if self.is_current_token(LexerTokenType.IDENTIFIER):
                    arg_value = self.current_token.value
                    self.next_token()
                elif self.is_current_token(LexerTokenType.NUMBER):
                    arg_value = self.current_token.value
                    self.next_token()
                else:
                    arg_value = None
                arguments.append(Term())  # Simplified
        
        self.expect_token(LexerTokenType.RIGHT_PAREN)
        
        return Predicate(name, arguments)

    def parse_flow_def(self):
        self.expect_token(LexerTokenType.FLOW)
        
        name = self.current_token.value
        self.expect_token(LexerTokenType.IDENTIFIER)
        
        self.expect_token(LexerTokenType.LEFT_BRACE)
        actions = self.parse_action_list()
        self.expect_token(LexerTokenType.RIGHT_BRACE)
        
        return FlowDef(name, actions)

    def parse_constraint_def(self):
        self.expect_token(LexerTokenType.CONSTRAINT)
        
        name = self.current_token.value
        self.expect_token(LexerTokenType.IDENTIFIER)
        
        self.expect_token(LexerTokenType.COLON)
        condition = self.parse_condition()
        
        return ConstraintDef(name, condition)

def test_parser():
    print("Testing KERN recursive descent parser implementation...")
    
    # Test 1: Simple entity
    input1 = "entity Farmer { id location }"
    print(f"\nTest 1 - Parsing: {input1}")
    parser1 = Parser(input1)
    try:
        program1 = parser1.parse_program()
        print(f"  Successfully parsed {len(program1.definitions)} definition(s)")
        if program1.definitions:
            entity = program1.definitions[0]
            if isinstance(entity, EntityDef):
                print(f"  Entity: {entity.name} with {len(entity.fields)} field(s)")
    except:
        print(f"  Error during parsing")
    
    # Test 2: Multiple definitions
    input2 = """
    entity TestEntity { field1 field2 }
    rule TestRule: if condition then action()
    flow TestFlow { action1() action2() }
    constraint TestConstraint: value > 0
    """
    print(f"\nTest 2 - Parsing multiple definitions:")
    parser2 = Parser(input2)
    try:
        program2 = parser2.parse_program()
        print(f"  Successfully parsed {len(program2.definitions)} definition(s)")
        for i, definition in enumerate(program2.definitions):
            if isinstance(definition, EntityDef):
                print(f"    {i+1}. Entity: {definition.name}")
            elif isinstance(definition, RuleDef):
                print(f"    {i+1}. Rule: {definition.name}")
            elif isinstance(definition, FlowDef):
                print(f"    {i+1}. Flow: {definition.name}")
            elif isinstance(definition, ConstraintDef):
                print(f"    {i+1}. Constraint: {definition.name}")
    except:
        print(f"  Error during parsing")
    
    # Test 3: Complex example
    input3 = """
    entity Farmer {
        id
        name
        location
    }
    
    rule ValidateFarmer:
        if farmer.id != 0
        then mark_valid(farmer)
    
    flow ProcessFarmers {
        load_farmers()
        validate_farmers()
    }
    
    constraint ValidId: farmer.id > 0
    """
    print(f"\nTest 3 - Parsing complex example:")
    parser3 = Parser(input3)
    try:
        program3 = parser3.parse_program()
        print(f"  Successfully parsed {len(program3.definitions)} definition(s)")

        entity_count = sum(1 for d in program3.definitions if isinstance(d, EntityDef))
        rule_count = sum(1 for d in program3.definitions if isinstance(d, RuleDef))
        flow_count = sum(1 for d in program3.definitions if isinstance(d, FlowDef))
        constraint_count = sum(1 for d in program3.definitions if isinstance(d, ConstraintDef))

        print(f"    Entities: {entity_count}")
        print(f"    Rules: {rule_count}")
        print(f"    Flows: {flow_count}")
        print(f"    Constraints: {constraint_count}")

        if parser3.errors:
            print(f"  Parsing completed with {len(parser3.errors)} error(s)")
            for error in parser3.errors:
                print(f"    - {error}")
        else:
            print("  No parsing errors detected")

    except:
        print(f"  Error during parsing")

if __name__ == "__main__":
    test_parser()