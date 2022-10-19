use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct PoolCreationError {
    pub description: String,
}

impl Error for PoolCreationError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
