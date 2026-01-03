use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{PSI_MetaProgram, PSI_Brain};

pub fn create_generate_module_metaprogram() -> PSI_MetaProgram {
    PSI_MetaProgram {
        id: 1,
        name: "GenerateModule".to_string(),
        operators: vec![
            "DefineEntities".to_string(),
            "CreateRoutes".to_string(),
            "ImplementAuth".to_string(),
            "WriteTests".to_string(),
        ],
        domain: "code_generation".to_string(),
        adaptability: 80,
    }
}

pub fn create_refactor_code_metaprogram() -> PSI_MetaProgram {
    PSI_MetaProgram {
        id: 2,
        name: "RefactorCode".to_string(),
        operators: vec![
            "AnalyzePatterns".to_string(),
            "OptimizeQueries".to_string(),
            "ApplyRefactor".to_string(),
            "Validate".to_string(),
        ],
        domain: "refactoring".to_string(),
        adaptability: 70,
    }
}

pub fn create_debug_issue_metaprogram() -> PSI_MetaProgram {
    PSI_MetaProgram {
        id: 3,
        name: "DebugIssue".to_string(),
        operators: vec![
            "AnalyzePatterns".to_string(),
            "DetectRaceConditions".to_string(),
            "SuggestFixes".to_string(),
        ],
        domain: "debugging".to_string(),
        adaptability: 60,
    }
}

pub fn create_translate_code_metaprogram() -> PSI_MetaProgram {
    PSI_MetaProgram {
        id: 4,
        name: "TranslateCode".to_string(),
        operators: vec![
            "ParseAST".to_string(),
            "MapToLanguageTemplates".to_string(),
            "EmitCode".to_string(),
        ],
        domain: "translation".to_string(),
        adaptability: 90,
    }
}

pub fn create_explain_code_metaprogram() -> PSI_MetaProgram {
    PSI_MetaProgram {
        id: 5,
        name: "ExplainCode".to_string(),
        operators: vec![
            "ParseAST".to_string(),
            "MapOperators".to_string(),
            "GenerateExplanation".to_string(),
        ],
        domain: "explanation".to_string(),
        adaptability: 85,
    }
}

pub fn create_default_heuristics() -> Vec<crate::PSI_Heuristic> {
    vec![
        crate::PSI_Heuristic {
            id: 1,
            name: "generate_heuristic".to_string(),
            trigger_type: "request".to_string(),
            weight: 90,
            preferred_ops: vec![
                "DefineEntities".to_string(),
                "CreateRoutes".to_string(),
                "ImplementAuth".to_string(),
                "WriteTests".to_string(),
            ],
        },
        crate::PSI_Heuristic {
            id: 2,
            name: "refactor_heuristic".to_string(),
            trigger_type: "request".to_string(),
            weight: 85,
            preferred_ops: vec![
                "AnalyzePatterns".to_string(),
                "OptimizeQueries".to_string(),
                "ApplyRefactor".to_string(),
                "Validate".to_string(),
            ],
        },
        crate::PSI_Heuristic {
            id: 3,
            name: "debug_heuristic".to_string(),
            trigger_type: "error".to_string(),
            weight: 95,
            preferred_ops: vec![
                "AnalyzePatterns".to_string(),
                "DetectRaceConditions".to_string(),
                "SuggestFixes".to_string(),
            ],
        },
        crate::PSI_Heuristic {
            id: 4,
            name: "translate_heuristic".to_string(),
            trigger_type: "request".to_string(),
            weight: 80,
            preferred_ops: vec![
                "ParseAST".to_string(),
                "MapToLanguageTemplates".to_string(),
                "EmitCode".to_string(),
            ],
        },
    ]
}

// Function to select the best meta-program based on a query
pub fn select_metaprogram(query: &str, brain: &PSI_Brain) -> Option<PSI_MetaProgram> {
    let lower_query = query.to_lowercase();
    
    // Simple keyword matching for demonstration
    // In a real implementation, this would use more sophisticated NLP
    for mp in &brain.meta_programs {
        if lower_query.contains(&mp.name.to_lowercase()) {
            return Some(mp.clone());
        }
        
        // Check if any operator in the meta-program matches the query
        for op_name in &mp.operators {
            if lower_query.contains(&op_name.to_lowercase()) {
                return Some(mp.clone());
            }
        }
    }
    
    // If no exact match, use heuristics to find the best match
    apply_heuristics(&lower_query, brain)
}

// Apply heuristics to find the best meta-program
fn apply_heuristics(query: &str, brain: &PSI_Brain) -> Option<PSI_MetaProgram> {
    let mut scores: HashMap<String, u32> = HashMap::new();
    
    for heuristic in &brain.heuristics {
        let mut score = 0;
        
        // Check if query contains keywords related to this heuristic
        if query.contains("generate") || query.contains("create") || query.contains("new") {
            if heuristic.name == "generate_heuristic" {
                score += heuristic.weight as u32;
            }
        }
        
        if query.contains("refactor") || query.contains("improve") || query.contains("optimize") {
            if heuristic.name == "refactor_heuristic" {
                score += heuristic.weight as u32;
            }
        }
        
        if query.contains("debug") || query.contains("fix") || query.contains("error") || query.contains("issue") {
            if heuristic.name == "debug_heuristic" {
                score += heuristic.weight as u32;
            }
        }
        
        if query.contains("translate") || query.contains("convert") {
            if heuristic.name == "translate_heuristic" {
                score += heuristic.weight as u32;
            }
        }
        
        // Score based on preferred operators
        for op_name in &heuristic.preferred_ops {
            if query.contains(op_name) {
                score += 10; // Bonus for matching preferred operators
            }
        }
        
        if score > 0 {
            // Find the meta-program that contains the highest-scoring operators
            for mp in &brain.meta_programs {
                for op_name in &heuristic.preferred_ops {
                    if mp.operators.contains(op_name) {
                        *scores.entry(mp.name.clone()).or_insert(0) += score;
                    }
                }
            }
        }
    }
    
    // Find the meta-program with the highest score
    if let Some((best_mp_name, _)) = scores.iter().max_by_key(|(_, &score)| score) {
        return brain.meta_programs.iter()
            .find(|mp| &mp.name == best_mp_name)
            .cloned();
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PSI_Brain, PSI_Operator, operator_engine::common_operators};

    #[test]
    fn test_metaprogram_creation() {
        let mp = create_generate_module_metaprogram();
        assert_eq!(mp.name, "GenerateModule");
        assert_eq!(mp.operators.len(), 4);
        assert_eq!(mp.operators[0], "DefineEntities");
    }

    #[test]
    fn test_heuristics_creation() {
        let heuristics = create_default_heuristics();
        assert_eq!(heuristics.len(), 4);
        assert_eq!(heuristics[0].name, "generate_heuristic");
    }

    #[test]
    fn test_select_metaprogram() {
        let mut brain = PSI_Brain::new("test");
        
        // Add operators
        brain.add_operator(common_operators::create_define_entities_operator());
        brain.add_operator(common_operators::create_create_routes_operator());
        brain.add_operator(common_operators::create_implement_auth_operator());
        brain.add_operator(common_operators::create_write_tests_operator());
        
        // Add meta-programs
        brain.add_meta_program(create_generate_module_metaprogram());
        brain.add_meta_program(create_refactor_code_metaprogram());
        
        // Add heuristics
        for h in create_default_heuristics() {
            brain.add_heuristic(h);
        }
        
        // Test selection
        let mp = select_metaprogram("generate a new module", &brain);
        assert!(mp.is_some());
        assert_eq!(mp.unwrap().name, "GenerateModule");
    }
}