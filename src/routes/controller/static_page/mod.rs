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
use crate::routes::minify_html_response;
use crate::view::static_page::{
    AboutTemplate,
    HomeTemplate,
    ImprintTemplate,
    NotFoundTemplate,
};
use crate::AppState;


pub async fn get_static_page(
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    request: Request,
) -> impl IntoResponse {
    let context = Context::from_request(&request);
    let mut status_code = StatusCode::OK;

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
            eprintln!("couldnt find template for static-page {}", request.uri().path());
            status_code = StatusCode::NOT_FOUND;
            NotFoundTemplate {
                authenticated_user: &authenticated_user,
                notification: None,
                context: context,
            }.render()
        }
    };

    (status_code, minify_html_response(rendered_template.unwrap_or_default())).into_response()
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/imprint", get(get_static_page))
        .route("/about", get(get_static_page))
        .route("/", get(get_static_page))
}