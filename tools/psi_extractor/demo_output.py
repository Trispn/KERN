#!/usr/bin/env python3
"""
Standalone test that doesn't require the extractor module.
"""

import json

# Mock brain output
mock_brain = {
    "name": "extracted-psi-demo",
    "operators": [
        {
            "name": "ParseCode",
            "description": "Parse code into AST",
            "domain": "code",
            "kern_template": 'rule ParseCode: if 1 == 1 then log("code parsed")',
            "emissions": {
                "rust": "// parse code with tree-sitter",
                "python": "# parse code with ast module"
            }
        },
        {
            "name": "GenerateCode",
            "description": "Generate code from spec",
            "domain": "code",
            "kern_template": 'rule GenerateCode: if 1 == 1 then log("code generated")',
            "emissions": {
                "rust": "// generate code using templates",
                "python": "# generate code using templates"
            }
        },
        {
            "name": "TranslateText",
            "description": "Translate text between languages",
            "domain": "text",
            "kern_template": 'rule TranslateText: if 1 == 1 then log("text translated")',
            "emissions": {
                "rust": "// translate text",
                "python": "# translate text"
            }
        }
    ],
    "meta_programs": [
        {
            "name": "CodeTransform",
            "operators": ["ParseCode", "GenerateCode"],
            "description": "Transform and generate code"
        },
        {
            "name": "MultilingualGenerate",
            "operators": ["ParseCode", "TranslateText", "GenerateCode"],
            "description": "Generate code in multiple languages"
        }
    ],
    "heuristics": [
        {
            "name": "default",
            "weights": {
                "ParseCode": 10,
                "GenerateCode": 9,
                "TranslateText": 8
            }
        }
    ]
}

print("Mock extracted brain:")
print(json.dumps(mock_brain, indent=2))
print(f"\nExtracted:")
print(f"  {len(mock_brain['operators'])} operators")
print(f"  {len(mock_brain['meta_programs'])} meta-programs")
print(f"  {len(mock_brain['heuristics'])} heuristic sets")
