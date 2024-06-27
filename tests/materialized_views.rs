use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn create_materialized_view() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    client.create_mv(
        "test_keyspace",
        "test_mv",
        "test_table",
        "age = 22",
        "id"
    ).await?;

    Ok(())
}

#[tokio::test]
async fn drop_materialized_view() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    // Drop the materialized view
    client.drop_mv(
        "test_keyspace",
        "test_mv"
    ).await?;

    Ok(())
}