use std::error::Error;

use scylla::Session;

use crate::Keyspace;

impl<'a> Keyspace<'a> {

    pub fn new(
        keyspace_name: String,
        session: &'a Session,
    ) -> Self {
        Self { keyspace_name, session }
    }

    pub async fn create_keyspace(
        &self,
        replication_strategy: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!("CREATE KEYSPACE IF NOT EXISTS {} WITH replication = {}",
        self.keyspace_name, replication_strategy);
        self.session.query(query, ()).await?;
        Ok(())
    }

    pub async fn delete_keyspace(
        &self
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!("DROP KEYSPACE IF EXISTS {}",
        self.keyspace_name);
        self.session.query(query,()).await?;
        Ok(())
    }

    pub async fn create_table(
        &self,
        table_name: &str,
        schema: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!("CREATE TABLE IF NOT EXISTS {}.{} ({})",
        self.keyspace_name, table_name, schema);
        self.session.query(query,()).await?;
        Ok(())
    }

    pub async fn drop_table(
        &self,
        table_name: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!("DROP TABLE IF EXISTS {}.{}",
        self.keyspace_name, table_name);
        self.session.query(query,()).await?;
        Ok(())
    }

    pub async fn create_index(
        &self,
        index_name: &str,
        table_name: &str,
        column_name: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!("CREATE INDEX IF NOT EXISTS {} ON {}.{} ({})",
        index_name, self.keyspace_name, table_name, column_name);
        self.session.query(query,()).await?;
        Ok(())
    }

    pub async fn drop_index(
        &self,
        index_name: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!("DROP INDEX IF EXISTS {}.{}",
        self.keyspace_name, index_name);
        self.session.query(query,()).await?;
        Ok(())
    }

    pub async fn create_materialized_view(
        &self,
        view_name: &str,
        table_name: &str,
        schema: &str,
        primary_key: &str,
        condition: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!("CREATE MATERIALIZED VIEW IF NOT EXISTS {}.{} AS SELECT {} FROM {} WHERE {} PRIMARY KEY ({})",
        self.keyspace_name, view_name, schema, table_name, condition, primary_key);
        self.session.query(query,()).await?;
        Ok(())
    }

    pub async fn drop_materialized_view(
        &self,
        view_name: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!("DROP MATERIALIZED VIEW IF EXISTS {}.{}",
        self.keyspace_name, view_name);
        self.session.query(query,()).await?;
        Ok(())
    }
}


