use std::error::Error;

use scylla::Session;

use crate::Keyspace;

impl<'a> Keyspace<'a> {

    pub fn new(
        name: String,
        session: &'a Session,
    ) -> Self {
        Self { name, session }
    }

    pub async fn create(
        &self,
        replication_strategy: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query: String = format!(
        "CREATE KEYSPACE IF NOT EXISTS {} WITH replication = {}",
        self.name, replication_strategy);
        self.session.query(query, ()).await?;
        Ok(())
    }

    pub async fn drop(
        &self
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query: String = format!(
        "DROP KEYSPACE IF EXISTS {}",
        self.name);
        self.session.query(query,()).await?;
        Ok(())
    }


}


