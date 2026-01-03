use clap::Parser;
use kern_parser::Parser;
use kern_lexer::Lexer;
use kern_graph_builder::GraphBuilder;
use kern_bytecode::BytecodeCompiler;
use std::fs;
use std::path::Path;

/// KERN Compiler CLI - Compiles KERN source code to bytecode
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input KERN source file
    #[arg(short, long)]
    input: String,

    /// Output file path
    #[arg(short, long, default_value = "output.kbc")]
    output: String,

    /// Command to execute
    #[arg(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Compile source to bytecode
    Build,
    /// Parse and validate without output
    Check,
    /// Emit execution graph
    Graph,
    /// Show intermediate representation
    Ir,
    /// Verify existing bytecode file
    Verify,
    /// Report symbols, entities, rules
    Stats,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Build => {
            println!("Building KERN source: {}", args.input);
            compile_to_bytecode(&args.input, &args.output);
        },
        Commands::Check => {
            println!("Checking KERN source: {}", args.input);
            check_source(&args.input);
        },
        Commands::Graph => {
            println!("Generating execution graph for: {}", args.input);
            generate_graph(&args.input);
        },
        Commands::Ir => {
            println!("Showing intermediate representation for: {}", args.input);
            show_ir(&args.input);
        },
        Commands::Verify => {
            println!("Verifying bytecode file: {}", args.input);
            verify_bytecode(&args.input);
        },
        Commands::Stats => {
            println!("Reporting statistics for: {}", args.input);
            report_stats(&args.input);
        }
    }
}

fn compile_to_bytecode(input_file: &str, output_file: &str) {
    // Read the source file
    let source_code = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    // Tokenize the source
    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();

    // Parse the tokens
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    // Check for parsing errors
    if !parser.errors.is_empty() {
        eprintln!("Parsing errors found:");
        for error in &parser.errors {
            eprintln!("  {}", error);
        }
        return;
    }

    // Build execution graph
    let mut graph_builder = GraphBuilder::new();
    let execution_graph = graph_builder.build_execution_graph(&program);

    // Compile to bytecode
    let mut bytecode_compiler = BytecodeCompiler::new();
    let bytecode = bytecode_compiler.compile_graph(&execution_graph);

    // Write bytecode to output file
    fs::write(output_file, serde_json::to_string(&bytecode).unwrap())
        .expect("Failed to write bytecode to output file");

    println!("Successfully compiled {} to {}", input_file, output_file);
}

fn check_source(input_file: &str) {
    // Read the source file
    let source_code = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    // Tokenize the source
    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();

    // Parse the tokens
    let mut parser = Parser::new(tokens);
    let _program = parser.parse_program();

    // Check for parsing errors
    if !parser.errors.is_empty() {
        eprintln!("Parsing errors found:");
        for error in &parser.errors {
            eprintln!("  {}", error);
        }
    } else {
        println!("Source code is valid - no errors found");
    }
}

fn generate_graph(input_file: &str) {
    // Read the source file
    let source_code = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    // Tokenize the source
    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();

    // Parse the tokens
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    // Check for parsing errors
    if !parser.errors.is_empty() {
        eprintln!("Parsing errors found:");
        for error in &parser.errors {
            eprintln!("  {}", error);
        }
        return;
    }

    // Build execution graph
    let mut graph_builder = GraphBuilder::new();
    let execution_graph = graph_builder.build_execution_graph(&program);

    // Write graph to file
    let graph_output = format!("{}.kgraph", input_file.replace(".kern", ""));
    fs::write(&graph_output, serde_json::to_string(&execution_graph).unwrap())
        .expect("Failed to write graph to output file");

    println!("Execution graph saved to {}", graph_output);
}

fn show_ir(input_file: &str) {
    // Read the source file
    let source_code = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    // Tokenize the source
    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();

    // Parse the tokens
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    // Print the AST
    println!("AST representation:");
    println!("{:#?}", program);
}

fn verify_bytecode(bytecode_file: &str) {
    // Read the bytecode file
    let bytecode_content = fs::read_to_string(bytecode_file)
        .expect("Failed to read bytecode file");

    // Attempt to deserialize the bytecode
    match serde_json::from_str::<Vec<u8>>(&bytecode_content) {
        Ok(_) => println!("Bytecode file is valid"),
        Err(e) => eprintln!("Invalid bytecode file: {}", e),
    }
}

fn report_stats(input_file: &str) {
    // Read the source file
    let source_code = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    // Tokenize the source
    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();

    // Parse the tokens
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    // Count entities, rules, flows, and constraints
    let mut entity_count = 0;
    let mut rule_count = 0;
    let mut flow_count = 0;
    let mut constraint_count = 0;

    for definition in &program.definitions {
        match definition {
            kern_parser::ast_nodes::Definition::Entity(_) => entity_count += 1,
            kern_parser::ast_nodes::Definition::Rule(_) => rule_count += 1,
            kern_parser::ast_nodes::Definition::Flow(_) => flow_count += 1,
            kern_parser::ast_nodes::Definition::Constraint(_) => constraint_count += 1,
        }
    }

    println!("Statistics for {}: ", input_file);
    println!("  Entities: {}", entity_count);
    println!("  Rules: {}", rule_count);
    println!("  Flows: {}", flow_count);
    println!("  Constraints: {}", constraint_count);
    println!("  Total definitions: {}", entity_count + rule_count + flow_count + constraint_count);
}