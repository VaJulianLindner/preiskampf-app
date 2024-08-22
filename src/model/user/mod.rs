use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use pwhash::sha512_crypt;

const PW_HASH: &str = "$6$G/gkPn17kHYo0gTF$xhDFU0QYExdMH2ghOWKrrVtu1BuTpNMSJURCXk43.EYekmK8iwV6RNqftUUC8mqDel1J7m3JEbUkbu4YyqSyv/";

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub user: User,
    pub exp: usize,
}

impl SessionUser {
    pub fn new(user: User) -> Self {
        let exp = (Utc::now() + Duration::days(30)).timestamp_millis() as usize;
        Self {
            user,
            exp,
        }
    }
}

// TODO https://stackoverflow.com/questions/75431759/how-to-use-sqlx-query-as-to-fetch-some-of-the-model-fields#answer-76399876
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Option<i64>,
    pub email: String,
    pub password: Option<String>,
    pub username: Option<String>,
    #[sqlx(default)]
    pub selected_shopping_list_id: Option<i64>,
    #[sqlx(default)]
    pub address: Option<String>,
    #[sqlx(default)]
    pub address_lng: Option<f64>,
    #[sqlx(default)]
    pub address_lat: Option<f64>,
}

impl User {
    pub(crate) fn new(email: String, password: String) -> Self {
        let hashed_password = Self::hash_password(password);
        Self {
            email,
            password: Some(hashed_password),
            id: None,
            username: None,
            selected_shopping_list_id: None,
            address: None,
            address_lng: None,
            address_lat: None
        }
    }

    pub(crate) fn get_id(&self) -> &Option<i64> {
        &self.id
    }

    pub(crate) fn get_email(&self) -> &str {
        &self.email
    }

    pub(crate) fn get_username(&self) -> String {
        match self.username.as_ref() {
            Some(username) => username.to_owned(),
            None => "".to_owned()
        }
    }

    pub(crate) fn get_address(&self) -> String {
        match self.address.as_ref() {
            Some(address) => address.to_owned(),
            None => "".to_owned()
        }
    }

    pub(crate) fn hash_password(password: String) -> String {
        sha512_crypt::hash_with(PW_HASH, password).unwrap_or("default".to_string())
    }

    pub(crate) fn verify_password(self, password: &str) -> bool {
        if self.password.is_none() {
            return false;
        }
        sha512_crypt::verify(password, self.password.expect("you cant verify a password if there is no password").as_str())
    }
}

#[derive(Debug, Deserialize)]
pub struct UserUpdateForm {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub address: String,
    pub address_lng: Option<f64>,
    pub address_lat: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct UserSignUpForm {
    pub password: String,
    pub email: String,
}