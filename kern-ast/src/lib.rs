//! KERN AST (Abstract Syntax Tree) System
//! 
//! This module implements the complete AST for the KERN language according to the specification.
//! The AST is designed to be deterministic, immutable after construction, strongly typed,
//! serializable, and debuggable.

mod source_location;
mod identifier_node;
mod type_node;
mod expression_nodes;
mod entity_node;
mod rule_node;
mod flow_node;
mod constraint_node;
mod program_node;
mod visitor;
mod serializer;
mod deserializer;

pub use source_location::SourceLocation;
pub use identifier_node::IdentifierNode;
pub use type_node::TypeNode;
pub use expression_nodes::*;
pub use entity_node::EntityNode;
pub use rule_node::RuleNode;
pub use flow_node::FlowNode;
pub use constraint_node::ConstraintNode;
pub use program_node::ProgramNode;
pub use visitor::{ASTVisitor, ASTVisitorMut};
pub use serializer::serialize_ast;
pub use deserializer::deserialize_ast;