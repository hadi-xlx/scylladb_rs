
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::fmt::Display;

use serde_json::Value;
use scylla::QueryResult;

use crate::{
    QueryBuilder,
    Operations,
    ScyllaClient
};
use crate::query::utils::format_value;
impl<'a> QueryBuilder<'a> {
    
    pub fn new(
        operation: Operations,
        keyspace: &str,
        table: &str,
        client: &'a ScyllaClient
    ) -> Self {
        Self {
            operation,
            keyspace: keyspace.to_string(),
            table: table.to_string(),
            columns: Vec::new(),
            conditions: Vec::new(),
            clauses: Vec::new(),
            order: None,
            insert_options: Vec::new(),
            client,
        }
    }
    
    pub fn select(
        mut self,
        columns: &[&str]
    ) -> Self {
        self.columns = columns.iter().map(|&col| col.to_string()).collect();
        self
    }

    pub fn delete(mut self) -> Self {
        self.operation = Operations::Delete;
        self
    }


    pub fn update<T: Display + Send + 'a>(
        mut self,
        primary_key_name: &'a str,
        primary_key_value: T,
        json_body: Value
    ) -> Pin<Box<dyn Future<Output = Result<QueryResult, Box<dyn Error + Send + Sync>>> + Send + 'a>> {
        Box::pin(async move {
            self.operation = Operations::Update;
            if let Some(map) = json_body.as_object() {
                let set_clause: String = map.iter()
                    .map(|(col, val)| {
                        match val {
                            Value::String(s) => format!("{} = '{}'", col, s),
                            Value::Number(n) => format!("{} = {}", col, n),
                            Value::Bool(b) => format!("{} = {}", col, b),
                            _ => format!("{} = '{}'", col, val) // Fallback for other types
                        }
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
                self.client.session.query(query, &[]).await.map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
            } else {
                Err("Expected a JSON object".into())
            }
        })
    }


    pub fn insert<'b>(
        mut self,
        json_body: Value
    ) -> Pin<Box<dyn Future<Output = Result<QueryResult, Box<dyn Error + Send + Sync>>> + Send + 'b>>
    where 'a: 'b {
        Box::pin(async move {
            self.operation = Operations::Insert;
            let json_string: String = json_body.to_string();
            self.clauses.push(format!("JSON '{}'", json_string));
            let query = self.build();
            self.client.session.query(query, &[]).await.map_err(|e: scylla::transport::errors::QueryError| Box::new(e) as Box<dyn Error + Send + Sync>)
        })
    }

    pub fn insert_bulk<'b>(
        mut self,
        json_body: Value
    ) -> Pin<Box<dyn Future<Output = Result<QueryResult, Box<dyn Error + Send + Sync>>> + Send + 'b>>
    where 'a: 'b {
        Box::pin(async move {
            if let Some(records) = json_body.as_array() {
                self.operation = Operations::Insert;
                let mut batch_query = String::from("BEGIN BATCH ");

                for record in records {
                    let json_string: String = record.to_string();
                    let query: String = format!("INSERT INTO {}.{} JSON '{}';", self.keyspace, self.table, json_string);
                    batch_query.push_str(&query);
                }

                batch_query.push_str(" APPLY BATCH;");
                self.client.session.query(batch_query, &[]).await.map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
            } else {
                Err("Expected an array of JSON objects".into())
            }
        })
    }
}