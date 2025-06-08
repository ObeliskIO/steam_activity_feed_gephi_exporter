use mongodb::bson::oid::ObjectId;
// We will use the full path for serde_helpers to avoid ambiguity
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value; // For Schema.Types.Mixed

// Helper for serde default for boolean fields
fn default_true() -> bool {
    true
}

// Helper for serde default for priority field
fn default_priority() -> i64 {
    2
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FriendEntry {
    pub steam_id: String,
    pub friend_since: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FriendListChange {
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>,
    #[serde(default)] // If 'added' is missing or null, it will be an empty Vec
    pub added: Vec<FriendEntry>,
    #[serde(default)] // If 'removed' is missing or null, it will be an empty Vec
    pub removed: Vec<FriendEntry>,
    pub new_friend_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEntry {
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>,
    pub persona_state_text: Option<String>,
    pub current_game_name: Option<String>,
    pub owned_games_count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonaNameChange {
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>,
    pub old_name: Option<String>,
    pub new_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSummarySnapshotInDoc { // Renamed to avoid conflict if PlayerSummarySnapshot is imported
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>,
    pub summary: Value, // Schema.Types.Mixed
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MonitoredSteamUser {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub steam_id: String,
    pub custom_url: Option<String>,
    pub label: Option<String>,
    pub discord_channel_id: String,
    
    #[serde(default = "default_true")]
    pub monitor_comments: bool,
    #[serde(default = "default_true")]
    pub monitor_online_status: bool,
    
    pub last_comment_timestamp: Option<i64>,
    pub last_online_timestamp: Option<i64>, 
    pub last_persona_state_text: Option<String>,
    
    #[serde(default)]
    pub friend_list_private: bool,
    
    #[serde(default, alias = "currentFriends")] // Handles both "currentFriends" and "current_friends"
    pub current_friends: Vec<FriendEntry>, 

    #[serde(default)]
    pub friend_list_change_history: Vec<FriendListChange>,
    
    #[serde(default)]
    pub activity_history: Vec<ActivityEntry>,
    
    pub last_known_current_game_name: Option<String>,
    pub last_owned_games_count: Option<i64>,
    pub current_persona_name: Option<String>,
    
    #[serde(default)]
    pub persona_name_history: Vec<PersonaNameChange>,
    
    #[serde(default = "default_priority")]
    pub priority: i64, 
    
    #[serde(default)]
    pub is_profile_private: bool,

    pub current_player_summary: Option<Value>, 
    
    #[serde(default)]
    pub player_summary_history: Vec<PlayerSummarySnapshotInDoc>,
    
    pub added_by: String,
    
    // Timestamps from Mongoose { timestamps: true }
    #[serde(rename = "createdAt", with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime_optional", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt", with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime_optional", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}
