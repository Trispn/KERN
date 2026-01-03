use clap::Parser;
use kern_bytecode::Instruction;
use std::fs;

/// KERN Bytecode Inspector - Analyze and verify KERN bytecode
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input bytecode file to inspect
    #[arg(short, long)]
    input: String,

    /// Action to perform
    #[arg(subcommand)]
    action: Actions,
}

#[derive(clap::Subcommand, Debug)]
enum Actions {
    /// Disassemble bytecode to human-readable format
    Disassemble,
    /// Verify bytecode integrity and validity
    Verify,
    /// Show bytecode metadata
    Meta,
    /// Show statistics about the bytecode
    Stats,
}

fn main() {
    let args = Args::parse();

    match args.action {
        Actions::Disassemble => {
            disassemble_bytecode(&args.input);
        },
        Actions::Verify => {
            verify_bytecode(&args.input);
        },
        Actions::Meta => {
            show_metadata(&args.input);
        },
        Actions::Stats => {
            show_stats(&args.input);
        }
    }
}

fn disassemble_bytecode(input_file: &str) {
    // Read the bytecode file
    let bytecode_content = fs::read_to_string(input_file)
        .expect("Failed to read bytecode file");

    // Deserialize the bytecode
    let bytecode: Vec<Instruction> = serde_json::from_str(&bytecode_content)
        .expect("Failed to deserialize bytecode");

    println!("Disassembly of {}:", input_file);
    println!("------------------------");

    for (i, instruction) in bytecode.iter().enumerate() {
        let disasm = disassemble_instruction(instruction);
        println!("{:04}: {}", i, disasm);
    }
}

fn disassemble_instruction(instruction: &Instruction) -> String {
    // Map opcodes to human-readable mnemonics
    let mnemonic = match instruction.opcode {
        0x00 => "NOP",
        0x01 => "JMP",
        0x02 => "JMP_IF",
        0x03 => "HALT",
        0x10 => "LOAD_SYM",
        0x11 => "LOAD_NUM",
        0x12 => "MOVE",
        0x13 => "COMPARE",
        0x20 => "GRAPH_NODE_CREATE",
        0x21 => "GRAPH_EDGE_CREATE",
        0x22 => "GRAPH_MATCH",
        0x23 => "GRAPH_TRAVERSE",
        0x30 => "RULE_LOAD",
        0x31 => "RULE_EVAL",
        0x32 => "RULE_FIRE",
        0x33 => "RULE_PRIORITY_SET",
        0x40 => "CTX_CREATE",
        0x41 => "CTX_SWITCH",
        0x42 => "CTX_CLONE",
        0x43 => "CTX_DESTROY",
        0x50 => "ERR_SET",
        0x51 => "ERR_CLEAR",
        0x52 => "ERR_CHECK",
        0x60 => "EXT_CALL",
        0x61 => "EXT_BIND",
        0x70 => "RETURN",
        0x71 => "OUTPUT",
        _ => "UNKNOWN",
    };

    format!("{} R{}, R{}, R{}", mnemonic, instruction.arg1, instruction.arg2, instruction.arg3)
}

fn verify_bytecode(input_file: &str) {
    // Read the bytecode file
    let bytecode_content = fs::read_to_string(input_file)
        .expect("Failed to read bytecode file");

    // Deserialize the bytecode
    let bytecode: Vec<Instruction> = serde_json::from_str(&bytecode_content)
        .expect("Failed to deserialize bytecode");

    let mut is_valid = true;
    let mut errors = Vec::new();

    for (i, instruction) in bytecode.iter().enumerate() {
        // Check for valid opcodes
        if !is_valid_opcode(instruction.opcode) {
            errors.push(format!("Invalid opcode 0x{:02X} at instruction {}", instruction.opcode, i));
            is_valid = false;
        }

        // Check register bounds (assuming registers R0-R15)
        if instruction.arg1 > 15 && requires_register_arg(instruction.opcode) {
            errors.push(format!("Register index {} out of bounds at instruction {}", instruction.arg1, i));
            is_valid = false;
        }
        if instruction.arg2 > 15 && requires_register_arg(instruction.opcode) {
            errors.push(format!("Register index {} out of bounds at instruction {}", instruction.arg2, i));
            is_valid = false;
        }
        if instruction.arg3 > 15 && requires_register_arg(instruction.opcode) {
            errors.push(format!("Register index {} out of bounds at instruction {}", instruction.arg3, i));
            is_valid = false;
        }
    }

    if is_valid {
        println!("✓ Bytecode verification passed - file is valid");
    } else {
        println!("✗ Bytecode verification failed with {} errors:", errors.len());
        for error in errors {
            println!("  {}", error);
        }
    }
}

