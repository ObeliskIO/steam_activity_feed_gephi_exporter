use crate::steam_models::MonitoredSteamUser;
use crate::errors::AppError;
use csv::Writer;
use std::fs::File;
use colored::*;

const GEPHI_CSV_FILENAME: &str = "steam_friends_graph.csv";

#[derive(Debug, serde::Serialize)]
struct GephiEdge {
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "Target")]
    target: String,
    // Potentially add more attributes like 'Weight' or 'Type' if needed later
    // For now, 'FriendSince' is not directly part of the edge for Gephi,
    // but could be an edge attribute if desired.
}

pub fn export_to_gephi_csv(users: &[MonitoredSteamUser]) -> Result<(), AppError> {
    println!(
        "{} Starting Gephi CSV export to '{}'...",
        "📊".yellow(),
        GEPHI_CSV_FILENAME.cyan()
    );

    let file = File::create(GEPHI_CSV_FILENAME)?;
    let mut wtr = Writer::from_writer(file);

    // Write header
    // wtr.write_record(&["Source", "Target"])?; // serde_derive handles this if struct fields are named correctly

    let mut edge_count = 0;
    for user in users {
        let source_steam_id = &user.steam_id;
        if user.current_friends.is_empty() {
            println!(
                "{} User {} has no friends in currentFriends list, skipping.",
                "ℹ️".blue(),
                source_steam_id.cyan()
            );
            continue;
        }

        for friend_entry in &user.current_friends {
            let target_steam_id = &friend_entry.steam_id;
            let edge = GephiEdge {
                source: source_steam_id.clone(),
                target: target_steam_id.clone(),
            };
            wtr.serialize(edge)?;
            edge_count += 1;
        }
    }

    wtr.flush()?; // Ensure all data is written to the file

    println!(
        "{} Successfully exported {} edges to '{}'.",
        "✅".green(),
        edge_count.to_string().cyan(),
        GEPHI_CSV_FILENAME.cyan()
    );
    Ok(())
}
