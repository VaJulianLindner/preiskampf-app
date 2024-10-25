use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};
use sqlx::{FromRow, Row, postgres::PgRow};

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

#[derive(Debug, Deserialize)]
pub struct AddShoppingListItemForm {
    pub shopping_list_id: Option<i64>,
    pub product_id: String,
}

#[derive(Debug)]
pub enum ToggleShoppingListItemOp {
    Added,
    Removed,
    Unknown,
}

impl ToggleShoppingListItemOp {
    pub fn from_number(num: i16) -> Self {
        match num {
            0 => ToggleShoppingListItemOp::Removed,
            1 => ToggleShoppingListItemOp::Added,
            _ => ToggleShoppingListItemOp::Unknown,
        }
    }
}

impl<'r> FromRow<'r, PgRow> for ToggleShoppingListItemOp {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Result::Ok(ToggleShoppingListItemOp::from_number(row.try_get(0)?))
    }
}

#[derive(Debug)]
pub struct ShoppingListItem {
    pub product_id: String,
}

impl<'r> FromRow<'r, PgRow> for ShoppingListItem {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(ShoppingListItem {
            product_id: row.try_get("product_id")?
        })
    }
}

pub trait VecExt {
    fn to_hashset(&self) {

    }
}

impl VecExt for Vec<ShoppingListItem> {
    fn to_hashset(&self) {

    }
}