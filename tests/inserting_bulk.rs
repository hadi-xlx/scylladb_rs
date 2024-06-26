
use serde_json::json;

use scylladb_rs::ScyllaClient;
use scylladb_rs::query::utils::print_query_result;

use uuid::Uuid;

#[tokio::test]
async fn inserting_bulk() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    let json_body = json!([
        {
            "id": Uuid::new_v4().to_string(),
            "address": "123 Main St",
            "age": 33,
            "created_at": "2023-10-01T12:00:00Z",
            "email": "right@example.com",
            "is_active": false,
            "name": "Johnny Doe the first",
            "phone_number": "123-456-7890",
            "user_id": "user_123"
        },
        {
            "id": Uuid::new_v4().to_string(),
            "address": "456 Elm St",
            "age": 29,
            "created_at": "2023-10-02T12:00:00Z",
            "email": "leftistcat@example.com",
            "is_active": true,
            "name": "Jane Doe gangster",
            "phone_number": "987-654-3210",
            "user_id": "user_456"
        }
        // Add more JSON objects as needed
    ]);


    let insert_bulk = client
        .query("test", "test2_table")
        .insert_bulk(json_body)
        .await;

    match &insert_bulk {
        Ok(query_result) => print_query_result("Query 1:", query_result),
        Err(e) => println!("Query 1 failed: {:?}", e),
    }


    Ok(())
}

