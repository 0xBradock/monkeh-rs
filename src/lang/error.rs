use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct RoxRunError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for RoxRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[line {}]: Error: {}", self.line, self.message)
    }
}

impl Error for RoxRunError {
    fn description(&self) -> &str {
        "fsdafa"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
