#!/usr/bin/env python3
"""
Quick test of the extractor with a mock LLM response.
"""

import json
from extractor import LLMExtractor

class MockExtractor(LLMExtractor):
    """Mock extractor that returns pre-defined operators without calling an LLM."""
    
    def query(self, prompt):
        """Return a mock response based on prompt content."""
        if "operators" in prompt.lower():
            return json.dumps([
                {
                    "name": "ParseCode",
                    "description": "Parse code into AST",
                    "kern_template": 'rule ParseCode: if 1 == 1 then log("code parsed")',
                    "emissions": {
                        "rust": "// parse code",
                        "python": "# parse code"
                    }
                },
                {
                    "name": "GenerateCode",
                    "description": "Generate code from spec",
                    "kern_template": 'rule GenerateCode: if 1 == 1 then log("code generated")',
                    "emissions": {
                        "rust": "// generate code",
                        "python": "# generate code"
                    }
                }
            ])
        elif "meta" in prompt.lower():
            return json.dumps([
                {
                    "name": "CodeTransform",
                    "operators": ["ParseCode", "GenerateCode"],
                    "description": "Transform code"
                }
            ])
        return "[]"

if __name__ == "__main__":
    extractor = MockExtractor("http://localhost:11434/api/generate", "mistral")
    brain = extractor.extract_operators(["code"])
    
    print(json.dumps(brain, indent=2))
