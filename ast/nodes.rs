use crate::shared::source_location::Span;
use crate::lexer::token::TokenKind;

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
    While(WhileStatement),
    For(ForStatement),
    Return(ReturnStatement),
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
    pub initializer: Option<Statement>,
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