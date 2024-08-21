use std::path::PathBuf;
use std::convert::Into;
use std::sync::Arc;
use std::collections::HashMap;
use config::{Config, File};
use axum::Form;
use axum::extract::{FromRequest, Path, Request, State};
use axum::http::{StatusCode, HeaderMap, HeaderValue};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response, Html};
use jsonwebtoken::{decode, encode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use once_cell::sync::Lazy;
use serde_json::json;
use sqlx::Error;

use crate::routes::{minify_html_response, get_value_from_path};
use crate::model::user::{SessionUser, UserSignUpForm};
use crate::services::user::{check_if_user_exists, create_user, find_user};
use crate::AppState;

pub const COOKIE_NAME: &str = "preiskampf_auth_cookie";

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let settings = Config::builder()
        .add_source(File::from(PathBuf::from(format!("{}/config/auth.json", manifest_dir))))
        .build().expect("Auth config must be provided in /config/auth.json");
    Keys::new(settings.get::<String>("jwtSecret")
        .expect("\"jwtSecret\" must be provided in /config/auth.json").as_bytes())
});

pub async fn validate(
    path: Path<HashMap<String, String>>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let auth_session = match request.headers().get("cookie") {
        None => "".to_string(),
        Some(header) => {
            let split = header.to_str().unwrap().rsplit(";");
            let found_header = split.into_iter().find(|v| {
                let header_value_vec = v.trim().rsplit("=").collect::<Vec<&str>>();
                header_value_vec[1] == COOKIE_NAME
            });
            match found_header {
                None => "".to_string(),
                Some(cookie) => String::from(cookie.trim().rsplit("=").nth(0).unwrap_or(""))
            }
        }
    };

    let decoded_user = decode::<SessionUser>(&auth_session, &KEYS.decoding, &Validation::new(Algorithm::HS256));
    let authenticated_user = match decoded_user {
        Ok(u) => Some(u.claims.user),
        Err(_) => None,
    };

    // if user is not authenticated and trying to access a protected route, redirect to index
    if authenticated_user.is_none() {
        // TODO this blacklisting sucks with the controller and explicit routes
        let template_name = get_value_from_path(&path, "template_name");
        match template_name.as_ref() {
            "index" => (),
            "about" => (),
            "login" => (),
            "registrieren" => (),
            _ => ()
            // _ => {
            //     return Ok((StatusCode::FOUND, [("Location", "/")], minify_html_response(String::from(""))).into_response())
            // }
        }
    }

    request.extensions_mut().insert(Arc::new(authenticated_user));

    Ok(next.run(request).await)
}

pub async fn authorize(
    state: State<AppState>,
    request: Request,
) -> impl IntoResponse {
    let form_data = Form::<UserSignUpForm>::from_request(request, &state).await.unwrap();
    
    let mut headers = HeaderMap::new();
    let mut errors: Vec<String> = vec![];

    if form_data.password == "" {
        errors.push(format!("Bitte geben Sie ein gültiges Passwort an."));
    }
    if form_data.email == "" {
        errors.push(format!("Bitte geben Sie eine gültige Email an."));
    }

    if errors.len() == 0 {
        let existing_user_result = find_user(&state.db_pool, form_data.email.to_string(), form_data.password.to_string()).await;
        match existing_user_result {
            Ok(existing_user) => {
                let session_user = SessionUser::new(existing_user);
                headers.insert("set-cookie", create_auth_cookie_for_user(&session_user));
                headers.insert("hx-redirect", "/".parse().unwrap());
                return (StatusCode::FOUND, headers).into_response();
            },
            Err(e) => {
                match e {
                    Error::RowNotFound => {
                        errors.push(format!("Der Benutzer konnte nicht gefunden werden."));
                    }
                    _ => {
                        errors.push(format!("Ein unerwarteter Fehler ist aufgetreten"));
                    }
                }
                eprintln!("authorize, error: {e:?}");
            }
        }
    }

    let rendered_content = state.engine.render("partials/elements/login", &(json!({
        "errors": errors,
        "password": form_data.password.as_str(),
        "email": form_data.email.as_str(),
    }))).unwrap_or("".to_string());

    (StatusCode::OK, headers, minify_html_response(rendered_content)).into_response()
}

pub async fn register(
    state: State<AppState>,
    Form(form_data): Form<UserSignUpForm>,
) -> Result<Html<String>, (StatusCode, HeaderMap)> {
    let mut headers = HeaderMap::new();
    let mut errors: Vec<String> = vec![];

    if form_data.password == "" {
        errors.push(format!("Bitte geben Sie ein gültiges Passwort an."));
    }
    if form_data.email == "" {
        errors.push(format!("Bitte geben Sie eine gültige Email an."));
    }

    if errors.len() == 0 {
        let does_user_exist: bool = check_if_user_exists(&state.db_pool, form_data.email.as_str()).await;
        if does_user_exist {
            errors.push(format!("Ein Benutzer mit der Email \"{}\" existiert bereits.", form_data.email.as_str()));
        } else {
            let created_user_result = create_user(
                &state.db_pool,
                &form_data,
            ).await;
            match created_user_result {
                Ok(created_user) => {
                    let session_user = SessionUser::new(created_user);
                    let token = encode(&Header::default(), &session_user, &KEYS.encoding).unwrap_or("".to_string());
                    headers.insert("set-cookie", format!("{COOKIE_NAME}={}", token).parse().unwrap());
                    headers.insert("hx-redirect", "/".parse().unwrap());
                    return Err((StatusCode::FOUND, headers));
                },
                Err(e) => {
                    match e {
                        Error::RowNotFound => {
                            errors.push(format!("Der Benutzer konnte nicht erstellt werden."));
                        }
                        _ => {
                            errors.push(format!("Ein unerwarteter Fehler ist aufgetreten"));
                        }
                    }
                    eprintln!("register, error: {e:?}");
                }
            }
        }
    }

    let rendered_content = state.engine.render("partials/elements/register", &(json!({
        "errors": errors,
        "password": form_data.password.as_str(),
        "email": form_data.email.as_str(),
    }))).unwrap_or("".to_string());

    Ok(minify_html_response(rendered_content))
}

pub async fn logout(
    mut headers: HeaderMap,
) -> Result<Html<String>, (StatusCode, HeaderMap)> {
    headers.insert("set-cookie", format!("{COOKIE_NAME}=").parse().unwrap());
    headers.insert("hx-redirect", "/".parse().unwrap());
    Err((StatusCode::FOUND, headers))
}

pub fn create_auth_cookie_for_user(user: &SessionUser) -> HeaderValue {
    let token = encode(&Header::default(), user, &KEYS.encoding).unwrap_or("".to_string());
    format!("{COOKIE_NAME}={}; Path=/; Secure; HttpOnly", token).parse().unwrap()
}

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}