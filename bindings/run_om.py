import sys
import openmat_core

if len(sys.argv) < 2:
    print("Usage: python run_om.py <script.om>")
    sys.exit(1)

script_path = sys.argv[1]

# Initialize Engine
engine = openmat_core.PyEngine()

# Read & Evaluate Script
try:
    with open(script_path, 'r') as f:
        code = f.read()
    result = engine.eval(code)
    if result is not None:
        print(f"Final Output: {result}")
except Exception as e:
    print(f"Error executing {script_path}: {e}")
