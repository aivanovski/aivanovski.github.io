use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum AppError {
    IOError { message: String },
    GenericError { message: String },
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            AppError::IOError { message } => message,
            AppError::GenericError { message } => message,
        };

        write!(f, "AppError(message={message})")
    }
}
