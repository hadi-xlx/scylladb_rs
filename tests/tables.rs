use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn creating_table() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    println!("Connected to ScyllaDB");

    // Create a table
    client.create_table(
        "test_keyspace", //keyspace
        "test_table", //table
        &["id"], //primary keys
        &[("id", "uuid"), ("name", "text"), ("age", "int"),("score", "double")], //columns
        None, //Sorting
        None //time to live
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


