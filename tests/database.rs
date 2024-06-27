use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn databasing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let _client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    println!("Connected to ScyllaDB");
    Ok(())
}


