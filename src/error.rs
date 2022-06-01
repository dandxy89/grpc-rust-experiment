use std::{error::Error, fmt};

#[derive(Debug)]
pub struct SolarSystemError {
    message: String,
}

impl SolarSystemError {
    pub fn new(msg: &str) -> Self {
        SolarSystemError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for SolarSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SolarSystemError {}
