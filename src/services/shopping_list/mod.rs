use sqlx::{Error, Row, FromRow, Pool, Postgres};
use crate::model::shopping_list::{ShoppingList, ShoppingListUpdateForm};


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
    id: Option<i64>,
) -> Result<ShoppingList, Error> {
    sqlx::query_as::<_, ShoppingList>(include_str!("./find_shopping_list.sql"))
        .bind(id.unwrap_or(0))
        .fetch_one(db_pool)
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

pub async fn save_shopping_list_item(
    _db_pool: &Pool<Postgres>,
    _user_id: i64,
    _shopping_list_id: i64,
    _product_id: i64,
    _amount: i64,
) -> Result<(), Error> {
    // TODO add product to a shopping list for this user 
    /* sqlx::query("INSERT INTO shopping_list_items (shopping_list_id, product_id, amount) VALUES ($1, $2, $3)")
        .bind(user_id)
        .bind(shopping_list_id)
        .bind(product_id)
        .bind(amount)
        .execute(db_pool)
        .await?; */

    Ok(())
}