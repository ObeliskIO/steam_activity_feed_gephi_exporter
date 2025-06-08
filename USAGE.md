# Steam Activity Feed Gephi Exporter - Usage Guide

This guide explains how to set up, configure, and run the Steam Activity Feed Gephi Exporter.

## Table of Contents
1. [Introduction](#introduction)
2. [Prerequisites](#prerequisites)
3. [Installation](#installation)
4. [Configuration](#configuration)
5. [Running the Exporter](#running-the-exporter)
6. [Output](#output)
7. [Troubleshooting](#troubleshooting)

## 1. Introduction
The Steam Activity Feed Gephi Exporter is a tool designed to fetch Steam user activity data and export it into a format compatible with Gephi for network analysis and visualization.

## 2. Prerequisites
Before you begin, ensure you have the following installed and configured:
- **Rust:** This project is built with Rust. You can install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Gephi:** To visualize the output graph, you'll need Gephi. Download it from [gephi.org](https://gephi.org/).

## 3. Installation
1. **Clone the repository:**
   ```bash
   git clone <repository-url> # Replace <repository-url> with the actual URL
   cd steam_activity_feed_gephi_exporter
   ```
2. **Build the project:**
   Navigate to the project directory and run:
   ```bash
   cargo build --release
   ```
   This will compile the project and create an executable in the `target/release/` directory.

## 4. Configuration
The application requires a `.env` file in the root of the project directory with the following variables:

```env
DATABASE_URL="your_database_connection_string" # e.g., "sqlite:steam_activity.db"
STEAM_USER_ID="YOUR_STEAM_ID_64" # Your 64-bit Steam ID
```

- **`DATABASE_URL`**: Connection string for the SQLite database where data will be stored. If the file doesn't exist, it will be created.
- **`STEAM_USER_ID`**: The 64-bit Steam ID of the user whose activity feed and friends list you want to process.

Create a `.env` file in the project root and add your specific values.

Example `.env` file:
```
DATABASE_URL="sqlite:steam_data.db"
STEAM_USER_ID="76561197960287930"
```

## 5. Running the Exporter
Once the project is built and configured, you can run the exporter from the project's root directory:

```bash
cargo run --release
```
Or, if you prefer to run the compiled binary directly:
```bash
./target/release/steam_activity_feed_gephi_exporter
```

The application will:
1. Fetch the friends list for the `STEAM_USER_ID` specified in `.env`.
2. For each friend, fetch their recently played games.
3. Store this information in the SQLite database.
4. Generate a GEXF file (e.g., `steam_friends_graph.gexf`) in the project root, which can be imported into Gephi.

## 6. Output
The primary output of the exporter is a GEXF (Graph Exchange XML Format) file, typically named `steam_friends_graph.gexf`. This file contains:
- **Nodes:** Representing Steam users (your friends and yourself).
  - Node attributes may include `SteamID` and `PersonaName`.
- **Edges:** Representing relationships or shared activities.
  - In the current implementation, edges represent friendships.
  - Future enhancements could include edges based on shared games.

This GEXF file can be directly opened in Gephi for visualization and analysis of your Steam friend network and their gaming activities.
Additionally, a SQLite database file (e.g., `steam_data.db`) is created or updated, storing the raw data fetched from the Steam API.

## 7. Troubleshooting
- **Rate Limiting:** The Steam API has rate limits. If you make too many requests in a short period, you might be temporarily blocked. Wait for some time before running the exporter again. The application has some built-in delays to mitigate this, but heavy usage might still trigger limits.
- **Incorrect Steam User ID:** Ensure the `STEAM_USER_ID` is a valid 64-bit Steam ID. You can find your 64-bit Steam ID using online tools like [SteamID Finder](https://www.steamidfinder.com/).
- **Database Issues:**
    - If you have problems with the database (e.g., `Error creating/opening database`), ensure the path specified in `DATABASE_URL` is writable by the application.
    - If the database schema changes in future versions, you might need to delete the old database file to allow the application to recreate it.
- **No Output File Generated:**
    - Check the console output for any error messages during execution.
    - Ensure the application has write permissions in the project directory to create the `.gexf` and database files.
- **Compilation Errors:** If `cargo build` fails, ensure your Rust installation is up-to-date (`rustup update`) and that all dependencies listed in `Cargo.toml` can be resolved. Check for any specific error messages provided by the Rust compiler.
- **Empty Graph or Missing Data:**
    - Verify that the Steam profile of the `STEAM_USER_ID` and their friends' profiles are public enough to allow data fetching. Private profiles or profiles with restricted visibility for game details might result in incomplete data.
    - Check if the user and their friends have played games recently, as the API primarily returns recently played games.
- **Gephi Issues:** If you have trouble importing or visualizing the `.gexf` file in Gephi:
    - Ensure you are using a compatible version of Gephi.
    - Check the Gephi console for any import errors.
    - The generated graph might be very large if the user has many friends. Gephi might require significant memory and processing power for large networks.

If you encounter issues not listed here, please check the project's issue tracker on its repository or consider opening a new issue with detailed information about the problem.
