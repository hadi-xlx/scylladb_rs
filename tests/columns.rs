use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn creating_column() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    println!("Connected to ScyllaDB");


    // Add a column
    client.create_column(
        "test_keyspace",
        "test_table",
        "email",
        "text"
    ).await?;

    Ok(())
}


#[tokio::test]
async fn dropping_column() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    // Drop the column
    client.drop_column(
        "test_keyspace",
        "test_table",
        "age"
    ).await?;


    Ok(())
}


