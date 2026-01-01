use crate::{ProgramNode, SourceLocation, IdentifierNode, TypeNode, ExpressionNode, 
           EntityNode, RuleNode, FlowNode, ConstraintNode, ActionNode, ParameterNode, 
           FlowStepNode, AttributeNode, RuleRefNode, ConstraintRefNode, 
           AssignActionNode, EmitActionNode, BinaryExprNode, UnaryExprNode, 
           LiteralExprNode, IdentifierExprNode, CallExprNode, LiteralValue, 
           BinaryOperator, UnaryOperator, SeverityLevel};
use std::io::{Read, Result as IoResult, Error as IoError, ErrorKind};

/// Deserializes an AST from binary format
pub fn deserialize_ast<R: Read>(reader: &mut R) -> IoResult<ProgramNode> {
    // Read and validate header
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;
    
    if &magic != b"KAST" {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid magic number"));
    }
    
    let mut version_bytes = [0u8; 2];
    reader.read_exact(&mut version_bytes)?;
    let version = u16::from_le_bytes(version_bytes);
    
    if version != 1 {
        return Err(IoError::new(ErrorKind::InvalidData, "Unsupported version"));
    }
    
    // Deserialize the program node
    deserialize_program_node(reader)
}

fn deserialize_program_node<R: Read>(reader: &mut R) -> IoResult<ProgramNode> {
    // Read node kind (should be Program)
    let mut kind_bytes = [0u8; 2];
    reader.read_exact(&mut kind_bytes)?;
    let kind = u16::from_le_bytes(kind_bytes);
    
    if kind != 0 { // NodeKind::Program as u16
        return Err(IoError::new(ErrorKind::InvalidData, "Expected Program node"));
    }
    
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 4 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for Program node"));
    }
    
    // Deserialize entities
    let entities = deserialize_node_list(reader)?;
    
    // Deserialize rules
    let rules = deserialize_node_list(reader)?;
    
    // Deserialize flows
    let flows = deserialize_node_list(reader)?;
    
    // Deserialize constraints
    let constraints = deserialize_node_list(reader)?;
    
    Ok(ProgramNode {
        entities,
        rules,
        flows,
        constraints,
        location,
    })
}

fn deserialize_node_list<R: Read>(reader: &mut R) -> IoResult<Vec<EntityNode>> {
    // Read field kind (should be NodeList)
    let mut field_kind_byte = [0u8; 1];
    reader.read_exact(&mut field_kind_byte)?;
    
    if field_kind_byte[0] != 1 { // FieldKind::NodeList as u8
        return Err(IoError::new(ErrorKind::InvalidData, "Expected NodeList field"));
    }
    
    // Read count
    let mut count_bytes = [0u8; 4];
    reader.read_exact(&mut count_bytes)?;
    let count = u32::from_le_bytes(count_bytes);
    
    let mut entities = Vec::with_capacity(count as usize);
    for _ in 0..count {
        entities.push(deserialize_entity_node(reader)?);
    }
    
    Ok(entities)
}

fn deserialize_entity_node<R: Read>(reader: &mut R) -> IoResult<EntityNode> {
    // Read node kind (should be Entity)
    let mut kind_bytes = [0u8; 2];
    reader.read_exact(&mut kind_bytes)?;
    let kind = u16::from_le_bytes(kind_bytes);
    
    if kind != 1 { // NodeKind::Entity as u16
        return Err(IoError::new(ErrorKind::InvalidData, "Expected Entity node"));
    }
    
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 4 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for Entity node"));
    }
    
    // Deserialize name
    let name = deserialize_identifier_node(reader)?;
    
    // Deserialize attributes
    let attributes = deserialize_attribute_list(reader)?;
    
    // Deserialize rules
    let rules = deserialize_rule_ref_list(reader)?;
    
    // Deserialize constraints
    let constraints = deserialize_constraint_ref_list(reader)?;
    
    Ok(EntityNode {
        name,
        attributes,
        rules,
        constraints,
        location,
    })
}

