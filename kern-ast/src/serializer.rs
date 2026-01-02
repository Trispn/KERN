use crate::constraint_node::SeverityLevel;
use crate::entity_node::{AttributeNode, ConstraintRefNode, RuleRefNode};
use crate::flow_node::FlowStepNode;
use crate::rule_node::{ActionNode, AssignActionNode, EmitActionNode, ParameterNode};
use crate::{
    BinaryExprNode, BinaryOperator, CallExprNode, ConstraintNode, EntityNode, ExpressionNode,
    FlowNode, IdentifierExprNode, IdentifierNode, LiteralExprNode, LiteralValue, ProgramNode,
    RuleNode, TypeNode, UnaryExprNode, UnaryOperator,
};
use std::io::{Result as IoResult, Write};

const KAST_MAGIC: &[u8] = b"KAST";
const KAST_VERSION: u16 = 1;

/// FieldKind represents the type of a field in the serialized AST
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum FieldKind {
    NodeRef = 0,
    NodeList = 1,
    StringId = 2,
    Int = 3,
    Bool = 4,
    Enum = 5,
}

impl FieldKind {
    fn as_u8(&self) -> u8 {
        match self {
            FieldKind::NodeRef => 0,
            FieldKind::NodeList => 1,
            FieldKind::StringId => 2,
            FieldKind::Int => 3,
            FieldKind::Bool => 4,
            FieldKind::Enum => 5,
        }
    }
}

/// NodeKind represents the type of an AST node
#[derive(Debug, Clone, Copy)]
enum NodeKind {
    Program = 0,
    Entity = 1,
    Attribute = 2,
    Rule = 3,
    Parameter = 4,
    Flow = 5,
    FlowStep = 6,
    Constraint = 7,
    BinaryExpr = 8,
    UnaryExpr = 9,
    LiteralExpr = 10,
    IdentifierExpr = 11,
    CallExpr = 12,
    AssignAction = 13,
    EmitAction = 14,
    Type = 15,
    Identifier = 16,
    RuleRef = 17,
    ConstraintRef = 18,
}

impl NodeKind {
    fn as_u16(&self) -> u16 {
        match self {
            NodeKind::Program => 0,
            NodeKind::Entity => 1,
            NodeKind::Attribute => 2,
            NodeKind::Rule => 3,
            NodeKind::Parameter => 4,
            NodeKind::Flow => 5,
            NodeKind::FlowStep => 6,
            NodeKind::Constraint => 7,
            NodeKind::BinaryExpr => 8,
            NodeKind::UnaryExpr => 9,
            NodeKind::LiteralExpr => 10,
            NodeKind::IdentifierExpr => 11,
            NodeKind::CallExpr => 12,
            NodeKind::AssignAction => 13,
            NodeKind::EmitAction => 14,
            NodeKind::Type => 15,
            NodeKind::Identifier => 16,
            NodeKind::RuleRef => 17,
            NodeKind::ConstraintRef => 18,
        }
    }
}

/// Serializes an AST to binary format
pub fn serialize_ast<W: Write>(writer: &mut W, program: &ProgramNode) -> IoResult<()> {
    // Write header
    writer.write_all(KAST_MAGIC)?;
    writer.write_all(&KAST_VERSION.to_le_bytes())?;

    // For simplicity in this implementation, we'll serialize the AST directly
    // In a real implementation, we would build a symbol table and serialize
    // references to nodes and strings

    // Serialize the program node
    serialize_program_node(writer, program)?;

    Ok(())
}

