use scylla::QueryResult;
use scylla::transport::errors::QueryError;

use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn deleting() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let delete_query: Result<QueryResult, QueryError> = client
        .query("test_keyspace", "test_table")
        .delete()
        .eq("user_id", 3)
        .eq("age", 44)
        .execute()
        .await;

    match &delete_query {
        Ok(query_result) => println!("Query Succecsfull: {:?}", query_result),
        Err(e) => println!("Query 1failed: {:?}", e),
    }
    Ok(())
}

#[tokio::test]
async fn in_list_deleting() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    let delete_query: Result<QueryResult, QueryError> = client
        .query("test_keyspace", "test_table")
        .delete()
        .in_list("age", &vec![22, 55])
        .in_list("user_id", &vec![11, 22])
        .execute()
        .await;

    match &delete_query {
        Ok(query_result) => println!("Query Succecsfull: {:?}", query_result),
        Err(e) => println!("Query failed: {:?}", e),
    }


    Ok(())
}
