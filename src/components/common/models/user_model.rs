use serde::{Deserialize, Serialize};

/// Struct for the data of a user.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl User {
    pub fn get_query(key: &str) -> String {
        format!(
            "{}.id, {}.first_name, {}.last_name, {}.email",
            key, key, key, key
        )
    }
}
