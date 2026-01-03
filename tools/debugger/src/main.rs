use clap::Parser;
use kern_vm::{VirtualMachine, VmRegisters};
use kern_bytecode::Instruction;
use std::fs;
use std::io::{self, Write};

/// KERN Debugger - Interactive debugger for KERN programs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input bytecode file to debug
    #[arg(short, long)]
    input: String,

    /// Start in replay mode from a trace file
    #[arg(short, long)]
    replay: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(trace_file) = args.replay {
        replay_execution(&trace_file);
    } else {
        start_debug_session(&args.input);
    }
}

fn start_debug_session(bytecode_file: &str) {
    println!("Starting KERN debugger for: {}", bytecode_file);

    // Read the bytecode file
    let bytecode_content = fs::read_to_string(bytecode_file)
        .expect("Failed to read bytecode file");

    // Deserialize the bytecode
    let bytecode: Vec<Instruction> = serde_json::from_str(&bytecode_content)
        .expect("Failed to deserialize bytecode");

    // Create a VM instance
    let mut vm = VirtualMachine::new();
    vm.load_program(bytecode);

    println!("KERN Debugger started. Type 'help' for commands.");
    
    // Interactive debugging loop
    loop {
        print!("(kerndbg) ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "run" | "r" => {
                println!("Starting execution...");
                vm.execute().unwrap();
                println!("Execution completed.");
                break;
            },
            "step" | "s" => {
                match vm.step() {
                    Ok(_) => {
                        let registers = vm.get_registers();
                        println!("[STEP] PC={} | OP={:?}", registers.pc, get_current_instruction(&vm));
                        print_registers(&registers);
                    },
                    Err(e) => {
                        println!("Error during step: {}", e);
                        break;
                    }
                }
            },
            "next" | "n" => {
                println!("Executing next instruction...");
                match vm.step() {
                    Ok(_) => {
                        let registers = vm.get_registers();
                        println!("[NEXT] PC={} | OP={:?}", registers.pc, get_current_instruction(&vm));
                        print_registers(&registers);
                    },
                    Err(e) => {
                        println!("Error during next: {}", e);
                        break;
                    }
                }
            },
            "regs" => {
                let registers = vm.get_registers();
                print_registers(&registers);
            },
            "ctx" => {
                println!("Current context: {}", vm.get_context());
            },
            "trace" => {
                let trace = vm.trace_state();
                println!("{}", trace);
            },
            "break" | "b" => {
                println!("Setting breakpoints not yet implemented");
            },
            "mem" => {
                println!("Memory inspection not yet implemented");
            },
            "help" | "h" => {
                print_help();
            },
            "quit" | "q" | "exit" => {
                println!("Quitting debugger...");
                break;
            },
            "" => continue, // Empty input, just continue
            _ => {
                println!("Unknown command: '{}'. Type 'help' for available commands.", input);
            }
        }
    }
}

fn get_current_instruction(vm: &VirtualMachine) -> String {
    // This is a simplified implementation - in a real debugger, we'd need to access the current instruction
    format!("Instruction at PC")
}

fn print_registers(registers: &VmRegisters) {
    println!("Registers:");
    for i in 0..16 {
        println!("  R{}: {}", i, registers.r[i]);
    }
    println!("  PC: {}", registers.pc);
    println!("  CTX: {}", registers.ctx);
    println!("  FLAG: {}", registers.flag);
    println!("  ERR: {}", registers.err);
}

fn print_help() {
    println!("KERN Debugger Commands:");
    println!("  run (r)     - Start execution");
    println!("  step (s)    - Execute next instruction");
    println!("  next (n)    - Execute next instruction");
    println!("  regs        - Show registers");
    println!("  ctx         - Show current context");
    println!("  trace       - Show execution trace");
    println!("  break (b)   - Set breakpoint");
    println!("  mem         - Inspect memory");
    println!("  help (h)    - Show this help");
    println!("  quit (q)    - Quit debugger");
}

fn replay_execution(trace_file: &str) {
    println!("Replaying execution from trace: {}", trace_file);
    
    // Read the trace file
    let trace_content = fs::read_to_string(trace_file)
        .expect("Failed to read trace file");
    
    // In a real implementation, we would replay the execution trace
    println!("Trace content: {}", trace_content);
    println!("Replay functionality not yet fully implemented");
}