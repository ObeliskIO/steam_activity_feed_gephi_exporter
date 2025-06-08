// Declare modules
mod config;
mod db;
mod errors;
mod gephi_exporter;
mod steam_models;

use colored::*;
use config::Config;
use db::DbClient;
use errors::{AppError, print_error};
use gephi_exporter::export_to_gephi_csv;

#[tokio::main]
async fn main() {
    println!(
        "\n{} {} {} {}",
        "🚀".bright_yellow(),
        "Steam Activity Feed Gephi Exporter".bright_cyan().bold(),
        "v0.1.0".dimmed(),
        "🚀".bright_yellow()
    );
    println!("{}", "--------------------------------------------------".bright_black());

    if let Err(e) = run_exporter().await {
        print_error(&e);
        std::process::exit(1);
    }

    println!("{}", "--------------------------------------------------".bright_black());
    println!(
        "{} 🎉 {} 🎉 {}",
        "✨".magenta(),
        "Export process completed successfully!".green().bold(),
        "✨".magenta()
    );
    println!(
        "{} You can find the Gephi graph file at: {}",
        "💡".yellow(),
        "steam_friends_graph.csv".cyan().underline()
    );
}

async fn run_exporter() -> Result<(), AppError> {
    // 1. Load Configuration
    println!("\n{} {}", " مرحله 1:".bold().yellow(), "Loading Configuration".yellow());
    let app_config = Config::from_env()?;
    println!("{} Configuration loaded successfully.", "✔️".green());

    // 2. Connect to Database
    println!("\n{} {}", " مرحله 2:".bold().yellow(), "Connecting to Database".yellow());
    let db_client = DbClient::connect(&app_config).await?;
    println!("{} Database connection established.", "✔️".green());

    // 3. Fetch Monitored Steam Users
    println!("\n{} {}", " مرحله 3:".bold().yellow(), "Fetching Steam User Data".yellow());
    let monitored_users = db_client.get_all_monitored_steam_users().await?;
    if monitored_users.is_empty() {
        println!(
            "{} {} No users found in the database. Nothing to export.",
            "⚠️".yellow(),
            "Warning:".yellow().bold()
        );
        return Ok(()); // Successful exit, but nothing to do
    }
    println!(
        "{} Fetched data for {} users.",
        "✔️".green(),
        monitored_users.len().to_string().cyan()
    );

    // 4. Export to Gephi CSV
    println!("\n{} {}", " مرحله 4:".bold().yellow(), "Exporting to Gephi CSV".yellow());
    export_to_gephi_csv(&monitored_users)?;
    println!("{} Data exported to Gephi CSV successfully.", "✔️".green());

    Ok(())
}
