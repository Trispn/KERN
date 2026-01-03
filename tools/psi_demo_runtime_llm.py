#!/usr/bin/env python3
"""
PSI Runtime LLM Integration Demo

This script demonstrates PSI's ability to:
1. Load a brain from JSON
2. Fetch new operators from an LLM
3. List all available operators
4. Generate KERN code using the expanded brain

Requirements:
- Ollama, LM Studio, or vLLM running on configured port
- Python 3.10+
- requests library
"""

import json
import subprocess
import time
from pathlib import Path

# Configuration
BRAIN_FILE = "psi/brain_extracted.json"
LLM_ENDPOINT = "http://localhost:11434"  # Ollama default
PSCLI_PATH = "tools/psi-cli/target/debug/psi_cli.exe"

def test_scenario_1_list_operators():
    """Scenario 1: Load brain and list operators"""
    print("\n" + "="*60)
    print("SCENARIO 1: Load Brain and List Operators")
    print("="*60)
    
    # Create test commands
    test_commands = "help\nlist operators\n"
    test_file = "demo/psi_scenario1.txt"
    
    with open(test_file, "w") as f:
        f.write(test_commands)
    
    print(f"\nTest commands:\n{test_commands}")
    
    # Run psi-cli
    cmd = [
        PSCLI_PATH,
        "--load", BRAIN_FILE,
        "--batch", test_file
    ]
    
    print(f"\nRunning: {' '.join(cmd)}\n")
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    print("Output:")
    print(result.stdout)
    if result.stderr:
        print("Errors:", result.stderr)
    
    return result.returncode == 0

def test_scenario_2_generate_code():
    """Scenario 2: Generate code using brain operators"""
    print("\n" + "="*60)
    print("SCENARIO 2: Generate Code from Brain")
    print("="*60)
    
    test_commands = "generate login module in Rust\n"
    test_file = "demo/psi_scenario2.txt"
    
    with open(test_file, "w") as f:
        f.write(test_commands)
    
    print(f"\nTest command: {test_commands}")
    
    cmd = [
        PSCLI_PATH,
        "--load", BRAIN_FILE,
        "--batch", test_file
    ]
    
    print(f"\nRunning: {' '.join(cmd)}\n")
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    print("Output:")
    print(result.stdout[-2000:] if len(result.stdout) > 2000 else result.stdout)
    
    return "Execution finished successfully" in result.stdout

def test_scenario_3_with_llm_endpoint():
    """Scenario 3: Test with LLM endpoint configured (dry run)"""
    print("\n" + "="*60)
    print("SCENARIO 3: PSI with LLM Endpoint Configured")
    print("="*60)
    
    test_commands = "list operators\n"
    test_file = "demo/psi_scenario3.txt"
    
    with open(test_file, "w") as f:
        f.write(test_commands)
    
    print(f"\nTest setup:")
    print(f"  Brain: {BRAIN_FILE}")
    print(f"  LLM Endpoint: {LLM_ENDPOINT}")
    print(f"  Commands: {test_commands.strip()}")
    
    # Try with LLM endpoint (won't actually fetch without --fetch-operators)
    cmd = [
        PSCLI_PATH,
        "--load", BRAIN_FILE,
        "--llm-endpoint", LLM_ENDPOINT,
        "--batch", test_file
    ]
    
    print(f"\nRunning: {' '.join(cmd)}\n")
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
    
    print("Output:")
    print(result.stdout)
    
    if "Available operators" in result.stdout:
        print("\n✓ PSI successfully loaded brain and listed operators")
        return True
    
    return False

