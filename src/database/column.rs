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

        let column_definition: String = self.to_cql_definition(column,data_type);

        let query: String = format!(
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

        let query: String = format!(
            "ALTER TABLE {}.{} DROP {}",
            keyspace,
            table,
            column
        );

        self.session.query(query, ()).await?;

        Ok(())
    }


    fn to_cql_definition(&self, column: &str, data_type: &str) -> String {
        format!("{} {}", column, data_type)
    }
}