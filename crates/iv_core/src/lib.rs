use crate::errors::AppError;

pub mod configs;
pub mod errors;

pub type AppResult<T> = std::result::Result<T, AppError>;
