use scylladb_rs::ScyllaClient;
use scylladb_rs::query::query::*;

#[tokio::test]
async fn integration_test() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    let query = client.query("test", "example_table")
        .select(&["column1", "column2"])
        .where_condition("column1 = 'value'")
        .order_by("column1", OrderDirection::Asc)
        .build();
    
    println!("{}", query);  // For demonstration purposes
    Ok(())
}