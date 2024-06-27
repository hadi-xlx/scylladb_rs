use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn create_keyspace() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    // Create a keyspace
    client.create_ks_simple("test_keyspace", 1).await?;

    Ok(())
}

#[tokio::test]
async fn drop_keyspace() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    // Drop the keyspace
    client.drop_ks("test_keyspace").await?;

    Ok(())
}


