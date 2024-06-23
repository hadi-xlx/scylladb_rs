use scylladb_rs::ScyllaClient;
use scylladb_rs::query::query::*;
use serde_json::json;

#[tokio::test]
async fn integration_test() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let table_result = client.create_table(
        "test", 
        "test_table", 
        &["user_id", "email"],  // Updated primary keys
        &[
            ("user_id", "text"), 
            ("age", "int"), 
            ("email", "text"), 
            ("name", "text"), 
            ("created_at", "timestamp"), 
            ("address", "text"), 
            ("phone_number", "text"), 
            ("is_active", "boolean")
        ],  // Added more columns
        None, 
        None
    ).await?;

    let values = json!({
        "user_id": "12345",
        "email": "new_email@example.com",
        "age": 30  // Corrected to integer
    });

    let query_builder = client.prepared_query("test", "test_table").await;  // Await the future
    let query_result = query_builder.insert(values).await?;

    println!("Query executed successfully: {:?}", query_result);

    Ok(())
}