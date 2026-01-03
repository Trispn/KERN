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
    /// Enable streaming-like output in REPL (prints partial chunks)
    #[arg(long)]
    stream: bool,
    /// Conversation history file to append to (optional)
    #[arg(long)]
    history: Option<String>,
    /// Optional LLM backend URL for fallback (POST JSON {"prompt": ...})
    #[arg(long)]
    llm_backend: Option<String>,
    /// LLM endpoint for operator extraction (Ollama: http://localhost:11434/api/generate)
    #[arg(long)]
    llm_endpoint: Option<String>,
    /// Fetch operators from LLM on startup
    #[arg(long)]
    fetch_operators: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OperatorDef {
    name: String,
    kern_template: String,
    #[serde(default)]
    operator_type: String,
    #[serde(default)]
    inputs: Vec<String>,
    #[serde(default)]
    outputs: Vec<String>,
    emissions: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MetaProgram {
    name: String,
    operators: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Heuristic {
    name: String,
    weights: Option<HashMap<String, u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PsiBrain {
    name: String,
    operators: Vec<OperatorDef>,
    meta_programs: Vec<MetaProgram>,
    heuristics: Option<Vec<Heuristic>>,
}

fn main() {
    let args = Args::parse();

    let mut loaded_brain: Option<PsiBrain> = None;
    let stream_mode = args.stream;
    let history_path = args.history.clone();
    let llm_backend = args.llm_backend.clone();
    let llm_endpoint = args.llm_endpoint.clone();
    
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
    
    // Fetch operators from LLM if requested
    if args.fetch_operators {
        if let Some(endpoint) = &llm_endpoint {
            println!("Fetching operators from LLM: {}", endpoint);
            if let Ok(new_ops) = fetch_operators_from_llm(endpoint) {
                if let Some(brain) = &mut loaded_brain {
                    brain.operators.extend(new_ops);
                    println!("Added new operators to brain. Total: {}", brain.operators.len());
                } else {
                    loaded_brain = Some(PsiBrain {
                        name: "llm-fetched".to_string(),
                        operators: new_ops,
                        meta_programs: vec![],
                        heuristics: None,
                    });
                }
            } else {
                eprintln!("Failed to fetch operators from LLM");
            }
        } else {
            eprintln!("--fetch-operators requires --llm-endpoint");
        }
    }

    if args.interactive {
        repl(loaded_brain.as_ref(), stream_mode, history_path.as_deref(), llm_backend.as_deref(), llm_endpoint.as_deref());
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

fn repl(brain: Option<&PsiBrain>, stream: bool, history_path: Option<&str>, llm_backend: Option<&str>, llm_endpoint: Option<&str>) {
    println!("PSI CLI (prototype). Type 'exit' to quit, 'help' for commands.");
    let mut history: Vec<String> = Vec::new();
    let mut current_brain = brain.cloned();
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
        if cmd.eq_ignore_ascii_case("help") {
            println!("Commands:");
            println!("  generate <task>        - Generate KERN code for a task");
            println!("  translate <code>       - Translate code to KERN");
            println!("  debug <problem>        - Debug a problem");
            if llm_endpoint.is_some() {
                println!("  fetch operators        - Fetch new operators from connected LLM");
                println!("  list operators         - Show available operators");
            }
            println!("  help                   - Show this help");
            println!("  exit                   - Exit PSI");
            continue;
        }
        if cmd.eq_ignore_ascii_case("fetch operators") {
            if let Some(endpoint) = llm_endpoint {
                println!("Fetching operators from LLM...");
                match fetch_operators_from_llm(endpoint) {
                    Ok(new_ops) => {
                        println!("Successfully fetched {} new operators:", new_ops.len());
                        for op in &new_ops {
                            println!("  - {}", op.name);
                        }
                        if let Some(ref mut b) = current_brain {
                            b.operators.extend(new_ops);
                            println!("Brain updated. Total operators: {}", b.operators.len());
                        }
                    }
                    Err(e) => println!("Error fetching operators: {}", e),
                }
            } else {
                println!("No LLM endpoint configured. Use --llm-endpoint to enable.");
            }
            continue;
        }
        if cmd.eq_ignore_ascii_case("list operators") {
            if let Some(b) = &current_brain {
                println!("Available operators in brain '{}':", b.name);
                for op in &b.operators {
                    println!("  - {} (type: {})", op.name, op.operator_type);
                }
            } else {
                println!("No brain loaded.");
            }
            continue;
        }
        if cmd.is_empty() {
            continue;
        }
        history.push(cmd.to_string());
        if let Some(p) = history_path {
            let _ = append_history(p, cmd);
        }
        // If unrecognized and LLM backend provided, call fallback
        if let Some(backend) = llm_backend {
            // allow process_command to decide when to call LLM; here we pass backend
            process_command_with_llm(cmd, current_brain.as_ref(), stream, Some(backend));
        } else {
            process_command_streaming(cmd, current_brain.as_ref(), stream);
        }
    }
}

fn process_command(cmd: &str, brain: Option<&PsiBrain>) {
    process_command_streaming(cmd, brain, false);
}

fn process_command_streaming(cmd: &str, brain: Option<&PsiBrain>, stream: bool) {
    // Very small parser for demonstration: handle 'generate' and 'debug' and 'translate'
    let lower = cmd.to_lowercase();
    
    // Handle meta commands first
    if lower.eq_ignore_ascii_case("help") {
        println!("Commands:");
        println!("  generate <task>        - Generate KERN code for a task");
        println!("  translate <code>       - Translate code to KERN");
        println!("  debug <problem>        - Debug a problem");
        println!("  list operators         - Show available operators");
        println!("  help                   - Show this help");
        println!("  exit                   - Exit PSI");
        return;
    }
    
    if lower.eq_ignore_ascii_case("list operators") {
        if let Some(b) = brain {
            println!("Available operators in brain '{}':", b.name);
            for op in &b.operators {
                let op_type = if op.operator_type.is_empty() { "unknown" } else { &op.operator_type };
                println!("  - {} (type: {})", op.name, op_type);
            }
        } else {
            println!("No brain loaded.");
        }
        return;
    }
    
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
        // Streaming-like print of generated KERN if requested
        if stream {
            stream_print(&kern_src);
        } else {
            println!("Wrote KERN to {}", out_path);
        }
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

fn process_command_with_llm(cmd: &str, brain: Option<&PsiBrain>, stream: bool, backend: Option<&str>) {
    // If command is recognized by prototype, handle locally; else consult LLM
    let lower = cmd.to_lowercase();
    if lower.starts_with("generate") || lower.starts_with("debug") || lower.starts_with("translate") {
        process_command_streaming(cmd, brain, stream);
        return;
    }
    // Fallback to LLM
    if let Some(url) = backend {
        match llm_fallback(url, cmd) {
            Ok(resp) => {
                if stream {
                    stream_print(&resp);
                } else {
                    println!("LLM response:\n{}", resp);
                }
            }
            Err(e) => eprintln!("LLM fallback failed: {}", e),
        }
    } else {
        println!("Unrecognized command and no LLM backend configured: {}", cmd);
    }
}

fn stream_print(s: &str) {
    use std::{thread, time};
    let chunk_size = 60;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let end = std::cmp::min(i + chunk_size, bytes.len());
        let part = &bytes[i..end];
        if let Ok(strpart) = std::str::from_utf8(part) {
            print!("{}", strpart);
            io::stdout().flush().ok();
        }
        i = end;
        thread::sleep(time::Duration::from_millis(30));
    }
    println!("");
}

fn append_history(path: &str, entry: &str) -> std::io::Result<()> {
    use std::fs::OpenOptions;
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    use std::io::Write;
    writeln!(f, "{}", entry)?;
    Ok(())
}

fn llm_fallback(url: &str, prompt: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let mut payload = std::collections::HashMap::new();
    payload.insert("prompt", prompt);
    let res = client.post(url).json(&payload).send().map_err(|e| e.to_string())?;
    if !res.status().is_success() {
        return Err(format!("non-200: {}", res.status()));
    }
    let text = res.text().map_err(|e| e.to_string())?;
    // If JSON object with `response` field, try to parse; otherwise return raw
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
        if let Some(r) = v.get("response") {
            return Ok(r.to_string().trim_matches('"').to_string());
        }
    }
    Ok(text)
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

fn fetch_operators_from_llm(endpoint: &str) -> Result<Vec<OperatorDef>, String> {
    // Query the LLM endpoint for operator definitions
    // Supports Ollama (port 11434), LM Studio (port 1234), vLLM (port 8000)
    
    let prompt = r#"
You are an expert at defining deterministic reasoning operators for the KERN language.
Generate 3 new reasoning operators in JSON format. Each operator should have:
- name: operator identifier (e.g., "ParseCode", "GenerateCode")
- operator_type: one of "reasoning", "action", "transform", "validation"
- inputs: list of input parameter names
- outputs: list of output names
- kern_template: a simple KERN rule template for this operator

Return ONLY a JSON array, no markdown or explanations.
Example format:
[
  {
    "name": "AnalyzeText",
    "operator_type": "reasoning",
    "inputs": ["text"],
    "outputs": ["entities", "sentiment"],
    "kern_template": "rule AnalyzeText: if true then log(\"Analysis complete\")"
  }
]
"#;

    // Detect LLM type by port and format request accordingly
    let (url, body, parse_fn): (String, String, Box<dyn Fn(&str) -> Result<String, String>>) = 
        if endpoint.contains(":11434") {
            // Ollama
            (format!("{}/api/generate", endpoint),
             serde_json::json!({
                 "model": "mistral",
                 "prompt": prompt,
                 "stream": false
             }).to_string(),
             Box::new(|resp| {
                 if let Ok(val) = serde_json::from_str::<serde_json::Value>(resp) {
                     if let Some(text) = val.get("response").and_then(|v| v.as_str()) {
                         return Ok(text.to_string());
                     }
                 }
                 Err("Failed to parse Ollama response".to_string())
             }))
        } else if endpoint.contains(":1234") || endpoint.contains(":8000") {
            // LM Studio or vLLM (OpenAI-compatible)
            (format!("{}/v1/chat/completions", endpoint),
             serde_json::json!({
                 "model": "local-model",
                 "messages": [{
                     "role": "user",
                     "content": prompt
                 }],
                 "temperature": 0.7,
                 "max_tokens": 2000
             }).to_string(),
             Box::new(|resp| {
                 if let Ok(val) = serde_json::from_str::<serde_json::Value>(resp) {
                     if let Some(content) = val.get("choices")
                         .and_then(|c| c.get(0))
                         .and_then(|c| c.get("message"))
                         .and_then(|m| m.get("content"))
                         .and_then(|c| c.as_str()) {
                         return Ok(content.to_string());
                     }
                 }
                 Err("Failed to parse OpenAI-compatible response".to_string())
             }))
        } else {
            return Err(format!("Unknown LLM endpoint format: {}", endpoint));
        };

    // Make HTTP request
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let resp_text = resp.text().map_err(|e| format!("Failed to read response: {}", e))?;
    
    // Parse response based on LLM type
    let content = parse_fn(&resp_text)?;
    
    // Extract JSON from content (may be wrapped in markdown code blocks)
    let json_str = if content.contains("```json") {
        content
            .split("```json")
            .nth(1)
            .and_then(|s| s.split("```").next())
            .unwrap_or(&content)
    } else if content.contains("```") {
        content
            .split("```")
            .nth(1)
            .unwrap_or(&content)
    } else {
        &content
    };

    // Parse operators JSON
    serde_json::from_str::<Vec<OperatorDef>>(json_str.trim())
        .map_err(|e| format!("Failed to parse operators JSON: {}", e))
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
