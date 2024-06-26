
use crate::{
    QueryBuilder,
    Operations,
    OrderDirection
    ,InsertOptions
};
use scylla::QueryResult;
use scylla::transport::errors::QueryError;

impl<'a> QueryBuilder<'a> {

    pub async fn execute(self) -> Result<QueryResult, QueryError> {
        let query_string = self.build();
        let result = self.client.session.query(query_string, &[]).await?;
        Ok(result)
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

    pub fn add_filtering_clause(&mut self) {
        if self.operation != Operations::Delete && !self.clauses.contains(&"ALLOW FILTERING".to_string()) {
            self.clauses.push("ALLOW FILTERING".to_string());
        }
    }

    pub fn clause(mut self, clause: &str) -> Self {
        self.clauses.push(clause.to_string());
        self
    }

    pub fn order_by(mut self, column: &str, direction: OrderDirection) -> Self {
        self.order = Some((column.to_string(), direction));
        self
    }

    pub fn insert_option(mut self, option: InsertOptions) -> Self {
        self.insert_options.push(option);
        self
    }
}