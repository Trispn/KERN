# How to Use Open Source LLMs to Extract PSI Knowledge

This guide explains how to use open source LLMs (running locally or remotely) to extract operator definitions and populate the PSI brain.

## Overview

The extraction pipeline:
1. Start an open source LLM server locally (Ollama, LM Studio, or vLLM)
2. Run `extractor.py` to query the LLM with structured prompts
3. Parse and normalize LLM responses into operator definitions
4. (Optional) Verify operators by compiling KERN and running bytecode
5. Output a brain JSON file that `tools/psi-cli` can load

## Supported Open Source LLMs

### 1. Ollama (Recommended)

**Install:**
```bash
# macOS / Linux
curl -fsSL https://ollama.ai/install.sh | sh

# Windows
# Download from https://ollama.ai/download
```

**Run a model:**
```bash
ollama pull mistral
ollama serve
```

The API will be available at `http://localhost:11434/api/generate`.

**Supported models:** Mistral, Llama 2, Neural Chat, and others.

### 2. LM Studio

**Download:** https://lmstudio.ai

**Steps:**
1. Download and install LM Studio
2. Load a model (e.g., Mistral 7B Instruct)
3. Start the local API server (usually http://localhost:1234/v1)

### 3. vLLM (High Performance)

**Install:**
```bash
pip install vllm
```

**Run a model:**
```bash
python -m vllm.entrypoints.openai_api_server --model mistralai/Mistral-7B-Instruct-v0.1
```

The API will be available at `http://localhost:8000/v1`.

## Installation

```bash
cd tools/psi_extractor
pip install -r requirements.txt
```

## Usage Examples

### Example 1: Extract with Ollama

```bash
python extractor.py \
  --endpoint http://localhost:11434/api/generate \
  --model mistral \
  --domains code,text \
  --output ../../psi/brain.json
```

### Example 2: Extract with LM Studio

```bash
python extractor.py \
  --endpoint http://localhost:1234/v1/chat/completions \
  --model any-model \
  --api-type openai \
  --domains code,text,design \
  --output ../../psi/brain.json
```

### Example 3: Extract with vLLM

```bash
python extractor.py \
  --endpoint http://localhost:8000/v1/chat/completions \
  --model mistralai/Mistral-7B-Instruct-v0.1 \
  --api-type vllm \
  --domains code \
  --output ../../psi/brain.json
```

## Output Format

The extractor produces a JSON file with the structure:

```json
{
  "name": "extracted-psi",
  "operators": [
    {
      "name": "OperatorName",
      "description": "What this operator does",
      "domain": "code",
      "kern_template": "rule OperatorName: if 1 == 1 then log(...)",
      "emissions": {
        "rust": "// Rust emission template",
        "python": "# Python emission template"
      }
    }
  ],
  "meta_programs": [
    {
      "name": "MetaProgramName",
      "operators": ["Op1", "Op2"],
      "description": "Description"
    }
  ],
  "heuristics": [
    {
      "name": "default",
      "weights": {"Op1": 10, "Op2": 8}
    }
  ]
}
```

This file is compatible with `tools/psi-cli --load`.

## Verification

After extraction, verify operators compile and run correctly:

```bash
python verify_operators.py --brain ../../psi/brain.json
```

Or verify a specific meta-program:

```bash
python verify_operators.py \
  --brain ../../psi/brain.json \
  --meta-program GenerateModule
```

## Tips for Better Extractions

1. **Use specialized models**: Mistral 7B or Neural Chat are optimized for instruction-following.
2. **Iterate prompts**: Edit prompts in `extractor.py` to refine operator extraction.
3. **Temperature tuning**: Lower temperature (0.0-0.3) reduces hallucinations.
4. **Multi-pass extraction**: Run multiple times and aggregate high-confidence results.
5. **Human review**: Always inspect extracted operators for correctness before using.

## Example Workflow

```bash
# 1. Start Ollama with Mistral
ollama pull mistral
ollama serve

# 2. In another terminal, extract operators
cd tools/psi_extractor
python extractor.py \
  --endpoint http://localhost:11434/api/generate \
  --model mistral \
  --domains code,text \
  --output ../../psi/brain.json

# 3. Verify (optional but recommended)
python verify_operators.py --brain ../../psi/brain.json

# 4. Use in psi-cli
cd ../..
cargo run --manifest-path tools\psi-cli\Cargo.toml -- --load psi/brain.json --interactive
```

## Customization

Edit `extractor.py`:
- `_make_operator_prompt()`: customize extraction prompt for operators
- `_make_metaprogram_prompt()`: customize meta-program extraction
- `_default_heuristics()`: adjust default operator weights

## Troubleshooting

- **Connection refused**: Check that your LLM server is running and accessible at the endpoint.
- **Timeout errors**: Increase `--timeout` parameter or check LLM performance.
- **Invalid JSON in response**: Some models may wrap JSON in markdown; the extractor tries to extract JSON from the response.

## Notes

- Extraction is deterministic when using low temperature (0.1) and fixed seed.
- Extracted operators should be verified before production use.
- Brain files are pure JSON; you can edit them manually.
