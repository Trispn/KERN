use crate::{IdentifierNode, SourceLocation};

/// FlowNode represents a flow definition in KERN.
#[derive(Debug, Clone, PartialEq)]
pub struct FlowNode {
    /// Name of the flow
    pub name: IdentifierNode,

    /// Steps in the flow
    pub steps: Vec<FlowStepNode>,

    /// Source location of the flow
    pub location: SourceLocation,
}

/// FlowStepNode represents a step in a flow
#[derive(Debug, Clone, PartialEq)]
pub struct FlowStepNode {
    /// Source state/step
    pub from: IdentifierNode,

    /// Destination state/step
    pub to: IdentifierNode,

    /// Condition for the step (optional)
    pub condition: Option<crate::ExpressionNode>,

    /// Source location of the step
    pub location: SourceLocation,
}

impl FlowNode {
    /// Creates a new flow node
    pub fn new(name: IdentifierNode, steps: Vec<FlowStepNode>, location: SourceLocation) -> Self {
        FlowNode {
            name,
            steps,
            location,
        }
    }
}

impl FlowStepNode {
    /// Creates a new flow step node
    pub fn new(
        from: IdentifierNode,
        to: IdentifierNode,
        condition: Option<crate::ExpressionNode>,
        location: SourceLocation,
    ) -> Self {
        FlowStepNode {
            from,
            to,
            condition,
            location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LiteralExprNode, LiteralValue};

    #[test]
    fn test_flow_node_creation() {
        let name = IdentifierNode::new_with_default_location("ProcessOrder".to_string());
        let from = IdentifierNode::new_with_default_location("Submitted".to_string());
        let to = IdentifierNode::new_with_default_location("Approved".to_string());

        let condition = Some(crate::ExpressionNode::Literal(LiteralExprNode {
            value: LiteralValue::Boolean(true),
            location: SourceLocation::default(),
        }));

        let step = FlowStepNode::new(from, to, condition, SourceLocation::default());
        let flow = FlowNode::new(name, vec![step], SourceLocation::new(1, 15, 1, 25));

        assert_eq!(flow.name.text(), "ProcessOrder");
        assert_eq!(flow.steps.len(), 1);
        assert_eq!(flow.steps[0].from.text(), "Submitted");
        assert_eq!(flow.steps[0].to.text(), "Approved");
    }
}
