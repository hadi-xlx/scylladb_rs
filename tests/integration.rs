use scylladb_rs::ScyllaClient;
use scylladb_rs::query::query::*;
use serde_json::json;

#[tokio::test]
async fn integration_test() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let values = json!([
        {
            "user_id": "12345",
            "email": "new_email@example.com",
            "address": "123 Main St",
            "age": 30,
            "created_at": "2023-01-01T00:00:00Z",
            "is_active": true,
            "name": "John Doe",
            "phone_number": "555-1234"
        },
        {
            "user_id": "12346",
            "email": "another_email@example.com",
            "address": "456 Elm St",
            "age": 25,
            "created_at": "2023-01-02T00:00:00Z",
            "is_active": false,
            "name": "Jane Smith",
            "phone_number": "555-5678"
        },
        {
            "user_id": "12347",
            "email": "third_email@example.com",
            "address": "789 Oak St",
            "age": 40,
            "created_at": "2023-01-03T00:00:00Z",
            "is_active": true,
            "name": "Alice Johnson",
            "phone_number": "555-9012"
        },
        {
            "user_id": "12348",
            "email": "fourth_email@example.com",
            "address": "101 Pine St",
            "age": 35,
            "created_at": "2023-01-04T00:00:00Z",
            "is_active": false,
            "name": "Bob Brown",
            "phone_number": "555-3456"
        }
    ]);

    let query_builder = client.prepared_query("test", "test_table").await;  // Await the future
    let bulk_insert_result = query_builder.insert_bulk(values).await?;

    println!("Bulk insert executed successfully: {:?}", bulk_insert_result);

    Ok(())
}