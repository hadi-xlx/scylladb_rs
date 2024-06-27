//! # Scyla SDK for Rust
//!
//! ## Features
//! [**`Insert`**](#insert): Add new rows to a table.



use scylla::Session;

pub mod auth;

pub mod query {
    pub mod query;
    pub mod query_builder;
    pub mod utils;
    pub mod operators;
}

pub mod database {
    pub mod column;
    pub mod keyspace;
    pub mod materialized_view;
    pub mod table;
    pub mod index;
}

#[derive(Debug)]
pub struct ScyllaClient {
    pub session: Session,
}

#[derive(Debug,Clone)]
pub struct QueryBuilder<'a> {
    pub operation: Operations,
    pub keyspace: String,
    pub table: String,
    pub columns: Vec<String>,
    pub conditions: Vec<String>,
    pub clauses: Vec<String>,
    pub order: Option<(String, OrderDirection)>,
    pub insert_options: Vec<InsertOptions>,
    pub client: &'a ScyllaClient,
}

#[derive(Debug,PartialEq,Clone)]
pub enum Operations {
    Select,
    Insert,
    InsertIfNotExists,
    Update,
    Delete,
}

pub enum ComparisonOperators {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
}

pub enum LogicalOperators {
    And,
    Not,
}

pub enum SetOperators {
    In,
    NotIn,
    Between,
}

pub enum PatternMatchingOperators {
    Like,
}

pub enum Clauses {
    OrderBy,
    Limit,
    AllowFiltering,
    GroupBy,       
}

#[derive(Debug,Clone)]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug,Clone)]
pub enum InsertOptions {
    UsingTimestamp(i64),
    UsingTTL(i32),
}


