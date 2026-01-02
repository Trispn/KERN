use crate::{
    ConstraintNode, EntityNode, ExpressionNode, FlowNode, IdentifierNode, ProgramNode, RuleNode,
    TypeNode,
};

/// ASTVisitor defines the interface for visiting AST nodes.
///
/// This implements the visitor pattern for AST traversal.
pub trait ASTVisitor {
    /// Visit the root program node
    fn visit_program(&mut self, node: &ProgramNode) {
        // Default implementation that visits children
        for entity in &node.entities {
            self.visit_entity(entity);
        }

        for rule in &node.rules {
            self.visit_rule(rule);
        }

        for flow in &node.flows {
            self.visit_flow(flow);
        }

        for constraint in &node.constraints {
            self.visit_constraint(constraint);
        }
    }

    /// Visit an entity node
    fn visit_entity(&mut self, node: &EntityNode) {
        // Visit children
        self.visit_identifier(&node.name);

        for attr in &node.attributes {
            self.visit_attribute(attr);
        }

        for rule_ref in &node.rules {
            self.visit_rule_ref(rule_ref);
        }

        for constraint_ref in &node.constraints {
            self.visit_constraint_ref(constraint_ref);
        }
    }

    /// Visit an attribute node
    fn visit_attribute(&mut self, node: &crate::AttributeNode) {
        self.visit_identifier(&node.name);
        self.visit_type(&node.r#type);

        if let Some(default_value) = &node.default_value {
            self.visit_expression(default_value);
        }
    }

    /// Visit a rule reference node
    fn visit_rule_ref(&mut self, node: &crate::RuleRefNode) {
        self.visit_identifier(&node.name);
    }

    /// Visit a constraint reference node
    fn visit_constraint_ref(&mut self, node: &crate::ConstraintRefNode) {
        self.visit_identifier(&node.name);
    }

    /// Visit a rule node
    fn visit_rule(&mut self, node: &RuleNode) {
        self.visit_identifier(&node.name);

        for param in &node.parameters {
            self.visit_parameter(param);
        }

        self.visit_expression(&node.condition);

        for action in &node.actions {
            self.visit_action(action);
        }
    }

    /// Visit a parameter node
    fn visit_parameter(&mut self, node: &crate::ParameterNode) {
        self.visit_identifier(&node.name);
        self.visit_type(&node.r#type);
    }

    /// Visit an action node
    fn visit_action(&mut self, node: &crate::ActionNode) {
        match node {
            crate::ActionNode::Assign(assign) => {
                self.visit_identifier(&assign.target);
                self.visit_expression(&assign.value);
            }
            crate::ActionNode::Emit(emit) => {
                self.visit_identifier(&emit.event);
            }
        }
    }

    /// Visit a flow node
    fn visit_flow(&mut self, node: &FlowNode) {
        self.visit_identifier(&node.name);

        for step in &node.steps {
            self.visit_flow_step(step);
        }
    }

    /// Visit a flow step node
    fn visit_flow_step(&mut self, node: &crate::FlowStepNode) {
        self.visit_identifier(&node.from);
        self.visit_identifier(&node.to);

        if let Some(condition) = &node.condition {
            self.visit_expression(condition);
        }
    }

    /// Visit a constraint node
    fn visit_constraint(&mut self, node: &ConstraintNode) {
        self.visit_identifier(&node.name);
        self.visit_expression(&node.expression);
    }

    /// Visit an expression node
    fn visit_expression(&mut self, node: &ExpressionNode) {
        match node {
            crate::ExpressionNode::Binary(binary) => {
                self.visit_expression(&binary.left);
                self.visit_expression(&binary.right);
            }
            crate::ExpressionNode::Unary(unary) => {
                self.visit_expression(&unary.operand);
            }
            crate::ExpressionNode::Literal(_) => {
                // Literals have no children to visit
            }
            crate::ExpressionNode::Identifier(ident) => {
                self.visit_identifier(&ident.name);
            }
            crate::ExpressionNode::Call(call) => {
                self.visit_identifier(&call.callee);
                for arg in &call.args {
                    self.visit_expression(arg);
                }
            }
        }
    }

    /// Visit a type node
    fn visit_type(&mut self, node: &TypeNode) {
        self.visit_identifier(&node.name);
    }

    /// Visit an identifier node
    fn visit_identifier(&mut self, _node: &IdentifierNode) {
        // Identifiers have no children to visit
    }
}

/// ASTVisitorMut is like ASTVisitor but allows mutation of the AST.
///
/// This is useful for AST transformations.
pub trait ASTVisitorMut {
    /// Visit the root program node
    fn visit_program(&mut self, node: &mut ProgramNode) {
        for entity in &mut node.entities {
            self.visit_entity(entity);
        }

        for rule in &mut node.rules {
            self.visit_rule(rule);
        }

        for flow in &mut node.flows {
            self.visit_flow(flow);
        }

        for constraint in &mut node.constraints {
            self.visit_constraint(constraint);
        }
    }

    /// Visit an entity node
    fn visit_entity(&mut self, node: &mut EntityNode) {
        self.visit_identifier(&mut node.name);

        for attr in &mut node.attributes {
            self.visit_attribute(attr);
        }

        for rule_ref in &mut node.rules {
            self.visit_rule_ref(rule_ref);
        }

        for constraint_ref in &mut node.constraints {
            self.visit_constraint_ref(constraint_ref);
        }
    }

    /// Visit an attribute node
    fn visit_attribute(&mut self, node: &mut crate::AttributeNode) {
        self.visit_identifier(&mut node.name);
        self.visit_type(&mut node.r#type);

        if let Some(ref mut default_value) = node.default_value {
            self.visit_expression(default_value);
        }
    }

    /// Visit a rule reference node
    fn visit_rule_ref(&mut self, node: &mut crate::RuleRefNode) {
        self.visit_identifier(&mut node.name);
    }

    /// Visit a constraint reference node
    fn visit_constraint_ref(&mut self, node: &mut crate::ConstraintRefNode) {
        self.visit_identifier(&mut node.name);
    }

    /// Visit a rule node
    fn visit_rule(&mut self, node: &mut RuleNode) {
        self.visit_identifier(&mut node.name);

        for param in &mut node.parameters {
            self.visit_parameter(param);
        }

        self.visit_expression(&mut node.condition);

        for action in &mut node.actions {
            self.visit_action(action);
        }
    }

    /// Visit a parameter node
    fn visit_parameter(&mut self, node: &mut crate::ParameterNode) {
        self.visit_identifier(&mut node.name);
        self.visit_type(&mut node.r#type);
    }

    /// Visit an action node
    fn visit_action(&mut self, node: &mut crate::ActionNode) {
        match node {
            crate::ActionNode::Assign(ref mut assign) => {
                self.visit_identifier(&mut assign.target);
                self.visit_expression(&mut assign.value);
            }
            crate::ActionNode::Emit(ref mut emit) => {
                self.visit_identifier(&mut emit.event);
            }
        }
    }

    /// Visit a flow node
    fn visit_flow(&mut self, node: &mut FlowNode) {
        self.visit_identifier(&mut node.name);

        for step in &mut node.steps {
            self.visit_flow_step(step);
        }
    }

    /// Visit a flow step node
    fn visit_flow_step(&mut self, node: &mut crate::FlowStepNode) {
        self.visit_identifier(&mut node.from);
        self.visit_identifier(&mut node.to);

        if let Some(ref mut condition) = node.condition {
            self.visit_expression(condition);
        }
    }

    /// Visit a constraint node
    fn visit_constraint(&mut self, node: &mut ConstraintNode) {
        self.visit_identifier(&mut node.name);
        self.visit_expression(&mut node.expression);
    }

    /// Visit an expression node
    fn visit_expression(&mut self, node: &mut ExpressionNode) {
        match node {
            crate::ExpressionNode::Binary(ref mut binary) => {
                self.visit_expression(&mut binary.left);
                self.visit_expression(&mut binary.right);
            }
            crate::ExpressionNode::Unary(ref mut unary) => {
                self.visit_expression(&mut unary.operand);
            }
            crate::ExpressionNode::Literal(_) => {
                // Literals have no children to visit
            }
            crate::ExpressionNode::Identifier(ref mut ident) => {
                self.visit_identifier(&mut ident.name);
            }
            crate::ExpressionNode::Call(ref mut call) => {
                self.visit_identifier(&mut call.callee);
                for arg in &mut call.args {
                    self.visit_expression(arg);
                }
            }
        }
    }

    /// Visit a type node
    fn visit_type(&mut self, node: &mut TypeNode) {
        self.visit_identifier(&mut node.name);
    }

    /// Visit an identifier node
    fn visit_identifier(&mut self, _node: &mut IdentifierNode) {
        // Identifiers have no children to visit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IdentifierNode;

    struct TestVisitor {
        visited_nodes: Vec<String>,
    }

    impl TestVisitor {
        fn new() -> Self {
            TestVisitor {
                visited_nodes: Vec::new(),
            }
        }
    }

    impl ASTVisitor for TestVisitor {
        fn visit_identifier(&mut self, node: &IdentifierNode) {
            self.visited_nodes
                .push(format!("identifier: {}", node.text()));
        }
    }

    #[test]
    fn test_visitor_pattern() {
        let mut visitor = TestVisitor::new();
        let ident = IdentifierNode::new_with_default_location("test".to_string());
        visitor.visit_identifier(&ident);

        assert_eq!(visitor.visited_nodes, vec!["identifier: test"]);
    }
}
