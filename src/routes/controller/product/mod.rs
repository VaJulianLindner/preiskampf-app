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
use futures::try_join;

use crate::{
    core::{context::Context, pagination::Pagination, query_params::StateParams},
    services::{
        product::{find_product, find_products, find_product_prices},
        shopping_list::find_shopping_list_items,
    },
};
use crate::routes::{minify_html_response, get_value_from_path};
use crate::AppState;
use crate::model::{user::User, product::ListProduct};
use crate::view::product::{ProductDetailTemplate, ProductListTemplate};

pub async fn get_product_detail_page(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    path: Path<HashMap<String, String>>,
    request: Request,
) -> impl IntoResponse {
    let product_id = get_value_from_path(&path, "product_id");

    match try_join!(
        find_product(&state.db_pool, product_id.as_str()),
        find_product_prices(&state.db_pool, product_id.as_str()),
    ) {
        Ok(val) => {
            let template = ProductDetailTemplate {
                product: &val.0,
                prices: &val.1,
                authenticated_user: &authenticated_user,
                notification: None,
                context: Context::new(request.uri(), request.headers()),
            };
        
            (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
        },
        Err(sqlx::Error::RowNotFound) => {
            eprintln!("couldnt find product {:?}", product_id);
            return (StatusCode::TEMPORARY_REDIRECT, [("Location", format!("/nicht-gefunden/produkt/{}", product_id))]).into_response();
        },
        Err(sqlx::Error::PoolTimedOut) => {
            return (StatusCode::TOO_MANY_REQUESTS).into_response();
        }
        Err(e) => {
            eprintln!("error in get_product_detail_page: {:?}", e);
            return (StatusCode::TEMPORARY_REDIRECT, [("Location", "/einkaufstour")]).into_response();
        }
    }

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

    let user = authenticated_user.as_ref().as_ref().unwrap();
    let shopping_list_id = user.selected_shopping_list_id;
    let authenticated_user_id = user.get_id().expect("authenticated user must have an id");

    // TODO find which products are on the users current shopping_list
    // TODO probably some session cache for the liked items and then just join the fetched products list
    match try_join!(
        find_products(
            &state.db_pool,
            search_query,
            shopping_list_id,
            sort_by,
            sort_order,
            limit,
            offset
        ),
        find_shopping_list_items(
            &state.db_pool,
            &shopping_list_id.as_ref().unwrap_or(&0i64),
            &authenticated_user_id,
        ),
    ) {
        Ok(val) => {
            let (products, total) = val.0;
            let shopping_list_items = val.1;
            let mut selected_product_ids: Vec<&str> = Vec::with_capacity(shopping_list_items.len());
            for item in &shopping_list_items {
                selected_product_ids.push(item.product_id.as_str());
            }
            let list_products = products.iter().map(|p| {
                let is_liked = selected_product_ids.contains(&p.id.as_str());
                ListProduct {
                    product: p,
                    is_liked: is_liked,
                }
            }).collect::<Vec<ListProduct>>();

            let pagination = Pagination::from_query_params(&query_params)
                .with_total(total)
                .with_uri(request.uri().clone());

            let template = ProductListTemplate {
                products: list_products,
                authenticated_user: &authenticated_user,
                pagination: &pagination,
                notification: None,
                errors: &None,
                context: Context::from_request(&request),
            };

            (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
        },
        Err(e) => {
            eprintln!("unexpected error in controller::products::get_product_list_page {e:?}");
            (StatusCode::INTERNAL_SERVER_ERROR, minify_html_response("".to_string())).into_response()
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/produkt/:product_id", get(get_product_detail_page))
        .route("/einkaufstour", get(get_product_list_page))
}