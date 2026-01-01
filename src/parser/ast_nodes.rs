use crate::shared::source_location::Span;
use crate::lexer::token_kind::TokenKind;

// Base AST node types
#[derive(Debug, Clone)]
pub enum AstNode {
    Program(Program),
    EntityDeclaration(EntityDeclaration),
    RuleDeclaration(RuleDeclaration),
    FlowDeclaration(FlowDeclaration),
    ConstraintDeclaration(ConstraintDeclaration),
    Field(Field),
    RuleCondition(RuleCondition),
    RuleAction(RuleAction),
    FlowStep(FlowStep),
    ConstraintCondition(ConstraintCondition),
    ConstraintAction(ConstraintAction),
    Expression(Expression),
    Statement(Statement),
    Literal(Literal),
    Identifier(Identifier),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    GroupingExpression(GroupingExpression),
    VariableExpression(VariableExpression),
    AssignmentExpression(AssignmentExpression),
    CallExpression(CallExpression),
    MemberExpression(MemberExpression),
    IndexExpression(IndexExpression),
}

// Program node
#[derive(Debug, Clone)]
pub struct Program {
    pub declarations: Vec<Declaration>,
    pub span: Span,
}

// Declaration types
#[derive(Debug, Clone)]
pub enum Declaration {
    Entity(EntityDeclaration),
    Rule(RuleDeclaration),
    Flow(FlowDeclaration),
    Constraint(ConstraintDeclaration),
}

#[derive(Debug, Clone)]
pub struct EntityDeclaration {
    pub name: Identifier,
    pub fields: Vec<Field>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct RuleDeclaration {
    pub name: Identifier,
    pub condition: RuleCondition,
    pub action: RuleAction,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FlowDeclaration {
    pub name: Identifier,
    pub steps: Vec<FlowStep>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ConstraintDeclaration {
    pub name: Identifier,
    pub condition: ConstraintCondition,
    pub action: ConstraintAction,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: Identifier,
    pub field_type: Type,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Type {
    Sym,
    Num,
    Bool,
    Vec,
    Ref,
    Ctx,
    Identifier(Identifier),
}

#[derive(Debug, Clone)]
pub struct RuleCondition {
    pub expression: Expression,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct RuleAction {
    pub statements: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FlowStep {
    pub expression: Expression,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ConstraintCondition {
    pub expression: Expression,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ConstraintAction {
    pub statements: Vec<Statement>,
    pub span: Span,
}

// Expression types
#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Variable(VariableExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Grouping(GroupingExpression),
    Assignment(AssignmentExpression),
    Call(CallExpression),
    Member(MemberExpression),
    Index(IndexExpression),
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: LiteralValue,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct VariableExpression {
    pub name: Identifier,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: TokenKind,
    pub right: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub operator: TokenKind,
    pub operand: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct GroupingExpression {
    pub expression: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    pub target: Box<Expression>,
    pub value: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MemberExpression {
    pub object: Box<Expression>,
    pub property: Identifier,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct IndexExpression {
    pub object: Box<Expression>,
    pub index: Box<Expression>,
    pub span: Span,
}

// Statement types
#[derive(Debug, Clone)]
pub enum Statement {
    Expression(ExpressionStatement),
    Declaration(Declaration),
    Block(BlockStatement),
    If(IfStatement),
    While(Box<WhileStatement>),
    For(Box<ForStatement>),
    Return(Box<ReturnStatement>),
    Break(BreakStatement),
    Continue(ContinueStatement),
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ForStatement {
    pub initializer: Option<Box<Statement>>,
    pub condition: Expression,
    pub increment: Option<Expression>,
    pub body: Box<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BreakStatement {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ContinueStatement {
    pub span: Span,
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

impl From<crate::shared::source_location::SourceLocation> for Span {
    fn from(location: crate::shared::source_location::SourceLocation) -> Self {
        Span {
            start: location.clone(),
            end: location,
        }
    }
}