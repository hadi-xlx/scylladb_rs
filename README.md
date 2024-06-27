# scylladb_rs

`scylladb_rs` is an extremely light weight Scylla SDK for interacting with it's database.

## Database Features

- [x] Creating & dropping keyspaces
- [x] Creating & dropping tables
- [x] Creating & dropping materialized views
- [x] Creating & dropping columns
- [x] Creating & dropping indexes
- [x] Inserting
- [ ] Inserting if unique
- [x] Bulk Inserting
- [x] Delete (only with primary keys) (all primary keys must be provided)
- [x] Select
- [x] Applying Filters
- [ ] Counting total records
- [ ] User defined functions (UDF)
- [ ] User defined types


## Advanced Filtering over `select()`

- [x] Column is equal to a value
- [ ] Column is not equal to a value
- [x] Column is greater than a value
- [x] Column is less than a value
- [x] Column is greater than or equal to a value
- [x] Column is less than or equal to a value
- [x] Column is in a list of values
- [x] Column is between a range of values
- [x] Column is like a value
- [ ] Order the results
- [ ] Limit the number of rows returned
- [ ] Retrieve as a CSV


## Advanced table definition `create_table()`

- [x] defining partition keys
- [ ] defining clustering keys
- [x] defining optional sorting
- [x] defining optional ttl (time to live)


## Features


## Cargo.toml
```toml
[dependencies]
scylladb_rs = "0.1.0"
```
## Examples

### Initialize the Scylla Client and perform a select query
 ```rust

use scylla::QueryResult;
use scylla::transport::errors::QueryError;

use scylladb_rs::ScyllaClient;
use scylladb_rs::query::utils::print_query_result;

async fn intiliaze_client_and_select() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    let select_query_1: Result<QueryResult, QueryError> = client
        .query("test_keyspace", "test_table")
        .select(&["name", "score"])
        .eq("age", 33)
        .execute()
        .await;

    match &select_query_1 {
        Ok(query_result) => print_query_result("Query Result:", query_result),
        Err(e) => println!("Query failed: {:?}", e),
    }

    Ok(())
}
```
This will initialize the Scylla Client perferm a Select query and then print the result.

More examples will be present in the docs.