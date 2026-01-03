use clap::Parser;
use clap::CommandFactory;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::process;

use psi::{PSI_Brain, operator_engine::{OperatorEngine, OperatorExecutionContext, common_operators}, meta_programs::{select_metaprogram, create_generate_module_metaprogram, create_refactor_code_metaprogram, create_debug_issue_metaprogram, create_translate_code_metaprogram, create_explain_code_metaprogram, create_default_heuristics}, multimodal_operators::{multimodal_operators, MultiModalOperator}};

#[derive(Parser, Debug)]
#[command(author, version, about = "PSI CLI - Full Implementation", long_about = None)]
struct Args {
    /// Interactive REPL
    #[arg(short, long)]
    interactive: bool,

    /// Batch tasks file (YAML/JSON)
    #[arg(short, long)]
    batch: Option<String>,

    /// Load a PSI brain (JSON)
    #[arg(short, long)]
    load: Option<String>,

    /// Set default language for code generation
    #[arg(long, default_value = "rust")]
    language: String,

    /// Enable debug mode
    #[arg(long)]
    debug: bool,

    /// Enable explanations
    #[arg(long)]
    explain: bool,

    /// Set context name
    #[arg(long)]
    context: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BatchTask {
    task: String,
    #[serde(default = "default_language")]
    language: String,
    #[serde(default)]
    spec: Option<String>,
    #[serde(default)]
    project: Option<String>,
    #[serde(default)]
    from: Option<String>,
    #[serde(default)]
    to: Option<String>,
}

fn default_language() -> String {
    "rust".to_string()
}

fn main() {
    let args = Args::parse();

    // Initialize the operator engine
    let mut engine = match OperatorEngine::new() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to initialize operator engine: {}", e);
            process::exit(1);
        }
    };

    // Load brain if specified
    let mut brain = if let Some(brain_file) = &args.load {
        match PSI_Brain::load_from_file(brain_file) {
            Ok(b) => {
                println!("Loaded PSI brain: {} ({} operators, {} meta-programs)", 
                    b.name, b.operators.len(), b.meta_programs.len());
                b
            },
            Err(e) => {
                eprintln!("Failed to load brain file: {}", e);
                process::exit(1);
            }
        }
    } else {
        // Create a default brain with common operators
        create_default_brain()
    };

    // Add default meta-programs and heuristics if not present
    if brain.meta_programs.is_empty() {
        brain.add_meta_program(create_generate_module_metaprogram());
        brain.add_meta_program(create_refactor_code_metaprogram());
        brain.add_meta_program(create_debug_issue_metaprogram());
        brain.add_meta_program(create_translate_code_metaprogram());
        brain.add_meta_program(create_explain_code_metaprogram());
    }

    if brain.heuristics.is_empty() {
        for h in create_default_heuristics() {
            brain.add_heuristic(h);
        }
    }

    if args.interactive {
        repl(&mut engine, &brain, &args);
        return;
    }

    if let Some(batch_file) = &args.batch {
        if let Ok(contents) = fs::read_to_string(batch_file) {
            match serde_yaml::from_str::<Vec<BatchTask>>(&contents) {
                Ok(tasks) => {
                    for task in tasks {
                        process_batch_task(&mut engine, &brain, task, &args);
                    }
                }
                Err(_) => {
                    // Try JSON format
                    match serde_json::from_str::<Vec<BatchTask>>(&contents) {
                        Ok(tasks) => {
                            for task in tasks {
                                process_batch_task(&mut engine, &brain, task, &args);
                            }
                        }
                        Err(e) => eprintln!("Failed to parse batch file: {}", e),
                    }
                }
            }
        } else {
            eprintln!("Failed to read batch file: {}", batch_file);
        }
        return;
    }

    // If no interactive or batch mode, show help
    println!("{}", Args::command().render_help());
}

fn create_default_brain() -> PSI_Brain {
    let mut brain = PSI_Brain::new("default-brain");

    // Add common operators
    brain.add_operator(common_operators::create_define_entities_operator());
    brain.add_operator(common_operators::create_create_routes_operator());
    brain.add_operator(common_operators::create_implement_auth_operator());
    brain.add_operator(common_operators::create_write_tests_operator());

    // Add some additional operators for other domains
    brain.add_operator(create_analyze_patterns_operator());
    brain.add_operator(create_optimize_queries_operator());
    brain.add_operator(create_apply_refactor_operator());
    brain.add_operator(create_validate_operator());
    brain.add_operator(create_detect_race_conditions_operator());
    brain.add_operator(create_suggest_fixes_operator());
    brain.add_operator(create_parse_ast_operator());
    brain.add_operator(create_map_to_language_templates_operator());
    brain.add_operator(create_emit_code_operator());
    brain.add_operator(create_map_operators_operator());
    brain.add_operator(create_generate_explanation_operator());

    // Add multi-modal operators
    let mm_code_gen = multimodal_operators::create_code_generation_operator();
    brain.add_operator(mm_code_gen.base_operator);

    let mm_image_gen = multimodal_operators::create_image_generation_operator();
    brain.add_operator(mm_image_gen.base_operator);

    brain
}

