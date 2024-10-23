use sqlx::{postgres::PgQueryResult, Error, FromRow, Pool, Postgres, Row};
use crate::model::shopping_list::{
    ShoppingList, ShoppingListItem, ShoppingListUpdateForm, ToggleShoppingListItemOp
};


pub async fn upsert_shopping_list(
    db_pool: &Pool<Postgres>,
    user_id: &i64,
    form_data: &ShoppingListUpdateForm,
) -> Result<ShoppingList, Error> {
    let emoji = match form_data.emoji_presentation.as_ref() {
        Some(val) => val,
        None => "",
    };
    if form_data.id.is_some() {
        let query_builder = sqlx::query_as::<_, ShoppingList>(include_str!("./update_shopping_list.sql"));
        let query = query_builder
            .bind(user_id)
            .bind(form_data.name.as_str())
            .bind(emoji)
            .bind(form_data.id.unwrap());
        query.fetch_one(db_pool).await       
    } else {
        let query_builder = sqlx::query_as::<_, ShoppingList>(include_str!("./insert_shopping_list.sql"));
        let query = query_builder
            .bind(user_id)
            .bind(form_data.name.as_str())
            .bind(emoji);
        query.fetch_one(db_pool).await
    }
}

pub async fn find_shopping_list(
    db_pool: &Pool<Postgres>, 
    id: &i64,
    user_id: &i64,
) -> Result<ShoppingList, Error> {
    sqlx::query_as::<_, ShoppingList>(include_str!("./find_shopping_list.sql"))
        .bind(id)
        .bind(user_id)
        .fetch_one(db_pool)
        .await
}

pub async fn find_shopping_list_items(
    db_pool: &Pool<Postgres>, 
    id: &i64,
    user_id: &i64,
) -> Result<Vec<ShoppingListItem>, Error> {
    sqlx::query_as::<_, ShoppingListItem>(include_str!("./find_shopping_list_items.sql"))
        .bind(id)
        .bind(user_id)
        .fetch_all(db_pool)
        .await
}

pub async fn find_shopping_lists(
    db_pool: &Pool<Postgres>, 
    user_id: i64,
    limit: usize,
    offset: usize,
) -> (Vec<ShoppingList>, u64) {
    match sqlx::query::<_>(include_str!("./find_shopping_list_by_user_id.sql"))
        .bind(user_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(db_pool)
        .await {
            Ok(rows) => {
                let shopping_lists = rows.iter()
                    .map(|row| ShoppingList::from_row(row).unwrap())
                    .collect::<Vec<ShoppingList>>();
                let total: u64 = match rows.get(0) {
                    Some(row) => row.get::<i64, &str>("total") as u64,
                    None => 0
                };
                (shopping_lists, total)
            },
            Err(e) => {
                eprintln!("error in services::shopping_list::find_shopping_lists: {:?}", e);
                (vec![], 0)
            }
        }
}

pub async fn delete_shopping_list(
    db_pool: &Pool<Postgres>, 
    user_id: i64,
    id: i64,
) -> Result<ShoppingList, Error> {
    sqlx::query_as::<_, ShoppingList>(include_str!("./delete_shopping_list.sql"))
        .bind(user_id)
        .bind(id)
        .fetch_one(db_pool)
        .await
}

pub async fn add_product_to_list(
    db_pool: &Pool<Postgres>,
    user_id: &i64,
    shopping_list_id: &i64,
    product_id: &str,
    amount: i64,
) -> Result<PgQueryResult, Error> {
    sqlx::query(include_str!("./insert_shopping_list_item.sql"))
        .bind(user_id)
        .bind(product_id)
        .bind(shopping_list_id)
        .bind(amount)
        .execute(db_pool)
        .await
}

pub async fn toggle_shopping_list_item(
    db_pool: &Pool<Postgres>,
    user_id: &i64,
    shopping_list_id: &i64,
    product_id: &str,
    amount: i64,
) -> Result<ToggleShoppingListItemOp, Error> {
    sqlx::query_as::<_, ToggleShoppingListItemOp>(include_str!("./toggle_shopping_list_item.sql"))
        .bind(user_id)
        .bind(product_id)
        .bind(shopping_list_id)
        .bind(amount)
        .fetch_one(db_pool)
        .await
}