use crate::parser::parser_state::ParserState;
use crate::parser::ast_nodes::*;
use crate::shared::source_location::{SourceLocation, Span};
use crate::shared::diagnostics::ErrorCode;
use crate::parser::precedence::Precedence;
use crate::lexer::token_kind::TokenKind;

pub struct Parser {
    pub state: ParserState,
}

impl Parser {
    pub fn new(tokens: Vec<crate::lexer::token::Token>) -> Self {
        Self {
            state: ParserState::new(tokens),
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut declarations = Vec::new();

        while !self.state.is_at_end() {
            if let Some(decl) = self.declaration() {
                declarations.push(decl);
            } else if !self.state.is_at_end() {
                // If declaration parsing failed, try to synchronize and continue
                self.state.synchronize();
            }
        }

        let start = if !declarations.is_empty() {
            declarations[0].span().start.clone()
        } else {
            SourceLocation::new(1, 1, 0)
        };

        let end = if !self.state.tokens.is_empty() {
            self.state.tokens[self.state.tokens.len() - 1].location.clone()
        } else {
            SourceLocation::new(1, 1, 0)
        };

        Program {
            declarations,
            span: Span { start, end },
        }
    }

    fn declaration(&mut self) -> Option<Declaration> {
        if self.state.match_token(&[TokenKind::Entity]) {
            return self.entity_declaration();
        } else if self.state.match_token(&[TokenKind::Rule]) {
            return self.rule_declaration();
        } else if self.state.match_token(&[TokenKind::Flow]) {
            return self.flow_declaration();
        } else if self.state.match_token(&[TokenKind::Constraint]) {
            return self.constraint_declaration();
        }

        // If none of the above matched, we have an unexpected token
        let token = self.state.current_token().clone();
        self.state.error_at_current(
            ErrorCode::ExpectedDeclaration,
            format!("Expected entity, rule, flow, or constraint declaration, found {:?}", token.kind),
        );
        self.state.synchronize();
        None
    }

    fn entity_declaration(&mut self) -> Option<Declaration> {
        let name_token = self.state.previous_token().clone();
        let name = self.state.consume(TokenKind::Identifier, ErrorCode::ExpectedIdentifier, "Expected entity name");

        if name.is_none() {
            self.state.synchronize();
            return None;
        }
        let name = name.unwrap();

        self.state.consume(TokenKind::LeftBrace, ErrorCode::MissingRightBrace, "Expected '{' before entity fields");

        let mut fields = Vec::new();
        while !self.state.check(&TokenKind::RightBrace) && !self.state.is_at_end() {
            if let Some(field) = self.field_declaration() {
                fields.push(field);
            } else {
                // If field parsing failed, skip to next field or end of entity
                self.synchronize_field();
            }
        }

        let end_brace = self.state.consume(TokenKind::RightBrace, ErrorCode::MissingRightBrace, "Expected '}' after entity fields");
        if end_brace.is_none() {
            self.state.synchronize();
            return None;
        }

        Some(Declaration::Entity(EntityDeclaration {
            name: Identifier {
                name: name.lexeme,
                span: name.location.clone().into(),
            },
            fields,
            span: Span {
                start: name_token.location.clone(),
                end: self.state.previous_token().location.clone(),
            },
        }))
    }

    fn field_declaration(&mut self) -> Option<Field> {
        let name = self.state.consume(TokenKind::Identifier, ErrorCode::ExpectedIdentifier, "Expected field name");
        if name.is_none() {
            return None;
        }
        let name = name.unwrap();

        self.state.consume(TokenKind::Colon, ErrorCode::ExpectedDelimiter, "Expected ':' after field name");

        let field_type = self.field_type();
        if field_type.is_none() {
            return None;
        }
        let field_type = field_type.unwrap();

        Some(Field {
            name: Identifier {
                name: name.lexeme,
                span: name.location.clone().into(),
            },
            field_type,
            span: Span {
                start: name.location.clone(),
                end: self.state.previous_token().location.clone(),
            },
        })
    }

    fn field_type(&mut self) -> Option<Type> {
        match self.state.current_token().kind {
            TokenKind::Sym => {
                self.state.advance();
                Some(Type::Sym)
            }
            TokenKind::Num => {
                self.state.advance();
                Some(Type::Num)
            }
            TokenKind::Bool => {
                self.state.advance();
                Some(Type::Bool)
            }
            TokenKind::Vec => {
                self.state.advance();
                Some(Type::Vec)
            }
            TokenKind::Ref => {
                self.state.advance();
                Some(Type::Ref)
            }
            TokenKind::Ctx => {
                self.state.advance();
                Some(Type::Ctx)
            }
            TokenKind::Identifier => {
                let ident = self.state.advance().clone();
                Some(Type::Identifier(Identifier {
                    name: ident.lexeme,
                    span: ident.location.clone().into(),
                }))
            }
            _ => {
                self.state.error_at_current(
                    ErrorCode::ExpectedTypeName,
                    format!("Expected type name, found {:?}", self.state.current_token().kind),
                );
                None
            }
        }
    }

    fn synchronize_field(&mut self) {
        while !self.state.is_at_end() && !self.state.check(&TokenKind::RightBrace) && !self.state.check(&TokenKind::Identifier) {
            self.state.advance();
        }
    }

    fn rule_declaration(&mut self) -> Option<Declaration> {
        let name_token = self.state.previous_token().clone();
        let name = self.state.consume(TokenKind::Identifier, ErrorCode::ExpectedIdentifier, "Expected rule name");
        if name.is_none() {
            self.state.synchronize();
            return None;
        }
        let name = name.unwrap();

        self.state.consume(TokenKind::LeftBrace, ErrorCode::MissingRightBrace, "Expected '{' before rule condition");

        // Parse condition
        let condition_expr = self.expression();
        if condition_expr.is_none() {
            self.state.error_at_current(
                ErrorCode::ExpectedRuleCondition,
                "Expected rule condition".to_string(),
            );
            self.state.synchronize();
            return None;
        }
        let condition = RuleCondition {
            expression: condition_expr.unwrap(),
            span: self.state.previous_token().location.clone().into(),
        };

        // Expect 'then' keyword
        self.state.consume(TokenKind::Then, ErrorCode::ExpectedKeyword, "Expected 'then' keyword");

        // Parse action (block of statements)
        let action_statements = self.block_statement();
        if action_statements.is_none() {
            self.state.error_at_current(
                ErrorCode::ExpectedRuleAction,
                "Expected rule action".to_string(),
            );
            self.state.synchronize();
            return None;
        }
        let action = RuleAction {
            statements: match action_statements.unwrap() {
                Statement::Block(block) => block.statements,
                _ => vec![], // This shouldn't happen due to how block_statement works
            },
            span: self.state.previous_token().location.clone().into(),
        };

        let end_brace = self.state.consume(TokenKind::RightBrace, ErrorCode::MissingRightBrace, "Expected '}' after rule declaration");
        if end_brace.is_none() {
            self.state.synchronize();
            return None;
        }

        Some(Declaration::Rule(RuleDeclaration {
            name: Identifier {
                name: name.lexeme,
                span: name.location.clone().into(),
            },
            condition,
            action,
            span: Span {
                start: name_token.location.clone(),
                end: self.state.previous_token().location.clone(),
            },
        }))
    }

    fn flow_declaration(&mut self) -> Option<Declaration> {
        let name_token = self.state.previous_token().clone();
        let name = self.state.consume(TokenKind::Identifier, ErrorCode::ExpectedIdentifier, "Expected flow name");
        if name.is_none() {
            self.state.synchronize();
            return None;
        }
        let name = name.unwrap();

        self.state.consume(TokenKind::LeftBrace, ErrorCode::MissingRightBrace, "Expected '{' before flow steps");

        let mut steps = Vec::new();
        while !self.state.check(&TokenKind::RightBrace) && !self.state.is_at_end() {
            if let Some(expr) = self.expression() {
                steps.push(FlowStep {
                    expression: expr,
                    span: self.state.previous_token().location.clone().into(),
                });
            } else {
                // If expression parsing failed, skip to next step or end of flow
                self.synchronize_flow_step();
            }
        }

        let end_brace = self.state.consume(TokenKind::RightBrace, ErrorCode::MissingRightBrace, "Expected '}' after flow steps");
        if end_brace.is_none() {
            self.state.synchronize();
            return None;
        }

        Some(Declaration::Flow(FlowDeclaration {
            name: Identifier {
                name: name.lexeme,
                span: name.location.clone().into(),
            },
            steps,
            span: Span {
                start: name_token.location.clone(),
                end: self.state.previous_token().location.clone(),
            },
        }))
    }

    fn constraint_declaration(&mut self) -> Option<Declaration> {
        let name_token = self.state.previous_token().clone();
        let name = self.state.consume(TokenKind::Identifier, ErrorCode::ExpectedIdentifier, "Expected constraint name");
        if name.is_none() {
            self.state.synchronize();
            return None;
        }
        let name = name.unwrap();

        self.state.consume(TokenKind::LeftBrace, ErrorCode::MissingRightBrace, "Expected '{' before constraint condition");

        // Parse condition
        let condition_expr = self.expression();
        if condition_expr.is_none() {
            self.state.error_at_current(
                ErrorCode::ExpectedConstraintCondition,
                "Expected constraint condition".to_string(),
            );
            self.state.synchronize();
            return None;
        }
        let condition = ConstraintCondition {
            expression: condition_expr.unwrap(),
            span: self.state.previous_token().location.clone().into(),
        };

        // Expect 'then' keyword
        self.state.consume(TokenKind::Then, ErrorCode::ExpectedKeyword, "Expected 'then' keyword");

        // Parse action (block of statements)
        let action_statements = self.block_statement();
        if action_statements.is_none() {
            self.state.error_at_current(
                ErrorCode::ExpectedConstraintAction,
                "Expected constraint action".to_string(),
            );
            self.state.synchronize();
            return None;
        }
        let action = ConstraintAction {
            statements: match action_statements.unwrap() {
                Statement::Block(block) => block.statements,
                _ => vec![], // This shouldn't happen due to how block_statement works
            },
            span: self.state.previous_token().location.clone().into(),
        };

        let end_brace = self.state.consume(TokenKind::RightBrace, ErrorCode::MissingRightBrace, "Expected '}' after constraint declaration");
        if end_brace.is_none() {
            self.state.synchronize();
            return None;
        }

        Some(Declaration::Constraint(ConstraintDeclaration {
            name: Identifier {
                name: name.lexeme,
                span: name.location.clone().into(),
            },
            condition,
            action,
            span: Span {
                start: name_token.location.clone(),
                end: self.state.previous_token().location.clone(),
            },
        }))
    }

    fn synchronize_flow_step(&mut self) {
        while !self.state.is_at_end() && !self.state.check(&TokenKind::RightBrace) && !self.state.check(&TokenKind::Identifier) {
            self.state.advance();
        }
    }

    fn expression(&mut self) -> Option<Expression> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut expr = self.prefix()?;

        while precedence < Precedence::from_token(&self.state.current_token().kind) {
            expr = self.infix(expr)?;
        }

        Some(expr)
    }

