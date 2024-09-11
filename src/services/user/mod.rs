use sqlx::{Pool, Postgres, Error};

use crate::model::user::{User, UserUpdateForm, UserSignUpForm};

pub async fn check_if_user_exists(db_pool: &Pool<Postgres>, email: &str) -> bool {
    match sqlx::query::<_>(include_str!("./select_user_by_email.sql"))
        .bind(email)
        .fetch_one(db_pool)
        .await {
            Ok(_) => true,
            Err(sqlx::Error::RowNotFound) => false,
            Err(e) => {
                println!("check_if_user_exists, error: {:?} for email \"{}\"", e, email);
                false
            }
        }
}

pub async fn find_user(db_pool: &Pool<Postgres>, email: String, password: String) -> Result<User, Error> {
    let hashed_password = User::hash_password(password);
    sqlx::query_as::<_, User>(include_str!("./select_user.sql"))
        .bind(email)
        .bind(hashed_password)
        .fetch_one(db_pool)
        .await
}

pub async fn create_user(db_pool: &Pool<Postgres>, form_data: &UserSignUpForm) -> Result<User, Error> {
    sqlx::query_as::<_, User>(include_str!("./create_user.sql"))
        .bind(form_data.email.to_string())
        .bind(User::hash_password(form_data.password.to_string()))
        .fetch_one(db_pool)
        .await
}

pub async fn update_user(db_pool: &Pool<Postgres>, form_data: &UserUpdateForm) -> Result<User, Error> {
    sqlx::query_as::<_, User>(include_str!("./update_user.sql"))
        .bind(form_data.id)
        .bind(form_data.username.as_str())
        .bind(form_data.address.as_str())
        .bind(form_data.address_lng)
        .bind(form_data.address_lat)
        .fetch_one(db_pool)
        .await
}

pub async fn update_selected_shopping_list(
    db_pool: &Pool<Postgres>,
    user_id: i64,
    shopping_list_id: i64,
) -> Result<(), Error> {
    sqlx::query_as::<_, _>(include_str!("./update_selected_shopping_list.sql"))
        .bind(user_id)
        .bind(shopping_list_id)
        .fetch_one(db_pool)
        .await
}

pub async fn add_contact_request(
    db_pool: &Pool<Postgres>,
    user_id: i64,
    requested_users_email: &str,
) -> Result<(), Error> {
    sqlx::query_as::<_, _>(include_str!("./add_contact_request.sql"))
        .bind(user_id)
        .bind(requested_users_email)
        .fetch_one(db_pool)
        .await
}