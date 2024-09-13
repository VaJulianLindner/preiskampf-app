use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Contact {
    pub id: i64,
    pub by_user_id: i64,
    pub to_user_id: i64,
    state: String,
}

impl Contact {
    pub fn default() -> Self {
        Self {
            id: 0,
            by_user_id: 0,
            to_user_id: 0,
            state: "pending_contact_request".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AddContactRequestForm {
    pub contact_email: String,
}