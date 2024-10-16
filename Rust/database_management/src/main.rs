use anyhow::Result;

fn main() -> Result<()> {
    let config_path = "config/database.json";
    let db_path = "db/example.db";

    // Call the function from lib.rs to create the database
    database_creator::create_database_from_config(config_path, db_path)?;

    // Call the function from lib.rs to insert data into the database
    database_creator::insert_data_from_config(config_path, db_path)?;

    println!("Database created and populated successfully.");
    Ok(())
}
