#!/usr/bin/env python3
"""
PSI Operator Verifier

Converts extracted operators into KERN fragments, compiles and runs them
on the kern-vm to verify deterministic behavior.
"""

import json
import subprocess
import sys
import os
from typing import Dict, List, Any, Optional

class OperatorVerifier:
    def __init__(self, brain_path: str, compiler_path: Optional[str] = None):
        """
        Initialize verifier.
        
        Args:
            brain_path: path to psi/brain.json
            compiler_path: path to kernc binary (optional; will search workspace)
        """
        self.brain_path = brain_path
        self.compiler_path = compiler_path or self._find_kernc()
        
        with open(brain_path) as f:
            self.brain = json.load(f)
    
    def _find_kernc(self) -> str:
        """Find kernc binary in workspace."""
        # Try cargo run method
        return "cargo run --package kern_compiler_cli --bin kernc --"
    
    def verify_all(self) -> bool:
        """Verify all operators. Return True if all pass."""
        print(f"Verifying {len(self.brain['operators'])} operators...", file=sys.stderr)
        
        passed = 0
        failed = 0
        
        for op in self.brain['operators']:
            if self._verify_operator(op):
                passed += 1
                print(f"✓ {op['name']}", file=sys.stderr)
            else:
                failed += 1
                print(f"✗ {op['name']}", file=sys.stderr)
        
        print(f"\nVerification: {passed} passed, {failed} failed", file=sys.stderr)
        return failed == 0
    
    def _verify_operator(self, op: Dict[str, Any]) -> bool:
        """Verify a single operator by compiling and running its KERN template."""
        try:
            kern_src = op.get("kern_template", "")
            if not kern_src:
                print(f"Operator {op['name']} has no kern_template", file=sys.stderr)
                return False
            
            # Write temp KERN file
            temp_kern = f"/tmp/verify_{op['name']}.kern"
            with open(temp_kern, "w") as f:
                f.write(kern_src + "\n")
            
            # Compile
            print(f"  Compiling {op['name']}...", file=sys.stderr)
            result = subprocess.run(
                f"{self.compiler_path} --input {temp_kern} build",
                shell=True,
                capture_output=True,
                timeout=30
            )
            
            if result.returncode != 0:
                print(f"  Compile failed: {result.stderr.decode()}", file=sys.stderr)
                return False
            
            # Run bytecode
            print(f"  Running {op['name']}...", file=sys.stderr)
            result = subprocess.run(
                f"{self.compiler_path} --input output.kbc run",
                shell=True,
                capture_output=True,
                timeout=30
            )
            
            if result.returncode != 0:
                print(f"  Execution failed: {result.stderr.decode()}", file=sys.stderr)
                return False
            
            return True
        except subprocess.TimeoutExpired:
            print(f"  Verification timeout for {op['name']}", file=sys.stderr)
            return False
        except Exception as e:
            print(f"  Verification error: {e}", file=sys.stderr)
            return False
    
    def verify_metaprogram(self, mp_name: str) -> bool:
        """Verify a meta-program by expanding and executing it."""
        mp = next((m for m in self.brain['meta_programs'] if m['name'] == mp_name), None)
        if not mp:
            print(f"Meta-program {mp_name} not found", file=sys.stderr)
            return False
        
        try:
            # Concatenate operator templates
            kern_parts = []
            for op_name in mp['operators']:
                op = next((o for o in self.brain['operators'] if o['name'] == op_name), None)
                if op:
                    kern_parts.append(op.get('kern_template', ''))
            
            kern_src = "\n".join(kern_parts) + "\n"
            
            # Write temp file
            temp_kern = f"/tmp/verify_mp_{mp_name}.kern"
            with open(temp_kern, "w") as f:
                f.write(kern_src)
            
            # Compile and run
            print(f"Verifying meta-program {mp_name}...", file=sys.stderr)
            result = subprocess.run(
                f"{self.compiler_path} --input {temp_kern} build",
                shell=True,
                capture_output=True,
                timeout=30
            )
            
            if result.returncode != 0:
                print(f"Meta-program compile failed", file=sys.stderr)
                return False
            
            result = subprocess.run(
                f"{self.compiler_path} --input output.kbc run",
                shell=True,
                capture_output=True,
                timeout=30
            )
            
            if result.returncode != 0:
                print(f"Meta-program execution failed", file=sys.stderr)
                return False
            
            print(f"✓ Meta-program {mp_name} verified", file=sys.stderr)
            return True
        except Exception as e:
            print(f"Meta-program verification error: {e}", file=sys.stderr)
            return False


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Verify PSI operators")
    parser.add_argument("--brain", required=True, help="Path to psi/brain.json")
    parser.add_argument("--meta-program", help="Verify a specific meta-program")
    parser.add_argument("--compiler", help="Path to kernc")
    
    args = parser.parse_args()
    
    verifier = OperatorVerifier(args.brain, args.compiler)
    
    if args.meta_program:
        success = verifier.verify_metaprogram(args.meta_program)
    else:
        success = verifier.verify_all()
    
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
