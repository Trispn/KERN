use crate::ast::*;
use kern_lexer::{Lexer, Token, TokenType};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub position: usize,
}

// Enhanced error reporting with more specific error types
#[derive(Debug, Clone)]
pub enum ParseErrorType {
    UnexpectedToken { expected: String, actual: String },
    MissingToken { expected: String },
    InvalidSyntax { context: String },
    MismatchedDelimiters { opening: String, closing: String },
    InvalidIdentifier { context: String },
    InvalidNumber { context: String },
    InvalidExpression { context: String },
    InvalidCondition { context: String },
    InvalidAction { context: String },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parse error at {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

impl ParseError {
    pub fn unexpected_token(
        expected: &str,
        actual: &str,
        line: usize,
        column: usize,
        position: usize,
    ) -> Self {
        ParseError {
            message: format!("Expected {}, got {}", expected, actual),
            line,
            column,
            position,
        }
    }

    pub fn missing_token(expected: &str, line: usize, column: usize, position: usize) -> Self {
        ParseError {
            message: format!("Missing token: expected {}", expected),
            line,
            column,
            position,
        }
    }

    pub fn invalid_syntax(context: &str, line: usize, column: usize, position: usize) -> Self {
        ParseError {
            message: format!("Invalid syntax in: {}", context),
            line,
            column,
            position,
        }
    }

    pub fn mismatched_delimiters(
        opening: &str,
        closing: &str,
        line: usize,
        column: usize,
        position: usize,
    ) -> Self {
        ParseError {
            message: format!(
                "Mismatched delimiters: expected '{}' to close '{}'",
                closing, opening
            ),
            line,
            column,
            position,
        }
    }
}

impl std::error::Error for ParseError {}

impl From<ParseError> for Vec<ParseError> {
    fn from(error: ParseError) -> Self {
        vec![error]
    }
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    errors: Vec<ParseError>,
    pub recovery_enabled: bool, // Flag to enable/disable error recovery
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            errors: Vec::new(),
            recovery_enabled: true,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn peek_token(&mut self) -> Token {
        self.lexer.peek_next_token()
    }

    fn expect_token(&mut self, token_type: TokenType) -> Result<Token, ParseError> {
        if self.current_token.token_type == token_type {
            let token = self.current_token.clone();
            self.next_token();
            Ok(token)
        } else {
            let expected_str = format!("{:?}", token_type);
            let actual_str = format!("{:?}", self.current_token.token_type);
            let error = ParseError {
                message: format!("Expected {}, got {}", expected_str, actual_str),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            };
            self.errors.push(error.clone());
            Err(error)
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current_token.token_type, TokenType::Eof)
    }

    fn is_current_token(&self, token_type: &TokenType) -> bool {
        self.current_token.token_type == *token_type
    }

    fn is_peek_token(&mut self, token_type: &TokenType) -> bool {
        let peek = self.peek_token();
        peek.token_type == *token_type
    }

    // Skip tokens until we find one of the expected tokens or reach EOF
    fn skip_until(&mut self, expected: &[TokenType]) -> bool {
        while !self.is_at_end() {
            for expected_token in expected {
                if &self.current_token.token_type == expected_token {
                    return true;
                }
            }
            self.next_token();
        }
        false
    }

    pub fn parse_program(&mut self) -> Result<Program, Vec<ParseError>> {
        let mut definitions = Vec::new();

        while !self.is_at_end() {
            if let Ok(Some(definition)) = self.parse_definition() {
                definitions.push(definition);
            } else if self.recovery_enabled {
                // If we couldn't parse a definition, skip tokens until we find the start of another definition
                self.skip_until(&[
                    TokenType::Entity,
                    TokenType::Rule,
                    TokenType::Flow,
                    TokenType::Constraint,
                ]);
            } else {
                // If recovery is disabled, return with errors
                break;
            }
        }

        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }

        Ok(Program { definitions })
    }

    fn parse_definition(&mut self) -> Result<Option<Definition>, Vec<ParseError>> {
        match self.current_token.token_type {
            TokenType::Entity => {
                let entity = self.parse_entity_def()?;
                Ok(Some(Definition::Entity(entity)))
            }
            TokenType::Rule => {
                let rule = self.parse_rule_def()?;
                Ok(Some(Definition::Rule(rule)))
            }
            TokenType::Flow => {
                let flow = self.parse_flow_def()?;
                Ok(Some(Definition::Flow(flow)))
            }
            TokenType::Constraint => {
                let constraint = self.parse_constraint_def()?;
                Ok(Some(Definition::Constraint(constraint)))
            }
            TokenType::Eof => Ok(None),
            _ => {
                // If we encounter an unexpected token, create an error but continue
                let error = ParseError {
                    message: format!(
                        "Unexpected token {:?}, expected a definition keyword",
                        self.current_token.token_type
                    ),
                    line: self.current_token.line,
                    column: self.current_token.column,
                    position: self.current_token.position,
                };
                self.errors.push(error);

                // Skip this token and try to continue parsing
                self.next_token();
                Ok(None)
            }
        }
    }

    fn parse_entity_def(&mut self) -> Result<EntityDef, Vec<ParseError>> {
        self.expect_token(TokenType::Entity)?;

        let _name_token = self.current_token.clone();
        let name = if let TokenType::Identifier(name_str) = &self.current_token.token_type {
            name_str.clone()
        } else {
            let error = ParseError {
                message: format!(
                    "Expected identifier for entity name, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            };
            self.errors.push(error);
            return Err(self.errors.clone());
        };
        self.next_token(); // consume identifier

        if !self.is_current_token(&TokenType::LeftBrace) {
            let error = ParseError {
                message: format!(
                    "Expected '{{' after entity name, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            };
            self.errors.push(error);
            return Err(self.errors.clone());
        }
        self.next_token(); // consume '{'

        let mut fields = Vec::new();
        while !self.is_current_token(&TokenType::RightBrace) && !self.is_at_end() {
            if let Some(field) = self.parse_field_def()? {
                fields.push(field);
            } else {
                // If we can't parse a field, report an error and try to continue
                let error = ParseError {
                    message: format!(
                        "Expected field identifier, got {:?}",
                        self.current_token.token_type
                    ),
                    line: self.current_token.line,
                    column: self.current_token.column,
                    position: self.current_token.position,
                };
                self.errors.push(error);

                // Skip token to avoid infinite loops
                if self.is_at_end() {
                    break;
                }
                self.next_token();
            }
        }

        if !self.is_current_token(&TokenType::RightBrace) {
            let error = ParseError {
                message: format!(
                    "Expected '}}' to close entity definition, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            };
            self.errors.push(error);
            return Err(self.errors.clone());
        }
        self.next_token(); // consume '}'

        Ok(EntityDef { name, fields })
    }

    fn parse_field_def(&mut self) -> Result<Option<FieldDef>, Vec<ParseError>> {
        if let TokenType::Identifier(name) = &self.current_token.token_type {
            let field_name = name.clone();
            self.next_token();
            Ok(Some(FieldDef { name: field_name }))
        } else {
            Ok(None)
        }
    }

    fn parse_rule_def(&mut self) -> Result<RuleDef, Vec<ParseError>> {
        self.expect_token(TokenType::Rule)?;

        let _name_token = self.current_token.clone();
        let name = if let TokenType::Identifier(name_str) = &self.current_token.token_type {
            name_str.clone()
        } else {
            let error = ParseError {
                message: format!(
                    "Expected identifier for rule name, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            };
            self.errors.push(error);
            return Err(self.errors.clone());
        };
        self.next_token(); // consume identifier

        if !self.is_current_token(&TokenType::Colon) {
            let error = ParseError {
                message: format!(
                    "Expected ':' after rule name, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            };
            self.errors.push(error);
            return Err(self.errors.clone());
        }
        self.next_token(); // consume ':'

        if !self.is_current_token(&TokenType::If) {
            let error = ParseError {
                message: format!(
                    "Expected 'if' keyword after rule colon, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            };
            self.errors.push(error);
            return Err(self.errors.clone());
        }
        self.next_token(); // consume 'if'

        let condition = self.parse_condition()?;

        if !self.is_current_token(&TokenType::Then) {
            let error = ParseError {
                message: format!(
                    "Expected 'then' keyword after rule condition, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            };
            self.errors.push(error);
            return Err(self.errors.clone());
        }
        self.next_token(); // consume 'then'

        let actions = self.parse_action_list()?;

        Ok(RuleDef {
            name,
            condition,
            actions,
        })
    }

    fn parse_condition(&mut self) -> Result<Condition, Vec<ParseError>> {
        self.parse_condition_or()
    }

    fn parse_condition_or(&mut self) -> Result<Condition, Vec<ParseError>> {
        let mut left = self.parse_condition_and()?;

        while self.is_current_token(&TokenType::Or) {
            self.next_token(); // consume 'or' operator
            let right = self.parse_condition_and()?;
            left = Condition::LogicalOp(Box::new(left), LogicalOp::Or, Box::new(right));
        }

        Ok(left)
    }

    fn parse_condition_and(&mut self) -> Result<Condition, Vec<ParseError>> {
        let mut left = self.parse_condition_term()?;

        while self.is_current_token(&TokenType::And) {
            self.next_token(); // consume 'and' operator
            let right = self.parse_condition_term()?;
            left = Condition::LogicalOp(Box::new(left), LogicalOp::And, Box::new(right));
        }

        Ok(left)
    }

    fn parse_condition_term(&mut self) -> Result<Condition, Vec<ParseError>> {
        let expr = self.parse_expression()?;
        Ok(Condition::Expression(expr))
    }

    fn parse_expression(&mut self) -> Result<Expression, Vec<ParseError>> {
        let left = self.parse_term()?;

        if self.is_comparison_operator() {
            let op = self.parse_comparator()?;
            let right = self.parse_term()?;
            Ok(Expression::Comparison {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        } else {
            // Check if it's a predicate
            if let Term::Identifier(name) = &left {
                if self.is_current_token(&TokenType::LeftParen) {
                    // This looks like a predicate call
                    return self.parse_predicate_as_expression(name.clone());
                }
            }
            Ok(Expression::Predicate(self.term_to_predicate(left)?))
        }
    }

    fn is_comparison_operator(&self) -> bool {
        matches!(
            self.current_token.token_type,
            TokenType::Equal
                | TokenType::NotEqual
                | TokenType::Greater
                | TokenType::Less
                | TokenType::GreaterEqual
                | TokenType::LessEqual
        )
    }

    fn parse_comparator(&mut self) -> Result<Comparator, Vec<ParseError>> {
        match self.current_token.token_type {
            TokenType::Equal => {
                self.next_token();
                Ok(Comparator::Equal)
            }
            TokenType::NotEqual => {
                self.next_token();
                Ok(Comparator::NotEqual)
            }
            TokenType::Greater => {
                self.next_token();
                Ok(Comparator::Greater)
            }
            TokenType::Less => {
                self.next_token();
                Ok(Comparator::Less)
            }
            TokenType::GreaterEqual => {
                self.next_token();
                Ok(Comparator::GreaterEqual)
            }
            TokenType::LessEqual => {
                self.next_token();
                Ok(Comparator::LessEqual)
            }
            _ => Err(vec![ParseError {
                message: format!(
                    "Expected comparator, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            }]),
        }
    }

    fn parse_term(&mut self) -> Result<Term, Vec<ParseError>> {
        match &self.current_token.token_type {
            TokenType::Identifier(name) => {
                let identifier = name.clone();
                self.next_token();

                // Check if this is a qualified reference (identifier.identifier)
                if self.is_current_token(&TokenType::Dot) {
                    self.next_token(); // consume dot

                    if let TokenType::Identifier(field_name) = &self.current_token.token_type {
                        let field = field_name.clone();
                        self.next_token();
                        Ok(Term::QualifiedRef(identifier, field))
                    } else {
                        Err(vec![ParseError {
                            message: format!(
                                "Expected identifier after dot, got {:?}",
                                self.current_token.token_type
                            ),
                            line: self.current_token.line,
                            column: self.current_token.column,
                            position: self.current_token.position,
                        }])
                    }
                } else {
                    Ok(Term::Identifier(identifier))
                }
            }
            TokenType::Number(value) => {
                let value = *value; // Copy the value before advancing
                self.next_token();
                Ok(Term::Number(value))
            }
            _ => Err(vec![ParseError {
                message: format!("Expected term, got {:?}", self.current_token.token_type),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            }]),
        }
    }

    fn parse_action_list(&mut self) -> Result<Vec<Action>, Vec<ParseError>> {
        let mut actions = Vec::new();

        // Parse first action
        if let Some(action) = self.parse_action()? {
            actions.push(action);
        }

        // Parse additional actions separated by commas
        while self.is_current_token(&TokenType::Comma) {
            self.next_token(); // consume comma

            if let Some(action) = self.parse_action()? {
                actions.push(action);
            } else {
                break; // Can't parse more actions
            }
        }

        Ok(actions)
    }

    fn parse_action(&mut self) -> Result<Option<Action>, Vec<ParseError>> {
        match &self.current_token.token_type {
            TokenType::Identifier(_) => {
                // Check if this is a predicate call or assignment
                let identifier_token = self.current_token.clone();
                let identifier = if let TokenType::Identifier(name) = &identifier_token.token_type {
                    name.clone()
                } else {
                    unreachable!() // We already checked it's an identifier
                };

                // Look ahead to see if this is followed by '(' (predicate) or '=' (assignment)
                let peek = self.lexer.peek_next_token();
                let is_paren = matches!(peek.token_type, TokenType::LeftParen);
                let is_assignment = matches!(peek.token_type, TokenType::Assignment);

                if is_paren {
                    // This is a predicate call
                    let predicate = self.parse_predicate()?;
                    Ok(Some(Action::Predicate(predicate)))
                } else if is_assignment {
                    // This is an assignment
                    self.next_token(); // consume identifier
                    self.expect_token(TokenType::Assignment)?;
                    let value = self.parse_term()?;
                    Ok(Some(Action::Assignment(Assignment {
                        variable: identifier,
                        value,
                    })))
                } else {
                    // This might be a predicate without arguments
                    let predicate = self.parse_predicate()?;
                    Ok(Some(Action::Predicate(predicate)))
                }
            }
            TokenType::If => {
                let if_action = self.parse_if_action()?;
                Ok(Some(Action::Control(ControlAction::If(if_action))))
            }
            TokenType::Loop => {
                let loop_action = self.parse_loop_action()?;
                Ok(Some(Action::Control(ControlAction::Loop(loop_action))))
            }
            TokenType::Halt => {
                self.next_token();
                Ok(Some(Action::Control(ControlAction::Halt(HaltAction))))
            }
            _ => Ok(None), // Not an action we can parse at this position
        }
    }

    fn parse_predicate(&mut self) -> Result<Predicate, Vec<ParseError>> {
        let name = if let TokenType::Identifier(name) = &self.current_token.token_type {
            name.clone()
        } else {
            return Err(vec![ParseError {
                message: format!(
                    "Expected identifier for predicate name, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            }]);
        };
        self.next_token(); // consume identifier

        self.expect_token(TokenType::LeftParen)?;

        let mut arguments = Vec::new();
        if !self.is_current_token(&TokenType::RightParen) {
            arguments.push(self.parse_term()?);

            while self.is_current_token(&TokenType::Comma) {
                self.next_token(); // consume comma
                arguments.push(self.parse_term()?);
            }
        }

        self.expect_token(TokenType::RightParen)?;

        Ok(Predicate { name, arguments })
    }

    fn parse_predicate_as_expression(
        &mut self,
        name: String,
    ) -> Result<Expression, Vec<ParseError>> {
        self.expect_token(TokenType::LeftParen)?;

        let mut arguments = Vec::new();
        if !self.is_current_token(&TokenType::RightParen) {
            arguments.push(self.parse_term()?);

            while self.is_current_token(&TokenType::Comma) {
                self.next_token(); // consume comma
                arguments.push(self.parse_term()?);
            }
        }

        self.expect_token(TokenType::RightParen)?;

        Ok(Expression::Predicate(Predicate { name, arguments }))
    }

    fn term_to_predicate(&mut self, term: Term) -> Result<Predicate, Vec<ParseError>> {
        match term {
            Term::Identifier(name) => Ok(Predicate {
                name,
                arguments: vec![],
            }),
            _ => Err(vec![ParseError {
                message: format!("Cannot convert term to predicate: {:?}", term),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            }]),
        }
    }

    fn parse_if_action(&mut self) -> Result<IfAction, Vec<ParseError>> {
        self.expect_token(TokenType::If)?;
        let condition = self.parse_condition()?;
        self.expect_token(TokenType::Then)?;
        let then_actions = self.parse_action_list()?;

        let else_actions = if self.is_current_token(&TokenType::Else) {
            self.next_token(); // consume else
            Some(self.parse_action_list()?)
        } else {
            None
        };

        Ok(IfAction {
            condition,
            then_actions,
            else_actions,
        })
    }

    fn parse_loop_action(&mut self) -> Result<LoopAction, Vec<ParseError>> {
        self.expect_token(TokenType::Loop)?;
        self.expect_token(TokenType::LeftBrace)?;

        let actions = self.parse_action_list()?;

        self.expect_token(TokenType::RightBrace)?;

        Ok(LoopAction { actions })
    }

    fn parse_flow_def(&mut self) -> Result<FlowDef, Vec<ParseError>> {
        self.expect_token(TokenType::Flow)?;

        let name = if let TokenType::Identifier(name_str) = &self.current_token.token_type {
            name_str.clone()
        } else {
            return Err(vec![ParseError {
                message: format!(
                    "Expected identifier for flow name, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            }]);
        };
        self.next_token(); // consume identifier

        if !self.is_current_token(&TokenType::LeftBrace) {
            return Err(vec![ParseError {
                message: format!(
                    "Expected '{{' after flow name, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            }]);
        }
        self.next_token(); // consume '{'

        let actions = self.parse_action_list()?;

        if !self.is_current_token(&TokenType::RightBrace) {
            return Err(vec![ParseError {
                message: format!(
                    "Expected '}}' to close flow definition, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            }]);
        }
        self.next_token(); // consume '}'

        Ok(FlowDef { name, actions })
    }

    fn parse_constraint_def(&mut self) -> Result<ConstraintDef, Vec<ParseError>> {
        self.expect_token(TokenType::Constraint)?;

        let name = if let TokenType::Identifier(name_str) = &self.current_token.token_type {
            name_str.clone()
        } else {
            return Err(vec![ParseError {
                message: format!(
                    "Expected identifier for constraint name, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            }]);
        };
        self.next_token(); // consume identifier

        if !self.is_current_token(&TokenType::Colon) {
            return Err(vec![ParseError {
                message: format!(
                    "Expected ':' after constraint name, got {:?}",
                    self.current_token.token_type
                ),
                line: self.current_token.line,
                column: self.current_token.column,
                position: self.current_token.position,
            }]);
        }
        self.next_token(); // consume ':'

        let condition = self.parse_condition()?;

        Ok(ConstraintDef { name, condition })
    }

    pub fn get_errors(&self) -> &Vec<ParseError> {
        &self.errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_entity() {
        let input = "entity Farmer { id location }";
        let mut parser = Parser::new(input);
        let result = parser.parse_program();

        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.definitions.len(), 1);

        if let Definition::Entity(entity) = &program.definitions[0] {
            assert_eq!(entity.name, "Farmer");
            assert_eq!(entity.fields.len(), 2);
            assert_eq!(entity.fields[0].name, "id");
            assert_eq!(entity.fields[1].name, "location");
        } else {
            panic!("Expected entity definition");
        }
    }

    #[test]
    fn test_parse_entity_with_multiple_fields() {
        let input = "entity TestEntity { field1 field2 field3 }";
        let mut parser = Parser::new(input);
        let result = parser.parse_program();

        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.definitions.len(), 1);

        if let Definition::Entity(entity) = &program.definitions[0] {
            assert_eq!(entity.name, "TestEntity");
            assert_eq!(entity.fields.len(), 3);
            assert_eq!(entity.fields[0].name, "field1");
            assert_eq!(entity.fields[1].name, "field2");
            assert_eq!(entity.fields[2].name, "field3");
        } else {
            panic!("Expected entity definition");
        }
    }

    #[test]
    fn test_parse_simple_rule() {
        let input = "rule TestRule: if condition then action()";
        let mut parser = Parser::new(input);
        let result = parser.parse_program();

        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.definitions.len(), 1);

        if let Definition::Rule(rule) = &program.definitions[0] {
            assert_eq!(rule.name, "TestRule");
            // Additional assertions would go here
        } else {
            panic!("Expected rule definition");
        }
    }

    #[test]
    fn test_parse_rule_with_comparison() {
        let input = "rule CompareRule: if value == 42 then do_something()";
        let mut parser = Parser::new(input);
        let result = parser.parse_program();

        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.definitions.len(), 1);

        if let Definition::Rule(rule) = &program.definitions[0] {
            assert_eq!(rule.name, "CompareRule");
        } else {
            panic!("Expected rule definition");
        }
    }

    #[test]
    fn test_parse_flow() {
        let input = "flow TestFlow { action1() action2() }";
        let mut parser = Parser::new(input);
        let result = parser.parse_program();

        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.definitions.len(), 1);

        if let Definition::Flow(flow) = &program.definitions[0] {
            assert_eq!(flow.name, "TestFlow");
        } else {
            panic!("Expected flow definition");
        }
    }

    #[test]
    fn test_parse_constraint() {
        let input = "constraint TestConstraint: value > 0";
        let mut parser = Parser::new(input);
        let result = parser.parse_program();

        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.definitions.len(), 1);

        if let Definition::Constraint(constraint) = &program.definitions[0] {
            assert_eq!(constraint.name, "TestConstraint");
        } else {
            panic!("Expected constraint definition");
        }
    }

    #[test]
    fn test_parse_multiple_definitions() {
        let input = r#"
        entity Farmer { id name location }
        rule CheckFarmer: if farmer.id != 0 then validate_farmer(farmer)
        flow ProcessFarmers { load_farmers() validate_farmers() }
        constraint ValidLocation: farmer.location != ""
        "#;
        let mut parser = Parser::new(input);
        let result = parser.parse_program();

        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.definitions.len(), 4);

        // Check that all definition types are present
        let mut entity_count = 0;
        let mut rule_count = 0;
        let mut flow_count = 0;
        let mut constraint_count = 0;

        for def in &program.definitions {
            match def {
                Definition::Entity(_) => entity_count += 1,
                Definition::Rule(_) => rule_count += 1,
                Definition::Flow(_) => flow_count += 1,
                Definition::Constraint(_) => constraint_count += 1,
            }
        }

        assert_eq!(entity_count, 1);
        assert_eq!(rule_count, 1);
        assert_eq!(flow_count, 1);
        assert_eq!(constraint_count, 1);
    }

    #[test]
    fn test_parse_error_handling() {
        let input = "entity IncompleteEntity { id";
        let mut parser = Parser::new(input);
        let result = parser.parse_program();

        // This should produce an error since the entity definition is incomplete
        assert!(result.is_err());
        let errors = result.err().unwrap();
        assert!(!errors.is_empty());
    }
}
