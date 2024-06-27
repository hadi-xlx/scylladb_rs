use std::error::Error;
use std::collections::HashMap;

use crate::ScyllaClient;

impl ScyllaClient{

    pub async fn create_ks_simple(
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

    pub async fn create_ks_network(
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

    pub async fn drop_ks(
        &self,
        keyspace: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let query: String = format!(
        "DROP KEYSPACE IF EXISTS {}",
        keyspace);

        self.session.query(query,()).await?;
        
        Ok(())
    }

}


