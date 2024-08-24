use std::collections::HashMap;
use std::sync::Arc;
use axum::{
    extract::{Extension, Path, Query, Request, State}, 
    http::StatusCode,
    response::IntoResponse, 
    routing::get, 
    Router,
};
use askama::Template;

use crate::{core::{context::Context, pagination::Pagination, query_params::StateParams}, services::product::{find_product, find_products}};
use crate::routes::{minify_html_response, get_value_from_path};
use crate::AppState;
use crate::model::user::User;
use crate::view::product::{ProductDetailTemplate, ProductListTemplate};

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

pub async fn get_product_list_page(
    Query(query_params): Query<StateParams>,
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    request: Request,
) -> impl IntoResponse {

    let search_query = query_params.get_q();
    let sort_by = query_params.get_sort_by().unwrap_or("created_at".to_string());
    let sort_order = query_params.get_sort_order().unwrap_or("".to_string());
    let limit: usize = query_params.get_limit().unwrap_or(10);
    let page: usize = query_params.get_page().unwrap_or(0);
    let offset = page * limit;
    // TODO pass request or uri reference reference to find_products, so that query-params and the db_pool can be parsed inside the service
    let (products, total) = find_products(
        &state.db_pool,
        search_query,
        sort_by,
        sort_order,
        limit,
        offset
    ).await;

    let pagination = Pagination::from_query_params(&query_params)
        .with_total(total)
        .with_uri(request.uri().clone());

    let template = ProductListTemplate {
        products: products,
        authenticated_user: &authenticated_user,
        pagination: &pagination,
        notification: None,
        context: Context::from_request(&request),
    };

    (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/produkt/:product_id", get(get_product_detail_page))
        .route("/einkaufstour", get(get_product_list_page))
}