    fn prefix(&mut self) -> Option<Expression> {
        let token = self.state.current_token().clone();
        match token.kind {
            TokenKind::NumberLiteral => {
                self.state.advance();
                let value = token.lexeme.parse::<f64>().ok();
                if value.is_none() {
                    self.state.error_at_current(
                        ErrorCode::InvalidNumber,
                        format!("Invalid number literal: {}", token.lexeme),
                    );
                    return None;
                }

                Some(Expression::Literal(Literal {
                    value: LiteralValue::Number(value.unwrap()),
                    span: token.location.clone().into(),
                }))
            }
            TokenKind::StringLiteral => {
                self.state.advance();
                Some(Expression::Literal(Literal {
                    value: LiteralValue::String(token.lexeme[1..token.lexeme.len()-1].to_string()), // Remove quotes
                    span: token.location.clone().into(),
                }))
            }
            TokenKind::True | TokenKind::False => {
                self.state.advance();
                Some(Expression::Literal(Literal {
                    value: LiteralValue::Boolean(token.kind == TokenKind::True),
                    span: token.location.clone().into(),
                }))
            }
            TokenKind::Identifier => {
                self.state.advance();
                Some(Expression::Variable(VariableExpression {
                    name: Identifier {
                        name: token.lexeme.clone(),
                        span: token.location.clone().into(),
                    },
                    span: token.location.clone().into(),
                }))
            }
            TokenKind::LeftParen => {
                self.state.advance(); // consume '('
                let expr = self.expression()?;
                self.state.consume(TokenKind::RightParen, ErrorCode::MissingRightParen, "Expected ')' after expression");

                Some(Expression::Grouping(GroupingExpression {
                    expression: Box::new(expr),
                    span: Span {
                        start: token.location.clone(),
                        end: self.state.previous_token().location.clone(),
                    },
                }))
            }
            TokenKind::Bang | TokenKind::Minus => {
                self.state.advance(); // consume operator
                let operator = token.kind;
                let right = self.prefix()?;

                Some(Expression::Unary(UnaryExpression {
                    operator,
                    operand: Box::new(right.clone()),
                    span: Span {
                        start: token.location.clone(),
                        end: right.span().end.clone(),
                    },
                }))
            }
            _ => {
                self.state.error_at_current(
                    ErrorCode::ExpectedExpression,
                    format!("Expected expression, found {:?}", token.kind),
                );
                None
            }
        }
    }

