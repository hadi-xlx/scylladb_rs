use scylla::Session;


pub mod auth;

pub mod query {
    pub mod delete;
    pub mod insert_if_unique;
    pub mod insert;
    pub mod query;
    pub mod select;
    pub mod update;
}

pub mod database {
    pub mod column;
    pub mod keyspace;
    pub mod materialized_view;
    pub mod table;
}

pub struct ScyllaClient {
    pub session: Session,
}
