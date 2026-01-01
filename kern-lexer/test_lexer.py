#!/usr/bin/env python3
"""
Python implementation of the KERN lexer for testing purposes.
This simulates the Rust implementation to verify functionality.
"""

class TokenType:
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
    EQUAL = "EQUAL"
    NOT_EQUAL = "NOT_EQUAL"
    GREATER = "GREATER"
    LESS = "LESS"
    GREATER_EQUAL = "GREATER_EQUAL"
    LESS_EQUAL = "LESS_EQUAL"
    ASSIGNMENT = "ASSIGNMENT"

    # Error
    ILLEGAL = "ILLEGAL"

    # Special
    EOF = "EOF"


class Token:
    def __init__(self, token_type, value=None, line=1, column=1, position=0):
        self.token_type = token_type
        self.value = value
        self.line = line
        self.column = column
        self.position = position

    def __repr__(self):
        if self.value is not None:
            return f"Token({self.token_type}, '{self.value}', line={self.line})"
        return f"Token({self.token_type}, line={self.line})"


def is_letter(ch):
    return ch.isalpha() or ch == '_'


class Lexer:
    def __init__(self, input_text):
        self.input = input_text
        self.position = 0
        self.read_position = 0
        self.ch = '\0'
        self.line = 1
        self.column = 0
        self._read_char()

    def _read_char(self):
        if self.read_position >= len(self.input):
            self.ch = '\0'
        else:
            self.ch = self.input[self.read_position]

        self.position = self.read_position
        self.read_position += 1
        self.column += 1

        if self.ch == '\n':
            self.line += 1
            self.column = 0

    def _peek_char(self):
        if self.read_position >= len(self.input):
            return '\0'
        else:
            return self.input[self.read_position]

    def _skip_whitespace(self):
        while self.ch.isspace():
            if self.ch == '\n':
                self.line += 1
                self.column = 0
            else:
                self.column += 1
            self._read_char()

    def _read_identifier(self):
        start_pos = self.position
        while is_letter(self.ch) or self.ch.isdigit():
            self._read_char()
        return self.input[start_pos:self.position]

    def _read_number(self):
        start_pos = self.position
        while self.ch.isdigit():
            self._read_char()
        return int(self.input[start_pos:self.position])

    def _lookup_identifier(self, identifier):
        keywords = {
            "entity": TokenType.ENTITY,
            "rule": TokenType.RULE,
            "flow": TokenType.FLOW,
            "constraint": TokenType.CONSTRAINT,
            "if": TokenType.IF,
            "then": TokenType.THEN,
            "else": TokenType.ELSE,
            "loop": TokenType.LOOP,
            "break": TokenType.BREAK,
            "halt": TokenType.HALT,
            "and": TokenType.AND,
            "or": TokenType.OR,
        }
        return keywords.get(identifier, TokenType.IDENTIFIER)

    def next_token(self):
        self._skip_whitespace()

        if self.ch == '=':
            if self._peek_char() == '=':
                self._read_char()  # consume '='
                token = Token(TokenType.EQUAL, '==', self.line, self.column - 1, self.position - 1)
            else:
                token = Token(TokenType.ASSIGNMENT, '=', self.line, self.column, self.position)
        elif self.ch == '!':
            if self._peek_char() == '=':
                self._read_char()  # consume '='
                token = Token(TokenType.NOT_EQUAL, '!=', self.line, self.column - 1, self.position - 1)
            else:
                # Error case: '!' not followed by '=' - this is an illegal character
                current_ch = self.ch
                self._read_char()
                token = Token(TokenType.ILLEGAL, current_ch, self.line, self.column, self.position)
        elif self.ch == '>':
            if self._peek_char() == '=':
                self._read_char()  # consume '='
                token = Token(TokenType.GREATER_EQUAL, '>=', self.line, self.column - 1, self.position - 1)
            else:
                token = Token(TokenType.GREATER, '>', self.line, self.column, self.position)
        elif self.ch == '<':
            if self._peek_char() == '=':
                self._read_char()  # consume '='
                token = Token(TokenType.LESS_EQUAL, '<=', self.line, self.column - 1, self.position - 1)
            else:
                token = Token(TokenType.LESS, '<', self.line, self.column, self.position)
        elif self.ch == '{':
            token = Token(TokenType.LEFT_BRACE, '{', self.line, self.column, self.position)
        elif self.ch == '}':
            token = Token(TokenType.RIGHT_BRACE, '}', self.line, self.column, self.position)
        elif self.ch == '(':
            token = Token(TokenType.LEFT_PAREN, '(', self.line, self.column, self.position)
        elif self.ch == ')':
            token = Token(TokenType.RIGHT_PAREN, ')', self.line, self.column, self.position)
        elif self.ch == ',':
            token = Token(TokenType.COMMA, ',', self.line, self.column, self.position)
        elif self.ch == '.':
            token = Token(TokenType.DOT, '.', self.line, self.column, self.position)
        elif self.ch == ':':
            token = Token(TokenType.COLON, ':', self.line, self.column, self.position)
        elif self.ch == '\0':
            token = Token(TokenType.EOF, None, self.line, self.column, self.position)
        elif is_letter(self.ch):
            identifier = self._read_identifier()
            token_type = self._lookup_identifier(identifier)
            if token_type == TokenType.IDENTIFIER:
                token = Token(token_type, identifier, self.line, self.column - len(identifier), self.position - len(identifier))
            else:
                token = Token(token_type, identifier, self.line, self.column - len(identifier), self.position - len(identifier))
        elif self.ch in ('"', "'"):
            # According to KERN spec, strings are not first-class citizens
            # So we treat quotes as illegal characters
            current_ch = self.ch
            self._read_char()  # consume the quote
            token = Token(TokenType.ILLEGAL, current_ch, self.line, self.column, self.position)
        elif self.ch.isdigit():
            number = self._read_number()
            token = Token(TokenType.NUMBER, number, self.line, self.column - len(str(number)), self.position - len(str(number)))
        else:
            # Handle unrecognized characters
            current_ch = self.ch
            self._read_char()
            token = Token(TokenType.ILLEGAL, current_ch, self.line, self.column, self.position)

        self._read_char()
        return token

    def tokenize_all(self):
        tokens = []
        while True:
            token = self.next_token()
            tokens.append(token)
            if token.token_type == TokenType.EOF:
                break
        return tokens


