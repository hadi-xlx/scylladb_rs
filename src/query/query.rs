#[derive(Debug)]
pub struct QueryBuilder {
    pub operation: Operations,
    pub keyspace: String,
    pub table: String,
    pub columns: Vec<String>,
    pub conditions: Vec<String>,
    pub clauses: Vec<String>,
    pub order: Option<(String, OrderDirection)>,
    pub insert_options: Vec<InsertOptions>,
}


#[derive(Debug,PartialEq )]
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

pub fn column(name: &str) -> String {
    name.to_string()
}

pub fn condition(column: &str, operator: ComparisonOperators, value: &str) -> String {
    let op = match operator {
        ComparisonOperators::Eq => "=",
        ComparisonOperators::Neq => "!=",
        ComparisonOperators::Gt => ">",
        ComparisonOperators::Gte => ">=",
        ComparisonOperators::Lt => "<",
        ComparisonOperators::Lte => "<=",
    };

    format!("{} {} {}", column, op, value)
}

pub fn logical_condition(cond1: &str, operator: LogicalOperators, cond2: &str) -> String {
    let op = match operator {
        LogicalOperators::And => "AND",
        LogicalOperators::Or => "OR",
        LogicalOperators::Not => "NOT",
    };

    format!("({} {} {})", cond1, op, cond2)
}

pub fn set_condition(column: &str, operator: SetOperators, values: &[&str]) -> String {
    let op = match operator {
        SetOperators::In => "IN",
        SetOperators::NotIn => "NOT IN",
        SetOperators::Between => "BETWEEN",
        SetOperators::NotBetween => "NOT BETWEEN",
    };

    match operator {
        SetOperators::In | SetOperators::NotIn => format!("{} {} ({})", column, op, values.join(", ")),
        SetOperators::Between | SetOperators::NotBetween => format!("{} {} {} AND {}", column, op, values[0], values[1]),
    }
}

pub fn pattern_condition(column: &str, operator: PatternMatchingOperators, pattern: &str) -> String {
    let op = match operator {
        PatternMatchingOperators::Like => "LIKE",
    };

    format!("{} {} '{}'", column, op, pattern)
}

pub fn null_condition(column: &str, operator: NullOperators) -> String {
    let op = match operator {
        NullOperators::IsNull => "IS NULL",
        NullOperators::IsNotNull => "IS NOT NULL",
    };

    format!("{} {}", column, op)
}