fn deserialize_attribute_list<R: Read>(reader: &mut R) -> IoResult<Vec<AttributeNode>> {
    // Read field kind (should be NodeList)
    let mut field_kind_byte = [0u8; 1];
    reader.read_exact(&mut field_kind_byte)?;
    
    if field_kind_byte[0] != 1 { // FieldKind::NodeList as u8
        return Err(IoError::new(ErrorKind::InvalidData, "Expected NodeList field"));
    }
    
    // Read count
    let mut count_bytes = [0u8; 4];
    reader.read_exact(&mut count_bytes)?;
    let count = u32::from_le_bytes(count_bytes);
    
    let mut attributes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        attributes.push(deserialize_attribute_node(reader)?);
    }
    
    Ok(attributes)
}

fn deserialize_rule_ref_list<R: Read>(reader: &mut R) -> IoResult<Vec<RuleRefNode>> {
    // Read field kind (should be NodeList)
    let mut field_kind_byte = [0u8; 1];
    reader.read_exact(&mut field_kind_byte)?;
    
    if field_kind_byte[0] != 1 { // FieldKind::NodeList as u8
        return Err(IoError::new(ErrorKind::InvalidData, "Expected NodeList field"));
    }
    
    // Read count
    let mut count_bytes = [0u8; 4];
    reader.read_exact(&mut count_bytes)?;
    let count = u32::from_le_bytes(count_bytes);
    
    let mut rule_refs = Vec::with_capacity(count as usize);
    for _ in 0..count {
        rule_refs.push(deserialize_rule_ref_node(reader)?);
    }
    
    Ok(rule_refs)
}

fn deserialize_constraint_ref_list<R: Read>(reader: &mut R) -> IoResult<Vec<ConstraintRefNode>> {
    // Read field kind (should be NodeList)
    let mut field_kind_byte = [0u8; 1];
    reader.read_exact(&mut field_kind_byte)?;
    
    if field_kind_byte[0] != 1 { // FieldKind::NodeList as u8
        return Err(IoError::new(ErrorKind::InvalidData, "Expected NodeList field"));
    }
    
    // Read count
    let mut count_bytes = [0u8; 4];
    reader.read_exact(&mut count_bytes)?;
    let count = u32::from_le_bytes(count_bytes);
    
    let mut constraint_refs = Vec::with_capacity(count as usize);
    for _ in 0..count {
        constraint_refs.push(deserialize_constraint_ref_node(reader)?);
    }
    
    Ok(constraint_refs)
}

fn deserialize_attribute_node<R: Read>(reader: &mut R) -> IoResult<AttributeNode> {
    // Read node kind (should be Attribute)
    let mut kind_bytes = [0u8; 2];
    reader.read_exact(&mut kind_bytes)?;
    let kind = u16::from_le_bytes(kind_bytes);
    
    if kind != 2 { // NodeKind::Attribute as u16
        return Err(IoError::new(ErrorKind::InvalidData, "Expected Attribute node"));
    }
    
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count < 2 || field_count > 3 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for Attribute node"));
    }
    
    // Deserialize name
    let name = deserialize_identifier_node(reader)?;
    
    // Deserialize type
    let r#type = deserialize_type_node(reader)?;
    
    // Deserialize default value (if present)
    let default_value = if field_count == 3 {
        Some(deserialize_expression_node(reader)?)
    } else {
        None
    };
    
    Ok(AttributeNode {
        name,
        r#type,
        default_value,
        location,
    })
}

fn deserialize_rule_ref_node<R: Read>(reader: &mut R) -> IoResult<RuleRefNode> {
    // Read node kind (should be RuleRef)
    let mut kind_bytes = [0u8; 2];
    reader.read_exact(&mut kind_bytes)?;
    let kind = u16::from_le_bytes(kind_bytes);
    
    if kind != 17 { // NodeKind::RuleRef as u16
        return Err(IoError::new(ErrorKind::InvalidData, "Expected RuleRef node"));
    }
    
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 1 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for RuleRef node"));
    }
    
    // Deserialize name
    let name = deserialize_identifier_node(reader)?;
    
    Ok(RuleRefNode { name, location })
}

