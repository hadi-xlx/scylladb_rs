use std::fmt::Display;
use crate::query::utils::format_value;
use crate::QueryBuilder;

impl<'a> QueryBuilder<'a> {

    //works
    pub fn eq<T: Display>(
        mut self,
        column: &str,
        value: T
    ) -> Self {
        let condition: String = format!("{} = {}", column, format_value(value));
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //doesnt work 
    pub fn neq<T: Display>(
        mut self,
        column: &str,
        value: T
    ) -> Self {
        let condition: String = format!("{} != {}", column, format_value(value));
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn gt<T: Display>(
        mut self,
        column: &str,
        value: T
    ) -> Self {
        let condition: String = format!("{} > {}", column, format_value(value));
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn gte<T: Display>(
        mut self,
        column: &str,
        value: T
    ) -> Self {
        let condition: String = format!("{} >= {}", column, format_value(value));
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn lt<T: Display>(
        mut self,
        column: &str,
        value: T
    ) -> Self {
        let condition: String = format!("{} < {}", column, format_value(value));
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn lte<T: Display>(
        mut self,
        column: &str,
        value: T
    ) -> Self {
        let condition: String = format!("{} <= {}", column, format_value(value));
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn in_list<T: Display>(
        mut self,
        column: &str,
        values: &[T]
    ) -> Self {
        let value_list: String = values.iter().map(|v| format_value(v)).collect::<Vec<_>>().join(", ");
        let condition: String = format!("{} IN ({})", column, value_list);
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn between<T: Display>(
        mut self,
        column: &str,
        lower: T,
        upper: T
    ) -> Self {
        let lower_condition: String = format!("{} > {}", column, format_value(lower));
        let upper_condition: String = format!("{} < {}", column, format_value(upper));
        self.conditions.push(lower_condition);
        self.conditions.push(upper_condition);
        self.add_filtering_clause();
        self
    }

    //works
    pub fn like(
        mut self,
        column: &str,
        pattern: &str
    ) -> Self {
        let condition: String = format!("{} LIKE '{}'", column, pattern);
        self.conditions.push(condition);
        self.add_filtering_clause();
        self
    }

}