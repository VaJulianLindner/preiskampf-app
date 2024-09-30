use sqlx::{postgres::{PgQueryResult, PgRow}, Error, Pool, Postgres};
use crate::model::user::contacts::{LinkedContact, AddContactRequestForm};

pub async fn find_contacts(
    db_pool: &Pool<Postgres>,
    user_id: &i64,
) -> Result<Vec<LinkedContact>, Error> {
    // TODO this will need a limit/pagination, esp. for existing contacts => join futures and to several db-reqs/a transaction
    sqlx::query_as::<_, LinkedContact>(include_str!("./select_contacts.sql"))
        .bind(user_id)
        .fetch_all(db_pool)
        .await
}

pub async fn find_requested_contacts(
    db_pool: &Pool<Postgres>,
    user_id: &i64,
) -> Result<Vec<LinkedContact>, Error> {
    sqlx::query_as::<_, LinkedContact>(include_str!("./select_requested_contacts.sql"))
        .bind(user_id)
        .fetch_all(db_pool)
        .await
}

pub async fn find_pending_contacts(
    db_pool: &Pool<Postgres>,
    user_id: &i64,
) -> Result<Vec<LinkedContact>, Error> {
    sqlx::query_as::<_, LinkedContact>(include_str!("./select_pending_contacts.sql"))
        .bind(user_id)
        .fetch_all(db_pool)
        .await
}

pub async fn delete_request(
    db_pool: &Pool<Postgres>,
    user_id: &i64,
    contact_id: &i64,
) -> Result<PgRow, Error> {
    sqlx::query::<_>(include_str!("./delete_request.sql"))
        .bind(user_id)
        .bind(contact_id)
        .fetch_one(db_pool)
        .await
}

pub async fn confirm_request(
    db_pool: &Pool<Postgres>,
    by_user_id: &i64,
) -> Result<LinkedContact, Error> {
    sqlx::query_as::<_, LinkedContact>(include_str!("./confirm_request.sql"))
        .bind(by_user_id)
        .fetch_one(db_pool)
        .await
}

pub async fn add_contact_request(
    db_pool: &Pool<Postgres>,
    by_user_id: &i64,
    to_user_id: &i64,
    state: Option<&str>,
) -> Result<PgQueryResult, Error> {
    let state = match state {
        Some(val) => val,
        None => "pending_contact_request",
    };
    sqlx::query(include_str!("./add_contact_request.sql"))
        .bind(by_user_id)
        .bind(to_user_id)
        .bind(state)
        .execute(db_pool)
        .await
}

pub async fn add_contact_request_by_email(
    db_pool: &Pool<Postgres>,
    user_id: &i64,
    form_data: &AddContactRequestForm,
    state: Option<&str>,
) -> Result<LinkedContact, Error> {
    let state = match state {
        Some(val) => val,
        None => "pending_contact_request",
    };
    sqlx::query_as::<_, LinkedContact>(include_str!("./add_contact_request_by_email.sql"))
        .bind(user_id)
        .bind(form_data.contact_email.as_str())
        .bind(state)
        .fetch_one(db_pool)
        .await
}