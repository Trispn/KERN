//! KERN AST (Abstract Syntax Tree) System
//!
//! This module implements the complete AST for the KERN language according to the specification.
//! The AST is designed to be deterministic, immutable after construction, strongly typed,
//! serializable, and debuggable.

mod constraint_node;
mod deserializer;
mod entity_node;
mod expression_nodes;
mod flow_node;
mod identifier_node;
mod program_node;
mod rule_node;
mod serializer;
mod source_location;
mod type_node;
mod visitor;

pub use constraint_node::*;
pub use deserializer::deserialize_ast;
pub use entity_node::*;
pub use expression_nodes::*;
pub use flow_node::*;
pub use identifier_node::IdentifierNode;
pub use program_node::ProgramNode;
pub use rule_node::*;
pub use serializer::serialize_ast;
pub use source_location::SourceLocation;
pub use type_node::TypeNode;
pub use visitor::{ASTVisitor, ASTVisitorMut};
