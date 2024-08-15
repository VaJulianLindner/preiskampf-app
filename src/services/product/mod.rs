use sqlx::{Error, FromRow, Pool, Postgres, Row};
use crate::core::query_params::SortOrder;

use crate::model::product::Product;

// TODO pass reference to &StateParams or sth like pagination::Pagination instead of every param as single 
// and do the injection checking there
pub async fn find_products(
    db_pool: &Pool<Postgres>,
    search_query: Option<String>,
    sort_by: String,
    sort_order: String,
    limit: usize,
    offset: usize,
) -> (Vec<Product>, u64) {
    // https://www.reddit.com/r/rust/comments/17hoxzl/performance_on_multiple_statements_sqlx_sql/
    // get page-count without another db round-trip => transaction?
    // or use begin() and commit() as shown above?
    // right now its just an additional count() in the statement
    // because i have a bigger connection pool, joining futures might also be a win? https://docs.rs/futures/latest/futures/macro.join.html
    // TODO => performance tests!

    // TODO sanitize inputs by putting in .hbs or somethign? => nur "sort_by" ist theoretisch ungecheckt
    // TODO absolutely have to sanitize the search_query!!
    let query_sort_order = SortOrder::from_str(sort_order.as_str()).to_string();
    // let statement = if let Some(search_query) = search_query {
    let statement = if search_query.as_deref().is_some_and(|q| q != "") {
        format!(
            include_str!("./find_products_by_text.sql"),
            "german", // TODO localization is needed, do some proper Enum
            search_query.expect("make sure \"search_query\" got checked by \"is_some_and\" function call"),
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
            // TODO might some real generic unwrapping + finding total on db-queries
            // TODO also further up in this file and in the routes::templates i can find a clean abstract way for handling the query-param parsing+input
            // and then parse the list output here
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