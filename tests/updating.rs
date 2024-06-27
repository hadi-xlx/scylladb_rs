use std::collections::HashMap;

use serde_json::json;

use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn updating() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let json_body = json!(
        {
            "score": 66.0
        }
    );

    let mut primary_keys: HashMap<&str, i32> = HashMap::new();
    primary_keys.insert("age", 44);
    primary_keys.insert("user_id", 2);

    let update = client
        .query("test_keyspace", "test_table")
        .update(
            primary_keys,
            json_body
        )
        .await;

    match &update {
        Ok(query_result) => println!("Query Succecsfull: {:?}", query_result),
        Err(e) => println!("Query 1 failed: {:?}", e),
    }

    Ok(())
}