fn deserialize_constraint_ref_node<R: Read>(reader: &mut R) -> IoResult<ConstraintRefNode> {
    // Read node kind (should be ConstraintRef)
    let mut kind_bytes = [0u8; 2];
    reader.read_exact(&mut kind_bytes)?;
    let kind = u16::from_le_bytes(kind_bytes);
    
    if kind != 18 { // NodeKind::ConstraintRef as u16
        return Err(IoError::new(ErrorKind::InvalidData, "Expected ConstraintRef node"));
    }
    
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 1 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for ConstraintRef node"));
    }
    
    // Deserialize name
    let name = deserialize_identifier_node(reader)?;
    
    Ok(ConstraintRefNode { name, location })
}

fn deserialize_rule_node<R: Read>(reader: &mut R) -> IoResult<RuleNode> {
    // Read node kind (should be Rule)
    let mut kind_bytes = [0u8; 2];
    reader.read_exact(&mut kind_bytes)?;
    let kind = u16::from_le_bytes(kind_bytes);
    
    if kind != 3 { // NodeKind::Rule as u16
        return Err(IoError::new(ErrorKind::InvalidData, "Expected Rule node"));
    }
    
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 4 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for Rule node"));
    }
    
    // Deserialize name
    let name = deserialize_identifier_node(reader)?;
    
    // Deserialize parameters
    let parameters = deserialize_parameter_list(reader)?;
    
    // Deserialize condition
    let condition = deserialize_expression_node(reader)?;
    
    // Deserialize actions
    let actions = deserialize_action_list(reader)?;
    
    Ok(RuleNode {
        name,
        parameters,
        condition,
        actions,
        location,
    })
}

fn deserialize_parameter_list<R: Read>(reader: &mut R) -> IoResult<Vec<ParameterNode>> {
    // Read field kind (should be NodeList)
    let mut field_kind_byte = [0u8; 1];
    reader.read_exact(&mut field_kind_byte)?;
    
    if field_kind_byte[0] != 1 { // FieldKind::NodeList as u8
        return Err(IoError::new(ErrorKind::InvalidData, "Expected NodeList field"));
    }
    
    // Read count
    let mut count_bytes = [0u8; 4];
    reader.read_exact(&mut count_bytes)?;
    let count = u32::from_le_bytes(count_bytes);
    
    let mut parameters = Vec::with_capacity(count as usize);
    for _ in 0..count {
        parameters.push(deserialize_parameter_node(reader)?);
    }
    
    Ok(parameters)
}

fn deserialize_action_list<R: Read>(reader: &mut R) -> IoResult<Vec<ActionNode>> {
    // Read field kind (should be NodeList)
    let mut field_kind_byte = [0u8; 1];
    reader.read_exact(&mut field_kind_byte)?;
    
    if field_kind_byte[0] != 1 { // FieldKind::NodeList as u8
        return Err(IoError::new(ErrorKind::InvalidData, "Expected NodeList field"));
    }
    
    // Read count
    let mut count_bytes = [0u8; 4];
    reader.read_exact(&mut count_bytes)?;
    let count = u32::from_le_bytes(count_bytes);
    
    let mut actions = Vec::with_capacity(count as usize);
    for _ in 0..count {
        actions.push(deserialize_action_node(reader)?);
    }
    
    Ok(actions)
}

