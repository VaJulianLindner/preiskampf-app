use std::collections::HashMap;
use std::sync::Arc;
use axum::{response::IntoResponse, http, extract};
use http::{StatusCode, Uri, HeaderMap, header};
use extract::{Path, State, Extension, Query};
use serde_json::{json, Value, Map};
use sqlx::{Pool, Postgres};

use crate::routes::{get_value_from_path, minify_html_response};
use crate::services::shopping_list::find_shopping_lists;
use crate::model::user::User;
use crate::services::product::find_products;
use crate::core::query_params::StateParams;
use crate::core::pagination::Pagination;
use crate::core::request_extension::HttpExt;
use crate::AppState;

pub async fn page_template(
    state: State<AppState>,
    headers: HeaderMap,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    Query(query_params): Query<StateParams>,
    path: Path<HashMap<String, String>>,
    uri: Uri,
) -> impl IntoResponse {
    let template_name = get_value_from_path(&path, "template_name");

    let is_navigation_action = headers.is_hx_request();
    let is_boosted_action = headers.is_boosted_request();

    let authenticated_user_id = match *authenticated_user {
        Some(ref u) => Some(u.get_id().expect("authenticated user must have an id")),
        None => None
    };
    // TODO is this the most efficient way to create the template_data in json and possibly pass it into the error handler?
    let mut template_data = create_template_data(
        &query_params,
        uri.path(),
        &state.db_pool,
        authenticated_user_id,
    ).await;
    template_data.insert("is_navigation_action".to_string(), json!(is_navigation_action));
    template_data.insert("is_boosted_action".to_string(), json!(is_boosted_action));
    template_data.insert("authenticated_user".to_string(), json!(authenticated_user));
    template_data.insert("navigation".to_string(), json!(state.navigation));
    template_data.insert("requested_path".to_string(), json!(uri.path()));
    template_data.insert("query_params".to_string(), json!(query_params));

    let data = json!(template_data);
    match state.engine.render(template_name.as_str(), &data) {
        Ok(rendered_content) => {
            (StatusCode::OK, [(header::VARY, "Hx-Request, Hx-Boosted")], minify_html_response(rendered_content)).into_response()
        },
        Err(e) => {
            println!("page_template: error rendering template: {}", e);
            let rendered_content = state.engine.render("404", &data).unwrap_or_else(|e| {
                println!("page_template: error rendering template: {}", e);
                String::from("")
            });
            (StatusCode::NOT_FOUND, minify_html_response(rendered_content)).into_response()
        },
    }
}

async fn create_template_data(
    query_params: &StateParams,
    path: &str,
    db_pool: &Pool<Postgres>, 
    user_id: Option<i64>,
) -> Map<String, Value> {
    let mut template_data = Map::new();

    match path {
        // TODO this abstraction needs to get cleaner.. (query_params into db_query)
        // TODO plus im double copying the values of String (from params here, and from here to the service)..

        "/einkaufszettel" => {
            let limit: usize = query_params.get_limit().unwrap_or(10);
            let page: usize = query_params.get_page().unwrap_or(0);
            let offset = page * limit;
            let (shopping_lists, total) = find_shopping_lists(
                db_pool, 
                user_id.unwrap(),
                limit,
                offset,
            ).await;
            template_data.insert("shopping_lists".to_string(), json!(shopping_lists));
            template_data.insert("pagination".to_string(), Pagination::from_query_params(&query_params).as_json(Some(total)));
        },
        "/einkaufstour" => {
            let search_query = query_params.get_q();
            let sort_by = query_params.get_sort_by().unwrap_or("created_at".to_string());
            let sort_order = query_params.get_sort_order().unwrap_or("".to_string());
            let limit: usize = query_params.get_limit().unwrap_or(10);
            let page: usize = query_params.get_page().unwrap_or(0);
            let offset = page * limit;
            let (products, total) = find_products(
                db_pool,
                search_query,
                sort_by,
                sort_order,
                limit,
                offset
            ).await;
            template_data.insert("products".to_string(), json!(products));
            template_data.insert("pagination".to_string(), Pagination::from_query_params(&query_params).as_json(Some(total)));
        }
        _ => (),
    }

    template_data
}