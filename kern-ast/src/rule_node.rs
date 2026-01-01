use crate::{IdentifierNode, ParameterNode, ExpressionNode, ActionNode, SourceLocation};

/// RuleNode represents a rule definition in KERN.
#[derive(Debug, Clone, PartialEq)]
pub struct RuleNode {
    /// Name of the rule
    pub name: IdentifierNode,
    
    /// Parameters of the rule
    pub parameters: Vec<ParameterNode>,
    
    /// Condition that triggers the rule
    pub condition: ExpressionNode,
    
    /// Actions to execute when the rule fires
    pub actions: Vec<ActionNode>,
    
    /// Source location of the rule
    pub location: SourceLocation,
}

/// ParameterNode represents a parameter in a rule
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParameterNode {
    /// Name of the parameter
    pub name: IdentifierNode,
    
    /// Type of the parameter
    pub r#type: crate::TypeNode,
    
    /// Source location of the parameter
    pub location: SourceLocation,
}

/// ActionNode represents an action in a rule
#[derive(Debug, Clone, PartialEq)]
pub enum ActionNode {
    Assign(AssignActionNode),
    Emit(EmitActionNode),
    // Add other action types as needed
}

impl ActionNode {
    /// Returns the source location of the action
    pub fn location(&self) -> &SourceLocation {
        match self {
            ActionNode::Assign(node) => &node.location,
            ActionNode::Emit(node) => &node.location,
        }
    }
}

/// AssignActionNode represents an assignment action
#[derive(Debug, Clone, PartialEq)]
pub struct AssignActionNode {
    pub target: IdentifierNode,
    pub value: ExpressionNode,
    pub location: SourceLocation,
}

/// EmitActionNode represents an event emission action
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmitActionNode {
    pub event: IdentifierNode,
    pub location: SourceLocation,
}

impl RuleNode {
    /// Creates a new rule node
    pub fn new(
        name: IdentifierNode,
        parameters: Vec<ParameterNode>,
        condition: ExpressionNode,
        actions: Vec<ActionNode>,
        location: SourceLocation,
    ) -> Self {
        RuleNode {
            name,
            parameters,
            condition,
            actions,
            location,
        }
    }
}

impl ParameterNode {
    /// Creates a new parameter node
    pub fn new(name: IdentifierNode, r#type: crate::TypeNode, location: SourceLocation) -> Self {
        ParameterNode { name, r#type, location }
    }
}

impl AssignActionNode {
    /// Creates a new assignment action node
    pub fn new(target: IdentifierNode, value: ExpressionNode, location: SourceLocation) -> Self {
        AssignActionNode { target, value, location }
    }
}

impl EmitActionNode {
    /// Creates a new emit action node
    pub fn new(event: IdentifierNode, location: SourceLocation) -> Self {
        EmitActionNode { event, location }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TypeNode, LiteralExprNode, LiteralValue};

    #[test]
    fn test_rule_node_creation() {
        let name = IdentifierNode::new_with_default_location("CheckAge".to_string());
        let param_name = IdentifierNode::new_with_default_location("person".to_string());
        let param_type = TypeNode::new_with_default_location(
            IdentifierNode::new_with_default_location("Person".to_string()),
            false,
        );
        
        let param = ParameterNode::new(param_name, param_type, SourceLocation::default());
        
        let condition = crate::ExpressionNode::Binary(crate::BinaryExprNode {
            left: Box::new(crate::ExpressionNode::Identifier(crate::IdentifierExprNode {
                name: IdentifierNode::new_with_default_location("person".to_string()),
                location: SourceLocation::default(),
            })),
            operator: crate::BinaryOperator::Greater,
            right: Box::new(crate::ExpressionNode::Literal(LiteralExprNode {
                value: LiteralValue::Integer(18),
                location: SourceLocation::default(),
            })),
            location: SourceLocation::default(),
        });
        
        let target = IdentifierNode::new_with_default_location("result".to_string());
        let value = crate::ExpressionNode::Literal(LiteralExprNode {
            value: LiteralValue::Boolean(true),
            location: SourceLocation::default(),
        });
        
        let action = ActionNode::Assign(AssignActionNode::new(target, value, SourceLocation::default()));
        
        let rule = RuleNode::new(
            name,
            vec![param],
            condition,
            vec![action],
            SourceLocation::new(1, 10, 1, 20),
        );
        
        assert_eq!(rule.name.text(), "CheckAge");
        assert_eq!(rule.parameters.len(), 1);
        assert_eq!(rule.actions.len(), 1);
    }
}