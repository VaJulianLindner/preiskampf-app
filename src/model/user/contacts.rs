use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, postgres::PgRow};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Contact {
    pub id: i64,
    pub by_user_id: i64,
    pub to_user_id: i64,
    // TODO need an enum because this is whacky shit
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedContact {
    pub contact: Contact,
    pub email: String,
}

impl<'r> FromRow<'r, PgRow> for LinkedContact {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            contact: Contact {
                id: row.try_get("id")?,
                by_user_id: row.try_get("by_user_id")?,
                to_user_id: row.try_get("to_user_id")?,
                state: row.try_get("state")?,
            },
            email: row.try_get("email")?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct AddContactRequestForm {
    pub contact_email: String,
}