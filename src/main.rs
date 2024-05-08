use std::{env, fs};

/// main launches the rox interpreter
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // Read contents of file if only one argument passed to the program
        2 => run_file(&args[1]),
        // Execute prompt when calling with a number of arguments != 1
        _ => run_prompt(),
    }
}

/// run_file executes the interpreter on the contents of the given file
fn run_file(path: &String) {
    println!("Running file: {path}");

    let contents = fs::read_to_string(path).expect("failed to read contents of {path}");
    println!("{contents}");
}

/// run_prompt launches a REPL for the language
fn run_prompt() {
    todo!()
}
