use std::fmt;
use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
pub enum VbsError {
    ResourceNotFound(String),
    InvalidPeriod,
}

impl fmt::Display for VbsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VbsError::ResourceNotFound(message) => write!(f, "Resource not found: {}", message),
            VbsError::InvalidPeriod => write!(f, "Invalid period specified"),
        }
    }
}

impl std::error::Error for VbsError {}
