use dotenv::dotenv;
use mongodb::{bson::{doc, oid::ObjectId, DateTime as BsonDateTime}, options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use csv::Writer;
use futures::stream::TryStreamExt; // Required for cursor.try_next()

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Friend {
    steam_id: String,
    friend_since: i64, // Changed from u32 to i64 to match schema (Number can be large)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FriendListChange {
    timestamp: BsonDateTime, // Changed from chrono::DateTime<chrono::Utc>
    added: Vec<Friend>,
    removed: Vec<Friend>,
    new_friend_count: i32, // Changed from u32
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActivityHistoryItem {
    timestamp: BsonDateTime, // Changed from chrono::DateTime<chrono::Utc>
    persona_state_text: Option<String>,
    current_game_name: Option<String>,
    owned_games_count: Option<i32>, // Changed from u32
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PersonaNameHistoryItem {
    timestamp: BsonDateTime, // Changed from chrono::DateTime<chrono::Utc>
    old_name: Option<String>,
    new_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlayerSummarySnapshot {
    timestamp: BsonDateTime, // Changed from chrono::DateTime<chrono::Utc>
    summary: serde_json::Value, // Using serde_json::Value for Mixed type
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MonitoredSteamUser {
    #[serde(rename = "_id")]
    id: Option<ObjectId>, // For MongoDB's _id field
    steam_id: String,
    custom_url: Option<String>,
    label: Option<String>,
    discord_channel_id: String,
    monitor_comments: Option<bool>,
    monitor_online_status: Option<bool>,
    last_comment_timestamp: Option<i64>, // Changed from u32
    last_online_timestamp: Option<i64>,  // Changed from u32
    last_persona_state_text: Option<String>,
    friend_list_private: Option<bool>,
    current_friends: Option<Vec<Friend>>,
    friend_list_change_history: Option<Vec<FriendListChange>>,
    activity_history: Option<Vec<ActivityHistoryItem>>,
    last_known_current_game_name: Option<String>,
    last_owned_games_count: Option<i32>, // Changed from u32
    current_persona_name: Option<String>,
    persona_name_history: Option<Vec<PersonaNameHistoryItem>>,
    priority: Option<i32>, // Changed from u32
    is_profile_private: Option<bool>,
    current_player_summary: Option<serde_json::Value>, // Using serde_json::Value for Mixed type
    player_summary_history: Option<Vec<PlayerSummarySnapshot>>,
    added_by: String,
    #[serde(rename = "createdAt")]
    created_at: Option<BsonDateTime>, // For Mongoose's timestamps: true
    #[serde(rename = "updatedAt")]
    updated_at: Option<BsonDateTime>, // For Mongoose's timestamps: true
}

#[derive(Debug, Serialize)]
struct GephiEdge {
    Source: String,
    Target: String,
    Type: String,
    Weight: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env file");

    let client_options = ClientOptions::parse(&mongodb_uri).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("steam_intel");
    let collection: Collection<MonitoredSteamUser> = db.collection("monitoredsteamusers");

    println!("Fetching users and their friend lists...");

    let mut users_cursor = collection.find(None, None).await?;

    let mut gephi_edges: Vec<GephiEdge> = Vec::new();

    while let Some(user_doc) = users_cursor.try_next().await? {
        if let Some(current_friends) = user_doc.current_friends {
            let source_steam_id = user_doc.steam_id.clone();
            for friend in current_friends {
                gephi_edges.push(GephiEdge {
                    Source: source_steam_id.clone(),
                    Target: friend.steam_id,
                    Type: "Directed".to_string(), // Or "Undirected" depending on your model
                    Weight: 1, // Default weight, or use friendSince if relevant
                });
            }
        }
    }

    if gephi_edges.is_empty() {
        println!("No friend list data found to export.");
        return Ok(());
    }

    let file_path = "friend_network.csv";
    let file = File::create(file_path)?;
    let mut wtr = Writer::from_writer(file);

    // Write header
    wtr.write_record(&["Source", "Target", "Type", "Weight"])?;

    for edge in gephi_edges {
        wtr.serialize(edge)?;
    }

    wtr.flush()?;
    println!("Successfully exported friend network to {}", file_path);

    Ok(())
}
