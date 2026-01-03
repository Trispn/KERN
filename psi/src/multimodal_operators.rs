use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{PSI_Operator, language_mappings::{LanguageMapping, ModalityMapping}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiModalOperator {
    pub base_operator: PSI_Operator,
    pub supported_modalities: Vec<String>, // e.g., ["text", "code", "image"]
    pub modality_transformations: HashMap<String, String>, // Maps between modalities
    pub language_adaptations: HashMap<String, String>, // Maps between languages
}

impl MultiModalOperator {
    pub fn new(
        base_operator: PSI_Operator,
        modalities: Vec<String>,
        transformations: HashMap<String, String>,
        adaptations: HashMap<String, String>,
    ) -> Self {
        MultiModalOperator {
            base_operator,
            supported_modalities: modalities,
            modality_transformations: transformations,
            language_adaptations: adaptations,
        }
    }

    pub fn supports_modality(&self, modality: &str) -> bool {
        self.supported_modalities.contains(&modality.to_string())
    }

    pub fn transform_modality(&self, input: &str, target_modality: &str) -> Option<String> {
        self.modality_transformations.get(target_modality).cloned()
    }

    pub fn adapt_language(&self, input: &str, target_language: &str) -> Option<String> {
        self.language_adaptations.get(target_language).cloned()
    }
}

// Multi-modal operator factory functions
pub mod multimodal_operators {
    use super::*;
    use crate::operator_engine::common_operators;
    use std::collections::HashMap;

    pub fn create_text_to_code_operator() -> MultiModalOperator {
        let base_op = common_operators::create_define_entities_operator(); // Use an existing operator as base
        
        let mut transformations = HashMap::new();
        transformations.insert("code".to_string(), "convert_text_to_code".to_string());
        transformations.insert("image".to_string(), "create_visual_representation".to_string());
        
        let mut adaptations = HashMap::new();
        adaptations.insert("rust".to_string(), "// Rust implementation".to_string());
        adaptations.insert("python".to_string(), "# Python implementation".to_string());
        adaptations.insert("go".to_string(), "// Go implementation".to_string());
        
        MultiModalOperator::new(
            base_op,
            vec!["text".to_string(), "code".to_string(), "image".to_string()],
            transformations,
            adaptations,
        )
    }

    pub fn create_code_generation_operator() -> MultiModalOperator {
        let mut base_emissions = HashMap::new();
        base_emissions.insert("rust".to_string(), "// Generated Rust code".to_string());
        base_emissions.insert("python".to_string(), "# Generated Python code".to_string());
        base_emissions.insert("go".to_string(), "// Generated Go code".to_string());
        base_emissions.insert("javascript".to_string(), "// Generated JavaScript code".to_string());

        let base_op = PSI_Operator {
            id: 100,
            name: "MultiModalCodeGeneration".to_string(),
            domain: "code_generation".to_string(),
            purity: 0,
            arity_in: 1,
            arity_out: 1,
            cost_hint: 20,
            kern_template: r#"rule MultiModalCodeGeneration:
    if has_specification()
    then generate_code_for_all_modalities()"#.to_string(),
            emissions: base_emissions,
        };

        let mut transformations = HashMap::new();
        transformations.insert("text".to_string(), "parse_specification".to_string());
        transformations.insert("code".to_string(), "generate_code".to_string());
        transformations.insert("image".to_string(), "create_architecture_diagram".to_string());
        
        let mut adaptations = HashMap::new();
        adaptations.insert("rust".to_string(), "// Rust implementation".to_string());
        adaptations.insert("python".to_string(), "# Python implementation".to_string());
        adaptations.insert("go".to_string(), "// Go implementation".to_string());
        adaptations.insert("javascript".to_string(), "// JavaScript implementation".to_string());
        
        MultiModalOperator::new(
            base_op,
            vec!["text".to_string(), "code".to_string(), "image".to_string()],
            transformations,
            adaptations,
        )
    }

    pub fn create_image_generation_operator() -> MultiModalOperator {
        let mut base_emissions = HashMap::new();
        base_emissions.insert("rust".to_string(), "// Image generation in Rust using image crate".to_string());
        base_emissions.insert("python".to_string(), "# Image generation in Python using PIL".to_string());
        
        let base_op = PSI_Operator {
            id: 101,
            name: "ImageGeneration".to_string(),
            domain: "image_generation".to_string(),
            purity: 0,
            arity_in: 1,
            arity_out: 1,
            cost_hint: 50,
            kern_template: r#"rule ImageGeneration:
    if has_description()
    then generate_image_from_description()"#.to_string(),
            emissions: base_emissions,
        };

        let mut transformations = HashMap::new();
        transformations.insert("text".to_string(), "parse_description_for_image".to_string());
        transformations.insert("image".to_string(), "generate_image".to_string());
        transformations.insert("code".to_string(), "generate_image_processing_code".to_string());
        
        let mut adaptations = HashMap::new();
        adaptations.insert("rust".to_string(), "// Use image crate for image generation".to_string());
        adaptations.insert("python".to_string(), "# Use PIL/Pillow for image generation".to_string());
        
        MultiModalOperator::new(
            base_op,
            vec!["text".to_string(), "image".to_string(), "code".to_string()],
            transformations,
            adaptations,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multimodal_operator_creation() {
        let mm_op = multimodal_operators::create_text_to_code_operator();
        assert!(mm_op.supports_modality("text"));
        assert!(mm_op.supports_modality("code"));
        assert!(!mm_op.supports_modality("video"));
    }

    #[test]
    fn test_modality_transformation() {
        let mm_op = multimodal_operators::create_text_to_code_operator();
        let result = mm_op.transform_modality("input", "code");
        assert!(result.is_some());
    }
}