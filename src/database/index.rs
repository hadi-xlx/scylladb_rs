use std::error::Error;

use crate::ScyllaClient;

impl ScyllaClient {

    // Create an index on the specified column
    pub async fn create_index(
        &self,
        keyspace: &str,
        table: &str,
        index: &str,
        column: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let query: String = format!(
        "CREATE INDEX IF NOT EXISTS {} ON {}.{} ({})",
        index,keyspace, table, column);

        self.session.query(query, ()).await?;

        Ok(())
    }

    // Drop an index
    pub async fn drop_index(
        &self,
        keyspace: &str,
        index: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let query: String = format!(
        "DROP INDEX IF EXISTS {}.{}",
        keyspace, index);

        self.session.query(query, ()).await?;

        Ok(())
    }

}
