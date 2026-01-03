use clap::Parser;
use kern_parser::Parser as KernParser;
use kern_parser::Definition;
use kern_graph_builder::GraphBuilder;
use kern_bytecode::{BytecodeCompiler, BytecodeModule};
use kern_vm::{VirtualMachine, VMConfig};
use kern_vm::vm_safety::sandbox::SandboxPolicy;
use std::fs;

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
    #[command(subcommand)]
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
    /// Execute bytecode
    Run,
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
        },
        Commands::Run => {
            println!("Running KERN bytecode: {}", args.input);
            run_bytecode(&args.input);
        }
    }
}

fn compile_to_bytecode(input_file: &str, output_file: &str) {
    // Read the source file
    let source_code = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    // Parse
    let mut parser = KernParser::new(&source_code);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(errors) => {
            eprintln!("Parsing errors found:");
            for error in errors {
                eprintln!("  {}", error);
            }
            return;
        }
    };

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

    // Parse
    let mut parser = KernParser::new(&source_code);
    match parser.parse_program() {
        Ok(_) => println!("Source code is valid - no errors found"),
        Err(errors) => {
            eprintln!("Parsing errors found:");
            for error in errors {
                eprintln!("  {}", error);
            }
        }
    }
}

fn generate_graph(input_file: &str) {
    // Read the source file
    let source_code = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    // Parse
    let mut parser = KernParser::new(&source_code);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(errors) => {
            eprintln!("Parsing errors found:");
            for error in errors {
                eprintln!("  {}", error);
            }
            return;
        }
    };

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

    // Parse
    let mut parser = KernParser::new(&source_code);
    match parser.parse_program() {
        Ok(program) => {
            // Print the AST
            println!("AST representation:");
            println!("{:#?}", program);
        },
        Err(errors) => {
            eprintln!("Parsing errors found:");
            for error in errors {
                eprintln!("  {}", error);
            }
        }
    }
}

fn verify_bytecode(bytecode_file: &str) {
    // Read the bytecode file
    let bytecode_content = fs::read_to_string(bytecode_file)
        .expect("Failed to read bytecode file");

    // Attempt to deserialize the bytecode
    match serde_json::from_str::<BytecodeModule>(&bytecode_content) {
        Ok(_) => println!("Bytecode file is valid"),
        Err(e) => eprintln!("Invalid bytecode file: {}", e),
    }
}

fn run_bytecode(input_file: &str) {
    let bytecode_content = fs::read_to_string(input_file)
        .expect("Failed to read bytecode file");
        
    let module: BytecodeModule = serde_json::from_str(&bytecode_content)
        .expect("Failed to deserialize bytecode");
        
    // Configure VM with sandbox
    let mut config = VMConfig::new();
    let mut policy = SandboxPolicy::new();
    
    // Allow standard IO and external functions used in examples
    policy.allow_io_channel("stdout");
    // TODO: Use symbol table to map function names to IDs
    // For now, allow common IDs
    policy.allow_function("extern_fn_0"); 
    policy.set_max_calls_for_function("extern_fn_0", 100);
    
    config.sandbox_policy = policy;

    let mut vm = VirtualMachine::with_config(config);
    vm.load_program(module.instruction_stream);
    
    match vm.execute() {
        Ok(_) => println!("Execution finished successfully."),
        Err(e) => eprintln!("VM Runtime Error: {:?}", e),
    }
}

fn report_stats(input_file: &str) {
    // Read the source file
    let source_code = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    // Parse
    let mut parser = KernParser::new(&source_code);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(errors) => {
            eprintln!("Parsing errors found:");
            for error in errors {
                eprintln!("  {}", error);
            }
            return;
        }
    };

    // Count entities, rules, flows, and constraints
    let mut entity_count = 0;
    let mut rule_count = 0;
    let mut flow_count = 0;
    let mut constraint_count = 0;

    for definition in &program.definitions {
        match definition {
            Definition::Entity(_) => entity_count += 1,
            Definition::Rule(_) => rule_count += 1,
            Definition::Flow(_) => flow_count += 1,
            Definition::Constraint(_) => constraint_count += 1,
        }
    }

    println!("Statistics for {}: ", input_file);
    println!("  Entities: {}", entity_count);
    println!("  Rules: {}", rule_count);
    println!("  Flows: {}", flow_count);
    println!("  Constraints: {}", constraint_count);
    println!("  Total definitions: {}", entity_count + rule_count + flow_count + constraint_count);
}
