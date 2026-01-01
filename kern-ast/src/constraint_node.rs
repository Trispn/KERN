use crate::{IdentifierNode, ExpressionNode, SourceLocation};

/// SeverityLevel defines the severity of a constraint violation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SeverityLevel {
    Info,
    Warn,
    Error,
    Fatal,
}

/// ConstraintNode represents a constraint definition in KERN.
#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintNode {
    /// Name of the constraint
    pub name: IdentifierNode,
    
    /// Expression that defines the constraint
    pub expression: ExpressionNode,
    
    /// Severity level of the constraint
    pub severity: SeverityLevel,
    
    /// Source location of the constraint
    pub location: SourceLocation,
}

impl ConstraintNode {
    /// Creates a new constraint node
    pub fn new(
        name: IdentifierNode,
        expression: ExpressionNode,
        severity: SeverityLevel,
        location: SourceLocation,
    ) -> Self {
        ConstraintNode {
            name,
            expression,
            severity,
            location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LiteralExprNode, LiteralValue};

    #[test]
    fn test_constraint_node_creation() {
        let name = IdentifierNode::new_with_default_location("ValidAge".to_string());
        let expression = crate::ExpressionNode::Literal(LiteralExprNode {
            value: LiteralValue::Boolean(true),
            location: SourceLocation::default(),
        });
        
        let constraint = ConstraintNode::new(
            name,
            expression,
            SeverityLevel::Error,
            SourceLocation::new(1, 20, 1, 15),
        );
        
        assert_eq!(constraint.name.text(), "ValidAge");
        assert_eq!(constraint.severity, SeverityLevel::Error);
    }
}