fn deserialize_parameter_node<R: Read>(reader: &mut R) -> IoResult<ParameterNode> {
    // Read node kind (should be Parameter)
    let mut kind_bytes = [0u8; 2];
    reader.read_exact(&mut kind_bytes)?;
    let kind = u16::from_le_bytes(kind_bytes);
    
    if kind != 4 { // NodeKind::Parameter as u16
        return Err(IoError::new(ErrorKind::InvalidData, "Expected Parameter node"));
    }
    
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 2 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for Parameter node"));
    }
    
    // Deserialize name
    let name = deserialize_identifier_node(reader)?;
    
    // Deserialize type
    let r#type = deserialize_type_node(reader)?;
    
    Ok(ParameterNode { name, r#type, location })
}

fn deserialize_flow_node<R: Read>(reader: &mut R) -> IoResult<FlowNode> {
    // Read node kind (should be Flow)
    let mut kind_bytes = [0u8; 2];
    reader.read_exact(&mut kind_bytes)?;
    let kind = u16::from_le_bytes(kind_bytes);
    
    if kind != 5 { // NodeKind::Flow as u16
        return Err(IoError::new(ErrorKind::InvalidData, "Expected Flow node"));
    }
    
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 2 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for Flow node"));
    }
    
    // Deserialize name
    let name = deserialize_identifier_node(reader)?;
    
    // Deserialize steps
    let steps = deserialize_flow_step_list(reader)?;
    
    Ok(FlowNode { name, steps, location })
}

fn deserialize_flow_step_list<R: Read>(reader: &mut R) -> IoResult<Vec<FlowStepNode>> {
    // Read field kind (should be NodeList)
    let mut field_kind_byte = [0u8; 1];
    reader.read_exact(&mut field_kind_byte)?;
    
    if field_kind_byte[0] != 1 { // FieldKind::NodeList as u8
        return Err(IoError::new(ErrorKind::InvalidData, "Expected NodeList field"));
    }
    
    // Read count
    let mut count_bytes = [0u8; 4];
    reader.read_exact(&mut count_bytes)?;
    let count = u32::from_le_bytes(count_bytes);
    
    let mut steps = Vec::with_capacity(count as usize);
    for _ in 0..count {
        steps.push(deserialize_flow_step_node(reader)?);
    }
    
    Ok(steps)
}

fn deserialize_constraint_node<R: Read>(reader: &mut R) -> IoResult<ConstraintNode> {
    // Read node kind (should be Constraint)
    let mut kind_bytes = [0u8; 2];
    reader.read_exact(&mut kind_bytes)?;
    let kind = u16::from_le_bytes(kind_bytes);
    
    if kind != 7 { // NodeKind::Constraint as u16
        return Err(IoError::new(ErrorKind::InvalidData, "Expected Constraint node"));
    }
    
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 3 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for Constraint node"));
    }
    
    // Deserialize name
    let name = deserialize_identifier_node(reader)?;
    
    // Deserialize expression
    let expression = deserialize_expression_node(reader)?;
    
    // Deserialize severity
    let severity = deserialize_severity_level(reader)?;
    
    Ok(ConstraintNode {
        name,
        expression,
        severity,
        location,
    })
}

fn deserialize_expression_node<R: Read>(reader: &mut R) -> IoResult<ExpressionNode> {
    // Read node kind
    let mut kind_bytes = [0u8; 2];
    reader.read_exact(&mut kind_bytes)?;
    let kind = u16::from_le_bytes(kind_bytes);
    
    match kind {
        8 => { // NodeKind::BinaryExpr
            let binary = deserialize_binary_expr_node(reader)?;
            Ok(ExpressionNode::Binary(binary))
        }
        9 => { // NodeKind::UnaryExpr
            let unary = deserialize_unary_expr_node(reader)?;
            Ok(ExpressionNode::Unary(unary))
        }
        10 => { // NodeKind::LiteralExpr
            let literal = deserialize_literal_expr_node(reader)?;
            Ok(ExpressionNode::Literal(literal))
        }
        11 => { // NodeKind::IdentifierExpr
            let ident = deserialize_identifier_expr_node(reader)?;
            Ok(ExpressionNode::Identifier(ident))
        }
        12 => { // NodeKind::CallExpr
            let call = deserialize_call_expr_node(reader)?;
            Ok(ExpressionNode::Call(call))
        }
        _ => Err(IoError::new(ErrorKind::InvalidData, "Invalid expression node kind")),
    }
}

