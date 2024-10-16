use rusqlite::{Connection, Result as SqlResult, params};
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn create_database_from_config(config_path: &str, db_path: &str) -> SqlResult<()> {
    // Check if the config file exists
    if !Path::new(config_path).exists() {
        println!("Failed to find the config file: {}", config_path);
        std::process::exit(1);
    }

    // Check if the database file exists
    if !Path::new(db_path).exists() {
        println!("Database not found: {}", db_path);
        std::process::exit(1);
    }

    // Read the config file
    let config_data = fs::read_to_string(config_path)
        .expect("Failed to read the config file");

    // Parse the config as JSON
    let config: Value = serde_json::from_str(&config_data)
        .expect("Failed to parse the config file");

    // Create a connection to the database
    let conn = Connection::open(db_path)?;

    // Create tables
    if let Some(tables) = config["tables"].as_object() {
        for (table_name, table_info) in tables {
            let columns = table_info["columns"].as_array().unwrap();
            let mut create_table_sql = format!("CREATE TABLE IF NOT EXISTS {} (", table_name);

            let mut column_definitions = Vec::new();
            let mut foreign_keys = Vec::new();  // Collect foreign key definitions

            for column in columns {
                let name = column["name"].as_str().unwrap();
                let column_type = column["type"].as_str().unwrap();
                let constraints = column["constraints"].as_array().unwrap();

                let mut constraints_str = Vec::new();
                for constraint in constraints {
                    let constraint_str = constraint.as_str().unwrap();
                    // Handle foreign key separately
                    if constraint_str.starts_with("FOREIGN KEY") {
                        foreign_keys.push(constraint_str.to_string());
                    } else {
                        constraints_str.push(constraint_str);
                    }
                }

                // Append column definition to the vector
                column_definitions.push(format!("{} {} {}", name, column_type, constraints_str.join(" ")));
            }

            // Join column definitions and finalize the SQL
            create_table_sql.push_str(&column_definitions.join(", "));

            // Add foreign key definitions at the end
            if !foreign_keys.is_empty() {
                create_table_sql.push_str(", ");
                create_table_sql.push_str(&foreign_keys.join(", "));
            }

            create_table_sql.push_str(");");

            // Execute the SQL statement
            println!("Executing SQL: {}", create_table_sql); // Print the SQL for debugging
            conn.execute(&create_table_sql, params![])?;
        }
    }

    Ok(())
}

pub fn insert_data_from_config (config_path: &str, db_path: &str) -> SqlResult<()> {
    // Check if the config file exists
    if !Path::new(config_path).exists() {
        println!("Failed to find the config file: {}", config_path);
        std::process::exit(1);
    }

    // Check if the database file exists
    if !Path::new(db_path).exists() {
        println!("Database not found: {}", db_path);
        std::process::exit(1);
    }

    // Read the config file
    let config_data = fs::read_to_string(config_path)
        .expect("Failed to read the config file");

    // Parse the config as JSON
    let config: Value = serde_json::from_str(&config_data)
        .expect("Failed to parse the config file");

    // Create a connection to the database
    let conn = Connection::open(db_path)?;

    if let Some(tables) = config["tables"].as_object() {
        for (table_name, table_info) in tables {
            let data = table_info["data"].as_array().unwrap();

            for item in data {
                
            }
        }
    }

    Ok(())
}