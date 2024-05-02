use scylladb_rs::{Keyspace, Table};
use scylla::{Session, SessionBuilder};

#[tokio::test]
async fn integration_test() {
    // Setup: Create a session and a Keyspace instance
    let session: Session = SessionBuilder::new()
        .known_node("localhost:9042")
        .build()
        .await
        .expect("Session should be created");

    let keyspace: Keyspace = Keyspace::new("scylladb_rs_test_keyspace".to_string(), &session);
    let table: Table = Table::new(
        "test_table".to_string(),
        "scylladb_rs_test_keyspace".to_string(),
        &session);

    // Test create_keyspace
    match keyspace.create_keyspace("{'class': 'SimpleStrategy', 'replication_factor': 1}").await {
        Ok(_) => println!("Keyspace created successfully."),
        Err(e) => println!("Failed to create keyspace: {}", e),
    }

    // Test create_table
    let table_schema = "id UUID PRIMARY KEY, name TEXT";
    match keyspace.create_table("test_table", table_schema).await {
        Ok(_) => println!("Table created successfully."),
        Err(e) => println!("Failed to create table: {}", e),
    }

    // Test create_index using Table struct
    match table.create_index("name_idx", "name").await {
        Ok(_) => println!("Index created successfully."),
        Err(e) => println!("Failed to create index: {}", e),
    }

    // Test create_materialized_view
    let view_schema: &str = "id, name";
    let view_primary_key: &str = "(id, name)"; // Composite primary key including 'name'
    let view_condition: &str = "name IS NOT NULL";
    match keyspace.create_materialized_view(
        "test_view", "test_table", view_schema, view_primary_key, view_condition
    ).await {
        Ok(_) => println!("Materialized view created successfully."),
        Err(e) => println!("Failed to create materialized view: {}", e),
    }

    // Test create_column
    match table.create_column("email", "TEXT").await {
        Ok(_) => println!("Column 'email' added successfully."),
        Err(e) => println!("Failed to add column 'email': {}", e),
    }

    // Test delete_column
    match table.delete_column("email").await {
        Ok(_) => println!("Column 'email' deleted successfully."),
        Err(e) => println!("Failed to delete column 'email': {}", e),
    }


    // Test drop_materialized_view
    match keyspace.drop_materialized_view("test_view").await {
        Ok(_) => println!("Materialized view dropped successfully."),
        Err(e) => println!("Failed to drop materialized view: {}", e),
    }

    // Test drop_index using Table struct
    match table.drop_index("name_idx").await {
        Ok(_) => println!("Index dropped successfully."),
        Err(e) => println!("Failed to drop index: {}", e),
    }

    // Test drop_table
    match keyspace.drop_table("test_table").await {
        Ok(_) => println!("Table dropped successfully."),
        Err(e) => println!("Failed to drop table: {}", e),
    }

    // Test drop_keyspace
    match keyspace.drop_keyspace().await {
        Ok(_) => println!("Keyspace dropped successfully."),
        Err(e) => println!("Failed to drop keyspace: {}", e),
    }

}