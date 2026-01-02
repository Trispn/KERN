//! KERN Type System
//!
//! Defines the type system for KERN language with explicit typing and deterministic behavior.

/// Core primitive types in KERN
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeKind {
    Int,
    Float,
    Bool,
    String,
    Void,
    Entity(String),                // Entity<T> where T is the entity name
    List(Box<TypeDescriptor>),     // List<T>
    Optional(Box<TypeDescriptor>), // Optional<T>
    Sym,
    Num,
    Ref,
    Vec,
    Ctx,
}

/// A type descriptor that represents a specific type in KERN
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeDescriptor {
    pub kind: TypeKind,
    pub name_id: Option<String>,         // Optional name for named types
    pub parameters: Vec<TypeDescriptor>, // Type parameters for generics
}

impl TypeDescriptor {
    /// Creates a new type descriptor
    pub fn new(kind: TypeKind) -> Self {
        TypeDescriptor {
            kind,
            name_id: None,
            parameters: Vec::new(),
        }
    }

    /// Creates a new type descriptor with a name
    pub fn new_named(kind: TypeKind, name: String) -> Self {
        TypeDescriptor {
            kind,
            name_id: Some(name),
            parameters: Vec::new(),
        }
    }

    /// Creates a composite type with parameters
    pub fn new_composite(kind: TypeKind, parameters: Vec<TypeDescriptor>) -> Self {
        TypeDescriptor {
            kind,
            name_id: None,
            parameters,
        }
    }

    /// Checks if this type is compatible with another type
    pub fn is_compatible(&self, other: &TypeDescriptor) -> bool {
        // For now, types must be exactly equal (no implicit conversions)
        self == other
    }

    /// Checks if this is a numeric type (Int or Float)
    pub fn is_numeric(&self) -> bool {
        matches!(self.kind, TypeKind::Int | TypeKind::Float)
    }

    /// Checks if this is a boolean type
    pub fn is_boolean(&self) -> bool {
        matches!(self.kind, TypeKind::Bool)
    }

    /// Checks if this is an entity type
    pub fn is_entity(&self) -> bool {
        matches!(self.kind, TypeKind::Entity(_))
    }

    /// Checks if this is an optional type
    pub fn isoptional(&self) -> bool {
        matches!(self.kind, TypeKind::Optional(_))
    }

    /// Gets the inner type of an optional type
    pub fn unwrapoptional(&self) -> Option<&TypeDescriptor> {
        match &self.kind {
            TypeKind::Optional(inner) => Some(inner),
            _ => None,
        }
    }
}

/// Type checker utility functions
pub struct TypeChecker;

impl TypeChecker {
    /// Validates that two types are compatible
    pub fn validate_compatibility(left: &TypeDescriptor, right: &TypeDescriptor) -> bool {
        left.is_compatible(right)
    }

    /// Validates that a type is numeric
    pub fn validate_numeric(ty: &TypeDescriptor) -> bool {
        ty.is_numeric()
    }

    /// Validates that a type is boolean
    pub fn validate_boolean(ty: &TypeDescriptor) -> bool {
        ty.is_boolean()
    }

    /// Validates that a type is an entity
    pub fn validate_entity(ty: &TypeDescriptor) -> bool {
        ty.is_entity()
    }

    /// Validates that a type is optional
    pub fn validateoptional(ty: &TypeDescriptor) -> bool {
        ty.isoptional()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_creation() {
        let int_type = TypeDescriptor::new(TypeKind::Int);
        assert_eq!(int_type.kind, TypeKind::Int);
        assert!(int_type.name_id.is_none());
        assert!(int_type.parameters.is_empty());
    }

    #[test]
    fn test_entity_type() {
        let entity_type =
            TypeDescriptor::new_named(TypeKind::Entity("Farmer".to_string()), "Farmer".to_string());
        assert!(entity_type.is_entity());
        assert!(!entity_type.is_numeric());
    }

    #[test]
    fn test_composite_types() {
        let int_type = TypeDescriptor::new(TypeKind::Int);
        let list_type = TypeDescriptor::new_composite(
            TypeKind::List(Box::new(int_type.clone())),
            vec![int_type.clone()],
        );
        assert!(!list_type.is_numeric());
    }

    #[test]
    fn test_type_compatibility() {
        let int_type1 = TypeDescriptor::new(TypeKind::Int);
        let int_type2 = TypeDescriptor::new(TypeKind::Int);
        assert!(int_type1.is_compatible(&int_type2));

        let bool_type = TypeDescriptor::new(TypeKind::Bool);
        assert!(!int_type1.is_compatible(&bool_type));
    }
}
