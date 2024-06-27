use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn creating_indexing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    match client.create_index(
        "test_keyspace",
        "test_table",
        "score_index",
        "score"
    ).await {
        Ok(_) => println!("Index created successfully"),
        Err(e) => println!("Failed to create index: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn dropping_indexing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    client.drop_index("test_keyspace", "test_index").await?;
    Ok(())
}