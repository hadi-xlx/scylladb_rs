use serde_json::Value;
use crate::query::query::*;
use std::collections::HashMap;
use crate::ScyllaClient;
use scylla::QueryResult;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::fmt::Display;
use uuid::Uuid;
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

    pub fn update(
        mut self,
        values: HashMap<&str,&str>
    ) -> Self {
        self.operation = Operations::Update;
        self.columns = values.into_iter()
            .map(|(col, val)| format!("{} = '{}'", col, val))
            .collect();
        self
    }

    pub fn insert<'b>(
        mut self,
        json_body: Value
    ) -> Pin<Box<dyn Future<Output = Result<QueryResult, Box<dyn Error + Send + Sync>>> + Send + 'b>>
    where 'a: 'b {
        Box::pin(async move {
            self.operation = Operations::Insert;
            let json_string = json_body.to_string();
            self.clauses.push(format!("JSON '{}'", json_string));
            let query = self.build();
            self.client.session.query(query, &[]).await.map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
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
                    let json_string = record.to_string();
                    let query = format!("INSERT INTO {}.{} JSON '{}';", self.keyspace, self.table, json_string);
                    batch_query.push_str(&query);
                }

                batch_query.push_str(" APPLY BATCH;");
                self.client.session.query(batch_query, &[]).await.map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
            } else {
                Err("Expected an array of JSON objects".into())
            }
        })
    }

    //works
    pub fn eq<T: Display>(
        mut self,
        column: &str,
        value: T
    ) -> Self {
        let condition = format!("{} = {}", column, format_value(value));
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //doesnt work 
    pub fn neq<T: Display>(
        mut self,
        column: &str,
        value: T
    ) -> Self {
        let condition = format!("{} != {}", column, format_value(value));
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn gt<T: Display>(
        mut self,
        column: &str,
        value: T
    ) -> Self {
        let condition = format!("{} > {}", column, format_value(value));
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn gte<T: Display>(
        mut self,
        column: &str,
        value: T
    ) -> Self {
        let condition = format!("{} >= {}", column, format_value(value));
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn lt<T: Display>(
        mut self,
        column: &str,
        value: T
    ) -> Self {
        let condition = format!("{} < {}", column, format_value(value));
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn lte<T: Display>(
        mut self,
        column: &str,
        value: T
    ) -> Self {
        let condition = format!("{} <= {}", column, format_value(value));
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn in_list<T: Display>(
        mut self,
        column: &str,
        values: &[T]
    ) -> Self {
        let value_list = values.iter().map(|v| format_value(v)).collect::<Vec<_>>().join(", ");
        let condition = format!("{} IN ({})", column, value_list);
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn between<T: Display>(
        mut self,
        column: &str,
        lower: T,
        upper: T
    ) -> Self {
        let lower_condition = format!("{} > {}", column, format_value(lower));
        let upper_condition = format!("{} < {}", column, format_value(upper));
        self.conditions.push(lower_condition);
        self.conditions.push(upper_condition);
        self.add_filtering_clause();
        self
    }

    //doesnt work, OR isnt supported maybe
    pub fn not_between<T: Display>(
        mut self,
        column: &str,
        lower: T,
        upper: T
    ) -> Self {
        let condition = format!("{} < {} or {} > {}", column, format_value(lower), column, format_value(upper));
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn like(
        mut self,
        column: &str,
        pattern: &str
    ) -> Self {
        let condition = format!("{} LIKE '{}'", column, pattern);
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }
    
    pub fn build(&self) -> String {
        let operation = match self.operation {
            Operations::Select => "SELECT",
            Operations::Insert => "INSERT INTO",
            Operations::InsertIfNotExists => "INSERT IF NOT EXISTS",
            Operations::Update => "UPDATE",
            Operations::Delete => "DELETE",
        };

        let columns = if self.columns.is_empty() {
            if self.operation == Operations::Delete {
                "".to_string()
            } else {
                "*".to_string()
            }
        } else {
            self.columns.join(", ")
        };

        let full_table_name = format!("{}.{}", self.keyspace, self.table);
        let mut query = match self.operation {
            Operations::Select => format!("{} {} FROM {}", operation, columns, full_table_name),
            Operations::Delete => format!("{} FROM {}", operation, full_table_name),
            Operations::Insert | Operations::InsertIfNotExists => {
                if !self.clauses.is_empty() && self.clauses[0].starts_with("JSON") {
                    format!("{} {} {}", operation, full_table_name, self.clauses.join(" "))
                } else {
                    format!("{} {}", operation, full_table_name)
                }
            },
            Operations::Update => format!("{} {}", operation, full_table_name),
        };

        if self.operation == Operations::Update && !self.columns.is_empty() {
            query.push_str(" SET ");
            query.push_str(&columns);
        }

        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }

        if let Some((ref col, ref dir)) = self.order {
            let dir_str = match dir {
                OrderDirection::Asc => "ASC",
                OrderDirection::Desc => "DESC",
            };
            query.push_str(&format!(" ORDER BY {} {}", col, dir_str));
        }

        if !self.clauses.is_empty() && !self.clauses[0].starts_with("JSON") {
            query.push_str(" ");
            query.push_str(&self.clauses.join(" "));
        }

        if !self.insert_options.is_empty() {
            query.push_str(" USING ");
            let options: Vec<String> = self.insert_options.iter().map(|option| {
                match option {
                    InsertOptions::UsingTimestamp(ts) => format!("TIMESTAMP {}", ts),
                    InsertOptions::UsingTTL(ttl) => format!("TTL {}", ttl),
                }
            }).collect();
            query.push_str(&options.join(" AND "));
        }

        query.push(';');
        println!("\nquery: {}\n", query);
        query
    }

    fn add_filtering_clause(&mut self) {
        if self.operation != Operations::Delete && !self.clauses.contains(&"ALLOW FILTERING".to_string()) {
            self.clauses.push("ALLOW FILTERING".to_string());
        }
    }

    pub async fn execute(
        &self,
        query: String
    ) -> Result<QueryResult, Box<dyn Error + Send + Sync>> {
        self.client.session.query(query, &[]).await.map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
    }

    pub fn clause(
        mut self,
        clause: &str
    ) -> Self {
        self.clauses.push(clause.to_string());
        self
    }

    pub fn order_by(
        mut self,
        column: &str,
        direction: OrderDirection
    ) -> Self {
        self.order = Some((column.to_string(), direction));
        self
    }

    pub fn insert_option(
        mut self,
        option: InsertOptions
    ) -> Self {
        self.insert_options.push(option);
        self
    }
}

fn format_value<T: Display>(value: T) -> String {
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
