
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

use serde_json::Value;
use scylla::QueryResult;

use crate::{
    QueryBuilder,
    Operations,
};

impl<'a> QueryBuilder<'a> {

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
            
            self.client
                .session
                .query(query, &[])
                .await
                .map_err(|e: scylla::transport::errors::QueryError| Box::new(e) as Box<dyn Error + Send + Sync>)
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
                    let query: String = format!(
                        "INSERT INTO {}.{} JSON '{}';",
                        self.keyspace,
                        self.table,
                        json_string
                    );
                    batch_query.push_str(&query);
                }

                batch_query.push_str(" APPLY BATCH;");
                
                self.client
                    .session
                    .query(batch_query, &[])
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
            } else {
                Err("Expected an array of JSON objects".into())
            }
        })
    }
}