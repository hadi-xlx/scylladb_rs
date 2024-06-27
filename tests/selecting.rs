use scylla::QueryResult;
use scylla::transport::errors::QueryError;

use scylladb_rs::ScyllaClient;
use scylladb_rs::query::utils::print_query_result;


#[tokio::test]
async fn selecting() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let select_query_1: Result<QueryResult, QueryError> = client
        .query("test_keyspace", "test_table")
        .select(&["name", "score"])
        .eq("age", 33)
        .execute()
        .await;

    match &select_query_1 {
        Ok(query_result) => print_query_result("Query 1:", query_result),
        Err(e) => println!("Query 1 failed: {:?}", e),
    }

    let select_query_2: Result<QueryResult, QueryError> = client
        .query("test_keyspace", "test_table")
        .select(&["name", "age"])
        .between("age", 20,50)
        .like("name","Jane%") // you may also do "%Jane%" or just "Jane"
        .execute()
        .await;

    match &select_query_2 {
        Ok(query_result) => print_query_result("Query 2:", query_result),
        Err(e) => println!("Query 2 failed: {:?}", e),
    }


    let select_query_3: Result<QueryResult, QueryError> = client
    .query("test_keyspace", "test_table")
    .select(&["name"])
    .in_list("age",&vec![28,22,40]) // the list/vec can also have strings
    .execute()
    .await;

    match &select_query_3 {
        Ok(query_result) => print_query_result("Query 2:", query_result),
        Err(e) => println!("Query 3 failed: {:?}", e),
    }

    Ok(())
}

