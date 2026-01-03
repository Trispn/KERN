use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub mod operator_engine;
pub mod meta_programs;
pub mod language_mappings;
pub mod multimodal_operators;

// PSI Operator - the smallest unit of intelligence
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PSI_Operator {
    pub id: u16,
    pub name: String,
    pub domain: String,        // logic, code, math, systems
    pub purity: u8,           // 0 = pure, 1 = impure
    pub arity_in: u8,         // number of input parameters
    pub arity_out: u8,        // number of output parameters
    pub cost_hint: u16,       // estimated computational cost
    pub kern_template: String, // KERN source code template
    pub emissions: HashMap<String, String>, // language-specific templates (e.g., "rust", "python")
}

// PSI Meta-Program - compressed cognitive strategies
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PSI_MetaProgram {
    pub id: u16,
    pub name: String,
    pub operators: Vec<String>, // references to operator names
    pub domain: String,
    pub adaptability: u8,      // how flexible the template is (0-100)
}

// PSI Heuristic - decision weighting
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PSI_Heuristic {
    pub id: u16,
    pub name: String,
    pub trigger_type: String,   // context, error, request
    pub weight: u8,
    pub preferred_ops: Vec<String>, // operator names with higher weights
}

// PSI Language Mapping - abstract logic to syntax
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PSI_LanguageMap {
    pub language_id: String,
    pub abstract_operator_id: String,
    pub emission_template_id: String,
}

// PSI Execution Graph
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PSI_Graph {
    pub id: u32,
    pub name: String,
    pub nodes: Vec<PSI_GraphNode>,
    pub edges: Vec<PSI_GraphEdge>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PSI_GraphNode {
    pub id: u32,
    pub operator_name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PSI_GraphEdge {
    pub from: u32,
    pub to: u32,
    pub edge_type: String, // "data", "control", "condition"
}

// PSI Context - isolated reasoning session
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PSI_Context {
    pub id: u32,
    pub name: String,
    pub domain: String,
    pub variables: HashMap<String, String>,
    pub constraints: Vec<String>,
}

// Main PSI Brain structure
#[derive(Serialize, Deserialize, Debug)]
pub struct PSI_Brain {
    pub name: String,
    pub version: String,
    pub operators: Vec<PSI_Operator>,
    pub meta_programs: Vec<PSI_MetaProgram>,
    pub heuristics: Vec<PSI_Heuristic>,
    pub language_maps: Vec<PSI_LanguageMap>,
    pub graphs: Vec<PSI_Graph>,
    pub contexts: Vec<PSI_Context>,
    pub active_context: Option<String>,
}

impl PSI_Brain {
    pub fn new(name: &str) -> Self {
        PSI_Brain {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            operators: Vec::new(),
            meta_programs: Vec::new(),
            heuristics: Vec::new(),
            language_maps: Vec::new(),
            graphs: Vec::new(),
            contexts: Vec::new(),
            active_context: None,
        }
    }

    pub fn add_operator(&mut self, operator: PSI_Operator) {
        self.operators.push(operator);
    }

    pub fn add_meta_program(&mut self, meta_program: PSI_MetaProgram) {
        self.meta_programs.push(meta_program);
    }

    pub fn add_heuristic(&mut self, heuristic: PSI_Heuristic) {
        self.heuristics.push(heuristic);
    }

    pub fn add_language_map(&mut self, language_map: PSI_LanguageMap) {
        self.language_maps.push(language_map);
    }

    pub fn add_graph(&mut self, graph: PSI_Graph) {
        self.graphs.push(graph);
    }

    pub fn add_context(&mut self, context: PSI_Context) {
        self.contexts.push(context);
    }

    pub fn serialize_to_binary(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string(self)?;
        Ok(json_string.into_bytes())
    }

    pub fn deserialize_from_binary(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let json_string = String::from_utf8(data.to_vec())?;
        let brain: PSI_Brain = serde_json::from_str(&json_string)?;
        Ok(brain)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json_string)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let brain: PSI_Brain = serde_json::from_str(&contents)?;
        Ok(brain)
    }
}

// Helper functions for creating common operators
impl PSI_Operator {
    pub fn new_simple(name: &str, kern_template: &str) -> Self {
        let mut emissions = HashMap::new();
        emissions.insert("rust".to_string(), format!("// Rust implementation for {}", name));
        emissions.insert("python".to_string(), format!("# Python implementation for {}", name));
        emissions.insert("go".to_string(), format!("// Go implementation for {}", name));
        emissions.insert("javascript".to_string(), format!("// JavaScript implementation for {}", name));

        PSI_Operator {
            id: 0, // Will be set by the brain
            name: name.to_string(),
            domain: "general".to_string(),
            purity: 1, // impure by default
            arity_in: 1,
            arity_out: 1,
            cost_hint: 10,
            kern_template: kern_template.to_string(),
            emissions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psi_brain_creation() {
        let mut brain = PSI_Brain::new("test-brain");
        
        // Add a simple operator
        let op = PSI_Operator::new_simple(
            "TestOperator", 
            "rule TestOperator: if 1 == 1 then log(\"Test\")"
        );
        brain.add_operator(op);
        
        // Add a meta-program
        let mp = PSI_MetaProgram {
            id: 1,
            name: "TestMetaProgram".to_string(),
            operators: vec!["TestOperator".to_string()],
            domain: "testing".to_string(),
            adaptability: 50,
        };
        brain.add_meta_program(mp);
        
        assert_eq!(brain.operators.len(), 1);
        assert_eq!(brain.meta_programs.len(), 1);
    }

    #[test]
    fn test_serialization() {
        let mut brain = PSI_Brain::new("serialization-test");
        let op = PSI_Operator::new_simple(
            "SerializationTest", 
            "rule SerializationTest: if 1 == 1 then log(\"Serialization\")"
        );
        brain.add_operator(op);
        
        // Test binary serialization
        let binary = brain.serialize_to_binary().unwrap();
        let deserialized = PSI_Brain::deserialize_from_binary(&binary).unwrap();
        
        assert_eq!(deserialized.name, "serialization-test");
        assert_eq!(deserialized.operators.len(), 1);
    }
}