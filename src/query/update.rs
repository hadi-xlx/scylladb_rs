
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::fmt::Display;

use serde_json::Value;
use scylla::QueryResult;

use crate::{
    QueryBuilder,
    Operations,
};

use crate::query::utils::format_value;

impl<'a> QueryBuilder<'a> {
    
    pub fn update<T: Display + Send + 'a>(
        mut self,
        primary_key_name: &'a str,
        primary_key_value: T,
        json_body: Value
    ) -> Pin<Box<dyn Future<Output = Result<QueryResult, Box<dyn Error + Send + Sync>>> + Send + 'a>> {
        Box::pin(async move {
            self.operation = Operations::Update;
            if let Some(map) = json_body.as_object() {
                let set_clause: String = map
                    .iter()
                    .map(|(col, val)| match val {
                        Value::String(s) => format!("{} = '{}'", col, s),
                        Value::Number(n) => format!("{} = {}", col, n),
                        Value::Bool(b) => format!("{} = {}", col, b),
                        _ => format!("{} = '{}'", col, val) // Fallback for other types
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                
                let query = format!(
                    "UPDATE {}.{} SET {} WHERE {} = {};",
                    self.keyspace,
                    self.table,
                    set_clause,
                    primary_key_name,
                    format_value(primary_key_value)
                );
                
                self.client
                    .session
                    .query(query, &[])
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
            } else {
                Err("Expected a JSON object".into())
            }
        })
    }
}