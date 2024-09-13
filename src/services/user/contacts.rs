use sqlx::{Pool, Postgres, Error};
use crate::model::user::contacts::{Contact, AddContactRequestForm};

pub async fn find_contacts(
    db_pool: &Pool<Postgres>,
    user_id: i64,
) -> Result<Vec<Contact>, Error> {
    Ok(vec![Contact::default()])
}

pub async fn add_contact_request(
    db_pool: &Pool<Postgres>,
    user_id: i64,
    form_data: &AddContactRequestForm,
) -> Result<(), Error> {
    sqlx::query_as::<_, _>(include_str!("./add_contact_request.sql"))
        .bind(user_id)
        .bind(form_data.contact_email.as_str())
        .bind("pending_contact_request")
        .fetch_one(db_pool)
        .await
}