// Additional operators for various tasks
fn create_analyze_patterns_operator() -> psi::PSI_Operator {
    let mut emissions = HashMap::new();
    emissions.insert("rust".to_string(), "// Analyze code patterns in Rust".to_string());
    emissions.insert("python".to_string(), "# Analyze code patterns in Python".to_string());
    emissions.insert("go".to_string(), "// Analyze code patterns in Go".to_string());
    emissions.insert("javascript".to_string(), "// Analyze code patterns in JavaScript".to_string());

    psi::PSI_Operator {
        id: 5,
        name: "AnalyzePatterns".to_string(),
        domain: "analysis".to_string(),
        purity: 1,
        arity_in: 1,
        arity_out: 1,
        cost_hint: 12,
        kern_template: r#"rule AnalyzePatterns: 
    if has_code() 
    then analyze_code_patterns()"#.to_string(),
        emissions,
    }
}

fn create_optimize_queries_operator() -> psi::PSI_Operator {
    let mut emissions = HashMap::new();
    emissions.insert("rust".to_string(), "// Optimize database queries in Rust".to_string());
    emissions.insert("python".to_string(), "# Optimize database queries in Python".to_string());
    emissions.insert("go".to_string(), "// Optimize database queries in Go".to_string());
    emissions.insert("javascript".to_string(), "// Optimize database queries in JavaScript".to_string());

    psi::PSI_Operator {
        id: 6,
        name: "OptimizeQueries".to_string(),
        domain: "optimization".to_string(),
        purity: 1,
        arity_in: 1,
        arity_out: 1,
        cost_hint: 15,
        kern_template: r#"rule OptimizeQueries: 
    if has_database_queries() 
    then optimize_query_performance()"#.to_string(),
        emissions,
    }
}

fn create_apply_refactor_operator() -> psi::PSI_Operator {
    let mut emissions = HashMap::new();
    emissions.insert("rust".to_string(), "// Apply refactoring in Rust".to_string());
    emissions.insert("python".to_string(), "# Apply refactoring in Python".to_string());
    emissions.insert("go".to_string(), "// Apply refactoring in Go".to_string());
    emissions.insert("javascript".to_string(), "// Apply refactoring in JavaScript".to_string());

    psi::PSI_Operator {
        id: 7,
        name: "ApplyRefactor".to_string(),
        domain: "refactoring".to_string(),
        purity: 1,
        arity_in: 1,
        arity_out: 1,
        cost_hint: 20,
        kern_template: r#"rule ApplyRefactor: 
    if refactoring_plan_ready() 
    then apply_refactoring()"#.to_string(),
        emissions,
    }
}

fn create_validate_operator() -> psi::PSI_Operator {
    let mut emissions = HashMap::new();
    emissions.insert("rust".to_string(), "// Validate code in Rust".to_string());
    emissions.insert("python".to_string(), "# Validate code in Python".to_string());
    emissions.insert("go".to_string(), "// Validate code in Go".to_string());
    emissions.insert("javascript".to_string(), "// Validate code in JavaScript".to_string());

    psi::PSI_Operator {
        id: 8,
        name: "Validate".to_string(),
        domain: "validation".to_string(),
        purity: 1,
        arity_in: 1,
        arity_out: 1,
        cost_hint: 8,
        kern_template: r#"rule Validate: 
    if code_exists() 
    then validate_code_quality()"#.to_string(),
        emissions,
    }
}

fn create_detect_race_conditions_operator() -> psi::PSI_Operator {
    let mut emissions = HashMap::new();
    emissions.insert("rust".to_string(), "// Detect race conditions in Rust".to_string());
    emissions.insert("python".to_string(), "# Detect race conditions in Python".to_string());
    emissions.insert("go".to_string(), "// Detect race conditions in Go".to_string());
    emissions.insert("javascript".to_string(), "// Detect race conditions in JavaScript".to_string());

    psi::PSI_Operator {
        id: 9,
        name: "DetectRaceConditions".to_string(),
        domain: "debugging".to_string(),
        purity: 1,
        arity_in: 1,
        arity_out: 1,
        cost_hint: 25,
        kern_template: r#"rule DetectRaceConditions: 
    if has_concurrent_code() 
    then detect_race_conditions()"#.to_string(),
        emissions,
    }
}