    fn infix(&mut self, left: Expression) -> Option<Expression> {
        let token = self.state.current_token().clone();
        let precedence = Precedence::from_token(&token.kind);

        match token.kind {
            TokenKind::Plus | TokenKind::Minus |
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent |
            TokenKind::EqualEqual | TokenKind::BangEqual |
            TokenKind::Less | TokenKind::LessEqual | TokenKind::Greater | TokenKind::GreaterEqual |
            TokenKind::AmpersandAmpersand | TokenKind::PipePipe => {
                self.state.advance(); // consume operator
                let right = self.parse_precedence(precedence)?;

                Some(Expression::Binary(BinaryExpression {
                    left: Box::new(left.clone()),
                    operator: token.kind,
                    right: Box::new(right.clone()),
                    span: Span {
                        start: left.span().start.clone(),
                        end: right.span().end.clone(),
                    },
                }))
            }
            TokenKind::Equal => {
                self.state.advance(); // consume '='
                let value = self.parse_precedence(Precedence::Assignment)?;

                Some(Expression::Assignment(AssignmentExpression {
                    target: Box::new(left.clone()),
                    value: Box::new(value.clone()),
                    span: Span {
                        start: left.span().start.clone(),
                        end: value.span().end.clone(),
                    },
                }))
            }
            TokenKind::LeftParen => {
                self.state.advance(); // consume '('
                let mut arguments = Vec::new();

                if !self.state.check(&TokenKind::RightParen) {
                    loop {
                        if let Some(arg) = self.expression() {
                            arguments.push(arg);
                        } else {
                            self.state.error_at_current(
                                ErrorCode::ExpectedArgumentList,
                                "Expected argument in call expression".to_string(),
                            );
                            return None;
                        }

                        if !self.state.match_token(&[TokenKind::Comma]) {
                            break;
                        }
                    }
                }

                self.state.consume(TokenKind::RightParen, ErrorCode::MissingRightParen, "Expected ')' after arguments");

                Some(Expression::Call(CallExpression {
                    callee: Box::new(left.clone()),
                    arguments,
                    span: Span {
                        start: left.span().start.clone(),
                        end: self.state.previous_token().location.clone(),
                    },
                }))
            }
            TokenKind::Dot => {
                self.state.advance(); // consume '.'
                let name = self.state.consume(TokenKind::Identifier, ErrorCode::ExpectedIdentifier, "Expected property name");
                if name.is_none() {
                    return None;
                }
                let name = name.unwrap();

                Some(Expression::Member(MemberExpression {
                    object: Box::new(left.clone()),
                    property: Identifier {
                        name: name.lexeme,
                        span: name.location.clone().into(),
                    },
                    span: Span {
                        start: left.span().start.clone(),
                        end: name.location.clone(),
                    },
                }))
            }
            TokenKind::LeftBracket => {
                self.state.advance(); // consume '['
                let index = self.expression()?;
                self.state.consume(TokenKind::RightBracket, ErrorCode::MissingRightBracket, "Expected ']' after index expression");

                Some(Expression::Index(IndexExpression {
                    object: Box::new(left.clone()),
                    index: Box::new(index),
                    span: Span {
                        start: left.span().start.clone(),
                        end: self.state.previous_token().location.clone(),
                    },
                }))
            }
            _ => {
                self.state.error_at_current(
                    ErrorCode::ExpectedOperator,
                    format!("Expected operator, found {:?}", token.kind),
                );
                None
            }
        }
    }

