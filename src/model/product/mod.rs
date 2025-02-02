use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, postgres::PgRow};

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

#[derive(Debug)]
pub struct ListProduct<'a> {
    pub product: &'a Product,
    pub is_liked: bool,
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
                    created_at: None,
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

impl Product {
    pub fn format_price(&self) -> String {
        if self.current_price.is_none() {
            return "--.--".to_string();
        }

        self.current_price.as_ref().expect("current_price must be some after the is_none check").format()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Price {
    pub price: Option<i32>,
    pub currency: String, // TODO it's actually an enum, "GBP", "EUR", "USD" usw.
    pub created_at: Option<DateTime<Utc>>,
}

impl Price {
    pub fn format(&self) -> String {
        if self.price.is_none() {
            return "--.--".to_string();
        }

        let price_val = self.price.expect("self.price must be some after the is_none check").to_string();
        let euros = if price_val.len() > 2 {
            &price_val[..(price_val.len() - 2)]
        } else {
            "0"
        };
        let cents = &price_val[(price_val.len() - 2)..];

        format!("{}.{} {}", euros, cents, self.currency)
    }
}

pub fn get_prices_stats(prices: &Vec<Price>) -> (u32, u32, i64) {
    if prices.len() == 0 {
        return (0, 0, 0);
    }

    let mut max = 0;
    let mut min = u32::MAX;
    let mut min_timestamp = i64::MAX;
    let mut max_timestamp = 0;

    prices.iter().for_each(|v| {
        match v.price {
            Some(price) => {
                let price: u32 = price.try_into().unwrap_or(0);
                if price > max {
                    max = price;
                }
                if price < min {
                    min = price;
                }
            },
            None => ()
        };
        match v.created_at {
            Some(date) => {
                let timestamp = date.timestamp();
                if timestamp > max_timestamp {
                    max_timestamp = timestamp;
                }
                if timestamp < min_timestamp {
                    min_timestamp = timestamp;
                }
            },
            None => ()
        }
    });

    (min, max, max_timestamp - min_timestamp)
}