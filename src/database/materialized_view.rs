use std::error::Error;

use crate::ScyllaClient;

impl ScyllaClient  {

    pub async fn create_mv(
        &self,
        keyspace: &str,
        materialized_view: &str,
        table: &str,
        condition: &str,
        primary_key: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let query: String = format!(
            "CREATE MATERIALIZED VIEW IF NOT EXISTS {}.{} AS SELECT * FROM {}.{} WHERE {} PRIMARY KEY ({})",
            keyspace, materialized_view, keyspace, table, condition, primary_key
        );

        self.session.query(query, ()).await?;

        Ok(())
    }

    pub async fn drop_mv(
        &self,
        keyspace: &str,
        materialized_view: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let query: String = format!(
            "DROP MATERIALIZED VIEW IF EXISTS {}.{}",
            keyspace, materialized_view
        );

        self.session.query(query, ()).await?;

        Ok(())
    }
}