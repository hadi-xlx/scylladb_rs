use std::fmt::Display;
use uuid::Uuid;
use scylla::QueryResult;

pub fn format_value<T: Display>(value: T) -> String {
    if value.to_string().parse::<Uuid>().is_ok() {
        value.to_string()
    } else if value.to_string().parse::<i64>().is_ok() {
        value.to_string()
    } else if value.to_string() == "true" {
        "True".to_string()
    } else if value.to_string() == "false" {
        "False".to_string()
    } else {
        format!("'{}'", value.to_string())
    }
}

pub fn print_query_result(label: &str, result: &QueryResult) {
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

