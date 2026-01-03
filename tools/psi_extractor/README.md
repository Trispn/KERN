# PSI Operator Extractor

Extracts operator definitions from open source LLMs and normalizes them into PSI brain format.

## Setup

1. Install Python dependencies:
```bash
pip install requests pyyaml
```

2. Start an open source LLM server:

**Option A: Ollama** (recommended for ease)
```bash
ollama pull mistral
ollama serve
```

**Option B: LM Studio**
- Download from https://lmstudio.ai
- Load a model and start the local API server (default: http://localhost:1234/v1)

**Option C: vLLM**
```bash
python -m vllm.entrypoints.openai_api_server --model mistralai/Mistral-7B-Instruct-v0.1
```

## Usage

Extract operators from a running LLM:
```bash
python extractor.py --endpoint http://localhost:11434/api/generate --model mistral --output ../../psi/brain.json
```

Extract with LM Studio (port 1234):
```bash
python extractor.py --endpoint http://localhost:1234/v1/chat/completions --model any-model --output ../../psi/brain.json --api-type openai
```

Verify extracted operators by compiling & running generated KERN:
```bash
python verify_operators.py --brain ../../psi/brain.json
```

## Output

Produces a `psi/brain.json` file with normalized operators, meta-programs, and heuristics ready for `tools/psi-cli`.

## Configuration

Edit `extraction_prompts.yaml` to customize extraction prompts and operator domains.
