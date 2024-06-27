use std::fmt::Display;
use uuid::Uuid;
use scylla::QueryResult;

pub fn format_value<T: Display>(value: T) -> String {
    let value_str = value.to_string();
    if value_str.parse::<Uuid>().is_ok() {
        value_str
    } else if value_str.parse::<i64>().is_ok() {
        value_str
    } else if value_str.parse::<f64>().is_ok() {
        value_str
    } else if value_str == "true" {
        "True".to_string()
    } else if value_str == "false" {
        "False".to_string()
    } else {
        format!("'{}'", value_str)
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