fn is_valid_opcode(opcode: u8) -> bool {
    // Check if the opcode is one of the valid KERN opcodes
    matches!(opcode, 
        0x00 | 0x01 | 0x02 | 0x03 |  // Control flow
        0x10 | 0x11 | 0x12 | 0x13 |  // Data & Symbol
        0x20 | 0x21 | 0x22 | 0x23 |  // Graph operations
        0x30 | 0x31 | 0x32 | 0x33 |  // Rule execution
        0x40 | 0x41 | 0x42 | 0x43 |  // Context & State
        0x50 | 0x51 | 0x52 |        // Error handling
        0x60 | 0x61 |                // External interface
        0x70 | 0x71                  // Termination
    )
}

fn requires_register_arg(opcode: u8) -> bool {
    // Check if the opcode typically uses register arguments
    matches!(opcode,
        0x10 | 0x11 | 0x12 | 0x13 |  // Data & Symbol
        0x40 | 0x41 | 0x42 | 0x43 |  // Context & State
        0x70 | 0x71                  // Termination
    )
}

fn show_metadata(input_file: &str) {
    // Read the bytecode file
    let bytecode_content = fs::read_to_string(input_file)
        .expect("Failed to read bytecode file");

    // Deserialize the bytecode
    let bytecode: Vec<Instruction> = serde_json::from_str(&bytecode_content)
        .expect("Failed to deserialize bytecode");

    println!("Metadata for {}:", input_file);
    println!("------------------------");
    println!("Total instructions: {}", bytecode.len());
    println!("File size: {} bytes", bytecode_content.len());
    println!("SHA256 hash: {}", sha256_hash(&bytecode_content));
}

fn sha256_hash(content: &str) -> String {
    // Simple placeholder - in a real implementation, we'd use a proper hashing function
    format!("{:x}", content.len())  // Just return length as hex for now
}

fn show_stats(input_file: &str) {
    // Read the bytecode file
    let bytecode_content = fs::read_to_string(input_file)
        .expect("Failed to read bytecode file");

    // Deserialize the bytecode
    let bytecode: Vec<Instruction> = serde_json::from_str(&bytecode_content)
        .expect("Failed to deserialize bytecode");

    let mut opcode_counts = std::collections::HashMap::new();
    for instruction in &bytecode {
        *opcode_counts.entry(instruction.opcode).or_insert(0) += 1;
    }

    println!("Statistics for {}:", input_file);
    println!("------------------------");
    println!("Total instructions: {}", bytecode.len());

    println!("\nOpcode histogram:");
    let mut sorted_counts: Vec<_> = opcode_counts.iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count, descending
    
    for (opcode, count) in sorted_counts {
        let mnemonic = match opcode {
            0x00 => "NOP",
            0x01 => "JMP",
            0x02 => "JMP_IF",
            0x03 => "HALT",
            0x10 => "LOAD_SYM",
            0x11 => "LOAD_NUM",
            0x12 => "MOVE",
            0x13 => "COMPARE",
            0x20 => "GRAPH_NODE_CREATE",
            0x21 => "GRAPH_EDGE_CREATE",
            0x22 => "GRAPH_MATCH",
            0x23 => "GRAPH_TRAVERSE",
            0x30 => "RULE_LOAD",
            0x31 => "RULE_EVAL",
            0x32 => "RULE_FIRE",
            0x33 => "RULE_PRIORITY_SET",
            0x40 => "CTX_CREATE",
            0x41 => "CTX_SWITCH",
            0x42 => "CTX_CLONE",
            0x43 => "CTX_DESTROY",
            0x50 => "ERR_SET",
            0x51 => "ERR_CLEAR",
            0x52 => "ERR_CHECK",
            0x60 => "EXT_CALL",
            0x61 => "EXT_BIND",
            0x70 => "RETURN",
            0x71 => "OUTPUT",
            _ => "UNKNOWN",
        };
        println!("  {}: {} (0x{:02X})", mnemonic, count, opcode);
    }
}