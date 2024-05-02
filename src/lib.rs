use scylla::Session;

pub mod query {
    pub mod delete;
    pub mod insert_if_unique;
    pub mod insert;
    pub mod select;
    pub mod update;
}


pub mod database {
    pub mod keyspace;
    pub mod table;
    pub mod materialized_view;
}

pub struct Keyspace<'a> {
    pub name: String,
    pub session: &'a Session,
}
pub struct Table<'a> {
    pub keyspace: &'a Keyspace<'a>,
    pub name: String,
}

pub struct MaterializedView<'a> {
    pub table: &'a Table<'a>,
    pub name: String,
    pub primary_key: String,
    pub condition: String,
}

pub struct Column<'a> {
    pub table: &'a Table<'a>,
    pub name: String,
    pub data_type: String,
    pub default: Option<String>,
    pub is_primary_key: bool,
    pub is_clustering_key: bool,
}
