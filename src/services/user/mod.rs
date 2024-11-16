pub mod contacts;
use std::hash::{DefaultHasher, Hash, Hasher};
use sqlx::{Pool, Postgres, Error};
use crate::model::user::{ConfirmRegistrationUser, User, UserSignUpForm, UserUpdateForm};

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

pub async fn find_login_user(
    db_pool: &Pool<Postgres>,
    email: &str,
    password: &str,
) -> Result<User, Error> {
    let hashed_password = User::hash_password(password);
    sqlx::query_as::<_, User>(include_str!("./select_login_user.sql"))
        .bind(email)
        .bind(hashed_password)
        .fetch_one(db_pool)
        .await
}

pub async fn activate_registered_user(
    db_pool: &Pool<Postgres>,
    confirmation_token: &str,
) -> Result<User, Error> {
    sqlx::query_as::<_, User>(include_str!("./update_inactive_user.sql"))
        .bind(confirmation_token)
        .fetch_one(db_pool)
        .await
}

pub async fn create_user(db_pool: &Pool<Postgres>, form_data: &UserSignUpForm) -> Result<ConfirmRegistrationUser, Error> {
    let mut hasher = DefaultHasher::new();
    form_data.email.hash(&mut hasher);
    let confirmation_token = hasher.finish().to_string();
    
    sqlx::query_as::<_, ConfirmRegistrationUser>(include_str!("./create_user.sql"))
        .bind(form_data.email.as_str())
        .bind(User::hash_password(form_data.password.as_str()).as_str())
        .bind(confirmation_token)
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