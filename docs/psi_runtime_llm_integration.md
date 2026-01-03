# PSI Runtime LLM Integration Guide

## Overview

PSI can now dynamically fetch and integrate operators from running LLMs at runtime. This allows PSI's brain to grow and learn new reasoning capabilities without rebuilding the binary.

## Features Added

### 1. CLI Flags for LLM Operator Fetching

**New Command-Line Arguments:**
- `--llm-endpoint <URL>` - Connect to an LLM server (Ollama, LM Studio, or vLLM)
- `--fetch-operators` - Automatically fetch operators from the LLM on startup

**Examples:**

```bash
# Fetch operators from Ollama on startup
cargo run --manifest-path tools/psi-cli/Cargo.toml -- \
  --load psi/brain.json \
  --llm-endpoint http://localhost:11434 \
  --fetch-operators

# Interactive mode with LLM endpoint available
cargo run --manifest-path tools/psi-cli/Cargo.toml -- \
  --load psi/brain.json \
  --llm-endpoint http://localhost:1234 \
  --interactive
```

### 2. Enhanced REPL Commands

In interactive mode, the following commands are now available:

**`help`**
- Shows all available commands
- Includes LLM-specific commands if endpoint is configured

**`list operators`**
- Shows all operators currently loaded in the brain
- Displays operator type (reasoning, action, transform, validation)
- Format: `- OperatorName (type: operator_type)`

**`fetch operators`** (when `--llm-endpoint` is configured)
- Queries the connected LLM for new operator definitions
- Automatically adds fetched operators to the current brain
- Updates total operator count

### 3. LLM Support

PSI automatically detects the LLM type by port:

| LLM | Port | URL Format |
|-----|------|-----------|
| Ollama | 11434 | `http://localhost:11434/api/generate` |
| LM Studio | 1234 | `http://localhost:1234/v1/chat/completions` |
| vLLM | 8000 | `http://localhost:8000/v1/chat/completions` |

Each LLM has optimized API calls:
- **Ollama**: Uses native generate endpoint, optimized for Mistral/Llama
- **LM Studio/vLLM**: Uses OpenAI-compatible chat API

### 4. Operator Format from LLM

When PSI queries an LLM for operators, it expects JSON response with this structure:

```json
[
  {
    "name": "OperatorName",
    "operator_type": "reasoning|action|transform|validation",
    "inputs": ["input1", "input2"],
    "outputs": ["output1", "output2"],
    "kern_template": "rule OperatorName: if condition then action"
  }
]
```

The LLM can return operators wrapped in markdown code blocks (```json...```), and PSI will extract them automatically.

## Usage Examples

### Example 1: Static Brain + LLM Fallback

```bash
cd C:\Users\LENOVO\OneDrive\Desktop\KERN

# Load existing brain and show operators
cargo run --manifest-path tools/psi-cli/Cargo.toml -- \
  --load psi/brain_extracted.json \
  --batch demo/psi_llm_test.txt
```

Output:
```
Loaded PSI brain: extracted-psi-demo (5 operators, 2 meta-programs)
Processing task: help
Commands:
  generate <task>        - Generate KERN code for a task
  translate <code>       - Translate code to KERN
  debug <problem>        - Debug a problem
  list operators         - Show available operators
  help                   - Show this help
  exit                   - Exit PSI

Processing task: list operators
Available operators in brain 'extracted-psi-demo':
  - ParseCode (type: reasoning)
  - GenerateCode (type: action)
  - TranslateText (type: transform)
  - DefineEntities (type: reasoning)
  - CreateRoutes (type: action)
```

### Example 2: Start Ollama and Fetch Operators

```bash
# In a separate terminal, start Ollama
ollama serve

# In another terminal, fetch operators
cd C:\Users\LENOVO\OneDrive\Desktop\KERN

cargo run --manifest-path tools/psi-cli/Cargo.toml -- \
  --load psi/brain_extracted.json \
  --llm-endpoint http://localhost:11434 \
  --fetch-operators --interactive
```

