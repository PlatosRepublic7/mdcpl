mod driver;
mod diagnostics;
mod lexer;
mod source;
use crate::driver::Driver;
use crate::diagnostics::DiagnosticEngine;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    // Get the arguments passed to this program
    let arg_vec: Vec<String> = env::args().collect();

    let mut diagnostic_engine: DiagnosticEngine = DiagnosticEngine::new();
    let mut driver: Driver = Driver::new(arg_vec);
    
    driver.run(&mut diagnostic_engine)?;
    Ok(())
}
