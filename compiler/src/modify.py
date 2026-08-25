import sys
content = open('engine.rs').read()

# 1. Update NdArray Struct
content = content.replace(
"""pub struct NdArray {
    pub data: Arc<Float64Array>,
    pub shape: Vec<usize>,
    pub grad: Arc<Mutex<Option<Float64Array>>>,
}""",
"""pub struct NdArray {
    pub data: Arc<Float64Array>,
    pub shape: Vec<usize>,
    pub grad: Arc<Mutex<Option<Float64Array>>>,
    pub columns: Option<Vec<String>>,
}""")

# 2. Update Engine Struct
content = content.replace(
"""pub struct Engine {
    pub variables: HashMap<String, NdArray>,
    pub functions: HashMap<String, FunctionDef>,
    #[cfg(not(target_arch = "wasm32"))]
    pub start_time: Option<std::time::Instant>,
}""",
"""pub struct Engine {
    pub variables: HashMap<String, NdArray>,
    pub functions: HashMap<String, FunctionDef>,
    pub vfs: HashMap<String, String>,
    #[cfg(not(target_arch = "wasm32"))]
    pub start_time: Option<std::time::Instant>,
}""")

# 3. Update Engine::new
content = content.replace(
"""    pub fn new() -> Self {
        let mut engine = Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            #[cfg(not(target_arch = "wasm32"))]
            start_time: None,
        };""",
"""    pub fn new() -> Self {
        let mut engine = Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            vfs: HashMap::new(),
            #[cfg(not(target_arch = "wasm32"))]
            start_time: None,
        };""")

# 3b. Update local_engine clone
content = content.replace(
"""                    let mut local_engine = Engine {
                        variables: self.variables.clone(),
                        functions: self.functions.clone(),
                        #[cfg(not(target_arch = "wasm32"))]
                        start_time: self.start_time.clone(),
                    };""",
"""                    let mut local_engine = Engine {
                        variables: self.variables.clone(),
                        functions: self.functions.clone(),
                        vfs: self.vfs.clone(),
                        #[cfg(not(target_arch = "wasm32"))]
                        start_time: self.start_time.clone(),
                    };""")

# 4. Update SliceAccess
old_slice = """            Expr::SliceAccess { target, start, end } => {
                let arr = if let Some(val) = self.variables.get(target) {
                    val.clone()
                } else {
                    return Err(format!("Undefined variable: {}", target));
                };

                let start_idx = self.evaluate_expr(start)?.get_val(0) as usize;
                let end_idx = match end {
                    Some(e) => self.evaluate_expr(e)?.get_val(0) as usize,
                    None => start_idx + 1,
                };

                if start_idx >= arr.len() || end_idx > arr.len() || start_idx >= end_idx {
                    return Err("Index out of bounds".into());
                }

                let length = end_idx - start_idx;
                let sliced_data = arr.data.slice(start_idx, length);
                let sliced_float = sliced_data.as_any().downcast_ref::<Float64Array>().unwrap().clone();
                
                Ok(NdArray {
                    data: Arc::new(sliced_float),
                    shape: vec![length],
                    grad: Arc::new(Mutex::new(None)),
                            columns: None,
                })
            }"""

new_slice = """            Expr::SliceAccess { target, start, end } => {
                let arr = if let Some(val) = self.variables.get(target) {
                    val.clone()
                } else {
                    return Err(format!("Undefined variable: {}", target));
                };

                // Check for string indexing (DataFrame column access)
                if let Expr::StringLiteral(col_name) = start.as_ref() {
                    if let Some(cols) = &arr.columns {
                        if let Some(col_idx) = cols.iter().position(|c| c == col_name) {
                            if arr.shape.len() != 2 {
                                return Err("Column access requires a 2D array".into());
                            }
                            let rows = arr.shape[0];
                            let cols_count = arr.shape[1];
                            let mut col_data = Vec::with_capacity(rows);
                            for r in 0..rows {
                                col_data.push(arr.data.value(r * cols_count + col_idx));
                            }
                            return Ok(NdArray {
                                data: Arc::new(Float64Array::from(col_data)),
                                shape: vec![rows],
                                grad: Arc::new(Mutex::new(None)),
                                columns: Some(vec![col_name.clone()]),
                            });
                        } else {
                            return Err(format!("Column not found: {}", col_name));
                        }
                    } else {
                        return Err("Array does not have column headers".into());
                    }
                }

                let start_idx = self.evaluate_expr(start)?.get_val(0) as usize;
                let end_idx = match end {
                    Some(e) => self.evaluate_expr(e)?.get_val(0) as usize,
                    None => start_idx + 1,
                };

                if start_idx >= arr.len() || end_idx > arr.len() || start_idx >= end_idx {
                    return Err("Index out of bounds".into());
                }

                let length = end_idx - start_idx;
                let sliced_data = arr.data.slice(start_idx, length);
                let sliced_float = sliced_data.as_any().downcast_ref::<Float64Array>().unwrap().clone();
                
                Ok(NdArray {
                    data: Arc::new(sliced_float),
                    shape: vec![length],
                    grad: Arc::new(Mutex::new(None)),
                    columns: None,
                })
            }"""

