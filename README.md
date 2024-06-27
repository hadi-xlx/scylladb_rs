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
- [**`Insert`**](#insert): Add new rows to a table.
- [**`Insert if unique`**](#insert-if-unique): Add a new row only if it does not violate a UNIQUE constraint.
- [**`Insert bulk`**](#insert-if-exists): Add multiple enw rows to a table.
- [**`Update`**](#update): Modify existing rows in a table based on a unique identifier.
- [**`Select`**](#select): Insert a new row into a table if it does not exist, or update it if it does.
- [**`Select with count`**](#select-with-count): Select rows from a table and count the number of rows that match the filter criteria.
- [**`Select with filter`**](#select-with-filter): Select rows from a table based on a filter criteria.
- [**`Select with filter and count`**](#selecting-with-filter-and-count): Select rows from a table based on a filter criteria and count the number of rows that match the filter criteria.
- [**`Delete`**](#delete): Delete a row from a table based on a unique identifier.

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