    fn block_statement(&mut self) -> Option<Statement> {
        self.state.consume(TokenKind::LeftBrace, ErrorCode::ExpectedBlock, "Expected '{' before block");
        
        let mut statements = Vec::new();
        while !self.state.check(&TokenKind::RightBrace) && !self.state.is_at_end() {
            if let Some(stmt) = self.statement() {
                statements.push(stmt);
            }
        }
        
        let end_brace = self.state.consume(TokenKind::RightBrace, ErrorCode::MissingRightBrace, "Expected '}' after block");
        if end_brace.is_none() {
            return None;
        }
        
        Some(Statement::Block(BlockStatement {
            statements,
            span: Span {
                start: self.state.previous_token().location.clone(), // This is the opening brace
                end: self.state.previous_token().location.clone(),   // This is the closing brace
            },
        }))
    }

    fn statement(&mut self) -> Option<Statement> {
        if self.state.match_token(&[TokenKind::LeftBrace]) {
            return self.block_statement();
        }
        
        if self.state.match_token(&[TokenKind::If]) {
            return self.if_statement();
        }
        
        if self.state.match_token(&[TokenKind::While]) {
            return self.while_statement();
        }
        
        if self.state.match_token(&[TokenKind::For]) {
            return self.for_statement();
        }
        
        if self.state.match_token(&[TokenKind::Return]) {
            return self.return_statement();
        }
        
        if self.state.match_token(&[TokenKind::Break]) {
            return Some(Statement::Break(BreakStatement {
                span: self.state.previous_token().location.clone().into(),
            }));
        }
        
        if self.state.match_token(&[TokenKind::Continue]) {
            return Some(Statement::Continue(ContinueStatement {
                span: self.state.previous_token().location.clone().into(),
            }));
        }
        
        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Option<Statement> {
        let expr = self.expression()?;
        self.state.consume(TokenKind::Semicolon, ErrorCode::MissingSemicolon, "Expected ';' after expression");
        
        Some(Statement::Expression(ExpressionStatement {
            expression: expr.clone(),
            span: expr.span().clone(),
        }))
    }

    fn if_statement(&mut self) -> Option<Statement> {
        let if_token = self.state.previous_token().clone();
        self.state.consume(TokenKind::LeftParen, ErrorCode::ExpectedDelimiter, "Expected '(' after 'if'");
        let condition = self.expression()?;
        self.state.consume(TokenKind::RightParen, ErrorCode::MissingRightParen, "Expected ')' after condition");

        let then_branch = Box::new(self.statement()?);

        let else_branch = if self.state.match_token(&[TokenKind::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Some(Statement::If(IfStatement {
            condition,
            then_branch: then_branch.clone(),
            else_branch: else_branch.clone(),
            span: Span {
                start: if_token.location.clone(),
                end: if let Some(ref else_stmt) = else_branch {
                    else_stmt.span().end.clone()
                } else {
                    then_branch.span().end.clone()
                },
            },
        }))
    }

    fn while_statement(&mut self) -> Option<Statement> {
        let while_token = self.state.previous_token().clone();
        self.state.consume(TokenKind::LeftParen, ErrorCode::ExpectedDelimiter, "Expected '(' after 'while'");
        let condition = self.expression()?;
        self.state.consume(TokenKind::RightParen, ErrorCode::MissingRightParen, "Expected ')' after condition");
        
        let body = Box::new(self.statement()?);
        
        Some(Statement::While(Box::new(WhileStatement {
            condition,
            body: body.clone(),
            span: Span {
                start: while_token.location.clone(),
                end: body.span().end.clone(),
            },
        })))
    }

    fn for_statement(&mut self) -> Option<Statement> {
        let for_token = self.state.previous_token().clone();
        self.state.consume(TokenKind::LeftParen, ErrorCode::ExpectedDelimiter, "Expected '(' after 'for'");
        
        let initializer = if self.state.match_token(&[TokenKind::Semicolon]) {
            None
        } else {
            let init_stmt = self.expression_statement();
            self.state.consume(TokenKind::Semicolon, ErrorCode::MissingSemicolon, "Expected ';' after for loop initializer");
            init_stmt
        };
        
        let condition = if self.state.check(&TokenKind::Semicolon) {
            // Empty condition
            self.state.advance(); // consume semicolon
            Expression::Literal(Literal {
                value: LiteralValue::Boolean(true),
                span: self.state.previous_token().location.clone().into(),
            })
        } else {
            let expr = self.expression()?;
            self.state.consume(TokenKind::Semicolon, ErrorCode::MissingSemicolon, "Expected ';' after for loop condition");
            expr
        };
        
        let increment = if self.state.check(&TokenKind::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        
        self.state.consume(TokenKind::RightParen, ErrorCode::MissingRightParen, "Expected ')' after for clauses");
        
        let body = Box::new(self.statement()?);
        
        Some(Statement::For(Box::new(ForStatement {
            initializer: initializer.map(|stmt| Box::new(stmt)),
            condition,
            increment,
            body: body.clone(),
            span: Span {
                start: for_token.location.clone(),
                end: body.span().end.clone(),
            },
        })))
    }

    fn return_statement(&mut self) -> Option<Statement> {
        let return_token = self.state.previous_token().clone();
        let value = if self.state.check(&TokenKind::Semicolon) {
            None
        } else {
            self.expression()
        };
        
        self.state.consume(TokenKind::Semicolon, ErrorCode::MissingSemicolon, "Expected ';' after return value");
        
        Some(Statement::Return(Box::new(ReturnStatement {
            value,
            span: Span {
                start: return_token.location.clone(),
                end: self.state.previous_token().location.clone(),
            },
        })))
    }

    pub fn get_diagnostics(&self) -> &crate::shared::diagnostics::Diagnostics {
        &self.state.diagnostics
    }
}