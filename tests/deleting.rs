use scylla::QueryResult;
use scylla::transport::errors::QueryError;

use scylladb_rs::ScyllaClient;
use scylladb_rs::query::utils::print_query_result;

#[tokio::test]
async fn deleting() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let delete_query_1: Result<QueryResult, QueryError> = client
        .query("test_keyspace", "test_table")
        .delete()
        .eq("age", 22)
        .execute()
        .await;

    match &delete_query_1 {
        Ok(query_result) => print_query_result("Query 1:", query_result),
        Err(e) => println!("Query 1failed: {:?}", e),
    }

    let delete_query_2: Result<QueryResult, QueryError> = client
        .query("test", "test2_table")
        .delete()
        .in_list("id", &vec![22, 55])
        .execute()
        .await;

    match &delete_query_2 {
        Ok(query_result) => print_query_result("Query 2:", query_result),
        Err(e) => println!("Query 2 failed: {:?}", e),
    }


    Ok(())
}

