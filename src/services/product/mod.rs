use sqlx::{Error, FromRow, Pool, Postgres, Row};
use crate::core::query_params::SortOrder;

use crate::model::product::{Price, Product};

pub async fn find_products(
    db_pool: &Pool<Postgres>,
    search_query: Option<String>,
    _shopping_list_id: Option<i64>,
    sort_by: String,
    sort_order: String,
    limit: usize,
    offset: usize,
) -> Result<Vec<Product>, Error> {
    // https://www.reddit.com/r/rust/comments/17hoxzl/performance_on_multiple_statements_sqlx_sql/

    let query_sort_order = SortOrder::from_str(sort_order.as_str()).to_string();
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
    let query = sqlx::query_as::<_, Product>(statement.as_str());

    query
        .fetch_all(db_pool)
        .await      
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
    sqlx::query_as::<_, Price>(include_str!("./find_product_prices.sql"))
        .bind(product_id)
        .fetch_all(db_pool)
        .await
}