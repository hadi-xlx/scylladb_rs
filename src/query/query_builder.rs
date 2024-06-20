use crate::query::query::*;

impl QueryBuilder {
    pub fn new(operation: Operations, table: &str) -> Self {
        Self {
            operation,
            table: table.to_string(),
            columns: Vec::new(),
            conditions: Vec::new(),
            clauses: Vec::new(),
            order: None,
            insert_options: Vec::new(),
        }
    }

    pub fn select(mut self, columns: &[&str]) -> Self {
        self.columns = columns.iter().map(|&col| col.to_string()).collect();
        self
    }

    pub fn where_condition(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }

    pub fn clause(mut self, clause: &str) -> Self {
        self.clauses.push(clause.to_string());
        self
    }

    pub fn order_by(mut self, column: &str, direction: OrderDirection) -> Self {
        self.order = Some((column.to_string(), direction));
        self
    }

    pub fn insert_option(mut self, option: InsertOptions) -> Self {
        self.insert_options.push(option);
        self
    }

    pub fn build(self) -> String {
        let operation = match self.operation {
            Operations::Select => "SELECT",
            Operations::Insert => "INSERT INTO",
            Operations::InsertIfNotExists => "INSERT IF NOT EXISTS",
            Operations::Update => "UPDATE",
            Operations::Delete => "DELETE",
        };

        let columns = if self.columns.is_empty() {
            "*".to_string()
        } else {
            self.columns.join(", ")
        };

        let mut query = format!("{} {} FROM {}", operation, columns, self.table);

        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }

        if let Some((col, dir)) = self.order {
            let dir_str = match dir {
                OrderDirection::Asc => "ASC",
                OrderDirection::Desc => "DESC",
            };
            query.push_str(&format!(" ORDER BY {} {}", col, dir_str));
        }

        if !self.clauses.is_empty() {
            query.push_str(" ");
            query.push_str(&self.clauses.join(" "));
        }

        if !self.insert_options.is_empty() {
            query.push_str(" USING ");
            let options: Vec<String> = self.insert_options.into_iter().map(|option| {
                match option {
                    InsertOptions::UsingTimestamp(ts) => format!("TIMESTAMP {}", ts),
                    InsertOptions::UsingTTL(ttl) => format!("TTL {}", ttl),
                }
            }).collect();
            query.push_str(&options.join(" AND "));
        }

        query
    }
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