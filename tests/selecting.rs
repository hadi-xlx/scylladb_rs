use scylla::QueryResult;
use scylla::transport::errors::QueryError;

use scylladb_rs::ScyllaClient;
use scylladb_rs::query::utils::print_query_result;


#[tokio::test]
async fn selecting() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let select_query: Result<QueryResult, QueryError> = client
        .query("test", "test2_table")
        .select(&["name", "age"])
        .eq("age", 48)
        .execute()
        .await;

    match &select_query {
        Ok(query_result) => print_query_result("Result", query_result),
        Err(e) => println!("Query failed: {:?}", e),
    }

    Ok(())
}

