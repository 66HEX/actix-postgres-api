use serde::{Deserialize, Serialize};

// Enum reprezentujący role użytkowników
#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum UserRole {
    Client,
    Trainer,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Client
    }
}

// Implementacja konwersji z i do stringa dla UserRole
impl ToString for UserRole {
    fn to_string(&self) -> String {
        match self {
            UserRole::Client => "client".to_string(),
            UserRole::Trainer => "trainer".to_string(),
        }
    }
}

impl From<&str> for UserRole {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "trainer" => UserRole::Trainer,
            _ => UserRole::Client, // domyślnie ustawiamy Client
        }
    }
}