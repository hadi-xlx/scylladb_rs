#[derive(Debug)]
pub struct QueryBuilder {
    pub operation: Operations,
    pub table: String,
    pub columns: Vec<String>,
    pub conditions: Vec<String>,
    pub clauses: Vec<String>,
    pub order: Option<(String, OrderDirection)>,
    pub insert_options: Vec<InsertOptions>,
}

#[derive(Debug)]
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
    Or,
    Not,
}

pub enum SetOperators {
    In,
    NotIn,
    Between,
    NotBetween,
}

pub enum PatternMatchingOperators {
    Like,
}

pub enum NullOperators {
    IsNull,
    IsNotNull, 
}

pub enum Clauses {
    OrderBy,
    Limit,
    AllowFiltering,
    GroupBy,    
    Having,     
}

#[derive(Debug)]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug)]
pub enum InsertOptions {
    UsingTimestamp(i64),
    UsingTTL(i32),
}

pub enum Functions {
    Ttl,
    Writetime,
    Min,
    Max,
    Avg,
    Sum,         
    Count,       
    Now,         
    DateOf,      
    UnixTimestampOf,
    ToDate,
    ToTimestamp,
    BlobAsText,  // Converts Blob to Text
    TextAsBlob,  // Converts Text to Blob
    TimeuuidAsTimestamp, // Converts TimeUUID to Timestamp
}

pub enum DataTypes {
    Int,
    Text,
    Blob,
    Boolean,
    Counter,
    Decimal,
    Double,
    Float,
    Inet,
    Timestamp,
    Date,       
    Time,        
    Timeuuid,
    Varint,
    List(Box<DataTypes>),   
    Set(Box<DataTypes>),   
    Map(Box<DataTypes>, Box<DataTypes>),
    Tuple(Vec<DataTypes>),
}