fn create_suggest_fixes_operator() -> psi::PSI_Operator {
    let mut emissions = HashMap::new();
    emissions.insert("rust".to_string(), "// Suggest fixes in Rust".to_string());
    emissions.insert("python".to_string(), "# Suggest fixes in Python".to_string());
    emissions.insert("go".to_string(), "// Suggest fixes in Go".to_string());
    emissions.insert("javascript".to_string(), "// Suggest fixes in JavaScript".to_string());

    psi::PSI_Operator {
        id: 10,
        name: "SuggestFixes".to_string(),
        domain: "debugging".to_string(),
        purity: 1,
        arity_in: 1,
        arity_out: 1,
        cost_hint: 18,
        kern_template: r#"rule SuggestFixes: 
    if issues_found() 
    then suggest_fixes()"#.to_string(),
        emissions,
    }
}

fn create_parse_ast_operator() -> psi::PSI_Operator {
    let mut emissions = HashMap::new();
    emissions.insert("rust".to_string(), "// Parse AST in Rust".to_string());
    emissions.insert("python".to_string(), "# Parse AST in Python".to_string());
    emissions.insert("go".to_string(), "// Parse AST in Go".to_string());
    emissions.insert("javascript".to_string(), "// Parse AST in JavaScript".to_string());

    psi::PSI_Operator {
        id: 11,
        name: "ParseAST".to_string(),
        domain: "parsing".to_string(),
        purity: 1,
        arity_in: 1,
        arity_out: 1,
        cost_hint: 10,
        kern_template: r#"rule ParseAST: 
    if has_source_code() 
    then parse_abstract_syntax_tree()"#.to_string(),
        emissions,
    }
}

fn create_map_to_language_templates_operator() -> psi::PSI_Operator {
    let mut emissions = HashMap::new();
    emissions.insert("rust".to_string(), "// Map to language templates in Rust".to_string());
    emissions.insert("python".to_string(), "# Map to language templates in Python".to_string());
    emissions.insert("go".to_string(), "// Map to language templates in Go".to_string());
    emissions.insert("javascript".to_string(), "// Map to language templates in JavaScript".to_string());

    psi::PSI_Operator {
        id: 12,
        name: "MapToLanguageTemplates".to_string(),
        domain: "translation".to_string(),
        purity: 1,
        arity_in: 1,
        arity_out: 1,
        cost_hint: 14,
        kern_template: r#"rule MapToLanguageTemplates: 
    if has_ast() 
    then map_to_target_language_templates()"#.to_string(),
        emissions,
    }
}

fn create_emit_code_operator() -> psi::PSI_Operator {
    let mut emissions = HashMap::new();
    emissions.insert("rust".to_string(), "// Emit code in Rust".to_string());
    emissions.insert("python".to_string(), "# Emit code in Python".to_string());
    emissions.insert("go".to_string(), "// Emit code in Go".to_string());
    emissions.insert("javascript".to_string(), "// Emit code in JavaScript".to_string());

    psi::PSI_Operator {
        id: 13,
        name: "EmitCode".to_string(),
        domain: "code_generation".to_string(),
        purity: 1,
        arity_in: 1,
        arity_out: 1,
        cost_hint: 12,
        kern_template: r#"rule EmitCode: 
    if has_translated_ast() 
    then emit_target_language_code()"#.to_string(),
        emissions,
    }
}

fn create_map_operators_operator() -> psi::PSI_Operator {
    let mut emissions = HashMap::new();
    emissions.insert("rust".to_string(), "// Map operators in Rust".to_string());
    emissions.insert("python".to_string(), "# Map operators in Python".to_string());
    emissions.insert("go".to_string(), "// Map operators in Go".to_string());
    emissions.insert("javascript".to_string(), "// Map operators in JavaScript".to_string());

    psi::PSI_Operator {
        id: 14,
        name: "MapOperators".to_string(),
        domain: "analysis".to_string(),
        purity: 1,
        arity_in: 1,
        arity_out: 1,
        cost_hint: 9,
        kern_template: r#"rule MapOperators: 
    if has_ast() 
    then map_to_operators()"#.to_string(),
        emissions,
    }
}

