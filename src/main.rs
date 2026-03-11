mod driver;
mod diagnostics;
mod lexer;
mod source;
mod parser;
use crate::driver::Driver;
use crate::diagnostics::DiagnosticEngine;

fn main() {
    let arg_vec: Vec<String> = std::env::args().collect();

    let mut diagnostic_engine: DiagnosticEngine = DiagnosticEngine::new();
    
    let mut driver = match Driver::new(arg_vec) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("fatal: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = driver.run(&mut diagnostic_engine) {
        diagnostic_engine.render();
        eprintln!("fatal: {}", e);
        std::process::exit(1);
    }

    diagnostic_engine.render();
    if diagnostic_engine.has_errors() {
        std::process::exit(1);
    }

    std::process::exit(0);
}
