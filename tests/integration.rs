use scylladb_rs::ScyllaClient;
use scylladb_rs::query::query::*;

#[tokio::test]
async fn integration_test() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = ScyllaClient::new(vec!["127.0.0.1"]).await?;
let query = client.query("test", "example_table")
    .select(&["column1", "column2"])
    .eq("column1", "value1")
    .neq("column2", "value2")
    .gt("column3", "value3")
    .gte("column4", "value4")
    .lt("column5", "value5")
    .lte("column6", "value6")
    .in_list("column7", &["value7", "value8", "value9"])
    .not_in_list("column8", &["value10", "value11"])
    .between("column9", "value12", "value13")
    .not_between("column10", "value14", "value15")
    .like("column11", "%pattern%")
    .is_null("column12")
    .is_not_null("column13")
    .order_by("column1", OrderDirection::Asc)
    .build();

    println!("{}", query);  // For demonstration purposes
    Ok(())
}