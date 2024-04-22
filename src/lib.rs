use scylla::Session;

pub mod cluster;
pub mod keyspace;
pub mod table;

pub use cluster::*;
pub use keyspace::*;
pub use table::*;

pub struct Cluster {

}

pub struct Keyspace {
    pub keyspace_name: String,
    pub session: Session,
}

pub struct Table {
    table_name: String,
    keyspace_name: String,
    session: Session,
}
