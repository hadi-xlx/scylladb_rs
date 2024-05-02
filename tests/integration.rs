use scylladb_rs::{Table, Keyspace, MaterializedView};
use scylla::{Session, SessionBuilder};

#[tokio::test]
async fn integration_test() {
    let session: Session = SessionBuilder::new()
        .known_node("localhost:9042")
        .build()
        .await
        .unwrap(); // Using unwrap here for simplicity, as the session creation is critical and not part of the test logic

    let keyspace: Keyspace = Keyspace::new(
        "scylladb_rs_test_keyspace".to_string(),
        &session
    );

    let replication_strategy = "{ 'class' : 'SimpleStrategy', 'replication_factor' : 1 }";
    match keyspace.create(replication_strategy).await {
        Ok(_) => println!("Keyspace created successfully."),
        Err(e) => println!("Failed to create keyspace: {}", e),
    }

    let table = Table::new(
        &keyspace, 
        "test_table".to_string()
    );

    let columns = [
        ("user_id", "UUID"), 
    ];
    
    let primary_keys = ["user_id"];
    
    // Create table
    match table.create(&primary_keys, &columns).await {
        Ok(_) => println!("Table created successfully."),
        Err(e) => println!("Failed to create table: {}", e),
    }

    match table.create_column("user_name", "text").await {
        Ok(_) => println!("Column added successfully."),
        Err(e) => println!("Failed to add column: {}", e),
    }

    // Create materialized view
    let materialized_view: MaterializedView = MaterializedView::new(
        &table, 
        "mv_test_table".to_string(),
        "user_id".to_string(),  // Include all primary key columns
        "user_id IS NOT NULL".to_string()  // Ensure all primary keys and the column of interest are not null
    ).await;

    match materialized_view.create_materialized_view().await {
        Ok(_) => println!("Materialized view created successfully."),
        Err(e) => println!("Failed to create materialized view: {}", e),
    }

    // Drop materialized view
    match materialized_view.drop_materialized_view().await {
        Ok(_) => println!("Materialized view dropped successfully."),
        Err(e) => println!("Failed to drop materialized view: {}", e),
    }

    // Drop index
    match table.drop_index("test_index").await {
        Ok(_) => println!("Index dropped successfully."),
        Err(e) => println!("Failed to drop index: {}", e),
    }

    // Delete column
    match table.delete_column("user_name").await {
        Ok(_) => println!("Column deleted successfully."),
        Err(e) => println!("Failed to delete column: {}", e),
    }

    // Truncate table
    match table.truncate().await {
        Ok(_) => println!("Table truncated successfully."),
        Err(e) => println!("Failed to truncate table: {}", e),
    }

    // Drop table
    match table.drop().await {
        Ok(_) => println!("Table dropped successfully."),
        Err(e) => println!("Failed to drop table: {}", e),
    }

    // Drop the keyspace
    match keyspace.drop().await {
        Ok(_) => println!("Keyspace dropped successfully."),
        Err(e) => println!("Failed to drop keyspace: {}", e),
    }
}