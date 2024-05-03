/*use crate::ScyllaClient;
use std::error::Error;
use std::collections::HashMap;
use scylla::prepared_statement::PreparedStatement;

pub enum Operator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals,
}

pub enum SortOrder {
    Ascending,
    Descending,
}

pub struct Filter {
    column: String,
    operator: Operator,
    value: String,
}

pub struct Sort {
    column: String,
    order: SortOrder,
}


pub struct QueryBuilder<'a> {
    client: &'a ScyllaClient<'a>,
    table_name: String,
    filters: Vec<Filter>,
    sorts: Vec<Sort>,
    prepared_statement: Option<PreparedStatement>,
}

impl<'a> QueryBuilder<'a> {
    pub fn new(client: &'a ScyllaClient, table_name: &str) -> Self {
        Self {
            client,
            table_name: table_name.to_owned(),
            filters: vec![],
            sorts: vec![],
            prepared_statement: None,
        }
    }

    pub fn add_filter(&mut self, filter: Filter) -> &mut Self {
        self.filters.push(filter);
        self
    }

    pub fn add_sort(&mut self, sort: Sort) -> &mut Self {
        self.sorts.push(sort);
        self
    }

    // Method to prepare the query
    pub async fn prepare(&mut self) -> Result<(), Box<dyn Error>> {
        let mut query = format!("SELECT * FROM {}", self.table_name);

        let mut params = vec![];
        if !self.filters.is_empty() {
            let filter_clauses = self.filters.iter().map(|f| {
                let operator = match f.operator {
                    Operator::Equals => "=",
                    Operator::NotEquals => "!=",
                    Operator::GreaterThan => ">",
                    Operator::LessThan => "<",
                    Operator::GreaterThanOrEquals => ">=",
                    Operator::LessThanOrEquals => "<=",
                };
                params.push(f.value.clone());
                format!("{} {} ?", f.column, operator)
            }).collect::<Vec<String>>().join(" AND ");
            query.push_str(" WHERE ");
            query.push_str(&filter_clauses);
        }

        if !self.sorts.is_empty() {
            let sort_clauses = self.sorts.iter().map(|s| {
                let order = match s.order {
                    SortOrder::Ascending => "ASC",
                    SortOrder::Descending => "DESC",
                };
                format!("{} {}", s.column, order)
            }).collect::<Vec<String>>().join(", ");
            query.push_str(" ORDER BY ");
            query.push_str(&sort_clauses);
        }

        self.prepared_statement = Some(self.client.session.prepare(query).await?);
        Ok(())
    }

    // Method to execute the prepared query
    pub async fn execute(&self) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
        let statement = self.prepared_statement.as_ref().ok_or("Query not prepared")?;
        let bound_statement = statement(self.filters.iter().map(|f| &f.value as &dyn scylla::Value).collect::<Vec<_>>());

        let rows = self.client.session.execute(bound_statement).await?;
        let mut results = vec![];

        if let Some(rows) = rows.rows {
            for row in rows {
                let mut result = HashMap::new();
                for (index, column_value) in row.columns.iter().enumerate() {
                    let column_name = self.client.session.get_column_spec(index)?.name.clone();
                    if let Some(value) = column_value {
                        let value_str = match value.as_text() {
                            Ok(text) => text.unwrap_or_default(),
                            Err(_) => continue,
                        };
                        result.insert(column_name, value_str);
                    }
                }
                results.push(result);
            }
        }

        Ok(results)
    }
}*/