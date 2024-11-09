use std::path::PathBuf;
use std::sync::Arc;
use askama::Template;
use config::{Config, File};
use axum::{Extension, Form, Router};
use axum::extract::{FromRequest, Query, Request, State};
use axum::http::{StatusCode, HeaderMap, HeaderValue, header::SET_COOKIE};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response, Html};
use axum::routing::{post, get};
use jsonwebtoken::{decode, encode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use once_cell::sync::Lazy;
use sqlx::Error;

use crate::core::{context::Context, query_params::ActivationParams};
use crate::routes::minify_html_response;
use crate::model::user::{SessionUser, UserSignUpForm, User};
use crate::services::{
    user::{check_if_user_exists, create_user, find_login_user},
    mail::send_registration_confirmation_mail,
};
use crate::view::auth::{LoginPageTemplate, RegisterPageTemplate};
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
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let auth_session = match request.headers().get("cookie") {
        None => "".to_string(),
        Some(header) => {
            let split = header.to_str().unwrap().rsplit(";");
            let found_header = split.into_iter().find(|v| {
                let header_value_vec = v.trim().rsplit("=").collect::<Vec<&str>>();
                match header_value_vec.get(1) {
                    Some(val) => *val == COOKIE_NAME,
                    None => false
                }
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
        let is_illegal_access = match request.uri().path() {
            "/" => false,
            "/imprint" => false,
            "/about" => false,
            "/login" => false,
            "/registrieren" => false,
            "/activate" => false,
            "/authorize" => false,
            "/register" => false,
            _ => true,
        };

        if is_illegal_access {
            println!("attempted access to \"{}\" is illegal", request.uri().path());
            return Ok((StatusCode::TEMPORARY_REDIRECT, [("Location", "/")]).into_response());
        }
    }

    request.extensions_mut().insert(Arc::new(authenticated_user));

    Ok(next.run(request).await)
}

pub async fn authorize(
    state: State<AppState>,
    request: Request,
) -> impl IntoResponse {
    let uri = request.uri().clone();
    let req_headers = request.headers().clone();
    let context = Context::new(&uri, &req_headers);
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
        match find_login_user(
            &state.db_pool,
            form_data.email.to_string(),
            form_data.password.to_string()
        ).await {
            Ok(existing_user) => {
                if existing_user.confirmation_token.is_none() {
                    let session_user = SessionUser::new(existing_user);
                    headers.insert(SET_COOKIE, create_auth_cookie_for_user(&session_user));
                    headers.insert("hx-redirect", "/".parse().unwrap());
                    return (StatusCode::FOUND, headers).into_response();
                }

                errors.push(format!("Der Benutzer wurde noch nicht aktiviert."));
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
        };
    }

    let template = LoginPageTemplate {
        authenticated_user: &None,
        notification: None,
        errors: &Some(errors),
        context: context,
    };

    (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
}

pub async fn register(
    state: State<AppState>,
    request: Request,
) -> impl IntoResponse {
    let uri = request.uri().clone();
    let req_headers = request.headers().clone();
    let context = Context::new(&uri, &req_headers);
    let form_data = Form::<UserSignUpForm>::from_request(request, &state).await.unwrap();

    let mut is_success = false;
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
            match create_user(
                &state.db_pool,
                &form_data,
            ).await {
                Ok(created_user) => {
                    match send_registration_confirmation_mail(
                        &form_data.email,
                        &created_user.confirmation_token,
                    ).await {
                        Ok(_) => {
                            is_success = true;
                            // TODO und die ganze /activate/?token sache noch
                        },
                        Err(e) => {
                            eprintln!("register, send_registration_confirmation_mail error: {e:?}");
                            errors.push(format!("Ein unerwarteter Fehler ist aufgetreten"));
                        }
                    }
                },
                Err(Error::RowNotFound) => {
                    errors.push(format!("Der Benutzer konnte nicht erstellt werden."));
                },
                Err(e) => {
                    eprintln!("register, error: {e:?}");
                    errors.push(format!("Ein unerwarteter Fehler ist aufgetreten"));
                }
            }
        }
    }

    let template = RegisterPageTemplate {
        authenticated_user: &None,
        notification: None,
        success: is_success,
        errors: &Some(errors),
        context: context,
    };

    (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
}

pub async fn logout() -> Result<Html<String>, (StatusCode, HeaderMap)> {
    let mut headers = HeaderMap::with_capacity(2);
    headers.insert(SET_COOKIE, format!("{COOKIE_NAME}=").parse().unwrap());
    headers.insert("hx-redirect", "/".parse().unwrap());
    Err((StatusCode::TEMPORARY_REDIRECT, headers))
}

pub async fn get_login_page(
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    request: Request,
) -> impl IntoResponse {
    if authenticated_user.is_some() {
        return (StatusCode::TEMPORARY_REDIRECT, [("Location", "/")]).into_response();
    }

    let context = Context::from_request(&request);
    let template = LoginPageTemplate {
        authenticated_user: &None,
        notification: None,
        errors: &None,
        context: context,
    };

    (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
}

pub async fn get_register_page(
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    request: Request,
) -> impl IntoResponse {
    if authenticated_user.is_some() {
        return (StatusCode::TEMPORARY_REDIRECT, [("Location", "/")]).into_response();
    }

    let context = Context::from_request(&request);
    let template = RegisterPageTemplate {
        authenticated_user: &None,
        notification: None,
        success: false,
        errors: &None,
        context: context,
    };

    (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
}

pub async fn activate_user(
    Query(query_params): Query<ActivationParams>,
    _state: State<AppState>,
    _request: Request,
) -> impl IntoResponse {
    println!("query_params.token {:?}", query_params.token);
    match query_params.token {
        Some(token) => {
            // TODO now confirm and login
            (StatusCode::TEMPORARY_REDIRECT, [("Location", "/mein-profil"), ("Set-Cookie", "mucki")]).into_response()
        },
        None => (StatusCode::TEMPORARY_REDIRECT, [("Location", "/")]).into_response()
    }
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

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/authorize", post(authorize))
        .route("/register", post(register))
        .route("/logout", post(logout))
        .route("/registrieren", get(get_register_page))
        .route("/activate", get(activate_user))
        .route("/login", get(get_login_page))
}