fn deserialize_binary_expr_node<R: Read>(reader: &mut R) -> IoResult<BinaryExprNode> {
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 3 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for BinaryExpr node"));
    }
    
    // Deserialize left
    let left = Box::new(deserialize_expression_node(reader)?);
    
    // Deserialize operator
    let operator = deserialize_binary_operator(reader)?;
    
    // Deserialize right
    let right = Box::new(deserialize_expression_node(reader)?);
    
    Ok(BinaryExprNode {
        left,
        operator,
        right,
        location,
    })
}

fn deserialize_unary_expr_node<R: Read>(reader: &mut R) -> IoResult<UnaryExprNode> {
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 2 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for UnaryExpr node"));
    }
    
    // Deserialize operator
    let operator = deserialize_unary_operator(reader)?;
    
    // Deserialize operand
    let operand = Box::new(deserialize_expression_node(reader)?);
    
    Ok(UnaryExprNode {
        operator,
        operand,
        location,
    })
}

fn deserialize_literal_expr_node<R: Read>(reader: &mut R) -> IoResult<LiteralExprNode> {
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 1 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for LiteralExpr node"));
    }
    
    // Deserialize value
    let value = deserialize_literal_value(reader)?;
    
    Ok(LiteralExprNode { value, location })
}

fn deserialize_identifier_expr_node<R: Read>(reader: &mut R) -> IoResult<IdentifierExprNode> {
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 1 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for IdentifierExpr node"));
    }
    
    // Deserialize name
    let name = deserialize_identifier_node(reader)?;
    
    Ok(IdentifierExprNode { name, location })
}

fn deserialize_call_expr_node<R: Read>(reader: &mut R) -> IoResult<CallExprNode> {
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 2 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for CallExpr node"));
    }
    
    // Deserialize callee
    let callee = deserialize_identifier_node(reader)?;
    
    // Deserialize args
    let args = deserialize_expression_list(reader)?;
    
    Ok(CallExprNode { callee, args, location })
}

fn deserialize_action_node<R: Read>(reader: &mut R) -> IoResult<ActionNode> {
    // Read node kind
    let mut kind_bytes = [0u8; 2];
    reader.read_exact(&mut kind_bytes)?;
    let kind = u16::from_le_bytes(kind_bytes);
    
    match kind {
        13 => { // NodeKind::AssignAction
            let assign = deserialize_assign_action_node(reader)?;
            Ok(ActionNode::Assign(assign))
        }
        14 => { // NodeKind::EmitAction
            let emit = deserialize_emit_action_node(reader)?;
            Ok(ActionNode::Emit(emit))
        }
        _ => Err(IoError::new(ErrorKind::InvalidData, "Invalid action node kind")),
    }
}

fn deserialize_assign_action_node<R: Read>(reader: &mut R) -> IoResult<AssignActionNode> {
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 2 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for AssignAction node"));
    }
    
    // Deserialize target
    let target = deserialize_identifier_node(reader)?;
    
    // Deserialize value
    let value = deserialize_expression_node(reader)?;
    
    Ok(AssignActionNode { target, value, location })
}

fn deserialize_emit_action_node<R: Read>(reader: &mut R) -> IoResult<EmitActionNode> {
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 1 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for EmitAction node"));
    }
    
    // Deserialize event
    let event = deserialize_identifier_node(reader)?;
    
    Ok(EmitActionNode { event, location })
}

fn deserialize_flow_step_node<R: Read>(reader: &mut R) -> IoResult<FlowStepNode> {
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count < 2 || field_count > 3 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for FlowStep node"));
    }
    
    // Deserialize from
    let from = deserialize_identifier_node(reader)?;
    
    // Deserialize to
    let to = deserialize_identifier_node(reader)?;
    
    // Deserialize condition (if present)
    let condition = if field_count == 3 {
        Some(deserialize_expression_node(reader)?)
    } else {
        None
    };
    
    Ok(FlowStepNode { from, to, condition, location })
}

