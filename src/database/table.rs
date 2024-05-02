use std::error::Error;

use scylla::QueryResult;
use scylla::transport::query_result::RowsExpectedError;

use crate::{Keyspace, Table};

impl<'a> Table<'a> {
    //Constructor
    pub fn new(
        keyspace: &'a Keyspace<'a>,
        name: String
    ) -> Self {
        Self {keyspace, name }
    }

    pub async fn create(
        &self
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query: String = format!(
        "CREATE TABLE IF NOT EXISTS {}.{} (id UUID PRIMARY KEY)",
        self.keyspace.name, self.name);
        self.keyspace.session.query(query,()).await?;
        Ok(())
    }

    pub async fn drop(
        &self,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query: String = format!(
        "DROP TABLE IF EXISTS {}.{}",
        self.keyspace.name, self.name);
        self.keyspace.session.query(query,()).await?;
        Ok(())
    }

        // Add a column to the specified table, WORKS
    pub async fn create_column(
        &self,
        column_name: &str,
        column_datatype: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!(
        "ALTER TABLE {}.{} ADD {} {}",
        self.keyspace.name, self.name, column_name, column_datatype);
        self.keyspace.session.query(query, ()).await?;
        Ok(())
    }

    // Remove a column from the specified table, WORKS
    pub async fn delete_column(
        &self,
         column_name: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!(
        "ALTER TABLE {}.{} DROP {}",
        self.keyspace.name, self.name, column_name);
        self.keyspace.session.query(query, ()).await?;
        Ok(())
    }

    // Create an index on the specified column
    pub async fn create_index(
        &self,
        index_name: &str,
        column_name: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!(
        "CREATE INDEX IF NOT EXISTS {} ON {}.{} ({})",
        index_name,self.keyspace.name, self.name, column_name);
        self.keyspace.session.query(query, ()).await?;
        Ok(())
    }

    // Drop an index
    pub async fn drop_index(
        &self,
        index_name: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!(
        "DROP INDEX IF EXISTS {}.{}",
        self.keyspace.name, index_name);
        self.keyspace.session.query(query, ()).await?;
        Ok(())
    }

    // Check for duplicates in a column of the table
    pub async fn check_duplicates(
        &self,
        column_name: &str
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let query: String = format!(
        "SELECT {}, COUNT(*) FROM {}.{} GROUP BY {} HAVING COUNT(*) > 1",
        column_name, self.keyspace.name, self.name, column_name);
        let query_result: QueryResult = self.keyspace.session.query(query, ()).await?;
        let rows_count: Result<usize,RowsExpectedError>  = query_result.rows_num();
        match rows_count {
            Ok(count) => Ok(count > 1),
            Err(e) => Err(Box::new(e)),
        }
    }

    // Count rows in the specified table
    pub async fn count_rows(
        &self
    ) -> Result<i64, Box<dyn Error + Send + Sync>> {
        let query: String = format!(
        "SELECT COUNT(*) FROM {}.{}",
        self.keyspace.name, self.name);
        let query_result: QueryResult  = self.keyspace.session.query(query, ()).await?;
        let rows_count: Result<usize,RowsExpectedError> = query_result.rows_num();

        match rows_count {
            Ok(count) => Ok(count as i64),
            Err(e) => Err(Box::new(e)),
        }
    }

    // Truncate the specified table, WOKRS
    pub async fn truncate(
        &self
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!(
        "TRUNCATE TABLE {}.{}",
        self.keyspace.name, self.name);
        self.keyspace.session.query(query, ()).await?;
        Ok(())
    }

}
