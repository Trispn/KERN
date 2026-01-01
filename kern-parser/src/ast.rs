#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Program(Program),
    EntityDef(EntityDef),
    RuleDef(RuleDef),
    FlowDef(FlowDef),
    ConstraintDef(ConstraintDef),
    FieldDef(FieldDef),
    Condition(Condition),
    Expression(Expression),
    Term(Term),
    Predicate(Predicate),
    Action(Action),
    IfAction(IfAction),
    LoopAction(LoopAction),
    HaltAction(HaltAction),
    Assignment(Assignment),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    Entity(EntityDef),
    Rule(RuleDef),
    Flow(FlowDef),
    Constraint(ConstraintDef),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityDef {
    pub name: String,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDef {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuleDef {
    pub name: String,
    pub condition: Condition,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    Expression(Expression),
    LogicalOp(Box<Condition>, LogicalOp, Box<Condition>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Comparison {
        left: Box<Term>,
        op: Comparator,
        right: Box<Term>,
    },
    Predicate(Predicate),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Comparator {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Identifier(String),
    Number(i64),
    QualifiedRef(String, String), // entity.field
}

#[derive(Debug, Clone, PartialEq)]
pub struct Predicate {
    pub name: String,
    pub arguments: Vec<Term>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Predicate(Predicate),
    Assignment(Assignment),
    Control(ControlAction),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControlAction {
    If(IfAction),
    Loop(LoopAction),
    Halt(HaltAction),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub variable: String,
    pub value: Term,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfAction {
    pub condition: Condition,
    pub then_actions: Vec<Action>,
    pub else_actions: Option<Vec<Action>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopAction {
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HaltAction;

#[derive(Debug, Clone, PartialEq)]
pub struct FlowDef {
    pub name: String,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintDef {
    pub name: String,
    pub condition: Condition,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_structures() {
        // Test creating a simple entity definition
        let entity = EntityDef {
            name: "Farmer".to_string(),
            fields: vec![
                FieldDef { name: "id".to_string() },
                FieldDef { name: "location".to_string() },
            ],
        };
        
        let definition = Definition::Entity(entity);
        assert!(matches!(definition, Definition::Entity(_)));
    }
}