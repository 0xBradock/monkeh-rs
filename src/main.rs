use std::{
    env, fs,
    io::{self, Write},
    path::Path,
};

/// main launches the rox interpreter
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // Read contents of file if only one argument passed to the program
        2 => run_file(Path::new(&args[1])),
        // Execute prompt when calling with a number of arguments != 1
        _ => run_prompt(),
    }
}

/// run_file executes the interpreter on the contents of the given file
fn run_file(path: &Path) -> Result<(), std::io::Error> {
    println!("Running file: {}", path.display());

    let contents = fs::read_to_string(path)?;
    println!("{contents}");

    run(contents)?;

    Ok(())
}

/// run_prompt launches a REPL for the language
fn run_prompt() -> Result<(), std::io::Error> {
    loop {
        // Print prompt
        io::stdout().write_all(b"~> ")?;
        io::stdout().flush()?;

        // Read line
        let mut line = String::new();
        let n = io::stdin().read_line(&mut line)?;

        if n > 1 {
            run(line)?;
        }
    }
}

fn run(source: String) -> Result<(), std::io::Error> {
    println!("running rox\n{}", source);

    Ok(())
}
