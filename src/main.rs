mod driver;
use crate::driver::Driver;
mod diagnostics;
use crate::diagnostics::DiagnosticEngine;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    // Get the arguments passed to this program
    let arg_vec: Vec<String> = env::args().collect();


    // Here is the sample interface we're looking for in terms of compiler diagnostics
    let mut diagnostic_engine: DiagnosticEngine = DiagnosticEngine::new();
    let mut driver: Driver = Driver::new(arg_vec);
    
    // We will rewrite this line to be:
    driver.run(&mut diagnostic_engine)?;
    Ok(())
}