fn deserialize_type_node<R: Read>(reader: &mut R) -> IoResult<TypeNode> {
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 2 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for Type node"));
    }
    
    // Deserialize name
    let name = deserialize_identifier_node(reader)?;
    
    // Deserialize nullable
    let mut nullable_byte = [0u8; 1];
    reader.read_exact(&mut nullable_byte)?;
    let nullable = nullable_byte[0] != 0;
    
    Ok(TypeNode { name, nullable, location })
}

fn deserialize_identifier_node<R: Read>(reader: &mut R) -> IoResult<IdentifierNode> {
    // Read location
    let mut location_bytes = [0u8; 16]; // 4 * u32 = 16 bytes
    reader.read_exact(&mut location_bytes)?;
    let location = SourceLocation {
        file_id: u32::from_le_bytes([location_bytes[0], location_bytes[1], location_bytes[2], location_bytes[3]]),
        line: u32::from_le_bytes([location_bytes[4], location_bytes[5], location_bytes[6], location_bytes[7]]),
        column: u32::from_le_bytes([location_bytes[8], location_bytes[9], location_bytes[10], location_bytes[11]]),
        length: u32::from_le_bytes([location_bytes[12], location_bytes[13], location_bytes[14], location_bytes[15]]),
    };
    
    // Read field count
    let mut field_count_bytes = [0u8; 2];
    reader.read_exact(&mut field_count_bytes)?;
    let field_count = u16::from_le_bytes(field_count_bytes);
    
    if field_count != 1 {
        return Err(IoError::new(ErrorKind::InvalidData, "Invalid field count for Identifier node"));
    }
    
    // Deserialize text
    let text = deserialize_string(reader)?;
    
    Ok(IdentifierNode { text, location })
}

fn deserialize_binary_operator<R: Read>(reader: &mut R) -> IoResult<BinaryOperator> {
    let mut field_kind_byte = [0u8; 1];
    reader.read_exact(&mut field_kind_byte)?;
    
    if field_kind_byte[0] != 5 { // FieldKind::Enum as u8
        return Err(IoError::new(ErrorKind::InvalidData, "Expected Enum field"));
    }
    
    let mut op_byte = [0u8; 1];
    reader.read_exact(&mut op_byte)?;
    
    match op_byte[0] {
        0 => Ok(BinaryOperator::Add),
        1 => Ok(BinaryOperator::Sub),
        2 => Ok(BinaryOperator::Mul),
        3 => Ok(BinaryOperator::Div),
        4 => Ok(BinaryOperator::Mod),
        5 => Ok(BinaryOperator::Equal),
        6 => Ok(BinaryOperator::NotEqual),
        7 => Ok(BinaryOperator::Less),
        8 => Ok(BinaryOperator::LessEqual),
        9 => Ok(BinaryOperator::Greater),
        10 => Ok(BinaryOperator::GreaterEqual),
        11 => Ok(BinaryOperator::And),
        12 => Ok(BinaryOperator::Or),
        13 => Ok(BinaryOperator::Xor),
        _ => Err(IoError::new(ErrorKind::InvalidData, "Invalid binary operator")),
    }
}

fn deserialize_unary_operator<R: Read>(reader: &mut R) -> IoResult<UnaryOperator> {
    let mut field_kind_byte = [0u8; 1];
    reader.read_exact(&mut field_kind_byte)?;
    
    if field_kind_byte[0] != 5 { // FieldKind::Enum as u8
        return Err(IoError::new(ErrorKind::InvalidData, "Expected Enum field"));
    }
    
    let mut op_byte = [0u8; 1];
    reader.read_exact(&mut op_byte)?;
    
    match op_byte[0] {
        0 => Ok(UnaryOperator::Neg),
        1 => Ok(UnaryOperator::Not),
        2 => Ok(UnaryOperator::Pos),
        _ => Err(IoError::new(ErrorKind::InvalidData, "Invalid unary operator")),
    }
}

