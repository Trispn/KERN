# Quick Start: Extract Knowledge from LLMs

This guide walks you through extracting operator definitions from open source LLMs running locally.

## Option 1: Ollama (Easiest) ⭐

### Install Ollama
- **macOS/Linux**: `curl -fsSL https://ollama.ai/install.sh | sh`
- **Windows**: Download from https://ollama.ai/download

### Run the extraction

**Terminal 1 (start Ollama):**
```bash
ollama pull mistral
ollama serve
```

**Terminal 2 (run extraction):**
```bash
cd tools/psi_extractor
python setup_and_extract.py
```

This will:
1. Check if Ollama is running
2. Pull a model if needed
3. Query the LLM to extract operator definitions
4. Save operators to `psi/brain_llm_extracted.json`
5. Show next steps

### Manual extraction (if you prefer)
```bash
python extractor.py \
  --endpoint http://localhost:11434/api/generate \
  --model mistral \
  --domains code,text \
  --output ../../psi/brain_llm_extracted.json
```

## Option 2: LM Studio (GUI Alternative)

1. Download from https://lmstudio.ai
2. Load a model and start the local API server
3. Run extraction:
```bash
python extractor.py \
  --endpoint http://localhost:1234/v1/chat/completions \
  --model any-model \
  --api-type openai \
  --output ../../psi/brain_llm_extracted.json
```

## Option 3: vLLM (High Performance)

```bash
# Install
pip install vllm

# Run server
python -m vllm.entrypoints.openai_api_server \
  --model mistralai/Mistral-7B-Instruct-v0.1

# Extract in another terminal
python extractor.py \
  --endpoint http://localhost:8000/v1/chat/completions \
  --model mistralai/Mistral-7B-Instruct-v0.1 \
  --api-type vllm \
  --output ../../psi/brain_llm_extracted.json
```

## Use the extracted brain in PSI

Once extraction completes, use the brain in psi-cli:

```bash
# Interactive REPL
cargo run --manifest-path tools/psi-cli/Cargo.toml -- \
  --load psi/brain_llm_extracted.json \
  --interactive

# Batch tasks
cargo run --manifest-path tools/psi-cli/Cargo.toml -- \
  --load psi/brain_llm_extracted.json \
  --batch demo/psi_tasks.txt

# With streaming output
cargo run --manifest-path tools/psi-cli/Cargo.toml -- \
  --load psi/brain_llm_extracted.json \
  --interactive \
  --stream
```

## Verify extracted operators

Compile and run operators to verify they work:

```bash
python verify_operators.py --brain psi/brain_llm_extracted.json
```

## Tips

- **Temperature**: Lower = more deterministic. Default is 0.1.
- **Models to try**:
  - `mistral` (7B, fast, good quality)
  - `llama2` (7B-70B sizes available)
  - `neural-chat` (specialized for code)
- **Extract multiple times**: Run extraction multiple times and manually select the best operators.
- **Edit manually**: Brain is JSON; you can edit it manually to fix extraction issues.

## Troubleshooting

| Issue | Fix |
|-------|-----|
| "Connection refused" | Make sure LLM server is running and accessible |
| "Model not found" | Pull the model: `ollama pull mistral` |
| "Timeout" | Server is slow; increase timeout in extractor or give it more time |
| "Invalid JSON" | Some models wrap output in markdown; extractor tries to extract JSON |

## Example Output

After running `setup_and_extract.py`, you'll see:
```
============================================================
  Running Operator Extraction
============================================================

Extracted operators from Ollama (model: mistral)...

Brain summary:
  Name: extracted-psi
  Operators: 8
  Meta-programs: 2

  Extracted operators:
    - ParseCode: Parse code into AST
    - GenerateCode: Generate code from spec
    - TestCode: Generate unit tests
    ...

Next Steps:
1. Use the extracted brain in PSI CLI:
   cargo run --manifest-path tools/psi-cli/Cargo.toml -- --load psi/brain_llm_extracted.json --interactive
```

That's it! You now have a brain populated with operators extracted from an LLM. 🎉
