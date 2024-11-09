pub mod contacts;

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

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Option<i64>,
    pub email: String,
    pub password: Option<String>,
    pub username: Option<String>,
    pub confirmation_token: Option<String>,
    #[sqlx(default)]
    pub selected_shopping_list_id: Option<i64>,
    #[sqlx(default)]
    pub address: Option<String>,
    #[sqlx(default)]
    pub address_lng: Option<f64>,
    #[sqlx(default)]
    pub address_lat: Option<f64>,
}

#[derive(FromRow)]
pub struct ConfirmRegistrationUser {
    pub confirmation_token: String,
}

impl User {
    pub(crate) fn get_id(&self) -> &Option<i64> {
        &self.id
    }

    pub(crate) fn get_email(&self) -> &str {
        &self.email
    }

    pub(crate) fn get_username(&self) -> &str {
        match self.username.as_ref() {
            Some(username) => username,
            None => ""
        }
    }

    pub(crate) fn get_address(&self) -> &str {
        match self.address.as_ref() {
            Some(address) => address,
            None => ""
        }
    }

    pub(crate) fn hash_password(password: &str) -> String {
        sha512_crypt::hash_with(PW_HASH, password).unwrap_or("default".to_string())
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