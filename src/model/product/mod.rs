use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, postgres::PgRow};

// TODO price should be normalized in the products table to avoid joins on query!
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub images: Vec<String>,
    pub url: String,
    pub market_id: i64, // TODO it's actually an enum, "LIDL", "TESCO", "SAINSBURY" usw. => get it from the ID! or via SQL JOIN
    pub current_price: Option<Price>,
}

impl<'r> FromRow<'r, PgRow> for Product {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let created_at = row.try_get("created_at")?;
        let name = row.try_get("name")?;
        let images = row.try_get("images")?;
        let url = row.try_get("url")?;
        let market_id = row.try_get("market_id")?;

        let price = row.try_get("price");
        let currency = row.try_get("currency");

        let mut current_price = None;
        if currency.is_ok() {
            current_price = Some(
                Price {
                    price: price.ok(),
                    currency: currency.unwrap(),
                }
            );
        }

        Ok(Product {
            id,
            created_at,
            name,
            images,
            url,
            market_id,
            current_price,
        })
    }
}

impl<'a> Product {
    pub fn format_price(&self) -> String {
        if self.current_price.is_none() {
            "--.--".to_string()
        } else {
            let price_ref = self.current_price.as_ref();
            if price_ref.unwrap().price.is_none() {
                "--.--".to_string()
            } else {
                let price = price_ref.unwrap();
                let mut price_val = price.price.unwrap().to_string();
                let cents = price_val.split_off(price_val.len() - 2);
                format!("{}.{} {}", price_val, cents, price.currency)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Price {
    pub price: Option<i32>,
    pub currency: String, // TODO it's actually an enum, "GBP", "EUR", "USD" usw.
}