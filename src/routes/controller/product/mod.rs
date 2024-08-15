use std::collections::HashMap;
use std::sync::Arc;
use axum::{
    extract::{Path, Extension, State, Request}, 
    http::StatusCode,
    response::IntoResponse, 
    routing::get, 
    Router,
};
use askama::Template;

use crate::{core::context::Context, services::product::find_product};
use crate::routes::{minify_html_response, get_value_from_path};
use crate::AppState;
use crate::model::user::User;
use crate::view::product::ProductDetailTemplate;

pub async fn get_product_detail_page(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    path: Path<HashMap<String, String>>,
    request: Request,
) -> impl IntoResponse {
    let product_id = get_value_from_path(&path, "product_id");

    let product = match find_product(
        &state.db_pool,
        product_id.as_str(),
    ).await {
        Ok(product) => product,
        Err(sqlx::Error::RowNotFound) => {
            eprintln!("couldnt find product {:?}", product_id);
            return (StatusCode::TEMPORARY_REDIRECT, [("Location", format!("/nicht-gefunden/produkt/{}", product_id))]).into_response();
        },
        Err(sqlx::Error::PoolTimedOut) => {
            return (StatusCode::TOO_MANY_REQUESTS).into_response();
        }
        Err(e) => {
            eprintln!("undefined error in find_product: {:?}", e);
            return (StatusCode::TEMPORARY_REDIRECT, [("Location", "/einkaufstour")]).into_response();
        }
    };

    let template = ProductDetailTemplate {
        product: &product,
        authenticated_user: &authenticated_user,
        notification: None,
        // navigation: &state.navigation,
        context: Context::new(request.uri(), request.headers()),
    };

    (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/produkt/:product_id", get(get_product_detail_page))
}