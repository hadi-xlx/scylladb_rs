//! # Scylla SDK for Rust
//!
//! ## Examples
//! 
//! ### Connect to ScyllaDB
//! ```rust
//! use scylladb_rs::ScyllaClient;
//!
//! async fn databasing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!    let _client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!    Ok(())
//! }
//!```
//! This will connect to ScyllaDB, any methods that aplly for the Session struct from the `scylla` crate can be used here.
//! 
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
//! use scylladb_rs::ScyllaClient;
//!
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
//! use scylladb_rs::ScyllaClient;
//!
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
//! use scylladb_rs::ScyllaClient;
//!
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
//! use scylladb_rs::ScyllaClient;
//!
//! async fn creating_table() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     client.create_table(
//!         "test_keyspace", // keyspace
//!         "test_table", // table
//!         &["user_id"], // partition keys
//!         &["age"], // clustering keys
//!         &[
//!           ("age", "int"),
//!           ("name", "text"),
//!           ("score", "double"),
//!           ("user_id", "int")
//!          ], // regular columns
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
//! use scylladb_rs::ScyllaClient;
//!
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
//! use scylladb_rs::ScyllaClient;
//!
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
//! use scylladb_rs::ScyllaClient;
//!
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
//! use scylladb_rs::ScyllaClient;
//!
//! async fn truncating_table() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!     client.truncate_table("test_keyspace", "test_table").await?;
//!     Ok(())
//! }
//! ```
//!
//! ### Create a column
//! ```rust
//! use scylladb_rs::ScyllaClient;
//!
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
//! use scylladb_rs::ScyllaClient;
//!
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
//! use scylladb_rs::ScyllaClient;
//! 
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
//! This is good to use on frequently queried columns of regular type.
//! 
//! ### Drop an index
//! ```rust
//! use scylladb_rs::ScyllaClient;
//! 
//! async fn dropping_indexing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     client.drop_index("test_keyspace", "test_index").await?;
//!     Ok(())
//! }
//! ```
//!
//! ### Create a materialized view
//! ```rust
//! use scylladb_rs::ScyllaClient;
//!
//! async fn create_materialized_view() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     client.create_mv(
//!         "test_keyspace",
//!         "test_mv",
//!         "test_table",
//!         "age = 22",
//!         "id"
//!     ).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Drop a materialized view
//! ```rust
//! use scylladb_rs::ScyllaClient;
//!
//! async fn drop_materialized_view() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     // Drop the materialized view
//!     client.drop_mv(
//!         "test_keyspace",
//!         "test_mv"
//!     ).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Select query 1
//! ```rust
//! use scylla::QueryResult;
//! use scylla::transport::errors::QueryError;
//!
//! use scylladb_rs::ScyllaClient;
//! use scylladb_rs::query::utils::print_query_result;
//!
//! async fn test_select_query_1() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     let select_query_1: Result<QueryResult, QueryError> = client
//!         .query("test_keyspace", "test_table")
//!         .select(&["name"])
//!         .eq("age", 33)
//!         .allow_filtering() 
//!         .execute()
//!         .await;
//!
//!     match &select_query_1 {
//!         Ok(query_result) => print_query_result("Query 1:", query_result),
//!         Err(e) => println!("Query 1 failed: {:?}", e),
//!     }
//!
//!     Ok(())
//! }
//! ```
//! Here we use ```allow_filtering()``` because we did't provide the complete key values.
//! 
//! ### Select query 2
//! ```rust
//! use scylla::QueryResult;
//! use scylla::transport::errors::QueryError;
//!
//! use scylladb_rs::ScyllaClient;
//! use scylladb_rs::query::utils::print_query_result;
//!
//! async fn test_select_query_2() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     let select_query_2: Result<QueryResult, QueryError> = client
//!         .query("test_keyspace", "test_table")
//!         .select(&["name", "age"])
//!         .eq("user_id", 2)
//!         .between("age", 20, 50)
//!         .like("name", "Jane%") // you may also do "%Jane%" or just "Jane"
//!         .allow_filtering()
//!         .execute()
//!         .await;
//!
//!     match &select_query_2 {
//!         Ok(query_result) => print_query_result("Query 2:", query_result),
//!         Err(e) => println!("Query 2 failed: {:?}", e),
//!     }
//!
//!     Ok(())
//! }
//! ```
//! Here we use ```allow_filtering()``` because only EQ and IN relation are supported on partition keys
//!
//! ### Select query 3
//! ```rust
//! use scylla::QueryResult;
//! use scylla::transport::errors::QueryError;
//!
//! use scylladb_rs::ScyllaClient;
//! use scylladb_rs::query::utils::print_query_result;
//!
//! async fn test_select_query_3() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     let select_query_3: Result<QueryResult, QueryError> = client
//!         .query("test_keyspace", "test_table")
//!         .select(&["name"])
//!         .eq("user_id", 2)
//!         .in_list("age", &vec![28, 44, 40]) // the vec can also have strings
//!         .execute()
//!         .await;
//!
//!     match &select_query_3 {
//!         Ok(query_result) => print_query_result("Query 3:", query_result),
//!         Err(e) => println!("Query 3 failed: {:?}", e),
//!     }
//!
//!     Ok(())
//! }
//! ```
//! Here we don't use ```allow_filtering()``` because we provided the complete key values
//! and all operations are suppotred for the partition keys.
//! 
//! ### Updating
//! ```rust
//! async fn updating() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     let json_body = json!(
//!         {
//!             "score": 66.0
//!         }
//!     );
//!
//!     let mut primary_keys: HashMap<&str, i32> = HashMap::new();
//!     primary_keys.insert("age", 44);
//!     primary_keys.insert("user_id", 2);
//!
//!     let update = client
//!         .query("test_keyspace", "test_table")
//!         .update(
//!             primary_keys,
//!             json_body
//!         )
//!         .await;
//!
//!     match &update {
//!         Ok(query_result) => println!("Query Succecsfull: {:?}", query_result),
//!         Err(e) => println!("Query 1 failed: {:?}", e),
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Inserting
//! ```rust
//! use std::error::Error;
//!
//! use serde_json::json;
//!
//! use scylladb_rs::ScyllaClient;
//! use scylladb_rs::query::utils::print_query_result;
//!
//! async fn inserting() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!
//!     let json_body = json!({
//!         "user_id": 3,
//!         "age": 44,
//!         "name": "Jane Doe the third",
//!         "score": 71.6,
//!         "is_active": true
//!     });
//!
//!
//!     let insert: Result<scylla::QueryResult, Box<dyn Error + Sync + Send>> = client
//!         .query("test_keyspace", "test_table")
//!         .insert(json_body)
//!         .await;
//!
//!     match &insert {
//!         Ok(query_result) => println!("Query Successful: {:?}", query_result),
//!         Err(e) => println!("Query failed: {:?}", e),
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Bulk Inserting
//! ```rust
//! use std::error::Error;
//!
//! use serde_json::json;
//!
//! use scylladb_rs::ScyllaClient;
//!
//! #[tokio::test]
//! async fn bulk_inserting() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!    
//!     let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
//!     let json_body = json!([
//!         {
//!             "user_id": 33,
//!             "age": 33,
//!             "name": "Johnny Doe the first",
//!             "score": 100.0,
//!             "is_active": true
//!         },
//!         {
//!             "user_id": 22,
//!             "age": 22,
//!             "name": "Johnny Doe the second",
//!             "score": 88.5,
//!             "is_active": false
//!         },
//!         {
//!             "user_id": 11,
//!             "age": 22,
//!             "name": "Luke Doe the third",
//!             "score": 77.0,
//!             "is_active": true
//!         }
//!         // Add more JSON objects as needed
//!     ]);
//!
//!     let insert_bulk: Result<scylla::QueryResult, Box<dyn Error + Sync + Send>> = client
//!         .query("test_keyspace", "test_table")
//!         .insert_bulk(json_body)
//!         .await;
//!
//!     match &insert_bulk {
//!         Ok(query_result) => println!("Query Succecsfull: {:?}", query_result),
//!         Err(e) => println!("Query failed: {:?}", e),
//!     }
//!
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