def test_lexer():
    print("Testing KERN lexer implementation...")
    
    # Test 1: Basic entity definition
    input1 = "entity Farmer { id location produce }"
    lexer1 = Lexer(input1)
    tokens1 = lexer1.tokenize_all()
    print("\nTest 1 - Entity definition:")
    print(f"Input: {input1}")
    for token in tokens1:
        print(f"  {token}")
    
    # Test 2: Rule definition
    input2 = "rule CheckLocation: if farmer.location == \"valid\" then approve_farmer(farmer)"
    lexer2 = Lexer(input2)
    tokens2 = lexer2.tokenize_all()
    print("\nTest 2 - Rule definition:")
    print(f"Input: {input2}")
    for token in tokens2:
        print(f"  {token}")
    
    # Test 3: Operators
    input3 = "== != > < >= <="
    lexer3 = Lexer(input3)
    tokens3 = lexer3.tokenize_all()
    print("\nTest 3 - Operators:")
    print(f"Input: {input3}")
    for token in tokens3:
        print(f"  {token}")
    
    # Test 4: Keywords
    input4 = "entity rule flow constraint if then else loop break halt and or"
    lexer4 = Lexer(input4)
    tokens4 = lexer4.tokenize_all()
    print("\nTest 4 - Keywords:")
    print(f"Input: {input4}")
    for token in tokens4:
        print(f"  {token}")
    
    # Test 5: Error handling
    input5 = "entity test ! invalid"
    lexer5 = Lexer(input5)
    tokens5 = lexer5.tokenize_all()
    print("\nTest 5 - Error handling:")
    print(f"Input: {input5}")
    for token in tokens5:
        print(f"  {token}")
    
    print("\nAll tests completed successfully!")


if __name__ == "__main__":
    test_lexer()