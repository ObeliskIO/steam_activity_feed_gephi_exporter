use mongodb::{Client, Collection, Database};
use mongodb::options::ClientOptions;
use futures::stream::TryStreamExt; // Required for cursor.try_collect()
use colored::*;

use crate::config::Config;
use crate::steam_models::MonitoredSteamUser;
use crate::errors::AppError;

const DATABASE_NAME: &str = "steam_intel";
const COLLECTION_NAME: &str = "monitoredsteamusers";

pub struct DbClient {
    client: Client,
}

impl DbClient {
    pub async fn connect(config: &Config) -> Result<Self, AppError> {
        println!("{} Attempting to connect to MongoDB...", "🔗".yellow());
        let client_options = ClientOptions::parse(&config.mongodb_uri).await?;
        let client = Client::with_options(client_options)?;
        
        // Ping the server to ensure the connection is established
        client
            .database("admin")
            .run_command(mongodb::bson::doc! { "ping": 1 }, None)
            .await?;
        println!("{} Successfully connected to MongoDB!", "✅".green());
        Ok(DbClient { client })
    }

    fn get_database(&self) -> Database {
        self.client.database(DATABASE_NAME)
    }

    fn get_monitored_users_collection(&self) -> Collection<MonitoredSteamUser> {
        self.get_database().collection::<MonitoredSteamUser>(COLLECTION_NAME)
    }

    pub async fn get_all_monitored_steam_users(&self) -> Result<Vec<MonitoredSteamUser>, AppError> {
        println!("{} Fetching all monitored Steam users from '{}' collection in '{}' database...", 
            "📥".yellow(), 
            COLLECTION_NAME.cyan(), 
            DATABASE_NAME.cyan()
        );
        let collection = self.get_monitored_users_collection();
        let mut cursor = collection.find(None, None).await?;
        
        let mut users: Vec<MonitoredSteamUser> = Vec::new();
        while let Some(result) = cursor.try_next().await? {
            users.push(result);
        }
        
        println!("{} Successfully fetched {} users.", "✅".green(), users.len().to_string().cyan());
        Ok(users)
    }
}
