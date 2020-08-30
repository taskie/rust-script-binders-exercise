use std::{error::Error, fmt};

#[derive(Debug)]
pub struct SimpleError(pub String);

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(self.0.as_str())
    }
}

impl Error for SimpleError {}
