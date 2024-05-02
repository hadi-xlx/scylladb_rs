use scylla::Session;

pub mod database {
    pub mod schema;
    pub mod keyspace;
    pub mod table;
}

pub struct Schema<'a> {
    pub schema_name: String,
    pub session: &'a Session,
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


