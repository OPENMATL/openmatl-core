import pyarrow
import openmat_core

print("Initializing OpenMat Engine in Python via PyO3...")
engine = openmat_core.PyEngine()

print("Evaluating: 'C = [2, 2, 2] * 4'")
result_array = engine.eval("C = [2, 2, 2] * 4")

print(f"Returned object type: {type(result_array)}")
print(f"Data:\n{result_array}")

print("\nEvaluating: 'plot(C)'")
engine.eval("plot(C)")
