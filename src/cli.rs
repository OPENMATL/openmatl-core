use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::engine::Engine;
use crate::parser::parse_program;

pub fn run_repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    
    let mut engine = Engine::new();
    
    println!("OpenMat CLI v0.1.0");
    println!("Type your equations. Try: C = [2, 2] * 4");
    println!("Ctrl-C or Ctrl-D to exit.");
    
    loop {
        let readline = rl.readline("om> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                
                rl.add_history_entry(input)?;
                
                match parse_program(input) {
                    Ok(program) => {
                        for stmt in program.statements {
                            match engine.execute_statement(&stmt) {
                                Ok(res) => {
                                    let result = match res {
                                        crate::engine::ExecutionResult::Value(v) => v,
                                        crate::engine::ExecutionResult::Return(v) => v,
                                        crate::engine::ExecutionResult::None => continue,
                                    };
                                    let mut out = String::new();
                                    out.push_str("[");
                                    for i in 0..result.len() {
                                        if i > 0 {
                                            out.push_str(", ");
                                        }
                                        out.push_str(&result.get_val(i).to_string());
                                    }
                                    out.push_str("]");
                                    if !result.shape.is_empty() {
                                        out.push_str(&format!(" (shape: {:?})", result.shape));
                                    }
                                    println!("= {}", out);
                                }
                                Err(e) => eprintln!("Execution Error: {}", e),
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Parse error: {}", e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")?;
    Ok(())
}
