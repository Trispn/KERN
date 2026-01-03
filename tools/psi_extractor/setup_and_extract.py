#!/usr/bin/env python3
"""
Quick setup and extraction script for PSI operators.
Guides user through Ollama setup and runs extraction.
"""

import subprocess
import sys
import os
import json
import time
from pathlib import Path

def check_ollama_running():
    """Check if Ollama is running."""
    try:
        import requests
        resp = requests.get("http://localhost:11434/api/tags", timeout=2)
        return resp.status_code == 200
    except Exception:
        return False

def print_section(title):
    print(f"\n{'='*60}")
    print(f"  {title}")
    print(f"{'='*60}\n")

def main():
    print_section("PSI Operator Extraction Setup")
    
    # Step 1: Check/start Ollama
    print("Step 1: Checking Ollama...")
    if check_ollama_running():
        print("✓ Ollama is running!")
    else:
        print("✗ Ollama is NOT running.")
        print("\nTo start Ollama, run in a separate terminal:")
        print("  ollama serve\n")
        print("(Install from https://ollama.ai if needed)\n")
        response = input("Have you started Ollama? (yes/no): ").strip().lower()
        if response != "yes":
            print("Please start Ollama and run this script again.")
            sys.exit(1)
        
        print("Waiting for Ollama to be ready...")
        for i in range(30):
            if check_ollama_running():
                print("✓ Ollama is now running!")
                break
            time.sleep(1)
        else:
            print("✗ Ollama did not start in time.")
            sys.exit(1)
    
    # Step 2: Ensure a model is pulled
    print("\nStep 2: Checking for models...")
    try:
        import requests
        resp = requests.get("http://localhost:11434/api/tags")
        data = resp.json()
        models = data.get("models", [])
        if models:
            print(f"✓ Found {len(models)} model(s):")
            for m in models:
                print(f"  - {m['name']}")
            model = models[0]['name'].split(':')[0]
        else:
            print("✗ No models found. Pulling Mistral...")
            subprocess.run(["ollama", "pull", "mistral"], check=False)
            model = "mistral"
    except Exception as e:
        print(f"Could not check models: {e}")
        model = "mistral"
        print(f"Will use model: {model}")
    
    # Step 3: Run extraction
    print_section("Running Operator Extraction")
    print(f"Extracting operators from Ollama (model: {model})...\n")
    
    extractor_path = Path(__file__).parent / "extractor.py"
    output_path = Path(__file__).parent.parent.parent / "psi" / "brain_llm_extracted.json"
    
    cmd = [
        sys.executable,
        str(extractor_path),
        "--endpoint", "http://localhost:11434/api/generate",
        "--model", model,
        "--domains", "code,text",
        "--output", str(output_path)
    ]
    
    print(f"Command: {' '.join(cmd)}\n")
    result = subprocess.run(cmd, capture_output=False)
    
    if result.returncode == 0:
        print_section("Extraction Complete!")
        print(f"✓ Brain saved to: {output_path}\n")
        
        # Load and display summary
        if output_path.exists():
            with open(output_path) as f:
                brain = json.load(f)
            
            print(f"Brain summary:")
            print(f"  Name: {brain.get('name')}")
            print(f"  Operators: {len(brain.get('operators', []))}")
            print(f"  Meta-programs: {len(brain.get('meta_programs', []))}")
            
            if brain.get('operators'):
                print(f"\n  Extracted operators:")
                for op in brain['operators']:
                    print(f"    - {op.get('name')}: {op.get('description', 'N/A')}")
            
            # Next steps
            print_section("Next Steps")
            print("1. Use the extracted brain in PSI CLI:")
            print(f"   cargo run --manifest-path tools/psi-cli/Cargo.toml -- --load psi/brain_llm_extracted.json --interactive\n")
            print("2. Or run a batch task:")
            print(f"   cargo run --manifest-path tools/psi-cli/Cargo.toml -- --load psi/brain_llm_extracted.json --batch demo/psi_tasks.txt\n")
            print("3. Or verify operators compile:")
            print(f"   python tools/psi_extractor/verify_operators.py --brain {output_path}\n")
    else:
        print("✗ Extraction failed. Check the error above.")
        sys.exit(1)

if __name__ == "__main__":
    main()