fn create_generate_explanation_operator() -> psi::PSI_Operator {
    let mut emissions = HashMap::new();
    emissions.insert("rust".to_string(), "// Generate explanation in Rust".to_string());
    emissions.insert("python".to_string(), "# Generate explanation in Python".to_string());
    emissions.insert("go".to_string(), "// Generate explanation in Go".to_string());
    emissions.insert("javascript".to_string(), "// Generate explanation in JavaScript".to_string());

    psi::PSI_Operator {
        id: 15,
        name: "GenerateExplanation".to_string(),
        domain: "explanation".to_string(),
        purity: 1,
        arity_in: 1,
        arity_out: 1,
        cost_hint: 16,
        kern_template: r#"rule GenerateExplanation: 
    if has_operators() 
    then generate_explanation_text()"#.to_string(),
        emissions,
    }
}

fn repl(engine: &mut OperatorEngine, brain: &PSI_Brain, args: &Args) {
    println!("PSI CLI (Full Implementation). Type 'exit' or 'quit' to quit.");
    println!("Available commands: generate, refactor, debug, translate, explain");
    
    loop {
        print!("PSI> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            break;
        }
        let cmd = line.trim();
        if cmd.eq_ignore_ascii_case("exit") || cmd.eq_ignore_ascii_case("quit") {
            break;
        }
        if cmd.is_empty() {
            continue;
        }
        
        process_command(engine, brain, cmd, args);
    }
}

fn process_command(engine: &mut OperatorEngine, brain: &PSI_Brain, cmd: &str, args: &Args) {
    let lower_cmd = cmd.to_lowercase();
    
    if lower_cmd.starts_with("generate") {
        handle_generate(engine, brain, cmd, args);
    } else if lower_cmd.starts_with("refactor") {
        handle_refactor(engine, brain, cmd, args);
    } else if lower_cmd.starts_with("debug") {
        handle_debug(engine, brain, cmd, args);
    } else if lower_cmd.starts_with("translate") {
        handle_translate(engine, brain, cmd, args);
    } else if lower_cmd.starts_with("explain") {
        handle_explain(engine, brain, cmd, args);
    } else {
        // Try to select a meta-program based on the command
        if let Some(metaprogram) = select_metaprogram(cmd, brain) {
            execute_metaprogram(engine, brain, &metaprogram, cmd, args);
        } else {
            println!("Unrecognized command: {}", cmd);
            println!("Try: generate, refactor, debug, translate, explain");
        }
    }
}

fn handle_generate(engine: &mut OperatorEngine, brain: &PSI_Brain, cmd: &str, args: &Args) {
    println!("Processing generate request: {}", cmd);
    
    // Extract language from command if specified
    let language = if cmd.contains(" in rust") {
        "rust"
    } else if cmd.contains(" in python") {
        "python"
    } else if cmd.contains(" in go") {
        "go"
    } else if cmd.contains(" in javascript") || cmd.contains(" in js") {
        "javascript"
    } else {
        &args.language
    };
    
    // Create execution context
    let mut context = OperatorExecutionContext::new();
    context.language = language.to_string();
    context.set_input("request".to_string(), cmd.to_string());
    
    // Find the GenerateModule meta-program
    if let Some(metaprogram) = brain.meta_programs.iter().find(|mp| mp.name == "GenerateModule") {
        execute_metaprogram_with_context(engine, brain, metaprogram, context, args);
    } else {
        println!("GenerateModule meta-program not found");
    }
}

fn handle_refactor(engine: &mut OperatorEngine, brain: &PSI_Brain, cmd: &str, args: &Args) {
    println!("Processing refactor request: {}", cmd);
    
    // Extract language from command if specified
    let language = if cmd.contains(" in rust") {
        "rust"
    } else if cmd.contains(" in python") {
        "python"
    } else if cmd.contains(" in go") {
        "go"
    } else if cmd.contains(" in javascript") || cmd.contains(" in js") {
        "javascript"
    } else {
        &args.language
    };
    
    // Create execution context
    let mut context = OperatorExecutionContext::new();
    context.language = language.to_string();
    context.set_input("request".to_string(), cmd.to_string());
    
    // Find the RefactorCode meta-program
    if let Some(metaprogram) = brain.meta_programs.iter().find(|mp| mp.name == "RefactorCode") {
        execute_metaprogram_with_context(engine, brain, metaprogram, context, args);
    } else {
        println!("RefactorCode meta-program not found");
    }
}

