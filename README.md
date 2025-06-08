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
- **MongoDB:** The application reads data from MongoDB. Ensure you have a running MongoDB instance and the necessary data populated. You can find installation instructions on [mongodb.com](https://www.mongodb.com/try/download/community).
- **CSV Viewer/Editor:** To view the output CSV file, you'll need a tool like Microsoft Excel, Google Sheets, LibreOffice Calc, or a simple text editor.

## 3. Installation
1. **Clone the repository:**
   ```bash
   git clone https://github.com/ObeliskIO/steam_activity_feed_gephi_exporter/
   cd steam_activity_feed_gephi_exporter
   ```
2. **Build the project:**
   Navigate to the project directory and run:
   ```bash
   cargo build --release
   ```
   This will compile the project and create an executable in the `target/release/` directory.

## 4. Configuration
The application requires a `.env` file in the root of the project directory with a single environment variable:

```env
MONGODB_URI="your_mongodb_connection_string"
```

- **`MONGODB_URI`**: Your MongoDB connection string (e.g., `mongodb://localhost:27017`). The application will connect to this MongoDB instance.

**Important Database and Collection Names:**
The exporter is hardcoded to use the following:
-   **Database Name:** `steam_intel`
-   **Collection Name:** `monitoredsteamusers`

Ensure that your MongoDB instance specified by `MONGODB_URI` has a database named `steam_intel` and within it, a collection named `monitoredsteamusers`. This collection should contain the documents representing the Steam users whose data you want to export. The structure of these documents should align with what the exporter expects (see `src/steam_models.rs` for `MonitoredSteamUser` structure if details are needed).

Create a `.env` file in the project root with your `MONGODB_URI`.

Example `.env` file:
```
MONGODB_URI="mongodb://myuser:mypassword@localhost:27017"
```

## 5. Running the Exporter
Once the project is built and configured with the `MONGODB_URI` in your `.env` file, you can run the exporter from the project's root directory:

```bash
cargo run --release
```
Or, if you prefer to run the compiled binary directly:
```bash
./target/release/steam_activity_feed_gephi_exporter
```

The application will:
1. Connect to the MongoDB instance specified by `MONGODB_URI`.
2. Access the `steam_intel` database.
3. Read data from the `monitoredsteamusers` collection within that database.
4. Process this data to extract user relationships and activities.
5. Generate a CSV (Comma Separated Values) file named `steam_friends_graph.csv` in the project root.

## 6. Output
The primary output of the exporter is a CSV file named `steam_friends_graph.csv`, located in the project's root directory. This file is formatted for easy import into network analysis tools like Gephi.

The CSV file typically contains columns representing graph edges, such as:
- **Source:** The ID of the source node (e.g., a Steam user ID).
- **Target:** The ID of the target node (e.g., another Steam user ID they are friends with).
- **Type:** The type of relationship (e.g., "Directed" or "Undirected").
- **Weight (Optional):** A numeric value representing the strength or frequency of the interaction/relationship.
- Additional columns might be present depending on the specific data exported (e.g., shared games, interaction timestamps).

This CSV can be imported into Gephi using the "Import spreadsheet" functionality to visualize the network of Steam users and their connections.
The exporter reads data from MongoDB and outputs this CSV file; it does not create or modify other database files during its standard operation.

## 7. Troubleshooting
- **MongoDB Connection Issues:**
    - Verify your `MONGODB_URI` in the `.env` file is correct and that your MongoDB instance is running and accessible.
    - Check MongoDB logs for any connection errors.
    - Ensure that the MongoDB instance has a database named `steam_intel` and, within it, a collection named `monitoredsteamusers`.
- **No CSV Output File Generated / Empty CSV (`steam_friends_graph.csv`):**
    - Check the console output for any error messages during execution (e.g., "No users found in the database").
    - Ensure the application has write permissions in the project directory to create `steam_friends_graph.csv`.
    - Verify that the `monitoredsteamusers` collection in the `steam_intel` database is not empty and contains valid user data that the exporter can process.
- **Incorrect Data in CSV:**
    - Double-check the data in your MongoDB collection.
    - Review the exporter's logic for reading from MongoDB and mapping data to CSV columns.
- **Compilation Errors:** If `cargo build` fails, ensure your Rust installation is up-to-date (`rustup update`) and that all dependencies listed in `Cargo.toml` (especially MongoDB drivers) can be resolved. Check for any specific error messages provided by the Rust compiler.
- **Issues Importing CSV into other tools (e.g., Gephi):**
    - Ensure the CSV format (delimiter, header row) is compatible with the import requirements of the tool.
    - Check for any special characters or encoding issues in the CSV data that might cause parsing problems.

If you encounter issues not listed here, please check the project's issue tracker on its repository or consider opening a new issue with detailed information about the problem.
