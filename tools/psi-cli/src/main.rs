use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about = "PSI CLI - prototype", long_about = None)]
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
}

#[derive(Serialize, Deserialize, Debug)]
struct OperatorDef {
    name: String,
    kern_template: String,
    emissions: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MetaProgram {
    name: String,
    operators: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Heuristic {
    name: String,
    weights: Option<HashMap<String, u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PsiBrain {
    name: String,
    operators: Vec<OperatorDef>,
    meta_programs: Vec<MetaProgram>,
    heuristics: Option<Vec<Heuristic>>,
}

fn main() {
    let args = Args::parse();

    let mut loaded_brain: Option<PsiBrain> = None;
    if let Some(brain_file) = args.load.clone() {
        match fs::read_to_string(&brain_file) {
            Ok(s) => match serde_json::from_str::<PsiBrain>(&s) {
                Ok(b) => {
                    println!("Loaded PSI brain: {} ({} operators, {} meta-programs)", b.name, b.operators.len(), b.meta_programs.len());
                    loaded_brain = Some(b);
                }
                Err(e) => eprintln!("Failed to parse brain file: {}", e),
            },
            Err(e) => eprintln!("Failed to read brain file: {}", e),
        }
    }

    if args.interactive {
        repl(loaded_brain.as_ref());
        return;
    }

    if let Some(batch_file) = args.batch {
        if let Ok(contents) = fs::read_to_string(&batch_file) {
            for line in contents.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                println!("Processing task: {}", line.trim());
                process_command(line.trim(), loaded_brain.as_ref());
            }
        } else {
            eprintln!("Failed to read batch file: {}", batch_file);
        }
    }
}

fn repl(brain: Option<&PsiBrain>) {
    println!("PSI CLI (prototype). Type 'exit' to quit.");
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
        process_command(cmd, brain);
    }
}

fn process_command(cmd: &str, brain: Option<&PsiBrain>) {
    // Very small parser for demonstration: handle 'generate' and 'debug' and 'translate'
    let lower = cmd.to_lowercase();
    if lower.starts_with("generate") {
        // Determine language
        let lang = if lower.contains("rust") { "rust" } else if lower.contains("python") { "python" } else { "kern" };
        let target = cmd.trim_start_matches("generate").trim();
        println!("Generate request -> lang: {}, target: {}", lang, target);
        // If brain and meta-programs exist, try to resolve a GenerateModule meta-program
        let kern_src = if let Some(b) = brain {
            if let Some(mp) = b.meta_programs.iter().find(|m| m.name.to_lowercase().contains("generate")) {
                build_kern_from_metaprogram(mp, b)
            } else {
                generate_kern_for_target(target)
            }
        } else {
            generate_kern_for_target(target)
        };
        let out_path = "demo/psi_generated.kern";
        if let Err(e) = fs::write(out_path, &kern_src) {
            eprintln!("Failed to write generated KERN: {}", e);
            return;
        }
        println!("Wrote KERN to {}", out_path);
        // Compile via kernc and run
        if compile_kern(out_path).is_ok() {
            run_bytecode("output.kbc");
        }
        // Emit stub source for requested language
        emit_language_stub(lang, target);
    } else if lower.starts_with("translate") {
        println!("Translate: not implemented in prototype (will emit stub)");
    } else if lower.starts_with("debug") {
        println!("Debugging operators: analysis not implemented in prototype.");
    } else {
        println!("Unrecognized command: {}", cmd);
    }
}

fn build_kern_from_metaprogram(mp: &MetaProgram, brain: &PsiBrain) -> String {
    let mut parts: Vec<String> = Vec::new();
    for op_name in &mp.operators {
        if let Some(op) = brain.operators.iter().find(|o| o.name == *op_name) {
            parts.push(op.kern_template.clone());
        } else {
            parts.push(format!("// Missing operator: {}", op_name));
        }
    }
    parts.join("\n") + "\n"
}

fn generate_kern_for_target(_target: &str) -> String {
    // Produce a simple rule that logs "Hello, World!" to demonstrate end-to-end
    let src = r##"rule HelloFromPSI: if 1 == 1 then log("Hello, World!")
"##;
    src.to_string()
}

fn compile_kern(input: &str) -> Result<(), ()> {
    println!("Compiling {} to bytecode...", input);
    let status = Command::new("cargo")
        .args(["run", "--package", "kern_compiler_cli", "--bin", "kernc", "--", "--input", input, "build"]) 
        .status();
    match status {
        Ok(s) if s.success() => {
            println!("Compile succeeded");
            Ok(())
        }
        Ok(_) | Err(_) => {
            eprintln!("Compile failed");
            Err(())
        }
    }
}

fn run_bytecode(bytecode: &str) {
    println!("Running bytecode: {}", bytecode);
    let status = Command::new("cargo")
        .args(["run", "--package", "kern_compiler_cli", "--bin", "kernc", "--", "--input", bytecode, "run"]) 
        .status();
    match status {
        Ok(s) if s.success() => println!("Execution finished"),
        _ => eprintln!("Execution failed"),
    }
}

fn emit_language_stub(lang: &str, target: &str) {
    let filename = match lang {
        "rust" => "demo/psi_output.rs",
        "python" => "demo/psi_output.py",
        _ => "demo/psi_output.txt",
    };
    let content = match lang {
        "rust" => format!("// Rust stub generated for: {}\nfn main() {{ println!(\"Hello from PSI Rust stub\"); }}\n", target),
        "python" => format!("# Python stub generated for: {}\nprint(\"Hello from PSI Python stub\")\n", target),
        _ => format!("Generated for: {}\n", target),
    };
    if let Err(e) = fs::write(filename, content) {
        eprintln!("Failed to write language stub {}: {}", filename, e);
    } else {
        println!("Wrote language stub to {}", filename);
    }
}
