use std::error::Error;

use serde_json::json;

use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn inserting() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let json_body = json!({
        "user_id": 3,
        "age": 23,
        "name": "Bro Doe the third",
        "score": 31.6,
        "is_active": true
    });


    let insert: Result<scylla::QueryResult, Box<dyn Error + Sync + Send>> = client
        .query("test_keyspace", "test_table")
        .insert(json_body)
        .await;

    match &insert {
        Ok(query_result) => println!("Query Successful: {:?}", query_result),
        Err(e) => println!("Query failed: {:?}", e),
    }

    Ok(())
}