Output:
```
Loaded PSI brain: extracted-psi-demo (5 operators, 2 meta-programs)
Fetching operators from LLM: http://localhost:11434
Successfully fetched 3 new operators:
  - AnalyzeText
  - ClassifyDocument
  - SummarizeContent
Brain updated. Total operators: 8
PSI CLI (prototype). Type 'exit' to quit, 'help' for commands.
PSI> 
```

### Example 3: Interactive REPL with LLM

```bash
cargo run --manifest-path tools/psi-cli/Cargo.toml -- \
  --load psi/brain_extracted.json \
  --llm-endpoint http://localhost:1234 \
  --interactive
```

REPL session:
```
PSI CLI (prototype). Type 'exit' to quit, 'help' for commands.
PSI> help
Commands:
  generate <task>        - Generate KERN code for a task
  translate <code>       - Translate code to KERN
  debug <problem>        - Debug a problem
  fetch operators        - Fetch new operators from connected LLM
  list operators         - Show available operators
  help                   - Show this help
  exit                   - Exit PSI

PSI> list operators
Available operators in brain 'extracted-psi-demo':
  - ParseCode (type: reasoning)
  - GenerateCode (type: action)
  - TranslateText (type: transform)
  - DefineEntities (type: reasoning)
  - CreateRoutes (type: action)

PSI> fetch operators
Fetching operators from LLM...
Successfully fetched 3 new operators:
  - AnalyzeText
  - ClassifyDocument
  - SummarizeContent
Brain updated. Total operators: 8

PSI> list operators
Available operators in brain 'extracted-psi-demo':
  - ParseCode (type: reasoning)
  - GenerateCode (type: action)
  - TranslateText (type: transform)
  - DefineEntities (type: reasoning)
  - CreateRoutes (type: action)
  - AnalyzeText (type: reasoning)
  - ClassifyDocument (type: transform)
  - SummarizeContent (type: reasoning)

PSI> exit
```

## Implementation Details

### LLM Query Prompt

PSI uses this prompt template when fetching operators:

```
You are an expert at defining deterministic reasoning operators for the KERN language.
Generate 3 new reasoning operators in JSON format. Each operator should have:
- name: operator identifier (e.g., "ParseCode", "GenerateCode")
- operator_type: one of "reasoning", "action", "transform", "validation"
- inputs: list of input parameter names
- outputs: list of output names
- kern_template: a simple KERN rule template for this operator

Return ONLY a JSON array, no markdown or explanations.
```

### Operator Type Classifications

| Type | Purpose | Example |
|------|---------|---------|
| `reasoning` | Analyzes and infers | ParseCode, AnalyzeText, AnalyzeSentiment |
| `action` | Performs operations | GenerateCode, CreateRoutes, SendEmail |
| `transform` | Converts between formats | TranslateText, ConvertFormat, SerializeData |
| `validation` | Checks conditions | ValidateSchema, VerifySignature, CheckConstraints |