fn serialize_program_node<W: Write>(writer: &mut W, node: &ProgramNode) -> IoResult<()> {
    // Write node kind
    writer.write_all(&NodeKind::Program.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 4u16; // entities, rules, flows, constraints
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize entities
    writer.write_all(&FieldKind::NodeList.as_u8().to_le_bytes())?;
    let entity_count = node.entities.len() as u32;
    writer.write_all(&entity_count.to_le_bytes())?;
    for entity in &node.entities {
        serialize_entity_node(writer, entity)?;
    }

    // Serialize rules
    writer.write_all(&FieldKind::NodeList.as_u8().to_le_bytes())?;
    let rule_count = node.rules.len() as u32;
    writer.write_all(&rule_count.to_le_bytes())?;
    for rule in &node.rules {
        serialize_rule_node(writer, rule)?;
    }

    // Serialize flows
    writer.write_all(&FieldKind::NodeList.as_u8().to_le_bytes())?;
    let flow_count = node.flows.len() as u32;
    writer.write_all(&flow_count.to_le_bytes())?;
    for flow in &node.flows {
        serialize_flow_node(writer, flow)?;
    }

    // Serialize constraints
    writer.write_all(&FieldKind::NodeList.as_u8().to_le_bytes())?;
    let constraint_count = node.constraints.len() as u32;
    writer.write_all(&constraint_count.to_le_bytes())?;
    for constraint in &node.constraints {
        serialize_constraint_node(writer, constraint)?;
    }

    Ok(())
}

fn serialize_entity_node<W: Write>(writer: &mut W, node: &EntityNode) -> IoResult<()> {
    writer.write_all(&NodeKind::Entity.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 4u16; // name, attributes, rules, constraints
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize name
    serialize_identifier_node(writer, &node.name)?;

    // Serialize attributes
    writer.write_all(&FieldKind::NodeList.as_u8().to_le_bytes())?;
    let attr_count = node.attributes.len() as u32;
    writer.write_all(&attr_count.to_le_bytes())?;
    for attr in &node.attributes {
        serialize_attribute_node(writer, attr)?;
    }

    // Serialize rules
    writer.write_all(&FieldKind::NodeList.as_u8().to_le_bytes())?;
    let rule_ref_count = node.rules.len() as u32;
    writer.write_all(&rule_ref_count.to_le_bytes())?;
    for rule_ref in &node.rules {
        serialize_rule_ref_node(writer, rule_ref)?;
    }

    // Serialize constraints
    writer.write_all(&FieldKind::NodeList.as_u8().to_le_bytes())?;
    let constraint_ref_count = node.constraints.len() as u32;
    writer.write_all(&constraint_ref_count.to_le_bytes())?;
    for constraint_ref in &node.constraints {
        serialize_constraint_ref_node(writer, constraint_ref)?;
    }

    Ok(())
}

fn serialize_attribute_node<W: Write>(writer: &mut W, node: &AttributeNode) -> IoResult<()> {
    writer.write_all(&NodeKind::Attribute.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 3u16; // name, type, default_value
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize name
    serialize_identifier_node(writer, &node.name)?;

    // Serialize type
    serialize_type_node(writer, &node.r#type)?;

    // Serialize default value (if present)
    match &node.default_value {
        Some(expr) => {
            writer.write_all(&FieldKind::NodeRef.as_u8().to_le_bytes())?;
            serialize_expression_node(writer, expr)?;
        }
        None => {
            // In a real implementation, we would have a null node reference
            // For now, we'll just skip
        }
    }

    Ok(())
}

fn serialize_rule_node<W: Write>(writer: &mut W, node: &RuleNode) -> IoResult<()> {
    writer.write_all(&NodeKind::Rule.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 4u16; // name, parameters, condition, actions
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize name
    serialize_identifier_node(writer, &node.name)?;

    // Serialize parameters
    writer.write_all(&FieldKind::NodeList.as_u8().to_le_bytes())?;
    let param_count = node.parameters.len() as u32;
    writer.write_all(&param_count.to_le_bytes())?;
    for param in &node.parameters {
        serialize_parameter_node(writer, param)?;
    }

    // Serialize condition
    writer.write_all(&FieldKind::NodeRef.as_u8().to_le_bytes())?;
    serialize_expression_node(writer, &node.condition)?;

    // Serialize actions
    writer.write_all(&FieldKind::NodeList.as_u8().to_le_bytes())?;
    let action_count = node.actions.len() as u32;
    writer.write_all(&action_count.to_le_bytes())?;
    for action in &node.actions {
        serialize_action_node(writer, action)?;
    }

    Ok(())
}

fn serialize_flow_node<W: Write>(writer: &mut W, node: &FlowNode) -> IoResult<()> {
    writer.write_all(&NodeKind::Flow.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 2u16; // name, steps
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize name
    serialize_identifier_node(writer, &node.name)?;

    // Serialize steps
    writer.write_all(&FieldKind::NodeList.as_u8().to_le_bytes())?;
    let step_count = node.steps.len() as u32;
    writer.write_all(&step_count.to_le_bytes())?;
    for step in &node.steps {
        serialize_flow_step_node(writer, step)?;
    }

    Ok(())
}

fn serialize_constraint_node<W: Write>(writer: &mut W, node: &ConstraintNode) -> IoResult<()> {
    writer.write_all(&NodeKind::Constraint.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 3u16; // name, expression, severity
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize name
    serialize_identifier_node(writer, &node.name)?;

    // Serialize expression
    writer.write_all(&FieldKind::NodeRef.as_u8().to_le_bytes())?;
    serialize_expression_node(writer, &node.expression)?;

    // Serialize severity
    writer.write_all(&FieldKind::Enum.as_u8().to_le_bytes())?;
    let severity_code = match node.severity {
        SeverityLevel::Info => 0u8,
        SeverityLevel::Warn => 1u8,
        SeverityLevel::Error => 2u8,
        SeverityLevel::Fatal => 3u8,
    };
    writer.write_all(&severity_code.to_le_bytes())?;

    Ok(())
}

fn serialize_expression_node<W: Write>(writer: &mut W, node: &ExpressionNode) -> IoResult<()> {
    match node {
        ExpressionNode::Binary(binary) => serialize_binary_expr_node(writer, binary),
        ExpressionNode::Unary(unary) => serialize_unary_expr_node(writer, unary),
        ExpressionNode::Literal(literal) => serialize_literal_expr_node(writer, literal),
        ExpressionNode::Identifier(ident) => serialize_identifier_expr_node(writer, ident),
        ExpressionNode::Call(call) => serialize_call_expr_node(writer, call),
    }
}

fn serialize_binary_expr_node<W: Write>(writer: &mut W, node: &BinaryExprNode) -> IoResult<()> {
    writer.write_all(&NodeKind::BinaryExpr.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 3u16; // left, operator, right
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize left
    writer.write_all(&FieldKind::NodeRef.as_u8().to_le_bytes())?;
    serialize_expression_node(writer, &node.left)?;

    // Serialize operator
    writer.write_all(&FieldKind::Enum.as_u8().to_le_bytes())?;
    let op_code = match node.operator {
        BinaryOperator::Add => 0u8,
        BinaryOperator::Sub => 1u8,
        BinaryOperator::Mul => 2u8,
        BinaryOperator::Div => 3u8,
        BinaryOperator::Mod => 4u8,
        BinaryOperator::Equal => 5u8,
        BinaryOperator::NotEqual => 6u8,
        BinaryOperator::Less => 7u8,
        BinaryOperator::LessEqual => 8u8,
        BinaryOperator::Greater => 9u8,
        BinaryOperator::GreaterEqual => 10u8,
        BinaryOperator::And => 11u8,
        BinaryOperator::Or => 12u8,
        BinaryOperator::Xor => 13u8,
    };
    writer.write_all(&op_code.to_le_bytes())?;

    // Serialize right
    writer.write_all(&FieldKind::NodeRef.as_u8().to_le_bytes())?;
    serialize_expression_node(writer, &node.right)?;

    Ok(())
}

fn serialize_unary_expr_node<W: Write>(writer: &mut W, node: &UnaryExprNode) -> IoResult<()> {
    writer.write_all(&NodeKind::UnaryExpr.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 2u16; // operator, operand
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize operator
    writer.write_all(&FieldKind::Enum.as_u8().to_le_bytes())?;
    let op_code = match node.operator {
        UnaryOperator::Neg => 0u8,
        UnaryOperator::Not => 1u8,
        UnaryOperator::Pos => 2u8,
    };
    writer.write_all(&op_code.to_le_bytes())?;

    // Serialize operand
    writer.write_all(&FieldKind::NodeRef.as_u8().to_le_bytes())?;
    serialize_expression_node(writer, &node.operand)?;

    Ok(())
}

fn serialize_literal_expr_node<W: Write>(writer: &mut W, node: &LiteralExprNode) -> IoResult<()> {
    writer.write_all(&NodeKind::LiteralExpr.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 1u16; // value
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize value
    writer.write_all(&FieldKind::Enum.as_u8().to_le_bytes())?;
    match &node.value {
        LiteralValue::Integer(i) => {
            writer.write_all(&0u8.to_le_bytes())?; // Integer type
            writer.write_all(&i.to_le_bytes())?;
        }
        LiteralValue::Float(f) => {
            writer.write_all(&1u8.to_le_bytes())?; // Float type
            writer.write_all(&f.to_bits().to_le_bytes())?;
        }
        LiteralValue::String(s) => {
            writer.write_all(&2u8.to_le_bytes())?; // String type
            let bytes = s.as_bytes();
            writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
            writer.write_all(bytes)?;
        }
        LiteralValue::Boolean(b) => {
            writer.write_all(&3u8.to_le_bytes())?; // Boolean type
            writer.write_all(&[*b as u8])?;
        }
        LiteralValue::Null => {
            writer.write_all(&4u8.to_le_bytes())?; // Null type
        }
    }

    Ok(())
}

fn serialize_identifier_expr_node<W: Write>(
    writer: &mut W,
    node: &IdentifierExprNode,
) -> IoResult<()> {
    writer.write_all(&NodeKind::IdentifierExpr.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 1u16; // name
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize name
    serialize_identifier_node(writer, &node.name)?;

    Ok(())
}

fn serialize_call_expr_node<W: Write>(writer: &mut W, node: &CallExprNode) -> IoResult<()> {
    writer.write_all(&NodeKind::CallExpr.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 2u16; // callee, args
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize callee
    serialize_identifier_node(writer, &node.callee)?;

    // Serialize args
    writer.write_all(&FieldKind::NodeList.as_u8().to_le_bytes())?;
    let arg_count = node.args.len() as u32;
    writer.write_all(&arg_count.to_le_bytes())?;
    for arg in &node.args {
        serialize_expression_node(writer, arg)?;
    }

    Ok(())
}

fn serialize_action_node<W: Write>(writer: &mut W, node: &ActionNode) -> IoResult<()> {
    match node {
        ActionNode::Assign(assign) => serialize_assign_action_node(writer, assign),
        ActionNode::Emit(emit) => serialize_emit_action_node(writer, emit),
    }
}

fn serialize_assign_action_node<W: Write>(writer: &mut W, node: &AssignActionNode) -> IoResult<()> {
    writer.write_all(&NodeKind::AssignAction.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 2u16; // target, value
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize target
    serialize_identifier_node(writer, &node.target)?;

    // Serialize value
    writer.write_all(&FieldKind::NodeRef.as_u8().to_le_bytes())?;
    serialize_expression_node(writer, &node.value)?;

    Ok(())
}

fn serialize_emit_action_node<W: Write>(writer: &mut W, node: &EmitActionNode) -> IoResult<()> {
    writer.write_all(&NodeKind::EmitAction.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 1u16; // event
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize event
    serialize_identifier_node(writer, &node.event)?;

    Ok(())
}

fn serialize_parameter_node<W: Write>(writer: &mut W, node: &ParameterNode) -> IoResult<()> {
    writer.write_all(&NodeKind::Parameter.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 2u16; // name, type
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize name
    serialize_identifier_node(writer, &node.name)?;

    // Serialize type
    serialize_type_node(writer, &node.r#type)?;

    Ok(())
}

fn serialize_flow_step_node<W: Write>(writer: &mut W, node: &FlowStepNode) -> IoResult<()> {
    writer.write_all(&NodeKind::FlowStep.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 3u16; // from, to, condition
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize from
    serialize_identifier_node(writer, &node.from)?;

    // Serialize to
    serialize_identifier_node(writer, &node.to)?;

    // Serialize condition (if present)
    match &node.condition {
        Some(expr) => {
            writer.write_all(&FieldKind::NodeRef.as_u8().to_le_bytes())?;
            serialize_expression_node(writer, expr)?;
        }
        None => {
            // In a real implementation, we would have a null node reference
            // For now, we'll just skip
        }
    }

    Ok(())
}

fn serialize_type_node<W: Write>(writer: &mut W, node: &TypeNode) -> IoResult<()> {
    writer.write_all(&NodeKind::Type.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 2u16; // name, nullable
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize name
    serialize_identifier_node(writer, &node.name)?;

    // Serialize nullable
    writer.write_all(&FieldKind::Bool.as_u8().to_le_bytes())?;
    writer.write_all(&[node.nullable as u8])?;

    Ok(())
}

fn serialize_identifier_node<W: Write>(writer: &mut W, node: &IdentifierNode) -> IoResult<()> {
    writer.write_all(&NodeKind::Identifier.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 1u16; // text
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize text
    writer.write_all(&FieldKind::StringId.as_u8().to_le_bytes())?;
    let bytes = node.text.as_bytes();
    writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
    writer.write_all(bytes)?;

    Ok(())
}

fn serialize_rule_ref_node<W: Write>(writer: &mut W, node: &RuleRefNode) -> IoResult<()> {
    writer.write_all(&NodeKind::RuleRef.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 1u16; // name
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize name
    serialize_identifier_node(writer, &node.name)?;

    Ok(())
}

fn serialize_constraint_ref_node<W: Write>(
    writer: &mut W,
    node: &ConstraintRefNode,
) -> IoResult<()> {
    writer.write_all(&NodeKind::ConstraintRef.as_u16().to_le_bytes())?;

    // Write location
    writer.write_all(&node.location.file_id.to_le_bytes())?;
    writer.write_all(&node.location.line.to_le_bytes())?;
    writer.write_all(&node.location.column.to_le_bytes())?;
    writer.write_all(&node.location.length.to_le_bytes())?;

    // Write field count
    let field_count = 1u16; // name
    writer.write_all(&field_count.to_le_bytes())?;

    // Serialize name
    serialize_identifier_node(writer, &node.name)?;

    Ok(())
}
