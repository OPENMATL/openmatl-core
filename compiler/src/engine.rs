use std::collections::HashMap;
use arrow::array::{Float64Array, Array};
use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use crate::ast::{Expr, Operator, Statement};

#[derive(Clone)]
pub struct NdArray {
    pub data: Arc<Float64Array>,
    pub shape: Vec<usize>,
    pub grad: Arc<Mutex<Option<Float64Array>>>,
}

impl NdArray {
    pub fn scalar(val: f64) -> Self {
        Self {
            data: Arc::new(Float64Array::from(vec![val])),
            shape: vec![],
            grad: Arc::new(Mutex::new(None)),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_val(&self, idx: usize) -> f64 {
        if self.len() == 1 {
            self.data.value(0)
        } else {
            self.data.value(idx)
        }
    }
}

#[derive(Clone)]
pub struct FunctionDef {
    pub params: Vec<String>,
    pub body: Vec<Statement>,
}

pub enum ExecutionResult {
    None,
    Value(NdArray),
    Return(NdArray),
}

pub struct Engine {
    pub variables: HashMap<String, NdArray>,
    pub functions: HashMap<String, FunctionDef>,
}

impl Engine {
    pub fn new() -> Self {
        let mut engine = Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        };
        
        let std_lib = include_str!("../../lib/std.om");
        if let Ok(program) = crate::parser::parse_program(std_lib) {
            for stmt in program.statements {
                let _ = engine.execute_statement(&stmt);
            }
        }
        
        engine
    }

    pub fn evaluate_expr(&mut self, expr: &Expr) -> Result<NdArray, String> {
        match expr {
            Expr::Number(n) => Ok(NdArray::scalar(*n)),
            Expr::Boolean(b) => {
                let val = if *b { 1.0 } else { 0.0 };
                Ok(NdArray::scalar(val))
            }
            Expr::StringLiteral(_) => {
                // Return dummy for now, used directly in function calls
                Ok(NdArray::scalar(0.0))
            }
            Expr::Identifier(name) => {
                if let Some(val) = self.variables.get(name) {
                    Ok(val.clone())
                } else {
                    Err(format!("Undefined variable: {}", name))
                }
            }
            Expr::Array(elements) => {
                if elements.is_empty() {
                    return Ok(NdArray {
                        data: Arc::new(Float64Array::from(Vec::<f64>::new())),
                        shape: vec![0],
                        grad: Arc::new(Mutex::new(None)),
                    });
                }

                let mut all_scalars = true;
                let mut rows = 0;
                let mut cols = 0;
                
                let mut flat_values = Vec::new();

                for el in elements {
                    let evaluated = self.evaluate_expr(el)?;
                    if evaluated.shape.is_empty() || evaluated.shape == vec![1] {
                        flat_values.push(evaluated.get_val(0));
                        rows += 1;
                    } else if evaluated.shape.len() == 1 {
                        all_scalars = false;
                        if cols == 0 {
                            cols = evaluated.len();
                        } else if cols != evaluated.len() {
                            return Err("Matrix rows have inconsistent lengths".into());
                        }
                        for i in 0..evaluated.len() {
                            flat_values.push(evaluated.get_val(i));
                        }
                        rows += 1;
                    } else {
                        return Err("Arrays deeper than 2D are not supported yet".into());
                    }
                }

                let shape = if all_scalars { vec![rows] } else { vec![rows, cols] };

                Ok(NdArray {
                    data: Arc::new(Float64Array::from(flat_values)),
                    shape,
                    grad: Arc::new(Mutex::new(None)),
                })
            }
            Expr::SliceAccess { target, start, end } => {
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
                })
            }
            Expr::BinaryOp { lhs, op, rhs } => {
                let left_arr = self.evaluate_expr(lhs)?;
                let right_arr = self.evaluate_expr(rhs)?;
                
                if matches!(op, Operator::Dot) {
                    if left_arr.shape.len() == 1 && right_arr.shape.len() == 1 {
                        if left_arr.len() != right_arr.len() {
                            return Err("1D dot product requires equal lengths".into());
                        }
                        let mut sum = 0.0;
                        for i in 0..left_arr.len() {
                            sum += left_arr.get_val(i) * right_arr.get_val(i);
                        }
                        return Ok(NdArray::scalar(sum));
                    }
                    if left_arr.shape.len() == 2 && right_arr.shape.len() == 2 {
                        let l_rows = left_arr.shape[0];
                        let l_cols = left_arr.shape[1];
                        let r_rows = right_arr.shape[0];
                        let r_cols = right_arr.shape[1];
                        
                        if l_cols != r_rows {
                            return Err(format!("Cannot dot multiply matrices of shapes {:?} and {:?}", left_arr.shape, right_arr.shape));
                        }
                        
                        let map_fn = |i: usize| -> f64 {
                            let row = i / r_cols;
                            let col = i % r_cols;
                            let mut sum = 0.0;
                            for k in 0..l_cols {
                                let l_val = left_arr.data.value(row * l_cols + k);
                                let r_val = right_arr.data.value(k * r_cols + col);
                                sum += l_val * r_val;
                            }
                            sum
                        };

                        let total = l_rows * r_cols;
                        
                        #[cfg(not(target_arch = "wasm32"))]
                        let result: Vec<f64> = (0..total).into_par_iter().map(map_fn).collect();

                        #[cfg(target_arch = "wasm32")]
                        let result: Vec<f64> = (0..total).map(map_fn).collect();

                        return Ok(NdArray {
                            data: Arc::new(Float64Array::from(result)),
                            shape: vec![l_rows, r_cols],
                            grad: Arc::new(Mutex::new(None)),
                        });
                    } else {
                        return Err("Dot product requires two 2D matrices".into());
                    }
                }

                let max_len = left_arr.len().max(right_arr.len());
                let op_clone = op.clone();
                
                let map_fn = |i: usize| -> f64 {
                    let l = left_arr.get_val(i);
                    let r = right_arr.get_val(i);
                    match op_clone {
                        Operator::Add => l + r,
                        Operator::Sub => l - r,
                        Operator::Mul => l * r,
                        Operator::Div => l / r,
                        Operator::Pow => l.powf(r),
                        Operator::Eq  => if l == r { 1.0 } else { 0.0 },
                        Operator::Neq => if l != r { 1.0 } else { 0.0 },
                        Operator::Gt  => if l > r { 1.0 } else { 0.0 },
                        Operator::Lt  => if l < r { 1.0 } else { 0.0 },
                        Operator::Gte => if l >= r { 1.0 } else { 0.0 },
                        Operator::Lte => if l <= r { 1.0 } else { 0.0 },
                        Operator::Dot => unreachable!(),
                    }
                };

                #[cfg(not(target_arch = "wasm32"))]
                let result: Vec<f64> = (0..max_len).into_par_iter().map(map_fn).collect();

                #[cfg(target_arch = "wasm32")]
                let result: Vec<f64> = (0..max_len).map(map_fn).collect();
                
                let out_shape = if left_arr.shape.len() >= right_arr.shape.len() {
                    left_arr.shape.clone()
                } else {
                    right_arr.shape.clone()
                };

                Ok(NdArray {
                    data: Arc::new(Float64Array::from(result)),
                    shape: out_shape,
                    grad: Arc::new(Mutex::new(None)),
                })
            }
            Expr::FunctionCall { name, arg } => {
                if name == "read_csv" {
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
                        });
                    } else {
                        return Err("read_csv expects a string literal path".into());
                    }
                    #[cfg(target_arch = "wasm32")]
                    return Err("read_csv is not supported in the WASM browser environment yet".into());
                }

                if name == "backward" {
                    let evaluated = self.evaluate_expr(arg)?;
                    let mut grad_lock = evaluated.grad.lock().unwrap();
                    let ones = vec![1.0; evaluated.len()];
                    *grad_lock = Some(Float64Array::from(ones));
                    return Ok(evaluated.clone());
                }

                if let Some(func_def) = self.functions.get(name).cloned() {
                    let evaluated_arg = self.evaluate_expr(arg)?;
                    
                    let mut local_engine = Engine {
                        variables: self.variables.clone(),
                        functions: self.functions.clone(),
                    };
                    
                    if !func_def.params.is_empty() {
                        local_engine.variables.insert(func_def.params[0].clone(), evaluated_arg);
                    }
                    
                    for stmt in &func_def.body {
                        let res = local_engine.execute_statement(stmt)?;
                        if let ExecutionResult::Return(v) = res {
                            return Ok(v);
                        }
                    }
                    return Ok(NdArray::scalar(0.0));
                }

                let evaluated = self.evaluate_expr(arg)?;
                
                if name == "plot" {
                    use textplots::{Chart, Plot, Shape};
                    let len = evaluated.len();
                    if len == 0 { return Err("Cannot plot an empty array".into()); }
                    
                    let mut points: Vec<(f32, f32)> = Vec::with_capacity(len);
                    for i in 0..len {
                        points.push((i as f32, evaluated.get_val(i) as f32));
                    }
                    
                    println!("Plot of length {}:", len);
                    Chart::new(120, 40, 0.0, (len - 1) as f32)
                        .lineplot(&Shape::Lines(&points))
                        .display();
                        
                    Ok(evaluated)
                } else if name == "zeros" || name == "ones" {
                    let val = if name == "zeros" { 0.0 } else { 1.0 };
                    let shape = match evaluated.shape.len() {
                        0 => vec![evaluated.get_val(0) as usize], // e.g. zeros(5)
                        _ => {
                            let mut s = Vec::new();
                            for i in 0..evaluated.len() {
                                s.push(evaluated.get_val(i) as usize);
                            }
                            s
                        }
                    };
                    let len = shape.iter().product();
                    Ok(NdArray {
                        data: Arc::new(Float64Array::from(vec![val; len])),
                        shape,
                        grad: Arc::new(Mutex::new(None)),
                    })
                } else if name == "linspace" {
                    if evaluated.len() != 3 {
                        return Err("linspace expects an array of [start, end, count]".into());
                    }
                    let start = evaluated.get_val(0);
                    let end = evaluated.get_val(1);
                    let count = evaluated.get_val(2) as usize;
                    if count == 0 { return Err("linspace count must be > 0".into()); }
                    
                    let step = if count > 1 { (end - start) / ((count - 1) as f64) } else { 0.0 };
                    let map_fn = |i: usize| start + (i as f64) * step;

                    #[cfg(not(target_arch = "wasm32"))]
                    let result: Vec<f64> = (0..count).into_par_iter().map(map_fn).collect();

                    #[cfg(target_arch = "wasm32")]
                    let result: Vec<f64> = (0..count).map(map_fn).collect();

                    Ok(NdArray {
                        data: Arc::new(Float64Array::from(result)),
                        shape: vec![count],
                        grad: Arc::new(Mutex::new(None)),
                    })
                } else if name == "sum" {
                    let len = evaluated.len();
                    if len == 0 { return Ok(NdArray::scalar(0.0)); }
                    
                    #[cfg(not(target_arch = "wasm32"))]
                    let sum: f64 = (0..len).into_par_iter().map(|i| evaluated.get_val(i)).sum();

                    #[cfg(target_arch = "wasm32")]
                    let sum: f64 = (0..len).map(|i| evaluated.get_val(i)).sum();
                    
                    Ok(NdArray::scalar(sum))
                } else if name == "mean" {
                    let len = evaluated.len();
                    if len == 0 { return Ok(NdArray::scalar(0.0)); }

                    #[cfg(not(target_arch = "wasm32"))]
                    let sum: f64 = (0..len).into_par_iter().map(|i| evaluated.get_val(i)).sum();

                    #[cfg(target_arch = "wasm32")]
                    let sum: f64 = (0..len).map(|i| evaluated.get_val(i)).sum();
                    
                    Ok(NdArray::scalar(sum / (len as f64)))
                } else if name == "transpose" {
                    if evaluated.shape.len() == 2 {
                        let rows = evaluated.shape[0];
                        let cols = evaluated.shape[1];
                        let map_fn = |idx: usize| -> f64 {
                            let i = idx / rows; // new row
                            let j = idx % rows; // new col
                            evaluated.get_val(j * cols + i)
                        };

                        let total = rows * cols;
                        #[cfg(not(target_arch = "wasm32"))]
                        let result: Vec<f64> = (0..total).into_par_iter().map(map_fn).collect();

                        #[cfg(target_arch = "wasm32")]
                        let result: Vec<f64> = (0..total).map(map_fn).collect();

                        Ok(NdArray {
                            data: Arc::new(Float64Array::from(result)),
                            shape: vec![cols, rows],
                            grad: Arc::new(Mutex::new(None)),
                        })
                    } else {
                        Ok(evaluated)
                    }
                } else if name == "sin" || name == "cos" || name == "tan" || name == "log" || name == "sqrt" || name == "relu" || name == "sigmoid" {
                    let map_fn = |i: usize| -> f64 {
                        let v = evaluated.get_val(i);
                        match name.as_str() {
                            "sin" => v.sin(),
                            "cos" => v.cos(),
                            "tan" => v.tan(),
                            "log" => v.ln(),
                            "sqrt" => v.sqrt(),
                            "relu" => if v > 0.0 { v } else { 0.0 },
                            "sigmoid" => 1.0 / (1.0 + (-v).exp()),
                            _ => unreachable!(),
                        }
                    };
                    
                    #[cfg(not(target_arch = "wasm32"))]
                    let result: Vec<f64> = (0..evaluated.len()).into_par_iter().map(map_fn).collect();

                    #[cfg(target_arch = "wasm32")]
                    let result: Vec<f64> = (0..evaluated.len()).map(map_fn).collect();

                    Ok(NdArray {
                        data: Arc::new(Float64Array::from(result)),
                        shape: evaluated.shape.clone(),
                        grad: Arc::new(Mutex::new(None)),
                    })
                } else {
                    Err(format!("Unknown function: {}", name))
                }
            }
        }
    }

    pub fn execute_statement(&mut self, stmt: &Statement) -> Result<ExecutionResult, String> {
        match stmt {
            Statement::Assignment { target, expr } => {
                let result = self.evaluate_expr(expr)?;
                self.variables.insert(target.clone(), result.clone());
                Ok(ExecutionResult::Value(result))
            }
            Statement::Expression(expr) => {
                let result = self.evaluate_expr(expr)?;
                Ok(ExecutionResult::Value(result))
            }
            Statement::IfElse { condition, true_block, false_block } => {
                let cond_res = self.evaluate_expr(condition)?;
                let is_true = cond_res.get_val(0) != 0.0;
                
                let block_to_execute = if is_true {
                    Some(true_block)
                } else {
                    false_block.as_ref()
                };

                let mut last_res = ExecutionResult::None;
                if let Some(block) = block_to_execute {
                    for s in block {
                        let res = self.execute_statement(s)?;
                        if let ExecutionResult::Return(v) = res {
                            return Ok(ExecutionResult::Return(v));
                        }
                        last_res = res;
                    }
                }
                
                Ok(last_res)
            }
            Statement::FunctionDef { name, params, body } => {
                self.functions.insert(name.clone(), FunctionDef {
                    params: params.clone(),
                    body: body.clone(),
                });
                Ok(ExecutionResult::None)
            }
            Statement::Return(expr) => {
                let result = self.evaluate_expr(expr)?;
                Ok(ExecutionResult::Return(result))
            }
        }
    }
}
