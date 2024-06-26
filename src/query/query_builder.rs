
use crate::{
    ScyllaClient,
    QueryBuilder,
    Operations,
    OrderDirection,
    InsertOptions
};
use scylla::QueryResult;
use scylla::transport::errors::QueryError;

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
            allow_filtering: false,
        }
    }

    pub fn allow_filtering(mut self) -> Self {
        self.allow_filtering = true;
        self
    }

    pub async fn execute(self) -> Result<QueryResult, QueryError> {
        let query_string: String = self.build();
        let result: QueryResult = self.client.session.query(query_string, &[]).await?;
        Ok(result)
    }
    
    pub fn build(&self) -> String {
        let operation: &str = match self.operation {
            Operations::Select => "SELECT",
            Operations::Insert => "INSERT INTO",
            Operations::Update => "UPDATE",
            Operations::Delete => "DELETE",
        };
    
        let columns: String = if self.columns.is_empty() {
            if self.operation == Operations::Delete {
                "".to_string()
            } else {
                "*".to_string()
            }
        } else {
            self.columns.join(", ")
        };
    
        let full_table_name: String = format!("{}.{}", self.keyspace, self.table);
        let mut query = match self.operation {
            Operations::Select => format!("{} {} FROM {}", operation, columns, full_table_name),
            Operations::Delete => format!("{} FROM {}", operation, full_table_name),
            Operations::Insert => {
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
    
        // Add ALLOW FILTERING if allow_filtering is true
        if self.allow_filtering {
            query.push_str(" ALLOW FILTERING");
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
        query
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