use scylladb_rs::ScyllaClient;
use scylladb_rs::query::query::*;
use serde_json::json;

#[tokio::test]
async fn integration_test() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let query_builder = client.prepared_query("test", "test_table").await;

    let select_query = query_builder
        .select(&["name", "is_active"])
        .gte("age", 30)
        .build();

    println!("select query: {}", select_query);

    let result = client.session.query(select_query, &[]).await?;

    println!("Data fetched: {:?}", result);
    Ok(())
}