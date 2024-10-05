use sqlx::{Error, FromRow, Pool, Postgres, Row};
use crate::core::query_params::SortOrder;

use crate::model::product::{Price, Product};

pub async fn find_products(
    db_pool: &Pool<Postgres>,
    search_query: Option<String>,
    sort_by: String,
    sort_order: String,
    limit: usize,
    offset: usize,
) -> (Vec<Product>, u64) {
    // https://www.reddit.com/r/rust/comments/17hoxzl/performance_on_multiple_statements_sqlx_sql/

    let query_sort_order = SortOrder::from_str(sort_order.as_str()).to_string();
    // TODO sanitize inputs for text search
    let statement = if search_query.as_deref().is_some_and(|q| q != "") {
        format!(
            // include_str!("./find_products_by_text.sql"),
            // "german", // TODO localization is needed, do some proper Enum
            include_str!("./find_products_by_similarity.sql"),
            search_query.as_ref().unwrap(),
            search_query.as_ref().unwrap(),
            limit as i64,
            offset as i64,
        )
    } else {
        format!(
            include_str!("./find_products.sql"),
            sort_by,
            query_sort_order,
            limit as i64,
            offset as i64,
        )
    };
    let query = sqlx::query::<_>(statement.as_str());

    match query
        .fetch_all(db_pool)
        .await {
            Ok(rows) => {
                let products = rows.iter()
                    .map(|row| Product::from_row(row).unwrap())
                    .collect::<Vec<Product>>();
                let total: u64 = match rows.get(0) {
                    Some(row) => row.try_get::<i64, &str>("total").unwrap_or_default() as u64,
                    None => 0
                };
                (products, total)
            },
            Err(e) => {
                eprintln!("error in services::product::find_products: {:?}", e);
                (vec![], 0)
            }
        }      
}

pub async fn find_product(
    db_pool: &Pool<Postgres>,
    product_id: &str,
) -> Result<Product, Error> {
    sqlx::query_as::<_, Product>(include_str!("./find_product.sql"))
        .bind(product_id)
        .fetch_one(db_pool)
        .await    
}

pub async fn find_product_prices(
    db_pool: &Pool<Postgres>,
    product_id: &str,
) -> Result<Vec<Price>, Error> {
    match sqlx::query::<_>(include_str!("./find_product_prices.sql"))
        .bind(product_id)
        .fetch_all(db_pool)
        .await {
            Ok(rows) => {
                Ok(
                    rows.iter()
                        .map(|row| Price::from_row(row).unwrap())
                        .collect::<Vec<Price>>()
                )
            },
            Err(e) => {
                eprintln!("error in product::services::find_product_prices {:?}", e);
                Ok(Vec::with_capacity(0))
            },
        }
}