use askama::Template;
use std::sync::Arc;
use axum::Extension;
use axum::{
    response::IntoResponse,
    Router,
    extract::Request,
    http::StatusCode,
    routing::get,
};
use crate::core::context::Context;
use crate::model::user::User;
use crate::routes::{minify_html_response, templates};
use crate::view::static_page::{
    AboutTemplate,
    HomeTemplate,
    ImprintTemplate,
};
use crate::AppState;


pub async fn get_static_page(
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    request: Request,
) -> impl IntoResponse {
    let context = Context::from_request(&request);

    let rendered_template = match request.uri().path() {
        "/imprint" => ImprintTemplate {
            authenticated_user: &authenticated_user,
            notification: None,
            context: context,
        }.render(),
        "/about" => AboutTemplate {
            authenticated_user: &authenticated_user,
            notification: None,
            context: context,
        }.render(),
        "/" => HomeTemplate {
            authenticated_user: &authenticated_user,
            notification: None,
            context: context,
        }.render(),
        _ => {
            // TODO 404-page
            eprintln!("couldnt find template for static-page {}", request.uri().path());
            return (StatusCode::TEMPORARY_REDIRECT, [("Location", "/nicht-gefunden")]).into_response();
        }
    };

    (StatusCode::OK, minify_html_response(rendered_template.unwrap_or_default())).into_response()
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/imprint", get(get_static_page))
        .route("/about", get(get_static_page))
        .route("/", get(get_static_page))
}