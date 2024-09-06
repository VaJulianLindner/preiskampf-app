use askama::Template;
use axum::{
    extract::{Extension, Path, Request}, http::{header, Method, StatusCode, Uri}, middleware::Next, response::{Html, IntoResponse}
};
use chrono::{DateTime, Local};
use html_minifier::minify;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use crate::{
    core::{
        context::Context, request_extension::HttpExt
    },
    model::{misc::Notification, user::User},
};
use crate::view::{
    static_page::NotFoundTemplate,
    misc::NotificationTemplate,
};

pub mod auth;
pub mod controller;
pub mod api;

pub async fn print_timestamp_middleware( 
    method: Method,
    uri: Uri,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let before = Instant::now();
    let response = next.run(request).await;
    println!("processed request took {:.2?} for \"{}: {}\"", before.elapsed(), method, uri);
    response
}

pub async fn handle_not_found(
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    method: Method,
    request: Request,
) -> impl IntoResponse {
    let context = Context::from_request(&request);
    // TODO return type based on http-header requested method/content-type
    println!("handle_not_found, method: {}, uri: {}", method, context.uri);
    eprintln!("handle_not_found, method: {}, uri: {}", method, context.uri);

    let rendered_content = if context.is_boosted_request() {
        String::from("")
    } else {
        NotFoundTemplate {
            authenticated_user: &authenticated_user,
            notification: None,
            context: context,
        }.render().unwrap_or_default()
    };

    (
        StatusCode::NOT_FOUND,
        [(header::VARY, "Hx-Request, Hx-Boosted")],
        minify_html_response(rendered_content),
    ).into_response()
}

pub fn render_notification(
    message: &str,
    is_success: bool,
) -> String {
    NotificationTemplate {
        notification: Notification {
            is_oob_swap: true,
            is_success: is_success,
            message: message,
            hint: None,
        }
    }.render().unwrap_or(String::from(""))
}

pub fn render_success_notification(message: Option<&str>) -> String {
    render_notification(
        message.unwrap_or("Erfolgreich gespeichert"), 
        true,
    )
}

pub fn render_error_notification(message: Option<&str>) -> String {
    render_notification(
        message.unwrap_or("Ein unerwarteter Fehler ist aufgetreten"),
        false,
    )
}

pub fn create_notification(
    message: &str,
    is_success: bool,
) -> NotificationTemplate {
    NotificationTemplate {
        notification: Notification {
            is_oob_swap: true,
            is_success: is_success,
            message: message,
            hint: None,
        }
    }
}

pub fn create_success_notification(message: Option<&str>) -> NotificationTemplate {
    create_notification(
        message.unwrap_or("Erfolgreich gespeichert"), 
        true,
    )
}

pub fn create_error_notification(message: Option<&str>) -> NotificationTemplate {
    create_notification(
        message.unwrap_or("Ein unerwarteter Fehler ist aufgetreten"),
        false,
    )
}

// TODO should return a result and Err instead of String "forbidden"
pub fn get_value_from_path(path: &Path<HashMap<String, String>>, name: &str) -> String {
    match path.get(name) {
        None => {
            if path.len() == 0 {
                String::from("index")
            } else {
                String::from("forbidden")
            }
        },
        Some(template_name) => template_name.to_string(),
    }
}

pub fn minify_html_response(unprocessed_html: String) -> Html<String> {
    // TODO need to add [(header::VARY, "Hx-Request, Hx-Boosted")] in all of the responses!
    Html(minify(unprocessed_html).expect("unexpected error during minification"))
}