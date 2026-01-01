use crate::{IdentifierNode, SourceLocation};

/// EntityNode represents an entity definition in KERN.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityNode {
    /// Name of the entity
    pub name: IdentifierNode,

    /// Attributes of the entity
    pub attributes: Vec<AttributeNode>,

    /// Rules associated with this entity
    pub rules: Vec<RuleRefNode>,

    /// Constraints associated with this entity
    pub constraints: Vec<ConstraintRefNode>,

    /// Source location of the entity
    pub location: SourceLocation,
}

/// AttributeNode represents an attribute of an entity
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeNode {
    /// Name of the attribute
    pub name: IdentifierNode,

    /// Type of the attribute
    pub r#type: crate::TypeNode,

    /// Default value of the attribute (optional)
    pub default_value: Option<crate::ExpressionNode>,

    /// Source location of the attribute
    pub location: SourceLocation,
}

/// RuleRefNode represents a reference to a rule from an entity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RuleRefNode {
    /// Name of the referenced rule
    pub name: IdentifierNode,

    /// Source location of the rule reference
    pub location: SourceLocation,
}

/// ConstraintRefNode represents a reference to a constraint from an entity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstraintRefNode {
    /// Name of the referenced constraint
    pub name: IdentifierNode,

    /// Source location of the constraint reference
    pub location: SourceLocation,
}

impl EntityNode {
    /// Creates a new entity node
    pub fn new(
        name: IdentifierNode,
        attributes: Vec<AttributeNode>,
        rules: Vec<RuleRefNode>,
        constraints: Vec<ConstraintRefNode>,
        location: SourceLocation,
    ) -> Self {
        EntityNode {
            name,
            attributes,
            rules,
            constraints,
            location,
        }
    }
}

impl AttributeNode {
    /// Creates a new attribute node
    pub fn new(
        name: IdentifierNode,
        r#type: crate::TypeNode,
        default_value: Option<crate::ExpressionNode>,
        location: SourceLocation,
    ) -> Self {
        AttributeNode {
            name,
            r#type,
            default_value,
            location,
        }
    }
}

impl RuleRefNode {
    /// Creates a new rule reference node
    pub fn new(name: IdentifierNode, location: SourceLocation) -> Self {
        RuleRefNode { name, location }
    }
}

impl ConstraintRefNode {
    /// Creates a new constraint reference node
    pub fn new(name: IdentifierNode, location: SourceLocation) -> Self {
        ConstraintRefNode { name, location }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LiteralExprNode, LiteralValue, TypeNode};

    #[test]
    fn test_entity_node_creation() {
        let name = IdentifierNode::new_with_default_location("Person".to_string());
        let attr_name = IdentifierNode::new_with_default_location("age".to_string());
        let attr_type = TypeNode::new_with_default_location(
            IdentifierNode::new_with_default_location("int".to_string()),
            false,
        );

        let default_value = Some(crate::ExpressionNode::Literal(LiteralExprNode {
            value: LiteralValue::Integer(0),
            location: SourceLocation::default(),
        }));

        let attribute = AttributeNode::new(
            attr_name,
            attr_type,
            default_value,
            SourceLocation::default(),
        );

        let entity = EntityNode::new(
            name,
            vec![attribute],
            vec![],
            vec![],
            SourceLocation::new(1, 5, 1, 15),
        );

        assert_eq!(entity.name.text(), "Person");
        assert_eq!(entity.attributes.len(), 1);
        assert_eq!(entity.attributes[0].name.text(), "age");
    }
}
