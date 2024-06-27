use std::error::Error;

use serde_json::json;

use scylladb_rs::ScyllaClient;
use scylladb_rs::query::utils::print_query_result;

#[tokio::test]
async fn inserting() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let json_body = json!({
        "age": 44,
        "name": "Jane Doe the third",
        "score": 75.8
    });


    let insert: Result<scylla::QueryResult, Box<dyn Error + Sync + Send>> = client
        .query("test_keyspace", "test_table")
        .insert(json_body)
        .await;

    match &insert {
        Ok(query_result) => print_query_result("Query:", query_result),
        Err(e) => println!("Query failed: {:?}", e),
    }


    Ok(())
}

