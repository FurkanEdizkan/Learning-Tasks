use rusqlite::{params, Connection, Result as SqlResult};
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

const DB_NAME: &str = "db/example.db";

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

#[derive(Debug)]
struct Address {
    id: i32,
    person_id: i32, // Foreign key to Person
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Debug)]
struct Order {
    id: i32,
    person_id: i32, // Foreign key to Person
    product: String,
    quantity: i32,
}

// Drop all tables in the database
fn drop_tables(conn: &Connection) -> SqlResult<()> {
    conn.execute("DROP TABLE IF EXISTS orders", [])?;
    conn.execute("DROP TABLE IF EXISTS address", [])?;
    conn.execute("DROP TABLE IF EXISTS person", [])?;
    println!("All tables dropped successfully.");
    Ok(())
}

fn main() -> Result<()> {
    // Ensure the folder exists
    let db_folder = Path::new("db/");
    if !db_folder.exists() {
        fs::create_dir_all(db_folder).context("Failed to create db folder")?;
    }

    // Open the database connection
    let conn = Connection::open(DB_NAME).context("Failed to open database")?;

    // Check if the database file exists, create if not
    if !db_exists(DB_NAME) {
        println!("Database does not exist. Creating it...");
        create_tables(&conn)?;
        insert_initial_data(&conn)?;
    } else {
        println!("Database exists. Checking tables...");

        // Drop all tables before inserting new data
        drop_tables(&conn)?;

        // Create tables and insert initial data
        create_tables(&conn)?;
        insert_initial_data(&conn)?;
    }

    // Proceed with CRUD operations
    perform_crud_operations(&conn)?;

    Ok(())
}

// Check if the database exists
fn db_exists(db_name: &str) -> bool {
    fs::metadata(db_name).is_ok()
}

// Check if a table exists in the database
// fn table_exists(conn: &Connection, table_name: &str) -> SqlResult<bool> {
//     let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
//     let exists = stmt.exists(params![table_name])?; // Remove the second `?`
//     Ok(exists)
// }

// Create the tables
fn create_tables(conn: &Connection) -> SqlResult<()> {
    // Create the person table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            age     INTEGER
        )",
        [],
    )?;

    // Create the address table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS address (
            id      INTEGER PRIMARY KEY,
            person_id INTEGER NOT NULL,
            street  TEXT NOT NULL,
            city    TEXT NOT NULL,
            state   TEXT NOT NULL,
            zip     TEXT NOT NULL,
            FOREIGN KEY(person_id) REFERENCES person(id)
        )",
        [],
    )?;

    // Create the order table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS orders (
            id      INTEGER PRIMARY KEY,
            person_id INTEGER NOT NULL,
            product TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            FOREIGN KEY(person_id) REFERENCES person(id)
        )",
        [],
    )?;

    println!("Tables created successfully.");
    Ok(())
}

