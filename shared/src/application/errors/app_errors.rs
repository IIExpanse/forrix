use core::fmt;

use thiserror::Error;

#[derive(Error, Debug)]
pub struct AppError {
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(cause) = self.cause.as_ref() {
            return write!(f, "(message: {}, cause: {:#?})", self.message, cause);
        }
        write!(f, "(message: {}, cause: None)", self.message)
    }
}
