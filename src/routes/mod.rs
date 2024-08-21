use askama::Template;
use axum::{
    extract::{Extension, Path, Request, State}, http::{header, HeaderMap, Method, StatusCode, Uri}, middleware::Next, response::{Html, IntoResponse}, RequestExt
};
use chrono::{DateTime, Local};
use handlebars::{handlebars_helper, Handlebars, Helper, Context, RenderContext, Output, HelperResult};
use html_minifier::minify;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use crate::{core::request_extension::HttpExt, model::{misc::Notification, user::User}};
use crate::AppState;
use crate::view::misc::NotificationTemplate;

pub mod auth;
pub mod details;
pub mod templates;
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
    println!("print_timestamp_middleware, processed request took {:.2?} for \"{}: {}\"", before.elapsed(), method, uri);
    response
}

pub async fn handle_not_found(
    state: State<AppState>,
    headers: HeaderMap,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    method: Method,
    uri: Uri,
) -> impl IntoResponse {
    // TODO return type based on http-header requested method/content-type
    println!("handle_not_found, method: {}, uri: {}", method, uri.path());

    let is_navigation_action = headers.is_hx_request();

    let rendered_content = if is_navigation_action {
        /* just return empty string if client tried to reload unknown element */
        String::from("")
    } else {
        state.engine.render("404", &(json!({
            "requested_path": uri.path(),
            "authenticated_user": authenticated_user,
            "navigation": state.navigation,
        }))).unwrap_or_else(|e| {
            println!("error rendering template: {}", e);
            String::from("")
        })
    };

    (
        StatusCode::NOT_FOUND,
        [(header::VARY, "Hx-Request, Hx-Boosted")],
        minify_html_response(rendered_content),
    )
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
    Html(minify(unprocessed_html).expect("unexpected error during minification"))
}

pub fn concat(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let collected = h.params().into_iter().map(|param| param.render()).collect::<String>();

    match out.write(collected.as_str()) {
        Ok(()) => (),
        Err(e) => eprintln!("error in concat: {}", e),
    };

    Ok(())
}

handlebars_helper!(button_type: |input: String| {
    match input.as_str() {
        "primary" => "cursor-pointer rounded-full bg-zinc-900 py-1 px-3 bg-emerald-400/10 text-emerald-400 ring-1 ring-inset ring-emerald-400/20 hover:bg-emerald-400/10 hover:text-emerald-300 hover:ring-emerald-300",
        "secondary" => "cursor-pointer rounded-full py-1 px-3 bg-zinc-800/40 text-zinc-400 ring-1 ring-inset ring-zinc-800 hover:bg-zinc-800 hover:text-zinc-300",
        "filled" => "cursor-pointer rounded-full py-1 px-3 bg-emerald-500 text-white hover:bg-emerald-400",
        "outline" => "cursor-pointer rounded-full py-1 px-3 ring-1 ring-inset text-zinc-400 ring-white/10 hover:bg-white/5 hover:text-white",
        "text" => "cursor-pointer text-emerald-400 hover:text-emerald-500",
        "link" => "cursor-pointer text-sm font-medium transition text-zinc-400 hover:text-white",
        _ => "cursor-pointer rounded-full bg-zinc-900 py-1 px-3 bg-emerald-400/10 text-emerald-400 ring-1 ring-inset ring-emerald-400/20 hover:bg-emerald-400/10 hover:text-emerald-300 hover:ring-emerald-300",
    }
});

handlebars_helper!(check_auth_required: |requires_auth: Option<bool>, email: Option<String>| {
    !requires_auth.unwrap_or(false) || (requires_auth.unwrap_or(false) && email.is_some())
});

handlebars_helper!(hide_if_authenticated: |hidden_if_authenticated: Option<bool>, email: Option<String>| {
    hidden_if_authenticated.unwrap_or(false) && email.is_some()
});

handlebars_helper!(format_price: |input: f32| {
    input.to_string()
});

handlebars_helper!(xor: |arg1: Value, arg2: Value| {
    match arg1.is_null() {
        true => arg2,
        false => arg1,
    }
});

handlebars_helper!(sub: |val: i64, subtract: i64, allow_negative: bool| {
    if allow_negative {
        val - subtract
    } else if subtract > val {
        0
    } else {
        val - subtract
    }  
});

handlebars_helper!(add: |val: u32, addendum: u32| {
    val + addendum
});

handlebars_helper!(humanize_utc_time: |utc_date: DateTime<Local>, with_hours: bool| {
    let fmt = if with_hours == true {
        "%d.%m.%Y %H:%M Uhr"
    } else {
        "%d.%m.%Y"
    };
    utc_date.format(fmt).to_string()
});

handlebars_helper!(emoji_list: | | {
    vec![
        128525, 128526, 129303, 129322, 128571, 9757, 9996, 128513,
        128020, 128022, 128025, 128035, 128048, 129424, 129445, 128106,
        128103, 129492, 128170, 128150, 9749, 127864, 129346, 129475,
        127829, 127791, 127831, 127843, 127847, 129360, 129386, 127814,
        129361, 129382, 128293,
    ]
});