use scylla::Session;

pub mod database {
    pub mod keyspace;
    pub mod table;

}

pub struct Keyspace<'a> {
    pub keyspace_name: String,
    pub session: &'a Session,
}
pub struct Table<'a> {
    table_name: String,
    keyspace_name: String,
    session: &'a Session,
}

pub mod query {
    pub mod delete;
    pub mod insert_if_unique;
    pub mod insert;
    pub mod select;
    pub mod update;
}