content = content.replace(old_slice, new_slice)

# 5. Update read_csv and add csvwrite
old_read_csv = """                if name == "read_csv" {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Expr::StringLiteral(path) = arg.as_ref() {
                        use std::fs;
                        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
                        let mut values = Vec::new();
                        for (i, line) in content.lines().enumerate() {
                            if i == 0 { continue; } // skip header
                            let trimmed = line.trim();
                            if trimmed.is_empty() { continue; }
                            // split commas if multiline
                            for part in trimmed.split(',') {
                                if let Ok(val) = part.trim().parse::<f64>() {
                                    values.push(val);
                                }
                            }
                        }
                        let len = values.len();
                        return Ok(NdArray {
                            data: Arc::new(Float64Array::from(values)),
                            shape: vec![len],
                            grad: Arc::new(Mutex::new(None)),
                            columns: None,
                        });
                    } else {
                        return Err("read_csv expects a string literal path".into());
                    }
                    #[cfg(target_arch = "wasm32")]
                    return Err("read_csv is not supported in the WASM browser environment yet".into());
                }"""

new_read_csv = """                if name == "read_csv" {
                    if let Expr::StringLiteral(path) = arg.as_ref() {
                        let mut file_content = None;
                        if let Some(c) = self.vfs.get(path) {
                            file_content = Some(c.clone());
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            if file_content.is_none() {
                                if let Ok(c) = std::fs::read_to_string(path) {
                                    file_content = Some(c);
                                }
                            }
                        }
                        
                        if let Some(content) = file_content {
                            let mut values = Vec::new();
                            let mut columns = Vec::new();
                            let mut num_cols = 0;
                            let mut num_rows = 0;
                            
                            for (i, line) in content.lines().enumerate() {
                                let trimmed = line.trim();
                                if trimmed.is_empty() { continue; }
                                let parts: Vec<&str> = trimmed.split(',').collect();
                                
                                if i == 0 {
                                    for part in &parts {
                                        columns.push(part.trim().to_string());
                                    }
                                    num_cols = columns.len();
                                    continue;
                                }
                                
                                for part in &parts {
                                    if let Ok(val) = part.trim().parse::<f64>() {
                                        values.push(val);
                                    } else {
                                        values.push(0.0);
                                    }
                                }
                                num_rows += 1;
                            }
                            return Ok(NdArray {
                                data: Arc::new(Float64Array::from(values)),
                                shape: vec![num_rows, num_cols],
                                grad: Arc::new(Mutex::new(None)),
                                columns: Some(columns),
                            });
                        } else {
                            return Err(format!("File not found: {}", path));
                        }
                    } else {
                        return Err("read_csv expects a string literal path".into());
                    }
                }
                
                if name == "csvwrite" {
                    let evaluated = self.evaluate_expr(arg)?;
                    let mut out = String::new();
                    if let Some(cols) = &evaluated.columns {
                        out.push_str(&cols.join(","));
                        out.push('\\n');
                    }
                    if evaluated.shape.len() == 2 {
                        let rows = evaluated.shape[0];
                        let cols = evaluated.shape[1];
                        for r in 0..rows {
                            for c in 0..cols {
                                out.push_str(&evaluated.data.value(r * cols + c).to_string());
                                if c < cols - 1 { out.push(','); }
                            }
                            out.push('\\n');
                        }
                    } else {
                        for i in 0..evaluated.len() {
                            out.push_str(&evaluated.data.value(i).to_string());
                            out.push('\\n');
                        }
                    }
                    self.vfs.insert("out.csv".to_string(), out.clone());
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let _ = std::fs::write("out.csv", out);
                    }
                    return Ok(NdArray::scalar(1.0));
                }"""

content = content.replace(old_read_csv, new_read_csv)
open('engine.rs', 'w').write(content)
