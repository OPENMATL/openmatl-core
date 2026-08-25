use openmat_core::cli;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        let cmd = &args[1];
        if cmd == "run" && args.len() > 2 {
            let script_path = &args[2];
            match std::fs::read_to_string(script_path) {
                Ok(content) => {
                    let mut engine = openmat_core::engine::Engine::new();
                    match openmat_core::parser::parse_program(&content) {
                        Ok(program) => {
                            for stmt in program.statements {
                                if let Err(e) = engine.execute_statement(&stmt) {
                                    eprintln!("Execution Error: {}", e);
                                    std::process::exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Parse error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read script {}: {}", script_path, e);
                    std::process::exit(1);
                }
            }
        } else {
            println!("Usage:");
            println!("  openmat run <script.om>");
            println!("  openmat (to start REPL)");
        }
    } else {
        if let Err(e) = cli::run_repl() {
            eprintln!("REPL Error: {:?}", e);
        }
    }
}
