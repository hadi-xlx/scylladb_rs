use std::error::Error;

use crate::ScyllaClient;

impl ScyllaClient  {

    pub async fn create_column(
        &self,
        keyspace: &str,
        table: &str,
        column: &str,
        data_type: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let column_definition = self.to_cql_definition(column,data_type);

        let query = format!(
            "ALTER TABLE {}.{} ADD {}",
            keyspace,
            table,
            column_definition
        );

        self.session.query(query, ()).await?;

        Ok(())
    }

    /// Drops this column from the database.
    pub async fn drop_column(
        &self,
        keyspace: &str,
        table: &str,
        column: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let query = format!(
            "ALTER TABLE {}.{} DROP {}",
            keyspace,
            table,
            column
        );

        self.session.query(query, ()).await?;

        Ok(())
    }

    /// Returns a formatted string representing the column definition for CQL queries.
    /// Returns a formatted string representing the column definition for CQL queries.
    fn to_cql_definition(&self, column: &str, data_type: &str) -> String {
        format!("{} {}", column, data_type)
    }
}