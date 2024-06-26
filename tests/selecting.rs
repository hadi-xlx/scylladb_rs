
use scylladb_rs::ScyllaClient;
use scylladb_rs::query::utils::print_query_result;

#[tokio::test]
async fn selecting() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    // Test is_null method
    let select_query= client
        .query("test", "test2_table")
        .select(&["name", "age"])
        .eq("age", 48)
        .execute()
        .await?;

    print_query_result("Result", &select_query);

    Ok(())
}