fn deserialize_severity_level<R: Read>(reader: &mut R) -> IoResult<SeverityLevel> {
    let mut field_kind_byte = [0u8; 1];
    reader.read_exact(&mut field_kind_byte)?;
    
    if field_kind_byte[0] != 5 { // FieldKind::Enum as u8
        return Err(IoError::new(ErrorKind::InvalidData, "Expected Enum field"));
    }
    
    let mut severity_byte = [0u8; 1];
    reader.read_exact(&mut severity_byte)?;
    
    match severity_byte[0] {
        0 => Ok(SeverityLevel::Info),
        1 => Ok(SeverityLevel::Warn),
        2 => Ok(SeverityLevel::Error),
        3 => Ok(SeverityLevel::Fatal),
        _ => Err(IoError::new(ErrorKind::InvalidData, "Invalid severity level")),
    }
}

fn deserialize_literal_value<R: Read>(reader: &mut R) -> IoResult<LiteralValue> {
    let mut field_kind_byte = [0u8; 1];
    reader.read_exact(&mut field_kind_byte)?;
    
    if field_kind_byte[0] != 5 { // FieldKind::Enum as u8
        return Err(IoError::new(ErrorKind::InvalidData, "Expected Enum field"));
    }
    
    let mut value_type_byte = [0u8; 1];
    reader.read_exact(&mut value_type_byte)?;
    
    match value_type_byte[0] {
        0 => { // Integer
            let mut int_bytes = [0u8; 8];
            reader.read_exact(&mut int_bytes)?;
            Ok(LiteralValue::Integer(i64::from_le_bytes(int_bytes)))
        }
        1 => { // Float
            let mut float_bytes = [0u8; 8];
            reader.read_exact(&mut float_bytes)?;
            Ok(LiteralValue::Float(f64::from_bits(u64::from_le_bytes(float_bytes))))
        }
        2 => { // String
            let s = deserialize_string(reader)?;
            Ok(LiteralValue::String(s))
        }
        3 => { // Boolean
            let mut bool_byte = [0u8; 1];
            reader.read_exact(&mut bool_byte)?;
            Ok(LiteralValue::Boolean(bool_byte[0] != 0))
        }
        4 => { // Null
            Ok(LiteralValue::Null)
        }
        _ => Err(IoError::new(ErrorKind::InvalidData, "Invalid literal value type")),
    }
}

fn deserialize_string<R: Read>(reader: &mut R) -> IoResult<String> {
    let mut field_kind_byte = [0u8; 1];
    reader.read_exact(&mut field_kind_byte)?;
    
    if field_kind_byte[0] != 2 { // FieldKind::StringId as u8
        return Err(IoError::new(ErrorKind::InvalidData, "Expected StringId field"));
    }
    
    let mut len_bytes = [0u8; 4];
    reader.read_exact(&mut len_bytes)?;
    let len = u32::from_le_bytes(len_bytes) as usize;
    
    let mut buffer = vec![0u8; len];
    reader.read_exact(&mut buffer)?;
    
    String::from_utf8(buffer)
        .map_err(|_| IoError::new(ErrorKind::InvalidData, "Invalid UTF-8 string"))
}

fn deserialize_expression_list<R: Read>(reader: &mut R) -> IoResult<Vec<ExpressionNode>> {
    // Read field kind (should be NodeList)
    let mut field_kind_byte = [0u8; 1];
    reader.read_exact(&mut field_kind_byte)?;
    
    if field_kind_byte[0] != 1 { // FieldKind::NodeList as u8
        return Err(IoError::new(ErrorKind::InvalidData, "Expected NodeList field"));
    }
    
    // Read count
    let mut count_bytes = [0u8; 4];
    reader.read_exact(&mut count_bytes)?;
    let count = u32::from_le_bytes(count_bytes);
    
    let mut expressions = Vec::with_capacity(count as usize);
    for _ in 0..count {
        expressions.push(deserialize_expression_node(reader)?);
    }
    
    Ok(expressions)
}