use crate::{EntityNode, RuleNode, FlowNode, ConstraintNode, SourceLocation};

/// ProgramNode represents the root of a KERN program AST.
#[derive(Debug, Clone, PartialEq)]
pub struct ProgramNode {
    /// List of entity definitions in the program
    pub entities: Vec<EntityNode>,
    
    /// List of rule definitions in the program
    pub rules: Vec<RuleNode>,
    
    /// List of flow definitions in the program
    pub flows: Vec<FlowNode>,
    
    /// List of constraint definitions in the program
    pub constraints: Vec<ConstraintNode>,
    
    /// Source location of the program (typically the file location)
    pub location: SourceLocation,
}

impl ProgramNode {
    /// Creates a new program node
    pub fn new(
        entities: Vec<EntityNode>,
        rules: Vec<RuleNode>,
        flows: Vec<FlowNode>,
        constraints: Vec<ConstraintNode>,
        location: SourceLocation,
    ) -> Self {
        ProgramNode {
            entities,
            rules,
            flows,
            constraints,
            location,
        }
    }
    
    /// Creates an empty program node
    pub fn empty() -> Self {
        ProgramNode {
            entities: Vec::new(),
            rules: Vec::new(),
            flows: Vec::new(),
            constraints: Vec::new(),
            location: SourceLocation::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_node_creation() {
        let program = ProgramNode::empty();
        assert_eq!(program.entities.len(), 0);
        assert_eq!(program.rules.len(), 0);
        assert_eq!(program.flows.len(), 0);
        assert_eq!(program.constraints.len(), 0);
    }
}