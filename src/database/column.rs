use std::error::Error;

use crate::{Column,Table};

impl<'a> Column<'a> {
    /// Creates a new Column instance.
    pub fn new(
        table: &'a Table<'a>,
        name: String,
        data_type: String,
    ) -> Self {
        Column {
            table,
            name,
            data_type,

        }
    }

    /// Creates this column in the database.
    pub async fn create(&self) -> Result<(), Box<dyn Error + Send + Sync>> {

        let column_definition = self.to_cql_definition();

        let query = format!(
            "ALTER TABLE {}.{} ADD {}",
            self.table.keyspace.name,
            self.table.name,
            column_definition
        );

        self.table.keyspace.session.query(query, ()).await?;

        Ok(())
    }

    /// Drops this column from the database.
    pub async fn drop(&self) -> Result<(), Box<dyn Error + Send + Sync>> {

        let query = format!(
            "ALTER TABLE {}.{} DROP {}",
            self.table.keyspace.name,
            self.table.name,
            self.name
        );

        self.table.keyspace.session.query(query, ()).await?;

        Ok(())
    }

    /// Returns a formatted string representing the column definition for CQL queries.
    fn to_cql_definition(&self) -> String {

        let definition: String = format!("{} {}", self.name, self.data_type);

        definition
    }
}