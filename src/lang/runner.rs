use std::{
    error::Error,
    fs,
    io::{self, Write},
    path::Path,
};


use crate::lang::{error, scanner::{Scanner, Token}};

/// run_file executes the interpreter on the contents of the given file
pub fn run_file(path: &Path) -> Result<(), Box<dyn Error>> {
    println!("Running file: {}", path.display());

    let contents = fs::read_to_string(path)?;
    println!("{contents}");

    run(contents)?;

    Ok(())
}

/// run_prompt launches a REPL for the language
pub fn run_prompt() -> Result<(), Box<dyn Error>> {
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

/// run reads the source code.
/// It will possibly read multiple lines from a file or each line from the prompt
pub fn run(source: String) -> Result<(), error::RoxRunError> {
    let s = Scanner::new(source);

    let tokens: Vec<Token> = s.scan();

    for token in tokens.iter() {
        println!("{:#?}", token.literal);
    }

    println!("{}", s.contents);

    Err(error::RoxRunError {
        line: 12,
        message: String::from("test message for now"),
    })
}
