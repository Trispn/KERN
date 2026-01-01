use crate::lexer::token::{Token, TokenKind};
use crate::ast::nodes::*;
use crate::shared::source_location::{SourceLocation, Span};
use crate::errors::diagnostics::{Diagnostics, ErrorCode};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub diagnostics: Diagnostics,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            diagnostics: Diagnostics::new(),
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut declarations = Vec::new();

        while !self.is_at_end() {
            if let Some(decl) = self.declaration() {
                declarations.push(decl);
            }
        }

        let start = if !declarations.is_empty() {
            declarations[0].span().start.clone()
        } else {
            SourceLocation::new(1, 1, 0)
        };

        let end = if !self.tokens.is_empty() {
            self.tokens[self.tokens.len() - 1].location.clone()
        } else {
            SourceLocation::new(1, 1, 0)
        };

        Program {
            declarations,
            span: Span { start, end },
        }
    }

    fn declaration(&mut self) -> Option<Declaration> {
        if self.match_token(&[TokenKind::Entity]) {
            return self.entity_declaration();
        } else if self.match_token(&[TokenKind::Rule]) {
            return self.rule_declaration();
        } else if self.match_token(&[TokenKind::Flow]) {
            return self.flow_declaration();
        } else if self.match_token(&[TokenKind::Constraint]) {
            return self.constraint_declaration();
        }

        // If none of the above matched, we have an unexpected token
        let token = self.peek();
        self.diagnostics.add_error(
            ErrorCode::ExpectedDeclaration,
            format!("Expected entity, rule, flow, or constraint declaration, found {:?}", token.kind),
            token.location.clone(),
            "unknown".to_string(),
        );
        self.synchronize();
        None
    }

    fn entity_declaration(&mut self) -> Option<Declaration> {
        let name_token = self.previous();
        let name = self.consume(TokenKind::Identifier, ErrorCode::ExpectedIdentifier, "Expected entity name");

        if name.is_none() {
            self.synchronize();
            return None;
        }
        let name = name.unwrap();

        self.consume(TokenKind::LeftBrace, ErrorCode::MissingRightBrace, "Expected '{' before entity fields");

        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if let Some(field) = self.field_declaration() {
                fields.push(field);
            } else {
                // If field parsing failed, skip to next field or end of entity
                self.synchronize_field();
            }
        }

        let end_brace = self.consume(TokenKind::RightBrace, ErrorCode::MissingRightBrace, "Expected '}' after entity fields");
        if end_brace.is_none() {
            self.synchronize();
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
                end: self.previous().location.clone(),
            },
        }))
    }

    fn field_declaration(&mut self) -> Option<Field> {
        let name = self.consume(TokenKind::Identifier, ErrorCode::ExpectedIdentifier, "Expected field name");
        if name.is_none() {
            return None;
        }
        let name = name.unwrap();

        self.consume(TokenKind::Colon, ErrorCode::ExpectedDelimiter, "Expected ':' after field name");

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
                end: self.previous().location.clone(),
            },
        })
    }

    fn field_type(&mut self) -> Option<Type> {
        match self.peek().kind {
            TokenKind::Sym => {
                self.advance();
                Some(Type::Sym)
            }
            TokenKind::Num => {
                self.advance();
                Some(Type::Num)
            }
            TokenKind::Bool => {
                self.advance();
                Some(Type::Bool)
            }
            TokenKind::Vec => {
                self.advance();
                Some(Type::Vec)
            }
            TokenKind::Ref => {
                self.advance();
                Some(Type::Ref)
            }
            TokenKind::Ctx => {
                self.advance();
                Some(Type::Ctx)
            }
            TokenKind::Identifier => {
                let ident = self.advance();
                Some(Type::Identifier(Identifier {
                    name: ident.lexeme,
                    span: ident.location.clone().into(),
                }))
            }
            _ => {
                self.diagnostics.add_error(
                    ErrorCode::ExpectedTypeName,
                    format!("Expected type name, found {:?}", self.peek().kind),
                    self.peek().location.clone(),
                    "unknown".to_string(),
                );
                None
            }
        }
    }

    fn synchronize(&mut self) {
        self.advance(); // Skip the problematic token

        while !self.is_at_end() {
            // Look for the start of a new declaration
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }

            match self.peek().kind {
                TokenKind::Entity | TokenKind::Rule | TokenKind::Flow | TokenKind::Constraint => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn synchronize_field(&mut self) {
        while !self.is_at_end() && !self.check(&TokenKind::RightBrace) && !self.check(&TokenKind::Identifier) {
            self.advance();
        }
    }

    fn rule_declaration(&mut self) -> Option<Declaration> {
        let name_token = self.previous();
        let name = self.consume(TokenKind::Identifier, ErrorCode::ExpectedIdentifier, "Expected rule name");
        if name.is_none() {
            self.synchronize();
            return None;
        }
        let name = name.unwrap();

        self.consume(TokenKind::LeftBrace, ErrorCode::MissingRightBrace, "Expected '{' before rule condition");

        // Parse condition
        let condition_expr = self.expression();
        if condition_expr.is_none() {
            self.diagnostics.add_error(
                ErrorCode::ExpectedRuleCondition,
                "Expected rule condition".to_string(),
                self.peek().location.clone(),
                "unknown".to_string(),
            );
            self.synchronize();
            return None;
        }
        let condition = RuleCondition {
            expression: condition_expr.unwrap(),
            span: self.previous().location.clone().into(),
        };

        // Expect 'then' keyword
        self.consume(TokenKind::Then, ErrorCode::ExpectedKeyword, "Expected 'then' keyword");

        // Parse action (block of statements)
        let action_statements = self.block_statement();
        if action_statements.is_none() {
            self.diagnostics.add_error(
                ErrorCode::ExpectedRuleAction,
                "Expected rule action".to_string(),
                self.peek().location.clone(),
                "unknown".to_string(),
            );
            self.synchronize();
            return None;
        }
        let action = RuleAction {
            statements: match action_statements.unwrap() {
                Statement::Block(block) => block.statements,
                _ => vec![], // This shouldn't happen due to how block_statement works
            },
            span: self.previous().location.clone().into(),
        };

        let end_brace = self.consume(TokenKind::RightBrace, ErrorCode::MissingRightBrace, "Expected '}' after rule declaration");
        if end_brace.is_none() {
            self.synchronize();
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
                end: self.previous().location.clone(),
            },
        }))
    }

    fn flow_declaration(&mut self) -> Option<Declaration> {
        let name_token = self.previous();
        let name = self.consume(TokenKind::Identifier, ErrorCode::ExpectedIdentifier, "Expected flow name");
        if name.is_none() {
            self.synchronize();
            return None;
        }
        let name = name.unwrap();

        self.consume(TokenKind::LeftBrace, ErrorCode::MissingRightBrace, "Expected '{' before flow steps");

        let mut steps = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if let Some(expr) = self.expression() {
                steps.push(FlowStep {
                    expression: expr,
                    span: self.previous().location.clone().into(),
                });
            } else {
                // If expression parsing failed, skip to next step or end of flow
                self.synchronize_flow_step();
            }
        }

        let end_brace = self.consume(TokenKind::RightBrace, ErrorCode::MissingRightBrace, "Expected '}' after flow steps");
        if end_brace.is_none() {
            self.synchronize();
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
                end: self.previous().location.clone(),
            },
        }))
    }

    fn constraint_declaration(&mut self) -> Option<Declaration> {
        let name_token = self.previous();
        let name = self.consume(TokenKind::Identifier, ErrorCode::ExpectedIdentifier, "Expected constraint name");
        if name.is_none() {
            self.synchronize();
            return None;
        }
        let name = name.unwrap();

        self.consume(TokenKind::LeftBrace, ErrorCode::MissingRightBrace, "Expected '{' before constraint condition");

        // Parse condition
        let condition_expr = self.expression();
        if condition_expr.is_none() {
            self.diagnostics.add_error(
                ErrorCode::ExpectedConstraintCondition,
                "Expected constraint condition".to_string(),
                self.peek().location.clone(),
                "unknown".to_string(),
            );
            self.synchronize();
            return None;
        }
        let condition = ConstraintCondition {
            expression: condition_expr.unwrap(),
            span: self.previous().location.clone().into(),
        };

        // Expect 'then' keyword
        self.consume(TokenKind::Then, ErrorCode::ExpectedKeyword, "Expected 'then' keyword");

        // Parse action (block of statements)
        let action_statements = self.block_statement();
        if action_statements.is_none() {
            self.diagnostics.add_error(
                ErrorCode::ExpectedConstraintAction,
                "Expected constraint action".to_string(),
                self.peek().location.clone(),
                "unknown".to_string(),
            );
            self.synchronize();
            return None;
        }
        let action = ConstraintAction {
            statements: match action_statements.unwrap() {
                Statement::Block(block) => block.statements,
                _ => vec![], // This shouldn't happen due to how block_statement works
            },
            span: self.previous().location.clone().into(),
        };

        let end_brace = self.consume(TokenKind::RightBrace, ErrorCode::MissingRightBrace, "Expected '}' after constraint declaration");
        if end_brace.is_none() {
            self.synchronize();
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
                end: self.previous().location.clone(),
            },
        }))
    }

    fn synchronize_flow_step(&mut self) {
        while !self.is_at_end() && !self.check(&TokenKind::RightBrace) && !self.check(&TokenKind::Identifier) {
            self.advance();
        }
    }

    fn expression(&mut self) -> Option<Expression> {
        self.assignment()
    }

    fn assignment(&mut self) -> Option<Expression> {
        let mut expr = self.logical_or()?;

        if self.match_token(&[TokenKind::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            return Some(Expression::Assignment(AssignmentExpression {
                target: Box::new(expr),
                value: Box::new(value),
                span: Span {
                    start: expr.span().start.clone(),
                    end: value.span().end.clone(),
                },
            }));
        }

        Some(expr)
    }

    fn logical_or(&mut self) -> Option<Expression> {
        let mut expr = self.logical_and()?;

        while self.match_token(&[TokenKind::PipePipe]) {
            let operator = self.previous();
            let right = self.logical_and()?;
            
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator.kind,
                right: Box::new(right),
                span: Span {
                    start: expr.span().start.clone(),
                    end: right.span().end.clone(),
                },
            });
        }

        Some(expr)
    }

    fn logical_and(&mut self) -> Option<Expression> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenKind::AmpersandAmpersand]) {
            let operator = self.previous();
            let right = self.equality()?;
            
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator.kind,
                right: Box::new(right),
                span: Span {
                    start: expr.span().start.clone(),
                    end: right.span().end.clone(),
                },
            });
        }

        Some(expr)
    }

    fn equality(&mut self) -> Option<Expression> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator.kind,
                right: Box::new(right),
                span: Span {
                    start: expr.span().start.clone(),
                    end: right.span().end.clone(),
                },
            });
        }

        Some(expr)
    }

    fn comparison(&mut self) -> Option<Expression> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator.kind,
                right: Box::new(right),
                span: Span {
                    start: expr.span().start.clone(),
                    end: right.span().end.clone(),
                },
            });
        }

        Some(expr)
    }

    fn term(&mut self) -> Option<Expression> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator.kind,
                right: Box::new(right),
                span: Span {
                    start: expr.span().start.clone(),
                    end: right.span().end.clone(),
                },
            });
        }

        Some(expr)
    }

    fn factor(&mut self) -> Option<Expression> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenKind::Star, TokenKind::Slash, TokenKind::Percent]) {
            let operator = self.previous();
            let right = self.unary()?;
            
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator.kind,
                right: Box::new(right),
                span: Span {
                    start: expr.span().start.clone(),
                    end: right.span().end.clone(),
                },
            });
        }

        Some(expr)
    }

    fn unary(&mut self) -> Option<Expression> {
        if self.match_token(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            
            return Some(Expression::Unary(UnaryExpression {
                operator: operator.kind,
                operand: Box::new(right),
                span: Span {
                    start: operator.location.clone(),
                    end: right.span().end.clone(),
                },
            }));
        }

        self.call()
    }

    fn call(&mut self) -> Option<Expression> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&[TokenKind::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(&[TokenKind::Dot]) {
                let name = self.consume(TokenKind::Identifier, ErrorCode::ExpectedIdentifier, "Expected property name");
                if name.is_none() {
                    return None;
                }
                let name = name.unwrap();
                
                expr = Expression::Member(MemberExpression {
                    object: Box::new(expr),
                    property: Identifier {
                        name: name.lexeme,
                        span: name.location.clone().into(),
                    },
                    span: Span {
                        start: expr.span().start.clone(),
                        end: name.location.clone(),
                    },
                });
            } else if self.match_token(&[TokenKind::LeftBracket]) {
                let index = self.expression()?;
                self.consume(TokenKind::RightBracket, ErrorCode::MissingRightBracket, "Expected ']' after index expression");
                
                expr = Expression::Index(IndexExpression {
                    object: Box::new(expr),
                    index: Box::new(index),
                    span: Span {
                        start: expr.span().start.clone(),
                        end: self.previous().location.clone(),
                    },
                });
            } else {
                break;
            }
        }

        Some(expr)
    }

    fn finish_call(&mut self, callee: Expression) -> Option<Expression> {
        let mut arguments = Vec::new();

        if !self.check(&TokenKind::RightParen) {
            loop {
                if let Some(arg) = self.expression() {
                    arguments.push(arg);
                } else {
                    self.diagnostics.add_error(
                        ErrorCode::ExpectedArgumentList,
                        "Expected argument in call expression".to_string(),
                        self.peek().location.clone(),
                        "unknown".to_string(),
                    );
                    return None;
                }

                if !self.match_token(&[TokenKind::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenKind::RightParen, ErrorCode::MissingRightParen, "Expected ')' after arguments");
        if paren.is_none() {
            return None;
        }

        Some(Expression::Call(CallExpression {
            callee: Box::new(callee),
            arguments,
            span: Span {
                start: callee.span().start.clone(),
                end: self.previous().location.clone(),
            },
        }))
    }

    fn primary(&mut self) -> Option<Expression> {
        if self.match_token(&[TokenKind::True]) {
            return Some(Expression::Literal(Literal {
                value: LiteralValue::Boolean(true),
                span: self.previous().location.clone().into(),
            }));
        }
        
        if self.match_token(&[TokenKind::False]) {
            return Some(Expression::Literal(Literal {
                value: LiteralValue::Boolean(false),
                span: self.previous().location.clone().into(),
            }));
        }
        
        if self.match_token(&[TokenKind::NumberLiteral]) {
            let token = self.previous();
            let value = token.lexeme.parse::<f64>().ok();
            if value.is_none() {
                self.diagnostics.add_error(
                    ErrorCode::InvalidNumber,
                    format!("Invalid number literal: {}", token.lexeme),
                    token.location.clone(),
                    "unknown".to_string(),
                );
                return None;
            }
            
            return Some(Expression::Literal(Literal {
                value: LiteralValue::Number(value.unwrap()),
                span: token.location.clone().into(),
            }));
        }
        
        if self.match_token(&[TokenKind::StringLiteral]) {
            let token = self.previous();
            return Some(Expression::Literal(Literal {
                value: LiteralValue::String(token.lexeme[1..token.lexeme.len()-1].to_string()), // Remove quotes
                span: token.location.clone().into(),
            }));
        }
        
        if self.match_token(&[TokenKind::Identifier]) {
            let token = self.previous();
            return Some(Expression::Variable(VariableExpression {
                name: Identifier {
                    name: token.lexeme.clone(),
                    span: token.location.clone().into(),
                },
                span: token.location.clone().into(),
            }));
        }
        
        if self.match_token(&[TokenKind::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenKind::RightParen, ErrorCode::MissingRightParen, "Expected ')' after expression");
            
            return Some(Expression::Grouping(GroupingExpression {
                expression: Box::new(expr),
                span: Span {
                    start: self.previous().location.clone(), // This is the closing paren
                    end: self.previous().location.clone(),
                },
            }));
        }

        self.diagnostics.add_error(
            ErrorCode::ExpectedExpression,
            format!("Expected expression, found {:?}", self.peek().kind),
            self.peek().location.clone(),
            "unknown".to_string(),
        );
        None
    }

    fn block_statement(&mut self) -> Option<Statement> {
        self.consume(TokenKind::LeftBrace, ErrorCode::ExpectedBlock, "Expected '{' before block");
        
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.statement() {
                statements.push(stmt);
            }
        }
        
        let end_brace = self.consume(TokenKind::RightBrace, ErrorCode::MissingRightBrace, "Expected '}' after block");
        if end_brace.is_none() {
            return None;
        }
        
        Some(Statement::Block(BlockStatement {
            statements,
            span: Span {
                start: self.previous().location.clone(), // This is the opening brace
                end: self.previous().location.clone(),   // This is the closing brace
            },
        }))
    }

    fn statement(&mut self) -> Option<Statement> {
        if self.match_token(&[TokenKind::LeftBrace]) {
            return self.block_statement();
        }
        
        if self.match_token(&[TokenKind::If]) {
            return self.if_statement();
        }
        
        if self.match_token(&[TokenKind::While]) {
            return self.while_statement();
        }
        
        if self.match_token(&[TokenKind::For]) {
            return self.for_statement();
        }
        
        if self.match_token(&[TokenKind::Return]) {
            return self.return_statement();
        }
        
        if self.match_token(&[TokenKind::Break]) {
            return Some(Statement::Break(BreakStatement {
                span: self.previous().location.clone().into(),
            }));
        }
        
        if self.match_token(&[TokenKind::Continue]) {
            return Some(Statement::Continue(ContinueStatement {
                span: self.previous().location.clone().into(),
            }));
        }
        
        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Option<Statement> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, ErrorCode::MissingSemicolon, "Expected ';' after expression");
        
        Some(Statement::Expression(ExpressionStatement {
            expression: expr,
            span: expr.span().clone(),
        }))
    }

    fn if_statement(&mut self) -> Option<Statement> {
        let if_token = self.previous();
        self.consume(TokenKind::LeftParen, ErrorCode::ExpectedDelimiter, "Expected '(' after 'if'");
        let condition = self.expression()?;
        self.consume(TokenKind::RightParen, ErrorCode::MissingRightParen, "Expected ')' after condition");
        
        let then_branch = Box::new(self.statement()?);
        
        let else_branch = if self.match_token(&[TokenKind::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        
        Some(Statement::If(IfStatement {
            condition,
            then_branch,
            else_branch,
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
        let while_token = self.previous();
        self.consume(TokenKind::LeftParen, ErrorCode::ExpectedDelimiter, "Expected '(' after 'while'");
        let condition = self.expression()?;
        self.consume(TokenKind::RightParen, ErrorCode::MissingRightParen, "Expected ')' after condition");
        
        let body = Box::new(self.statement()?);
        
        Some(Statement::While(WhileStatement {
            condition,
            body,
            span: Span {
                start: while_token.location.clone(),
                end: body.span().end.clone(),
            },
        }))
    }

    fn for_statement(&mut self) -> Option<Statement> {
        let for_token = self.previous();
        self.consume(TokenKind::LeftParen, ErrorCode::ExpectedDelimiter, "Expected '(' after 'for'");
        
        let initializer = if self.match_token(&[TokenKind::Semicolon]) {
            None
        } else if self.match_token(&[TokenKind::Var]) {
            // Handle variable declaration as initializer
            // For simplicity in this implementation, we'll treat it as an expression statement
            let init_stmt = self.expression_statement();
            self.consume(TokenKind::Semicolon, ErrorCode::MissingSemicolon, "Expected ';' after for loop initializer");
            init_stmt
        } else {
            let init_expr = self.expression_statement();
            self.consume(TokenKind::Semicolon, ErrorCode::MissingSemicolon, "Expected ';' after for loop initializer");
            init_expr
        };
        
        let condition = if self.check(&TokenKind::Semicolon) {
            // Empty condition
            self.advance(); // consume semicolon
            Expression::Literal(Literal {
                value: LiteralValue::Boolean(true),
                span: self.previous().location.clone().into(),
            })
        } else {
            let expr = self.expression()?;
            self.consume(TokenKind::Semicolon, ErrorCode::MissingSemicolon, "Expected ';' after for loop condition");
            expr
        };
        
        let increment = if self.check(&TokenKind::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        
        self.consume(TokenKind::RightParen, ErrorCode::MissingRightParen, "Expected ')' after for clauses");
        
        let body = Box::new(self.statement()?);
        
        Some(Statement::For(ForStatement {
            initializer,
            condition,
            increment,
            body,
            span: Span {
                start: for_token.location.clone(),
                end: body.span().end.clone(),
            },
        }))
    }

    fn return_statement(&mut self) -> Option<Statement> {
        let return_token = self.previous();
        let value = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            self.expression()
        };
        
        self.consume(TokenKind::Semicolon, ErrorCode::MissingSemicolon, "Expected ';' after return value");
        
        Some(Statement::Return(ReturnStatement {
            value,
            span: Span {
                start: return_token.location.clone(),
                end: self.previous().location.clone(),
            },
        }))
    }

    fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
        for &kind in kinds {
            if self.check(&kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind == *kind
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, kind: TokenKind, error_code: ErrorCode, message: &str) -> Option<Token> {
        if self.check(&kind) {
            return Some(self.advance());
        }

        self.diagnostics.add_error(
            error_code,
            message.to_string(),
            self.peek().location.clone(),
            "unknown".to_string(),
        );
        None
    }
}

// Helper trait to get span from AST nodes
impl AstNode {
    pub fn span(&self) -> &Span {
        match self {
            AstNode::Program(program) => &program.span,
            AstNode::EntityDeclaration(decl) => &decl.span,
            AstNode::RuleDeclaration(decl) => &decl.span,
            AstNode::FlowDeclaration(decl) => &decl.span,
            AstNode::ConstraintDeclaration(decl) => &decl.span,
            AstNode::Field(field) => &field.span,
            AstNode::RuleCondition(condition) => &condition.span,
            AstNode::RuleAction(action) => &action.span,
            AstNode::FlowStep(step) => &step.span,
            AstNode::ConstraintCondition(condition) => &condition.span,
            AstNode::ConstraintAction(action) => &action.span,
            AstNode::Expression(expr) => expr.span(),
            AstNode::Statement(stmt) => stmt.span(),
            AstNode::Literal(literal) => &literal.span,
            AstNode::Identifier(ident) => &ident.span,
            AstNode::BinaryExpression(expr) => &expr.span,
            AstNode::UnaryExpression(expr) => &expr.span,
            AstNode::GroupingExpression(expr) => &expr.span,
            AstNode::VariableExpression(expr) => &expr.span,
            AstNode::AssignmentExpression(expr) => &expr.span,
            AstNode::CallExpression(expr) => &expr.span,
            AstNode::MemberExpression(expr) => &expr.span,
            AstNode::IndexExpression(expr) => &expr.span,
        }
    }
}

impl Expression {
    pub fn span(&self) -> &Span {
        match self {
            Expression::Literal(literal) => &literal.span,
            Expression::Variable(var) => &var.span,
            Expression::Binary(binary) => &binary.span,
            Expression::Unary(unary) => &unary.span,
            Expression::Grouping(grouping) => &grouping.span,
            Expression::Assignment(assignment) => &assignment.span,
            Expression::Call(call) => &call.span,
            Expression::Member(member) => &member.span,
            Expression::Index(index) => &index.span,
        }
    }
}

impl Statement {
    pub fn span(&self) -> &Span {
        match self {
            Statement::Expression(expr) => &expr.span,
            Statement::Declaration(decl) => decl.span(),
            Statement::Block(block) => &block.span,
            Statement::If(if_stmt) => &if_stmt.span,
            Statement::While(while_stmt) => &while_stmt.span,
            Statement::For(for_stmt) => &for_stmt.span,
            Statement::Return(return_stmt) => &return_stmt.span,
            Statement::Break(break_stmt) => &break_stmt.span,
            Statement::Continue(continue_stmt) => &continue_stmt.span,
        }
    }
}

impl Declaration {
    pub fn span(&self) -> &Span {
        match self {
            Declaration::Entity(entity) => &entity.span,
            Declaration::Rule(rule) => &rule.span,
            Declaration::Flow(flow) => &flow.span,
            Declaration::Constraint(constraint) => &constraint.span,
        }
    }
}

impl From<SourceLocation> for Span {
    fn from(location: SourceLocation) -> Self {
        Span {
            start: location.clone(),
            end: location,
        }
    }
}