use sqlx::{Pool, Postgres, Error};
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

pub async fn add_contact_request(
    db_pool: &Pool<Postgres>,
    user_id: &i64,
    form_data: &AddContactRequestForm,
) -> Result<(), Error> {
    sqlx::query_as::<_, _>(include_str!("./add_contact_request.sql"))
        .bind(user_id)
        .bind(form_data.contact_email.as_str())
        .bind("pending_contact_request")
        .fetch_one(db_pool)
        .await
}