use std::error::Error;
use std::collections::HashMap;

use serde_json::{json, Value};

use scylla::IntoTypedRows;

use crate::ScyllaClient;

impl ScyllaClient{

    pub async fn create_keyspace_simple(
        &self,
        keyspace: &str,
        replication_factor: u64
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let query: String = format!(
            "CREATE KEYSPACE IF NOT EXISTS {} WITH replication = {{'class': 'SimpleStrategy','replication_factor': {}}}",
            keyspace, replication_factor);

        self.session.query(query, ()).await?;

        Ok(())
    }

    pub async fn create_keyspace_network(
        &self,
        keyspace: &str,
        datacenters: HashMap<&str, u64>
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let replication_factors: Vec<String> = datacenters
            .iter()
            .map(|(dc, rf)| format!("'{}': {}", dc, rf))
            .collect();

        let replication_factors_str = replication_factors.join(", ");
        
        let query: String = format!(
            "CREATE KEYSPACE IF NOT EXISTS {} WITH replication = {{'class': 'NetworkTopologyStrategy', {}}}",
            keyspace, replication_factors_str);

        self.session.query(query, ()).await?;

        Ok(())
    }

    pub async fn drop_keyspace(
        &self,
        keyspace: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let query: String = format!(
        "DROP KEYSPACE IF EXISTS {}",
        keyspace);

        self.session.query(query,()).await?;
        
        Ok(())
    }

    pub async fn get_keyspace_tables(
        &self,
        keyspace: &str
    ) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let query = format!(
            "SELECT table_name FROM system_schema.tables WHERE keyspace_name = '{}'",
            keyspace
        );

        let result = self.session.query(query, ()).await?;
        let rows = result.rows.ok_or("No rows found")?;
        let table_names: Vec<String> = rows.into_typed::<(String,)>()
            .map(|row| row.map(|(table_name,)| table_name))
            .collect::<Result<Vec<_>, _>>()?;

        let json_result = json!({
            "keyspace": keyspace,
            "tables": table_names,
        });

        Ok(json_result)
    }

}


