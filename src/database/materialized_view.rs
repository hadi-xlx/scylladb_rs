use std::error::Error;

use crate::{MaterializedView,Table};

impl<'a> MaterializedView<'a> {

    pub async fn new(
        table: &'a Table<'a>,
        name: String,
        primary_key: String,
        condition: String,
    ) -> MaterializedView<'a> {
        MaterializedView {
            table,
            name,
            primary_key,
            condition,
        }
    }

    pub async fn create_materialized_view(
        &self,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query: String = format!(
            "CREATE MATERIALIZED VIEW IF NOT EXISTS {}.{} AS SELECT * FROM {}.{} WHERE {} PRIMARY KEY ({})",
            self.table.keyspace.name,
            self.name,
            self.table.keyspace.name,
            self.table.name,
            self.condition,
            self.primary_key
        );
        self.table.keyspace.session.query(query, ()).await?;
        Ok(())
    }

    pub async fn drop_materialized_view(
        &self,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query: String = format!(
            "DROP MATERIALIZED VIEW IF EXISTS {}.{}",
            self.table.keyspace.name, self.name
        );
        self.table.keyspace.session.query(query, ()).await?;
        Ok(())
    }
}