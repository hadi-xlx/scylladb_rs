use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn creating_table() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    client.create_table(
        "test_keyspace", // keyspace
        "test_table", // table
        &["user_id"], // partition keys
        &["age"], // clustering keys
        &[("age", "int"), ("name", "text"), ("score", "double"), ("user_id", "int")], // regular columns
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
async fn counting_rows() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    let row_count: i64 = client.count_rows("test_keyspace", "test_table").await?;
    println!("Row count: {}", row_count);
    Ok(())
}

#[tokio::test]
async fn truncating_table() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    client.truncate_table("test_keyspace", "test_table").await?;
    Ok(())
}

