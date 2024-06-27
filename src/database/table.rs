use std::error::Error;

use serde_json::{json, Value};

use scylla::QueryResult;
use scylla::IntoTypedRows;

use crate::ScyllaClient;

impl ScyllaClient {

    pub async fn create_table(
        &self,
        keyspace: &str,
        table: &str,
        partition_keys: &[&str], 
        clustering_keys: &[&str],
        columns: &[(&str, &str)],
        sorting: Option<&[(&str, &str)]>,
        time_to_live: Option<u32>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let columns_definition: String = columns.iter()
        .map(|(name, type_)| format!("{} {}", name, type_))
        .collect::<Vec<String>>().join(", ");

        let partition_keys_definition: String = partition_keys.join(", ");
        let clustering_keys_definition = clustering_keys.join(", ");
        let primary_keys_definition: String = if clustering_keys.is_empty() {
            partition_keys_definition
        } else {
            format!("({}, {})", partition_keys_definition, clustering_keys_definition)
        };
        
        let sorting_definition: String = if let Some(sorting) = sorting {
            let sorting_clauses: String = sorting.iter()
                .map(|(column, order)| format!("{} {}", column, order))
                .collect::<Vec<String>>().join(", ");
            format!("CLUSTERING ORDER BY ({})", sorting_clauses)
        } else {
            String::new()
        };

        let ttl_definition: String = if let Some(ttl) = time_to_live {
            format!("default_time_to_live = {}", ttl)
        } else {
            String::new()
        };

        let with_clause: String = if !sorting_definition.is_empty() || !ttl_definition.is_empty() {
            let mut clauses = Vec::new();
            if !sorting_definition.is_empty() {
                clauses.push(sorting_definition);
            }
            if !ttl_definition.is_empty() {
                clauses.push(ttl_definition);
            }
            format!(" WITH {}", clauses.join(" AND "))
        } else {
            String::new()
        };

        let query: String = format!(
            "CREATE TABLE IF NOT EXISTS {}.{} ({}, PRIMARY KEY ({})){}",
            keyspace, table, columns_definition, primary_keys_definition, with_clause
        );

        println!("Query: {}", query);
    
        self.session.query(query,()).await?;

        Ok(())
    }

    pub async fn drop_table(
        &self,
        keyspace: &str,
        table: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let query: String = format!(
        "DROP TABLE IF EXISTS {}.{}",
        keyspace, table);

        self.session.query(query,()).await?;

        Ok(())
    }

    pub async fn count_rows(
        &self,
        keyspace: &str,
        table: &str
    ) -> Result<i64, Box<dyn Error + Send + Sync>> {
        let query: String = format!(
            "SELECT COUNT(*) FROM {}.{}",
            keyspace, table
        );

        let query_result: QueryResult = self.session.query(query, ()).await?;

        let rows = query_result.rows.ok_or("No rows found")?;
        let count_row = rows.into_typed::<(i64,)>().next().ok_or("No count row found")??;

        Ok(count_row.0)
    }

    pub async fn truncate_table(
        &self,
        keyspace: &str,
        table: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let query: String = format!(
        "TRUNCATE TABLE {}.{}",
        keyspace, table);

        self.session.query(query, ()).await?;
        
        Ok(())
    }

    pub async fn get_table_columns(
        &self,
        keyspace: &str,
        table: &str
    ) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let query = format!(
            "SELECT column_name, kind, type FROM system_schema.columns WHERE keyspace_name = '{}' AND table_name = '{}'",
            keyspace, table
        );

        let result: QueryResult = self.session.query(query, ()).await?;
        let rows = result.rows.ok_or("No rows found")?;
        
        let mut columns = vec![];
        for row in rows.into_typed::<(String, String, String)>() {
            let (column_name, kind, data_type) = row?;
            columns.push(json!({
                "column_name": column_name,
                "kind": kind,
                "data_type": data_type,
            }));
        }

        let json_result = json!({
            "keyspace": keyspace,
            "table": table,
            "columns": columns,
        });

        Ok(json_result)
    }

}
