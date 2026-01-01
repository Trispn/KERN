use crate::{IdentifierNode, SourceLocation, TypeNode};

/// BinaryOperator represents operators for binary expressions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Xor,
}

/// UnaryOperator represents operators for unary expressions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Neg,
    Not,
    Pos,
}

/// LiteralValue represents the value of a literal expression
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

/// ExpressionNode represents all possible expression types in KERN
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    Binary(BinaryExprNode),
    Unary(UnaryExprNode),
    Literal(LiteralExprNode),
    Identifier(IdentifierExprNode),
    Call(CallExprNode),
    // Add other expression types as needed
}

impl ExpressionNode {
    /// Returns the source location of the expression
    pub fn location(&self) -> &SourceLocation {
        match self {
            ExpressionNode::Binary(node) => &node.location,
            ExpressionNode::Unary(node) => &node.location,
            ExpressionNode::Literal(node) => &node.location,
            ExpressionNode::Identifier(node) => &node.location,
            ExpressionNode::Call(node) => &node.location,
        }
    }
}

/// BinaryExprNode represents a binary operation
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExprNode {
    pub left: Box<ExpressionNode>,
    pub operator: BinaryOperator,
    pub right: Box<ExpressionNode>,
    pub location: SourceLocation,
}

/// UnaryExprNode represents a unary operation
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExprNode {
    pub operator: UnaryOperator,
    pub operand: Box<ExpressionNode>,
    pub location: SourceLocation,
}

/// LiteralExprNode represents a literal value
#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExprNode {
    pub value: LiteralValue,
    pub location: SourceLocation,
}

/// IdentifierExprNode represents an identifier used as an expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentifierExprNode {
    pub name: IdentifierNode,
    pub location: SourceLocation,
}

/// CallExprNode represents a function or method call
#[derive(Debug, Clone, PartialEq)]
pub struct CallExprNode {
    pub callee: IdentifierNode,
    pub args: Vec<ExpressionNode>,
    pub location: SourceLocation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_expression_creation() {
        let left = ExpressionNode::Literal(LiteralExprNode {
            value: LiteralValue::Integer(5),
            location: SourceLocation::default(),
        });
        
        let right = ExpressionNode::Literal(LiteralExprNode {
            value: LiteralValue::Integer(3),
            location: SourceLocation::default(),
        });
        
        let binary_expr = ExpressionNode::Binary(BinaryExprNode {
            left: Box::new(left),
            operator: BinaryOperator::Add,
            right: Box::new(right),
            location: SourceLocation::new(1, 5, 10, 5),
        });
        
        if let ExpressionNode::Binary(bin_node) = &binary_expr {
            assert_eq!(bin_node.operator, BinaryOperator::Add);
            assert_eq!(bin_node.location.line, 5);
        } else {
            panic!("Expected Binary expression");
        }
    }
    
    #[test]
    fn test_literal_expression_creation() {
        let literal_expr = ExpressionNode::Literal(LiteralExprNode {
            value: LiteralValue::String("hello".to_string()),
            location: SourceLocation::new(1, 3, 5, 7),
        });
        
        if let ExpressionNode::Literal(lit_node) = &literal_expr {
            match &lit_node.value {
                LiteralValue::String(s) => assert_eq!(s, "hello"),
                _ => panic!("Expected String literal"),
            }
        } else {
            panic!("Expected Literal expression");
        }
    }
}