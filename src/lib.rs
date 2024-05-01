use scylla::Session;

pub mod database {
    pub mod cluster;
    pub mod keyspace;
    pub mod table;
}

pub struct Cluster<'a> {
    pub cluster_name: String,
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


