pub mod ast;
pub mod parser;
pub mod engine;
#[cfg(not(target_arch = "wasm32"))]
pub mod cli;

#[cfg(not(target_arch = "wasm32"))]
pub mod python_bindings {
    use pyo3::prelude::*;
    use pyo3::{Py, PyAny};
    use pyo3::types::PyList;
    use crate::engine::Engine;
    use crate::parser::parse_program;

    #[pyclass]
    pub struct PyEngine {
        engine: Engine,
    }

    #[pymethods]
    impl PyEngine {
        #[new]
        pub fn new() -> Self {
            PyEngine {
                engine: Engine::new(),
            }
        }

        pub fn eval(&mut self, py: Python, code: &str) -> PyResult<Option<Py<PyAny>>> {
            match parse_program(code) {
                Ok(program) => {
                    let mut last_result = None;
                    for stmt in program.statements {
                        match self.engine.execute_statement(&stmt) {
                            Ok(res) => {
                                let result = match res {
                                    crate::engine::ExecutionResult::Value(v) => v,
                                    crate::engine::ExecutionResult::Return(v) => v,
                                    crate::engine::ExecutionResult::None => continue,
                                };
                                // For MVP, we manually convert NdArray into nested python lists
                                // so we don't have to deal with complex PyArrow 2D nesting
                                let list = PyList::empty(py);
                                if result.shape.len() == 2 {
                                    let rows = result.shape[0];
                                    let cols = result.shape[1];
                                    for i in 0..rows {
                                        let row_list = PyList::empty(py);
                                        for j in 0..cols {
                                            row_list.append(result.get_val(i * cols + j)).unwrap();
                                        }
                                        list.append(row_list).unwrap();
                                    }
                                } else {
                                    for i in 0..result.len() {
                                        list.append(result.get_val(i)).unwrap();
                                    }
                                }
                                last_result = Some(list.into_any().unbind());
                            }
                            Err(e) => {
                                return Err(pyo3::exceptions::PyRuntimeError::new_err(format!("Execution Error: {}", e)));
                            }
                        }
                    }
                    Ok(last_result)
                }
                Err(e) => {
                    Err(pyo3::exceptions::PySyntaxError::new_err(format!("Parse error: {}", e)))
                }
            }
        }
    }

    #[pymodule]
    fn openmatl_core(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_class::<PyEngine>()?;
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
pub mod wasm_bindings {
    use wasm_bindgen::prelude::*;
    use crate::engine::Engine;
    use crate::parser::parse_program;

    #[wasm_bindgen]
    pub struct WasmEngine {
        engine: Engine,
    }

    #[wasm_bindgen]
    impl WasmEngine {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            WasmEngine {
                engine: Engine::new(),
            }
        }

        pub fn write_file(&mut self, name: &str, content: &str) {
            self.engine.vfs.insert(name.to_string(), content.to_string());
        }

        pub fn eval(&mut self, code: &str) -> Result<JsValue, JsValue> {
            match parse_program(code) {
                Ok(program) => {
                    let mut last_result = None;
                    for stmt in program.statements {
                        match self.engine.execute_statement(&stmt) {
                            Ok(res) => {
                                let result = match res {
                                    crate::engine::ExecutionResult::Value(v) => v,
                                    crate::engine::ExecutionResult::Return(v) => v,
                                    crate::engine::ExecutionResult::None => continue,
                                };
                                // Serialize NdArray shape & data into a JSON structure
                                // For simplicity, we just convert it to an array of arrays if it's 2D
                                let mut arr_json = String::new();
                                if result.shape.len() == 2 {
                                    arr_json.push('[');
                                    let rows = result.shape[0];
                                    let cols = result.shape[1];
                                    for i in 0..rows {
                                        arr_json.push('[');
                                        for j in 0..cols {
                                            arr_json.push_str(&result.get_val(i * cols + j).to_string());
                                            if j < cols - 1 { arr_json.push(','); }
                                        }
                                        arr_json.push(']');
                                        if i < rows - 1 { arr_json.push(','); }
                                    }
                                    arr_json.push(']');
                                } else {
                                    arr_json.push('[');
                                    for i in 0..result.len() {
                                        arr_json.push_str(&result.get_val(i).to_string());
                                        if i < result.len() - 1 { arr_json.push(','); }
                                    }
                                    arr_json.push(']');
                                }
                                last_result = Some(arr_json);
                            },
                            Err(e) => return Err(JsValue::from_str(&format!("Execution Error: {}", e))),
                        }
                    }
                    if let Some(res) = last_result {
                        Ok(JsValue::from_str(&res))
                    } else {
                        Ok(JsValue::NULL)
                    }
                }
                Err(e) => Err(JsValue::from_str(&format!("Parse error: {}", e))),
            }
        }
    }
}