def test_scenario_4_llm_setup_instructions():
    """Scenario 4: Show LLM setup instructions"""
    print("\n" + "="*60)
    print("SCENARIO 4: Setting Up LLM for PSI")
    print("="*60)
    
    instructions = """
To enable PSI to fetch operators from an LLM:

Option 1: Ollama (Recommended)
  1. Install from https://ollama.ai
  2. Pull a model: ollama pull mistral
  3. Start server: ollama serve
  4. Run PSI:
     cargo run --manifest-path tools/psi-cli/Cargo.toml -- \\
       --load psi/brain_extracted.json \\
       --llm-endpoint http://localhost:11434 \\
       --fetch-operators --interactive

Option 2: LM Studio
  1. Download from https://lmstudio.ai
  2. Start server on port 1234
  3. Load a local model
  4. Run PSI:
     cargo run --manifest-path tools/psi-cli/Cargo.toml -- \\
       --load psi/brain_extracted.json \\
       --llm-endpoint http://localhost:1234 \\
       --fetch-operators --interactive

Option 3: vLLM
  1. Install: pip install vllm
  2. Start: python -m vllm.entrypoints.openai.api_server --model meta-llama/Llama-2-7b-hf --port 8000
  3. Run PSI:
     cargo run --manifest-path tools/psi-cli/Cargo.toml -- \\
       --load psi/brain_extracted.json \\
       --llm-endpoint http://localhost:8000 \\
       --fetch-operators --interactive

Interactive REPL Commands:
  help                - Show available commands
  list operators      - List all loaded operators
  fetch operators     - Fetch new operators from LLM
  generate <task>     - Generate KERN code for task
  exit                - Exit PSI
"""
    print(instructions)
    return True

def test_scenario_5_batch_workflow():
    """Scenario 5: Complete batch workflow"""
    print("\n" + "="*60)
    print("SCENARIO 5: Complete Batch Workflow")
    print("="*60)
    
    test_commands = """help
list operators
generate user management system
"""
    test_file = "demo/psi_scenario5.txt"
    
    with open(test_file, "w") as f:
        f.write(test_commands)
    
    print(f"\nBatch commands:")
    for i, cmd in enumerate(test_commands.strip().split('\n'), 1):
        print(f"  {i}. {cmd}")
    
    cmd = [
        PSCLI_PATH,
        "--load", BRAIN_FILE,
        "--batch", test_file
    ]
    
    print(f"\nRunning batch workflow...\n")
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    # Check for successful completion
    output_lines = result.stdout.split('\n')
    print("Workflow Output:")
    for line in output_lines[-20:]:
        if line.strip():
            print(f"  {line}")
    
    return result.returncode == 0

def main():
    """Run all test scenarios"""
    print("\n" + "="*70)
    print(" PSI RUNTIME LLM INTEGRATION DEMO")
    print("="*70)
    
    # Verify files exist
    if not Path(PSCLI_PATH).exists():
        print(f"\n❌ Error: {PSCLI_PATH} not found. Run: cargo build --manifest-path tools/psi-cli/Cargo.toml")
        return False
    
    if not Path(BRAIN_FILE).exists():
        print(f"\n❌ Error: {BRAIN_FILE} not found")
        return False
    
    results = []
    
    # Run test scenarios
    try:
        results.append(("Scenario 1: List Operators", test_scenario_1_list_operators()))
        results.append(("Scenario 2: Generate Code", test_scenario_2_generate_code()))
        results.append(("Scenario 3: With LLM Endpoint", test_scenario_3_with_llm_endpoint()))
        results.append(("Scenario 4: LLM Setup Instructions", test_scenario_4_llm_setup_instructions()))
        results.append(("Scenario 5: Batch Workflow", test_scenario_5_batch_workflow()))
    except subprocess.TimeoutExpired:
        print("\n⏱️ Command timed out")
    except Exception as e:
        print(f"\n❌ Error running tests: {e}")
        return False
    
    # Summary
    print("\n" + "="*70)
    print(" TEST SUMMARY")
    print("="*70)
    
    for name, success in results:
        status = "✓ PASS" if success else "✗ FAIL"
        print(f"{status}: {name}")
    
    passed = sum(1 for _, success in results if success)
    total = len(results)
    
    print(f"\nTotal: {passed}/{total} scenarios passed")
    
    # Next steps
    print("\n" + "="*70)
    print(" NEXT STEPS FOR PRODUCTION")
    print("="*70)
    print("""
1. Set up an LLM:
   ollama pull mistral
   ollama serve

2. Start PSI with LLM operator fetching:
   cargo run --manifest-path tools/psi-cli/Cargo.toml -- \\
     --load psi/brain_extracted.json \\
     --llm-endpoint http://localhost:11434 \\
     --fetch-operators \\
     --interactive

3. Use PSI to fetch and deploy operators:
   PSI> fetch operators
   PSI> list operators
   PSI> generate <your task here>

4. Persist brain updates (future feature):
   - Automatically save fetched operators
   - Version control brain snapshots
   - Cache validated operators
""")
    
    return passed == total

if __name__ == "__main__":
    import sys
    success = main()
    sys.exit(0 if success else 1)
