use scylla::{Session, SessionBuilder};
use std::error::Error;

use crate::ScyllaClient;
use crate::{QueryBuilder, Operations};

impl ScyllaClient {

    pub async fn new(
        known_nodes: Vec<&str>
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {

        let mut builder = SessionBuilder::new();
        
        for node in known_nodes {
            builder = builder.known_node(node);
        }
        
        let session: Session = builder.build().await?;

        Ok(ScyllaClient { session })
    }

    pub fn query(&self, keyspace: &str, table: &str) -> QueryBuilder<'_> {
        QueryBuilder::new(Operations::Select, keyspace, table, self)
    }
}