use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ShoppingList {
    pub id: i64,
    pub created_at: DateTime<FixedOffset>,
    pub name: String,
    #[sqlx(default)]
    pub user_id: i64,
    pub emoji_presentation: Option<String>
}

impl ShoppingList {
    pub fn default() -> ShoppingList {
        Self {
            id: 0,
            created_at: DateTime::default(),
            name: String::default(),
            user_id: 0,
            emoji_presentation: None,
        }
    }

    pub fn get_id(&self) -> &i64 {
        &self.id
    }

    pub fn get_user_id(&self) -> &i64 {
        &self.user_id
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Debug, Deserialize)]
pub struct ShoppingListUpdateForm {
    pub id: Option<i64>,
    pub emoji_presentation: Option<String>,
    pub name: String,
}