// Insert initial data into the tables
fn insert_initial_data(conn: &Connection) -> SqlResult<()> {
    // Check if the 'person' table has any records
    let person_exists: SqlResult<bool> = conn.prepare("SELECT COUNT(*) > 0 FROM person")?.exists([]);

    if person_exists.unwrap_or(false) {
        println!("Initial data already exists in 'person'. Skipping insertion.");
    } else {
        let people = vec![
            Person { id: 1, name: "Alice".to_string(), age: 30 },
            Person { id: 2, name: "Bob".to_string(), age: 25 },
            Person { id: 3, name: "Charlie".to_string(), age: 28 },
        ];

        for person in &people {
            conn.execute(
                "INSERT INTO person (id, name, age) VALUES (?1, ?2, ?3)",
                params![person.id, person.name, person.age],
            )?;
        }

        println!("Initial data for 'person' inserted successfully.");
    }

    // Check if the 'address' table has any records
    let address_exists: SqlResult<bool> = conn.prepare("SELECT COUNT(*) > 0 FROM address")?.exists([]);

    if address_exists.unwrap_or(false) {
        println!("Initial data already exists in 'address'. Skipping insertion.");
    } else {
        let addresses = vec![
            Address { id: 1, person_id: 1, street: "123 Main St".to_string(), city: "Metropolis".to_string(), state: "NY".to_string(), zip: "10001".to_string() },
            Address { id: 2, person_id: 2, street: "456 Maple Ave".to_string(), city: "Gotham".to_string(), state: "NJ".to_string(), zip: "07001".to_string() },
            Address { id: 3, person_id: 3, street: "789 Oak Dr".to_string(), city: "Star City".to_string(), state: "CA".to_string(), zip: "90210".to_string() },
        ];

        for address in addresses {
            conn.execute(
                "INSERT INTO address (id, person_id, street, city, state, zip) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![address.id, address.person_id, address.street, address.city, address.state, address.zip],
            )?;
        }

        println!("Initial data for 'address' inserted successfully.");
    }

    // Check if the 'orders' table has any records
    let orders_exists: SqlResult<bool> = conn.prepare("SELECT COUNT(*) > 0 FROM orders")?.exists([]);

    if orders_exists.unwrap_or(false) {
        println!("Initial data already exists in 'orders'. Skipping insertion.");
    } else {
        let orders = vec![
            Order { id: 1, person_id: 1, product: "Laptop".to_string(), quantity: 1 },
            Order { id: 2, person_id: 1, product: "Phone".to_string(), quantity: 2 },
            Order { id: 3, person_id: 2, product: "Tablet".to_string(), quantity: 1 },
        ];

        for order in orders {
            conn.execute(
                "INSERT INTO orders (id, person_id, product, quantity) VALUES (?1, ?2, ?3, ?4)",
                params![order.id, order.person_id, order.product, order.quantity],
            )?;
        }

        println!("Initial data for 'orders' inserted successfully.");
    }

    Ok(())
}

// CRUD operations
fn perform_crud_operations(conn: &Connection) -> SqlResult<()> {
    // Create a new person (Create)
    create_person(conn, 4, "David", 32)?;

    // Create a new address for David
    create_address(conn, 4, "321 Pine St", "Central City", "IL", "62701")?;

    // Create a new order for David
    create_order(conn, 4, "Smartwatch", 1)?;

    // Read people (Read)
    let people = read_people(conn)?;
    println!("People in the database: {:?}", people);

    // Update a person's data (Update)
    update_person(conn, 4, "David Smith", 33)?;

    // Delete a person (Delete)
    delete_person(conn, 4)?;

    Ok(())
}

// CRUD functions for Person
fn create_person(conn: &Connection, id: i32, name: &str, age: i32) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO person (id, name, age) VALUES (?1, ?2, ?3)",
        params![id, name, age],
    )?;
    println!("Person {} added.", name);
    Ok(())
}

fn read_people(conn: &Connection) -> SqlResult<Vec<Person>> {
    let mut stmt = conn.prepare("SELECT id, name, age FROM person")?;
    let people_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    let mut people = Vec::new();
    for person in people_iter {
        people.push(person?);
    }

    Ok(people)
}

fn update_person(conn: &Connection, id: i32, name: &str, age: i32) -> SqlResult<()> {
    conn.execute(
        "UPDATE person SET name = ?1, age = ?2 WHERE id = ?3",
        params![name, age, id],
    )?;
    println!("Person with id {} updated.", id);
    Ok(())
}

fn delete_person(conn: &Connection, id: i32) -> SqlResult<()> {
    conn.execute("DELETE FROM person WHERE id = ?1", params![id])?;
    println!("Person with id {} deleted.", id);
    Ok(())
}

// CRUD functions for Address
fn create_address(conn: &Connection, id: i32, street: &str, city: &str, state: &str, zip: &str) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO address (id, person_id, street, city, state, zip) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, 4, street, city, state, zip], // Assuming person_id 4 for David
    )?;
    println!("Address for person id {} added.", id);
    Ok(())
}

// CRUD functions for Order
fn create_order(conn: &Connection, id: i32, product: &str, quantity: i32) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO orders (id, person_id, product, quantity) VALUES (?1, ?2, ?3, ?4)",
        params![id, 4, product, quantity], // Assuming id 4 for David
    )?;
    println!("Order for product {} added.", product);
    Ok(())
}
