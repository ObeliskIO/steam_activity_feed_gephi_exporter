use crate::errors::AppError;
use dotenv::dotenv;
use std::env;
use colored::*;

pub struct Config {
    pub mongodb_uri: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        println!("{} Loading environment variables...", "⚙️".yellow());
        dotenv().ok(); // Load .env file, .ok() converts Result to Option, ignoring if .env doesn't exist

        let mongodb_uri = env::var("MONGODB_URI")
            .map_err(|_| AppError::EnvVarNotFound("MONGODB_URI".to_string()))?;
        
        if mongodb_uri.is_empty() {
            return Err(AppError::EnvVarNotFound("MONGODB_URI value is empty".to_string()));
        }

        println!("{} Successfully loaded {} environment variable.", "✅".green(), "MONGODB_URI".cyan());
        Ok(Config { mongodb_uri })
    }
}
