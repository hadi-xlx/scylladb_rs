use std::fmt::Display;
use uuid::Uuid;

pub fn format_value<T: Display>(value: T) -> String {
    if value.to_string().parse::<Uuid>().is_ok() {
        value.to_string()
    } else if value.to_string().parse::<i64>().is_ok() {
        value.to_string()
    } else if value.to_string() == "true" {
        "True".to_string()
    } else if value.to_string() == "false" {
        "False".to_string()
    } else {
        format!("'{}'", value.to_string())
    }
}
