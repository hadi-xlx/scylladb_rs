use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn creating_table() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    println!("Connected to ScyllaDB");

    client.create_table(
        "test_keyspace", // keyspace
        "test_table", // table
        &["age"], // partition keys
        &[], // clustering keys
        &[("age", "int"), ("name", "text"), ("score", "double")], // regular columns
        None, // Sorting
        None // time to live
    ).await?;
    
    Ok(())
}

#[tokio::test]
async fn dropping_table() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    client.drop_table(
        "test_keyspace",
        "test_table"
    ).await?;

    Ok(())
}

#[tokio::test]
async fn getting_table_columns() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let columns = client.get_table_columns("test_keyspace", "test_table").await?;

    // Serialize columns to JSON
    let json_columns = serde_json::to_string_pretty(&columns)?;

    println!("{}", json_columns);

    Ok(())
}

#[tokio::test]
async fn checking_duplicates() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    let has_duplicates = client.check_duplicates("test_keyspace", "test_table", "age").await?;
    assert!(!has_duplicates, "There should be no duplicates in the 'age' column");
    Ok(())
}

#[tokio::test]
async fn counting_rows() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    let row_count: i64 = client.count_rows("test_keyspace", "test_table").await?;
    println!("Row count: {}", row_count);
    assert!(row_count >= 0, "Row count should be non-negative");
    Ok(())
}

#[tokio::test]
async fn truncating_table() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    client.truncate_table("test_keyspace", "test_table").await?;
    println!("Table truncated successfully");
    Ok(())
}

