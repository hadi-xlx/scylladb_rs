use scylladb_rs::ScyllaClient;
use scylladb_rs::QueryBuilder;
use scylla::QueryResult;
use uuid::Uuid;
use std::str::FromStr;

#[tokio::test]
async fn integration_test() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let query_builder: QueryBuilder = client.prepared_query("test", "test2_table").await;

    // Test is_null method
    let select_query5 = query_builder
        .clone()
        .delete()
        .eq("age",1)
        .build();

    let result5: QueryResult = client.session.query(select_query5, &[]).await?;
    print_query_result("Result 5", &result5);


    Ok(())
}

fn print_query_result(label: &str, result: &QueryResult) {
    println!("{}:", label);
    if let Some(rows) = &result.rows {
        for row in rows {
            println!("{:?}", row);
        }
    } else {
        println!("No rows found.");
    }
    if !result.warnings.is_empty() {
        println!("Warnings: {:?}", result.warnings);
    }
    if let Some(tracing_id) = &result.tracing_id {
        println!("Tracing ID: {:?}", tracing_id);
    }
    if let Some(paging_state) = &result.paging_state {
        println!("Paging State: {:?}", paging_state);
    }
    println!("Column Specs: {:?}", result.col_specs);
}