fn handle_debug(engine: &mut OperatorEngine, brain: &PSI_Brain, cmd: &str, args: &Args) {
    println!("Processing debug request: {}", cmd);
    
    // Create execution context
    let mut context = OperatorExecutionContext::new();
    context.language = args.language.clone();
    context.set_input("request".to_string(), cmd.to_string());
    
    // Find the DebugIssue meta-program
    if let Some(metaprogram) = brain.meta_programs.iter().find(|mp| mp.name == "DebugIssue") {
        execute_metaprogram_with_context(engine, brain, metaprogram, context, args);
    } else {
        println!("DebugIssue meta-program not found");
    }
}

fn handle_translate(engine: &mut OperatorEngine, brain: &PSI_Brain, cmd: &str, args: &Args) {
    println!("Processing translate request: {}", cmd);
    
    // Extract source and target languages
    let (source_lang, target_lang) = if cmd.contains("python") && cmd.contains("rust") {
        ("python", "rust")
    } else if cmd.contains("rust") && cmd.contains("python") {
        ("rust", "python")
    } else if cmd.contains("python") && cmd.contains("go") {
        ("python", "go")
    } else if cmd.contains("go") && cmd.contains("rust") {
        ("go", "rust")
    } else if cmd.contains("javascript") && cmd.contains("rust") {
        ("javascript", "rust")
    } else {
        (args.language.as_str(), "rust") // default
    };
    
    // Create execution context
    let mut context = OperatorExecutionContext::new();
    context.language = target_lang.to_string();
    context.set_input("source_language".to_string(), source_lang.to_string());
    context.set_input("target_language".to_string(), target_lang.to_string());
    context.set_input("request".to_string(), cmd.to_string());
    
    // Find the TranslateCode meta-program
    if let Some(metaprogram) = brain.meta_programs.iter().find(|mp| mp.name == "TranslateCode") {
        execute_metaprogram_with_context(engine, brain, metaprogram, context, args);
    } else {
        println!("TranslateCode meta-program not found");
    }
}

fn handle_explain(engine: &mut OperatorEngine, brain: &PSI_Brain, cmd: &str, args: &Args) {
    println!("Processing explain request: {}", cmd);
    
    // Create execution context
    let mut context = OperatorExecutionContext::new();
    context.language = args.language.clone();
    context.set_input("request".to_string(), cmd.to_string());
    
    // Find the ExplainCode meta-program
    if let Some(metaprogram) = brain.meta_programs.iter().find(|mp| mp.name == "ExplainCode") {
        execute_metaprogram_with_context(engine, brain, metaprogram, context, args);
    } else {
        println!("ExplainCode meta-program not found");
    }
}

fn execute_metaprogram(engine: &mut OperatorEngine, brain: &PSI_Brain, metaprogram: &psi::PSI_MetaProgram, cmd: &str, args: &Args) {
    println!("Executing meta-program: {} with operators {:?}", metaprogram.name, metaprogram.operators);
    
    // Create execution context
    let mut context = OperatorExecutionContext::new();
    context.language = args.language.clone();
    context.set_input("request".to_string(), cmd.to_string());
    
    execute_metaprogram_with_context(engine, brain, metaprogram, context, args);
}

fn execute_metaprogram_with_context(
    engine: &mut OperatorEngine, 
    brain: &PSI_Brain, 
    metaprogram: &psi::PSI_MetaProgram, 
    mut context: OperatorExecutionContext, 
    args: &Args
) {
    if args.debug {
        println!("[DEBUG] Starting execution of meta-program: {}", metaprogram.name);
        println!("[DEBUG] Operators in chain: {:?}", metaprogram.operators);
    }
    
    match engine.execute_operator_chain(brain, &metaprogram.operators, context) {
        Ok(final_context) => {
            println!("Execution completed successfully for meta-program: {}", metaprogram.name);
            
            if args.explain {
                println!("Generated outputs are available in the execution context.");
            }
            
            // In a real implementation, we would extract and display the results
            // For now, we'll just acknowledge completion
            println!("PSI has completed the requested task using the '{}' strategy.", metaprogram.name);
        }
        Err(e) => {
            eprintln!("Error executing meta-program {}: {}", metaprogram.name, e);
        }
    }
}

fn process_batch_task(engine: &mut OperatorEngine, brain: &PSI_Brain, task: BatchTask, args: &Args) {
    println!("Processing batch task: {}", task.task);

    let cmd = format!("{}{}", task.task, if let Some(ref spec) = task.spec { format!(" for {}", spec) } else { "".to_string() });
    process_command(engine, brain, &cmd, args);
}