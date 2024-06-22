use scylla::Session;

pub mod auth;

pub mod query {
    pub mod query;
    pub mod query_builder;
}

pub mod database {
    pub mod column;
    pub mod keyspace;
    pub mod materialized_view;
    pub mod table;
}

#[derive(Debug)]
pub struct ScyllaClient {
    pub session: Session,
}