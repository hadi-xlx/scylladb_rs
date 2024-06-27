
use serde_json::json;

use scylladb_rs::ScyllaClient;
use scylladb_rs::query::utils::print_query_result;

#[tokio::test]
async fn updating() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let json_body = json!(
        {
            "id" : "ee8b1a2e-f874-4011-8021-7bad54578efd",
            "address": "123 Main St",
            "age": 33,
            "created_at": "2023-10-01T12:00:00Z",
            "email": "right@example.com",
            "is_active": false,
            "name": "Johnny Doe the first",
            "phone_number": "123-456-7890",
            "user_id": "user_123"
        }
    );

    let update = client
        .query("test", "test2_table")
        .update(json_body) 
        .await;

    match &update {
        Ok(query_result) => print_query_result("Query 1:", query_result),
        Err(e) => println!("Query 1 failed: {:?}", e),
    }

    Ok(())
}