use std::{env, error::Error, path::Path};

pub mod lang;

/// main launches the rox interpreter
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // Read contents of file if only one argument passed to the program
        2 => lang::runner::run_file(Path::new(&args[1])),
        // Execute prompt when calling with a number of arguments != 1
        _ => lang::runner::run_prompt(),
    }
}