### Brain Data Structure

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
struct PsiBrain {
    name: String,
    operators: Vec<OperatorDef>,
    meta_programs: Vec<MetaProgram>,
    heuristics: Option<Vec<Heuristic>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OperatorDef {
    name: String,
    kern_template: String,
    operator_type: String,      // NEW
    inputs: Vec<String>,         // NEW
    outputs: Vec<String>,        // NEW
    emissions: Option<HashMap<String, String>>,
}
```

## Setting Up LLM Endpoints

### Ollama (Recommended)

```bash
# Install: https://ollama.ai
# Pull a model:
ollama pull mistral

# Start server (default port 11434):
ollama serve
```

### LM Studio

1. Download from https://lmstudio.ai
2. Start server on port 1234
3. Load a local model

### vLLM

```bash
pip install vllm

python -m vllm.entrypoints.openai.api_server \
  --model meta-llama/Llama-2-7b-hf \
  --port 8000
```

## Error Handling

If LLM connection fails:
- `fetch-operators` will show: "Failed to fetch operators from LLM: {error message}"
- Existing brain operators remain unchanged
- PSI continues to operate with current operators
- Retry with `--llm-endpoint` or use `fetch operators` REPL command

## Future Enhancements

1. **Persistent Brain Updates**
   - Automatically save fetched operators to brain JSON file
   - Create versioning system for brain snapshots

2. **Operator Validation**
   - Compile fetched KERN templates to verify syntax
   - Run simple test cases on VirtualMachine
   - Cache validated operators

3. **Multi-Modal Operators**
   - Support image, video, and audio processing
   - Extend emissions to support multiple media types

4. **Operator Caching**
   - Store successfully fetched operators locally
   - Avoid re-fetching known operators
   - Implement LRU cache for performance

5. **Brain Persistence**
   - `.PSI` binary format for optimized brain storage
   - Compress operator definitions
   - Fast load times for production use

## Testing

### Run basic REPL commands:
```bash
cargo run --manifest-path tools/psi-cli/Cargo.toml -- \
  --load psi/brain_extracted.json \
  --batch demo/psi_llm_test.txt
```

### Test with real Ollama:
```bash
# Terminal 1: Start Ollama
ollama serve

# Terminal 2: Test PSI with operator fetching
cd C:\Users\LENOVO\OneDrive\Desktop\KERN
cargo run --manifest-path tools/psi-cli/Cargo.toml -- \
  --load psi/brain_extracted.json \
  --llm-endpoint http://localhost:11434 \
  --fetch-operators
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Connection refused (port 11434) | Start Ollama: `ollama serve` |
| Connection refused (port 1234) | Start LM Studio server |
| JSON parse error from LLM | LLM returned malformed JSON; check prompt |
| No operators returned | LLM might be busy; retry or try different endpoint |
| Brain not updated | Check brain file permissions and JSON format |

## Architecture Diagram

```
┌─────────────────────────────────────────┐
│     PSI CLI (psi-cli/src/main.rs)       │
├─────────────────────────────────────────┤
│                                         │
│  Args Parsing                           │
│  ├── --llm-endpoint <URL>              │
│  ├── --fetch-operators                 │
│  └── --load <brain.json>               │
│                                         │
│  main()                                 │
│  ├── Load brain from JSON               │
│  ├── if --fetch-operators → run         │
│  │   fetch_operators_from_llm()         │
│  └── Start repl() or batch processing   │
│                                         │
│  repl()                                 │
│  ├── Handle "fetch operators" command   │
│  ├── Handle "list operators" command    │
│  └── Handle "help" command              │
│                                         │
│  fetch_operators_from_llm(&endpoint)    │
│  ├── Detect LLM type (port-based)       │
│  ├── Build appropriate HTTP request     │
│  ├── Query LLM with prompt              │
│  ├── Parse JSON response                │
│  └── Return Vec<OperatorDef>            │
│                                         │
└─────────────────────────────────────────┘
         │                    │
         ▼                    ▼
    ┌────────────┐   ┌──────────────────┐
    │   Brain    │   │  LLM Endpoint    │
    │brain.json  │   │  (Ollama/LM      │
    │            │   │   Studio/vLLM)   │
    └────────────┘   └──────────────────┘
         │                    │
         ├────────┬───────────┤
         ▼        ▼           ▼
      Parse   Extend    Generate KERN
```

## Summary

PSI now fully integrates with open-source LLMs at runtime:
- ✅ Load operators on startup with `--fetch-operators`
- ✅ Connect to multiple LLM providers (Ollama, LM Studio, vLLM)
- ✅ Fetch operators interactively in REPL
- ✅ List all available operators with types
- ✅ Extend brain dynamically without recompilation
- ✅ Automatic LLM type detection
- ✅ Robust error handling and fallback

**Next Steps:**
1. Set up Ollama or LM Studio
2. Run psi-cli with `--llm-endpoint` and `--fetch-operators`
3. Use `fetch operators` command to grow brain dynamically
4. Persist brain updates (future enhancement)
