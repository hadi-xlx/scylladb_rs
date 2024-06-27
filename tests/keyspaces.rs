use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn createing_keyspace() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    // Create a keyspace
    client.create_keyspace_simple("test_keyspace", 1).await?;

    Ok(())
}

#[tokio::test]
async fn dropping_keyspace() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    // Drop the keyspace
    client.drop_keyspace("test_keyspace").await?;

    Ok(())
}

#[tokio::test]
async fn getting_keyspace_tables() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let tables: serde_json::Value = client.get_keyspace_tables("test_keyspace").await?;

    println!("{:?}", tables);

    Ok(())
}

#[tokio::test]
async fn getting_keyspace_materialized_views() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let tables: serde_json::Value = client.get_keyspace_materialized_views("test_keyspace").await?;

    println!("{:?}", tables);

    Ok(())
}
