use thiserror::Error;
use colored::*;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("dotenv error: {0}")]
    Dotenv(#[from] dotenv::Error),

    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(String),

    #[error("MongoDB error: {0}")]
    Mongo(#[from] mongodb::error::Error),

    #[error("MongoDB BSON serialization/deserialization error: {0}")]
    MongoBson(#[from] mongodb::bson::de::Error), // For BSON deserialization errors

    #[error("CSV writing error: {0}")]
    Csv(#[from] csv::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Failed to parse ObjectId: {0}")]
    ObjectIdParse(#[from] mongodb::bson::oid::Error),
}

// Optional: A helper function to print errors in a colored format
pub fn print_error(error: &AppError) {
    eprintln!("{} {}", "❌ Error:".red().bold(), error.to_string().red());
}
