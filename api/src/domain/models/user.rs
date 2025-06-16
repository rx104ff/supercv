use serde::{Serialize, Deserialize};

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Free,
    Pro,
}
