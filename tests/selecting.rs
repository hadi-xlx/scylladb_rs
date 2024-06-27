use scylla::QueryResult;
use scylla::transport::errors::QueryError;

use scylladb_rs::ScyllaClient;
use scylladb_rs::query::utils::print_query_result;

#[tokio::test]
async fn test_select_query_1() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    // Here we use ALLOW FILTERING because we did't provide the complete key values
    let select_query_1: Result<QueryResult, QueryError> = client
        .query("test_keyspace", "test_table")
        .select(&["name"])
        .eq("age", 33)
        .allow_filtering() 
        .execute()
        .await;

    match &select_query_1 {
        Ok(query_result) => print_query_result("Query 1:", query_result),
        Err(e) => println!("Query 1 failed: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_select_query_2() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    //here we use ALlow filtering because only EQ and IN relation are supported on partition keys
    let select_query_2: Result<QueryResult, QueryError> = client
        .query("test_keyspace", "test_table")
        .select(&["name", "age"])
        .eq("user_id", 2)
        .between("age", 20, 50)
        .like("name", "Jane%") // you may also do "%Jane%" or just "Jane"
        .allow_filtering()
        .execute()
        .await;

    match &select_query_2 {
        Ok(query_result) => print_query_result("Query 2:", query_result),
        Err(e) => println!("Query 2 failed: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
async fn test_select_query_3() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    // here we don't use ALLOW FILTERING because we provided the complete key values
    // and all operations are suppotred for the partition keys
    let select_query_3: Result<QueryResult, QueryError> = client
        .query("test_keyspace", "test_table")
        .select(&["name"])
        .eq("user_id", 2)
        .in_list("age", &vec![28, 44, 40]) // the vec can also have strings
        .execute()
        .await;

    match &select_query_3 {
        Ok(query_result) => print_query_result("Query 3:", query_result),
        Err(e) => println!("Query 3 failed: {:?}", e),
    }

    Ok(())
}