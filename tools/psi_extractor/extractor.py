#!/usr/bin/env python3
"""
PSI Operator Extractor

Queries open source LLMs (Ollama, vLLM, LM Studio) to extract operator definitions
and normalizes them into PSI brain format (JSON).
"""

import json
import argparse
import requests
import sys
from typing import Dict, List, Any, Optional

class LLMExtractor:
    def __init__(self, endpoint: str, model: str, api_type: str = "ollama"):
        """
        Initialize extractor.
        
        Args:
            endpoint: LLM API endpoint (e.g., http://localhost:11434/api/generate)
            model: model name to query
            api_type: 'ollama', 'openai', or 'vllm'
        """
        self.endpoint = endpoint
        self.model = model
        self.api_type = api_type
    
    def query_ollama(self, prompt: str) -> Optional[str]:
        """Query Ollama API."""
        try:
            resp = requests.post(
                self.endpoint,
                json={"model": self.model, "prompt": prompt, "stream": False},
                timeout=120
            )
            resp.raise_for_status()
            return resp.json().get("response", "")
        except Exception as e:
            print(f"Ollama query failed: {e}", file=sys.stderr)
            return None
    
    def query_openai_api(self, prompt: str) -> Optional[str]:
        """Query OpenAI-compatible API (LM Studio, vLLM)."""
        try:
            resp = requests.post(
                self.endpoint,
                json={
                    "model": self.model,
                    "messages": [{"role": "user", "content": prompt}],
                    "temperature": 0.1
                },
                timeout=120
            )
            resp.raise_for_status()
            return resp.json()["choices"][0]["message"]["content"]
        except Exception as e:
            print(f"OpenAI API query failed: {e}", file=sys.stderr)
            return None
    
    def query(self, prompt: str) -> Optional[str]:
        """Query LLM with the given prompt."""
        if self.api_type == "ollama":
            return self.query_ollama(prompt)
        elif self.api_type in ["openai", "vllm", "lm-studio"]:
            return self.query_openai_api(prompt)
        else:
            print(f"Unknown API type: {self.api_type}", file=sys.stderr)
            return None
    
    def extract_operators(self, domains: List[str]) -> Dict[str, Any]:
        """
        Extract operator definitions for given domains.
        
        Args:
            domains: list of domains (e.g., ["code", "text"])
        
        Returns:
            dict with "operators", "meta_programs", "heuristics" keys
        """
        operators = []
        meta_programs = []
        
        for domain in domains:
            print(f"Extracting {domain} operators...", file=sys.stderr)
            prompt = self._make_operator_prompt(domain)
            response = self.query(prompt)
            if not response:
                print(f"Failed to extract {domain} operators", file=sys.stderr)
                continue
            
            # Parse JSON from response
            ops = self._parse_operator_response(response, domain)
            if ops:
                operators.extend(ops)
        
        # Extract meta-programs
        if operators:
            print("Extracting meta-programs...", file=sys.stderr)
            prompt = self._make_metaprogram_prompt(operators)
            response = self.query(prompt)
            if response:
                mps = self._parse_metaprogram_response(response)
                if mps:
                    meta_programs.extend(mps)
        
        return {
            "name": "extracted-psi",
            "operators": operators,
            "meta_programs": meta_programs if meta_programs else [
                {"name": "GenerateModule", "operators": [op["name"] for op in operators[:4]]}
            ],
            "heuristics": self._default_heuristics(operators)
        }
    
    def _make_operator_prompt(self, domain: str) -> str:
        """Generate prompt to extract operators for a domain."""
        return f"""You are an expert system designer. Extract 3 atomic operators for the '{domain}' domain.
For each operator, provide a JSON object with:
- "name": operator name (e.g., "ParseCode", "TranslateText")
- "description": brief description
- "kern_template": a short KERN rule that represents this operator's logic
- "emissions": dict with "rust" and "python" emission templates

Return a JSON array of operator objects. Example format:
[
  {{
    "name": "ParseCode",
    "description": "Parse code and extract AST",
    "kern_template": "rule ParseCode: if 1 == 1 then log(\\"code parsed\\")",
    "emissions": {{
      "rust": "// parse code using tree-sitter",
      "python": "# parse code using ast module"
    }}
  }}
]

Be deterministic and concise. Output ONLY valid JSON."""
    
    def _make_metaprogram_prompt(self, operators: List[Dict]) -> str:
        """Generate prompt to extract meta-programs."""
        op_names = [op["name"] for op in operators]
        return f"""You are an expert system designer. Given these operators: {op_names}

Propose 2 meta-programs (operator chains) that combine them for practical tasks.
Return a JSON array with objects:
- "name": meta-program name (e.g., "GenerateAPI")
- "operators": list of operator names in sequence
- "description": brief description

Example:
[
  {{
    "name": "GenerateAPI",
    "operators": ["ParseCode", "TranslateText", "GenerateCode"],
    "description": "Generate REST API from specification"
  }}
]

Output ONLY valid JSON."""
    
    def _parse_operator_response(self, response: str, domain: str) -> List[Dict]:
        """Parse operator JSON from LLM response."""
        try:
            # Try to extract JSON from response
            start = response.find("[")
            end = response.rfind("]") + 1
            if start >= 0 and end > start:
                json_str = response[start:end]
                ops = json.loads(json_str)
                
                # Normalize and validate
                normalized = []
                for op in ops:
                    if isinstance(op, dict) and "name" in op:
                        op["domain"] = domain
                        op.setdefault("kern_template", f'rule {op["name"]}: if 1 == 1 then log("{op["name"]} done")')
                        op.setdefault("emissions", {"rust": f"// {op['name']}", "python": f"# {op['name']}"})
                        normalized.append(op)
                return normalized
        except json.JSONDecodeError as e:
            print(f"Failed to parse JSON from response: {e}", file=sys.stderr)
        return []
    
    def _parse_metaprogram_response(self, response: str) -> List[Dict]:
        """Parse meta-program JSON from LLM response."""
        try:
            start = response.find("[")
            end = response.rfind("]") + 1
            if start >= 0 and end > start:
                json_str = response[start:end]
                return json.loads(json_str)
        except json.JSONDecodeError:
            pass
        return []
    
    def _default_heuristics(self, operators: List[Dict]) -> List[Dict]:
        """Generate default heuristics for operators."""
        weights = {}
        for i, op in enumerate(operators):
            weights[op["name"]] = max(1, 10 - i)  # descending priority
        
        return [{
            "name": "default",
            "weights": weights
        }]


def main():
    parser = argparse.ArgumentParser(description="Extract operators from open source LLMs")
    parser.add_argument("--endpoint", required=True, help="LLM API endpoint URL")
    parser.add_argument("--model", required=True, help="Model name")
    parser.add_argument("--api-type", default="ollama", choices=["ollama", "openai", "vllm", "lm-studio"],
                        help="API type")
    parser.add_argument("--output", default="psi/brain.json", help="Output brain JSON file")
    parser.add_argument("--domains", default="code,text", help="Domains to extract (comma-separated)")
    
    args = parser.parse_args()
    
    extractor = LLMExtractor(args.endpoint, args.model, args.api_type)
    domains = args.domains.split(",")
    
    print(f"Querying {args.api_type} at {args.endpoint} with model {args.model}...", file=sys.stderr)
    brain = extractor.extract_operators(domains)
    
    # Save to file
    try:
        with open(args.output, "w") as f:
            json.dump(brain, f, indent=2)
        print(f"Saved brain to {args.output}", file=sys.stderr)
        print(f"Extracted {len(brain['operators'])} operators, {len(brain['meta_programs'])} meta-programs")
    except Exception as e:
        print(f"Failed to save brain: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
