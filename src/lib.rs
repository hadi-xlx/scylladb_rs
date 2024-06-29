//! # Scyla SDK for Rust
//!
//! ## Examples
//! test
//! ### Create a keyspace
//! ```rust
//! use scylladb_rs::ScyllaClient;
//!
//! async fn createing_keyspace() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!        
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!    // Create a keyspace
//!    client.create_keyspace_simple("test_keyspace", 1).await?;
//!
//!    Ok(())
//! }
//! ```
//!
//! ### Drop a keyspace
//! ```rust
//! async fn dropping_keyspace() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!    // Drop the keyspace
//!    client.drop_keyspace("test_keyspace").await?;
//!
//!    Ok(())
//! }
//! ```
//!
//! ### Get keyspace tables
//! ```rust
//! async fn getting_keyspace_tables() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!    let tables: serde_json::Value = client.get_keyspace_tables("test_keyspace").await?;
//!
//!    println!("{:?}", tables);
//!
//!    Ok(())
//! }
//!```
//! 
//! ### Get keyspace materialized views
//! ```rust
//! async fn getting_keyspace_materialized_views() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!    let tables: serde_json::Value = client.get_keyspace_materialized_views("test_keyspace").await?;
//!
//!    println!("{:?}", tables);
//!
//!    Ok(())
//! }
//! ```
//!
//! ### Create a table
//! ```rust
//! async fn creating_table() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     client.create_table(
//!         "test_keyspace", // keyspace
//!         "test_table", // table
//!         &["user_id"], // partition keys
//!         &["age"], // clustering keys
//!         &[("age", "int"), ("name", "text"), ("score", "double"), ("user_id", "int")], // regular columns
//!         None, // Sorting
//!         None // time to live
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Drop a table
//! ```rust
//! async fn dropping_table() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     client.drop_table(
//!         "test_keyspace",
//!         "test_table"
//!     ).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Get table columns
//! ```rust
//! async fn getting_table_columns() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     let columns = client.get_table_columns("test_keyspace", "test_table").await?;
//!
//!     // Serialize columns to JSON
//!     let json_columns = serde_json::to_string_pretty(&columns)?;
//!
//!     println!("{}", json_columns);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Count rows
//! ```rust
//! async fn counting_rows() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!     let row_count: i64 = client.count_rows("test_keyspace", "test_table").await?;
//!     println!("Row count: {}", row_count);
//!     Ok(())
//! }
//! ```
//!
//! ### Truncate table
//! ```rust
//! async fn truncating_table() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!     client.truncate_table("test_keyspace", "test_table").await?;
//!     Ok(())
//! }
//! ```
//!
//! ### Create a column
//! ```rust
//! async fn creating_column() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     // Add a column
//!     client.create_column(
//!         "test_keyspace",
//!         "test_table",
//!         "is_active",
//!         "boolean"
//!     ).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Drop a column
//! ```rust
//! async fn dropping_column() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     // Drop the column
//!     client.drop_column(
//!         "test_keyspace",
//!         "test_table",
//!         "age"
//!     ).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Create an index
//! ```rust
//! async fn creating_indexing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     match client.create_index(
//!         "test_keyspace",
//!         "test_table",
//!         "score_index",
//!         "score"
//!     ).await {
//!         Ok(_) => println!("Index created successfully"),
//!         Err(e) => println!("Failed to create index: {:?}", e),
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Drop an index
//! ```rust
//! async fn dropping_indexing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     client.drop_index("test_keyspace", "test_index").await?;
//!     Ok(())
//! }
//! ```


use scylla::Session;

pub mod client;

pub mod query {
    pub mod query_builder;
    pub mod utils;
    pub mod operators;
    pub mod update;
    pub mod delete;
    pub mod insert;
    pub mod select;
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
    pub allow_filtering: bool,
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


