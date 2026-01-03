use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanguageMapping {
    pub id: String,
    pub source_language: String,
    pub target_language: String,
    pub translation_rules: HashMap<String, String>, // Maps constructs from source to target
    pub syntax_templates: HashMap<String, String>, // Templates for different language constructs
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModalityMapping {
    pub id: String,
    pub source_modality: String, // text, code, image, video, audio
    pub target_modality: String,
    pub transformation_rules: HashMap<String, String>,
    pub template_mappings: HashMap<String, String>,
}

impl LanguageMapping {
    pub fn new(
        id: &str,
        source_language: &str,
        target_language: &str,
        rules: HashMap<String, String>,
        templates: HashMap<String, String>,
    ) -> Self {
        LanguageMapping {
            id: id.to_string(),
            source_language: source_language.to_string(),
            target_language: target_language.to_string(),
            translation_rules: rules,
            syntax_templates: templates,
        }
    }

    pub fn translate_construct(&self, construct: &str) -> Option<String> {
        self.translation_rules.get(construct).cloned()
    }

    pub fn get_syntax_template(&self, construct_type: &str) -> Option<String> {
        self.syntax_templates.get(construct_type).cloned()
    }
}

impl ModalityMapping {
    pub fn new(
        id: &str,
        source_modality: &str,
        target_modality: &str,
        rules: HashMap<String, String>,
        templates: HashMap<String, String>,
    ) -> Self {
        ModalityMapping {
            id: id.to_string(),
            source_modality: source_modality.to_string(),
            target_modality: target_modality.to_string(),
            transformation_rules: rules,
            template_mappings: templates,
        }
    }

    pub fn transform(&self, input: &str) -> Option<String> {
        self.transformation_rules.get(input).cloned()
    }

    pub fn get_template(&self, template_type: &str) -> Option<String> {
        self.template_mappings.get(template_type).cloned()
    }
}

// Predefined language mappings
pub mod predefined_language_mappings {
    use super::*;

    pub fn create_rust_to_python_mapping() -> LanguageMapping {
        let mut rules = HashMap::new();
        rules.insert("struct".to_string(), "class".to_string());
        rules.insert("impl".to_string(), "class".to_string());
        rules.insert("fn".to_string(), "def".to_string());
        rules.insert("let".to_string(), "".to_string());
        rules.insert("mut".to_string(), "".to_string());
        rules.insert("String".to_string(), "str".to_string());
        rules.insert("Vec<T>".to_string(), "List[T]".to_string());
        rules.insert("HashMap<K, V>".to_string(), "Dict[K, V]".to_string());

        let mut templates = HashMap::new();
        templates.insert("function".to_string(), "def {name}({params}):\n    {body}".to_string());
        templates.insert("struct".to_string(), "class {name}:\n    def __init__(self, {fields}):\n        {field_assignments}".to_string());

        LanguageMapping::new(
            "rust_to_python",
            "rust",
            "python",
            rules,
            templates,
        )
    }

    pub fn create_python_to_rust_mapping() -> LanguageMapping {
        let mut rules = HashMap::new();
        rules.insert("class".to_string(), "struct".to_string());
        rules.insert("def".to_string(), "fn".to_string());
        rules.insert("self".to_string(), "self".to_string());
        rules.insert("str".to_string(), "String".to_string());
        rules.insert("List[T]".to_string(), "Vec<T>".to_string());
        rules.insert("Dict[K, V]".to_string(), "HashMap<K, V>".to_string());

        let mut templates = HashMap::new();
        templates.insert("function".to_string(), "fn {name}({params}) -> {return_type} {{\n    {body}\n}}".to_string());
        templates.insert("struct".to_string(), "struct {name} {{\n    {fields}\n}}".to_string());

        LanguageMapping::new(
            "python_to_rust",
            "python",
            "rust",
            rules,
            templates,
        )
    }

    pub fn create_rust_to_go_mapping() -> LanguageMapping {
        let mut rules = HashMap::new();
        rules.insert("struct".to_string(), "type".to_string());
        rules.insert("impl".to_string(), "func".to_string());
        rules.insert("fn".to_string(), "func".to_string());
        rules.insert("String".to_string(), "string".to_string());
        rules.insert("Vec<T>".to_string(), "[]T".to_string());
        rules.insert("HashMap<K, V>".to_string(), "map[K]V".to_string());

        let mut templates = HashMap::new();
        templates.insert("function".to_string(), "func {name}({params} {return_type}) {{\n    {body}\n}}".to_string());
        templates.insert("struct".to_string(), "type {name} struct {{\n    {fields}\n}}".to_string());

        LanguageMapping::new(
            "rust_to_go",
            "rust",
            "go",
            rules,
            templates,
        )
    }
}

// Predefined modality mappings
pub mod predefined_modality_mappings {
    use super::*;

    pub fn create_text_to_code_mapping() -> ModalityMapping {
        let mut rules = HashMap::new();
        rules.insert("create function".to_string(), "define_function".to_string());
        rules.insert("create class".to_string(), "define_class".to_string());
        rules.insert("loop through".to_string(), "create_loop".to_string());
        rules.insert("if condition".to_string(), "create_condition".to_string());

        let mut templates = HashMap::new();
        templates.insert("function".to_string(), "fn {name}() {{\n    // {description}\n}}".to_string());
        templates.insert("class".to_string(), "struct {name} {{\n    // {description}\n}}".to_string());

        ModalityMapping::new(
            "text_to_code",
            "text",
            "code",
            rules,
            templates,
        )
    }

    pub fn create_code_to_text_mapping() -> ModalityMapping {
        let mut rules = HashMap::new();
        rules.insert("fn".to_string(), "function".to_string());
        rules.insert("struct".to_string(), "class or data structure".to_string());
        rules.insert("impl".to_string(), "implementation block".to_string());
        rules.insert("trait".to_string(), "interface or contract".to_string());

        let mut templates = HashMap::new();
        templates.insert("function".to_string(), "The function {name} performs: {description}".to_string());
        templates.insert("struct".to_string(), "The structure {name} represents: {description}".to_string());

        ModalityMapping::new(
            "code_to_text",
            "code",
            "text",
            rules,
            templates,
        )
    }

    pub fn create_text_to_image_mapping() -> ModalityMapping {
        let mut rules = HashMap::new();
        rules.insert("portrait".to_string(), "portrait_image".to_string());
        rules.insert("landscape".to_string(), "landscape_image".to_string());
        rules.insert("chart".to_string(), "data_visualization".to_string());
        rules.insert("diagram".to_string(), "technical_diagram".to_string());

        let mut templates = HashMap::new();
        templates.insert("portrait".to_string(), "portrait of {subject}, {style}, {details}".to_string());
        templates.insert("landscape".to_string(), "landscape of {scene}, {style}, {details}".to_string());

        ModalityMapping::new(
            "text_to_image",
            "text",
            "image",
            rules,
            templates,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_mapping_creation() {
        let mapping = predefined_language_mappings::create_rust_to_python_mapping();
        assert_eq!(mapping.source_language, "rust");
        assert_eq!(mapping.target_language, "python");
        assert!(mapping.translate_construct("struct").is_some());
        assert_eq!(mapping.translate_construct("struct").unwrap(), "class");
    }

    #[test]
    fn test_modality_mapping_creation() {
        let mapping = predefined_modality_mappings::create_text_to_code_mapping();
        assert_eq!(mapping.source_modality, "text");
        assert_eq!(mapping.target_modality, "code");
        assert!(mapping.transform("create function").is_some());
        assert_eq!(mapping.transform("create function").unwrap(), "define_function");
    }
}