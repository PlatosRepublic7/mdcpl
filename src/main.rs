mod driver;
use crate::driver::Driver;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    // Get the arguments passed to this program
    let arg_vec: Vec<String> = env::args().collect();

    let mut driver: Driver = Driver::new(arg_vec);
    
    driver.run()?;
    Ok(())
}
