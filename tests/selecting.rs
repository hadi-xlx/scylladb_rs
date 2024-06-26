use scylla::QueryResult;
use scylla::transport::errors::QueryError;

use scylladb_rs::ScyllaClient;
use scylladb_rs::query::utils::print_query_result;


#[tokio::test]
async fn selecting() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let select_query_1: Result<QueryResult, QueryError> = client
        .query("test", "test2_table")
        .select(&["name", "phone_number"])
        .eq("age", 48)
        .execute()
        .await;

    match &select_query_1 {
        Ok(query_result) => print_query_result("Query 1:", query_result),
        Err(e) => println!("Query 1 failed: {:?}", e),
    }

    let select_query_2: Result<QueryResult, QueryError> = client
        .query("test", "test2_table")
        .select(&["name", "age"])
        .between("age", 20,40)
        .like("email","jane%") // you may also do "%jane%" or just "jane"
        .execute()
        .await;

    match &select_query_2 {
        Ok(query_result) => print_query_result("Query 2:", query_result),
        Err(e) => println!("Query 2 failed: {:?}", e),
    }


    let select_query_3: Result<QueryResult, QueryError> = client
    .query("test", "test2_table")
    .select(&["name", "user_id"])
    .in_list("age",&vec![28,22,40]) // the list/vec can also have strings
    .execute()
    .await;

    match &select_query_3 {
        Ok(query_result) => print_query_result("Query 2:", query_result),
        Err(e) => println!("Query 3 failed: {:?}", e),
    }

    Ok(())
}

