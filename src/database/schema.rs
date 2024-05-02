use std::error::Error;
use scylla::Session;
use crate::Schema;

impl<'a> Schema<'a> {
    pub fn new(
        schema_name: String, 
        session: &'a Session
    ) -> Self {
        Self { 
            schema_name, 
            session 
        }
    }

    pub async fn create_schema(
        &self
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!(
            "CREATE SCHEMA IF NOT EXISTS {}", 
            self.schema_name
        );
        self.session.query(query, ()).await?;
        Ok(())
    }

    pub async fn drop_schema(
        &self
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!(
            "DROP SCHEMA IF EXISTS {}", 
            self.schema_name
        );
        self.session.query(query, ()).await?;
        Ok(())
    }

    pub async fn check_schema_exists(
        &self
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let query = format!(
            "SELECT schema_name FROM system_schema.schemas WHERE schema_name = '{}'", 
            self.schema_name
        );
        let result = self.session.query(query, ()).await?;
        Ok(result.rows.is_some() && !result.rows.unwrap().is_empty())
    }

}