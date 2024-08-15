use std::collections::HashMap;
use std::sync::Arc;
use axum::response::IntoResponse;
use axum::{http, extract};
use http::{StatusCode, HeaderMap, header, Uri};
use extract::{Path, State, Extension, Query};
use serde_json::{json, Value, Map};
use sqlx::{Pool, Postgres};

use crate::core::request_extension::HttpExt;
use crate::routes::{get_value_from_path, minify_html_response};
use crate::core::query_params::StateParams;
use crate::core::path::DetailOperations;
use crate::services::shopping_list::find_shopping_list;
use crate::services::product::find_product;
use crate::model::user::User;
use crate::AppState;

pub async fn detail_template(
    state: State<AppState>,
    headers: HeaderMap,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    path: Path<HashMap<String, String>>,
    uri: Uri,
    query_params: Query<StateParams>,
) -> impl IntoResponse {
    let template_name = get_value_from_path(&path, "template_name");
    let detail_template_name = format!("details/{}", template_name);
    let requested_path = format!("/{template_name}");
    let resource_id = get_value_from_path(&path, "resource_id");

    let does_detail_view_exist = state.engine.has_template(detail_template_name.as_str());

    if !does_detail_view_exist {
        return (StatusCode::TEMPORARY_REDIRECT, [("Location", requested_path)]).into_response();
    }

    let detail_operation = DetailOperations::from_string(resource_id.to_string());
    let is_create_operation = match detail_operation {
        Some(DetailOperations::Create) => true,
        _ => false,
    };

    let is_navigation_action = headers.is_hx_request();

    let authenticated_user_id = match *authenticated_user {
        Some(ref u) => Some(u.get_id().expect("authenticated user must have an id")),
        None => None
    };
    let parsed_resource_id = match resource_id.parse::<String>() {
        Ok(id) => id,
        Err(_) => "".to_string(),
    };
    let (mut template_data, redirect_to) = if is_create_operation {
        (Map::new(), "".to_string())
    } else {
        create_template_data(
            requested_path.as_str(),
            &state.db_pool,
            authenticated_user_id,
            &parsed_resource_id,
        ).await
    };

    if !redirect_to.is_empty() {
        return (StatusCode::TEMPORARY_REDIRECT, [("Location", redirect_to.as_str())]).into_response();
    }

    template_data.insert("is_navigation_action".to_string(), json!(is_navigation_action));
    template_data.insert("authenticated_user".to_string(), json!(authenticated_user));
    template_data.insert("navigation".to_string(), json!(state.navigation));
    template_data.insert("is_create_operation".to_string(), json!(is_create_operation));
    if !is_navigation_action {
        template_data.insert("requested_path".to_string(), json!(uri.path()));
    }
    template_data.insert("notification".to_string(), query_params.success_state_json());

    match state.engine.render(detail_template_name.as_str(), &json!(template_data)) {
        Ok(rendered_content) => {
            (StatusCode::OK, [(header::VARY, "Hx-Request, Hx-Boosted")], minify_html_response(rendered_content)).into_response()
        },
        Err(e) => {
            println!("details_template: error rendering template: {}", e);
            (StatusCode::NOT_FOUND, minify_html_response(String::from(""))).into_response()
        },
    }
}

async fn create_template_data(
    path: &str,
    db_pool: &Pool<Postgres>,
    user_id: Option<i64>,
    resource_id: &str,
) -> (Map<String, Value>, String) {
    let mut template_data = Map::new();
    let mut redirect_to = String::from("");

    match path {
        "/einkaufszettel" => {
            match find_shopping_list(
                db_pool, 
                resource_id.parse::<i64>().ok(),
            ).await {
                Ok(shopping_list) => {
                    if &user_id.unwrap_or(0) == shopping_list.get_user_id() {
                        template_data.insert("shopping_list".to_string(), json!(shopping_list));
                    } else {
                        redirect_to = String::from("/einkaufszettel");
                    }
                },
                Err(e) => {
                    eprintln!("details.rs create_template_data: error: {} fetching shopping_list: {:?}", e, resource_id);
                    redirect_to = String::from("/einkaufszettel");
                }
            };
        },
        "/produkt" => {
            match find_product(
                db_pool,
                resource_id,
            ).await {
                Ok(product) => {
                    template_data.insert("product".to_string(), json!(product));
                },
                Err(sqlx::Error::RowNotFound) => {
                    eprintln!("couldnt find product {:?}", resource_id);
                    redirect_to = format!("/nicht-gefunden/produkt/{}", resource_id);
                },
                Err(e) => {
                    eprintln!("undefined error in find_product: {:?}", e);
                    redirect_to = "/einkaufstour".to_string();
                }
            }
        }
        _ => (),
    }

    (template_data, redirect_to)
}