use std::collections::HashMap;
use std::sync::Arc;
use askama::Template;
use axum::{
    extract::{FromRequest, Path, Query, Request, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse, routing::{delete, get, post, put},
    Extension, Form, RequestExt, Router
};
use futures::try_join;

use crate::{
    core::{context::Context, pagination::Pagination, query_params::StateParams, request_extension::HttpExt},
    model::{
        shopping_list::{
            AddShoppingListItemForm,
            ShoppingList,
            ShoppingListUpdateForm,
            ToggleShoppingListItemOp::Added,
        },
        user::User,
    },
    routes::{get_value_from_path, minify_html_response, render_error_notification, render_success_notification},
    services::shopping_list::{self},
    view::{product::AddProductToggle, shopping_list::{ShoppingListDetailTemplate, ShoppingListsTemplate}},
    AppState
};

pub async fn get_shopping_lists(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    mut request: Request,
) -> impl IntoResponse {

    let authenticated_user_id = match *authenticated_user {
        Some(ref u) => u.get_id().expect("authenticated user must have an id"),
        None => {
            return (StatusCode::FORBIDDEN, minify_html_response(String::from(""))).into_response();
        }
    };

    let query_params: Query<StateParams> = request.extract_parts_with_state::<Query::<StateParams>, _>(&state).await.unwrap();
    let limit: usize = query_params.get_limit().unwrap_or(10);
    let page: usize = query_params.get_page().unwrap_or(0);
    let offset = page * limit;
    let (shopping_lists, total) = shopping_list::find_shopping_lists(
        &state.db_pool,
        authenticated_user_id,
        limit,
        offset,
    ).await;

    let context = Context::new(request.uri(), request.headers());
    let pagination = Pagination::from_query_params(&query_params)
        .with_total(total)
        .with_uri(request.uri().clone());

    if page > 0 && shopping_lists.len() == 0 {
        let redirect_to = context.preserve_query_state(&0, true);
        let header_name = if context.is_hx_request() {
            "Xui-Redirect"
        } else {
            "Location"
        };
        return (StatusCode::SEE_OTHER, [(header_name, redirect_to)]).into_response();
    }

    let template = ShoppingListsTemplate {
        shopping_lists: shopping_lists,
        authenticated_user: &authenticated_user,
        notification: None,
        pagination: &pagination,
        errors: &None,
        // navigation: &state.navigation,
        context: context,
    };

    (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
}

pub async fn get_shopping_list_detail_page(
    Query(query_params): Query<StateParams>,
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    path: Path<HashMap<String, String>>,
    request: Request,
) -> impl IntoResponse {
    let id = get_value_from_path(&path, "id");
    let authenticated_user_id = authenticated_user.as_ref().as_ref().unwrap().get_id().as_ref().expect("the authenticated user must have an id");
    let context = Context::new(request.uri(), request.headers());
    let pagination = Pagination::from_query_params(&query_params).with_uri(request.uri().clone());
    // TODO check if this user owns the shopping_list! => or move it to service/db

    let (shopping_list, (selected_products, total)) = if context.is_create_operation() {
        (ShoppingList::default(), (vec![], 0))
    } else {
        let shopping_list_id = match id.parse::<i64>() {
            Ok(val) => val,
            Err(_) => {
                return (StatusCode::BAD_REQUEST, minify_html_response(String::from(""))).into_response();
            }
        };

        match try_join!(
            shopping_list::find_shopping_list(
                &state.db_pool,
                &shopping_list_id,
                &authenticated_user_id,
            ),
            shopping_list::find_shopping_list_products(
                &state.db_pool,
                &shopping_list_id,
                &pagination,
            )
        ) {
            Ok(val) => val,
            Err(sqlx::Error::RowNotFound) => {
                return (StatusCode::TEMPORARY_REDIRECT, [("Location", "/einkaufszettel")]).into_response();
            },
            Err(sqlx::Error::PoolTimedOut) => {
                return (StatusCode::TOO_MANY_REQUESTS).into_response();
            }
            Err(e) => {
                eprintln!("undefined error in get_shopping_list_detail_page: {:?}", e);
                return (StatusCode::TEMPORARY_REDIRECT, [("Location", "/einkaufszettel")]).into_response();
            }
        }
    };

    let pagination = pagination.with_total(total);

    let template = ShoppingListDetailTemplate {
        shopping_list: &shopping_list,
        selected_products: Some(&selected_products),
        pagination: Some(&pagination),
        authenticated_user: &authenticated_user,
        notification: None,
        errors: &None,
        context: context,
    };

    (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
}

pub async fn save_shopping_list(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    request: Request,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if authenticated_user.is_none() {
        return (StatusCode::FORBIDDEN, headers, minify_html_response(String::from("")));
    }

    // TODO check if this user owns the shopping_list! => or move it to service/db

    let uri = request.uri().clone();
    let req_headers = request.headers().clone();
    let context = Context::new(&uri, &req_headers);
    let authenticated_user_id = authenticated_user.as_ref().as_ref().unwrap().get_id().as_ref().expect("the authenticated user must have an id");
    let form_data = Form::<ShoppingListUpdateForm>::from_request(request, &state).await.unwrap();

    let updated_shopping_list = match shopping_list::upsert_shopping_list(&state.db_pool, authenticated_user_id, &form_data).await {
        Ok(shopping_list) => shopping_list,
        Err(e) => {
            eprintln!("error while upserting shopping list: {:?}", e);
            let notification = render_error_notification(None);
            headers.insert("hx-reswap", "none".parse().unwrap());
            return (StatusCode::UNPROCESSABLE_ENTITY, headers, minify_html_response(notification));
        },
    };

    if form_data.id.is_some() {
        let template = ShoppingListDetailTemplate {
            shopping_list: &updated_shopping_list,
            selected_products: None,
            pagination: None,
            authenticated_user: &authenticated_user,
            notification: None,
            errors: &None,
            // navigation: &state.navigation,
            context: context,
        };
        let mut rendered_content = template.render().unwrap_or_default();
        
        let notification = render_success_notification(Some("Einkaufszettel erfolgreich gespeichert"));
        rendered_content.push_str(notification.as_str());

        headers.insert("hx-reswap", "outerHTML transition:true".parse().unwrap());
        (StatusCode::OK, headers, minify_html_response(rendered_content))
    } else {
        headers.insert("hx-reswap", "none".parse().unwrap());
        headers.insert("xui-redirect", format!("/einkaufszettel/{}", updated_shopping_list.get_id()).parse().unwrap());
        let notification = render_success_notification(
            Some(format!("Einkaufszettel '{}' erfolgreich gespeichert", updated_shopping_list.get_name(),
        ).as_str()));
        (StatusCode::TEMPORARY_REDIRECT, headers, minify_html_response(notification))
    }
}

pub async fn delete_shopping_list(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    path: Path<HashMap<String, String>>,
    query_params: Query<StateParams>,
    request: Request,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    let authenticated_user_id = match *authenticated_user {
        Some(ref u) => match u.get_id() {
            Some(id) => id.to_owned(),
            None => {
                return (StatusCode::UNAUTHORIZED, headers, minify_html_response(String::from(""))).into_response();
            }
        },
        None => {
            return (StatusCode::UNAUTHORIZED, headers, minify_html_response(String::from(""))).into_response();
        }
    };

    let id: String = get_value_from_path(&path, "id");
    let parsed_resource_id = match id.parse::<i64>() {
        Ok(id) => id,
        Err(_) => {
            return (StatusCode::UNPROCESSABLE_ENTITY, headers, minify_html_response(String::from(""))).into_response();
        },
    };
    let context = Context::new(request.uri(), request.headers());

    match shopping_list::delete_shopping_list(
        &state.db_pool,
        authenticated_user_id,
        parsed_resource_id,
    ).await {
        Ok(deleted_shopping_list) => {
            let message = format!("Einkaufszettel \"{}\" wurde gelöscht", deleted_shopping_list.get_name());
            let notification = render_success_notification(Some(message.as_str())); 
            let redirect_to = format!(
                "/einkaufszettel?{}",
                context.preserve_query_state(&query_params.get_page().unwrap_or(0), false)
            );
            (StatusCode::TEMPORARY_REDIRECT, [("xui-redirect", redirect_to)], minify_html_response(notification)).into_response()
        },
        Err(e) => {
            headers.insert("hx-reswap", "none".parse().unwrap());
            match e {
                sqlx::Error::RowNotFound => {
                    let notification = render_error_notification(Some("Unerlaubter Zugriff"));
                    (StatusCode::UNAUTHORIZED, headers, minify_html_response(notification)).into_response()
                },
                _ => {
                    eprintln!("Error deleting shopping list: {:?}", e);
                    let notification = render_error_notification(Some("Einkaufszettel konnte nicht gelöscht werden"));
                    (StatusCode::UNPROCESSABLE_ENTITY, headers, minify_html_response(notification)).into_response()
                },
            }
        }
    }
}

pub async fn save_shopping_list_item(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    request: Request,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if authenticated_user.is_none() {
        let notification = render_error_notification(None);
        return (StatusCode::UNAUTHORIZED, headers, minify_html_response(notification));
    };

    let user = authenticated_user.as_ref().as_ref().unwrap();
    let authenticated_user_id = user.get_id().expect("authenticated_user must have an id");

    let form_data = match Form::<AddShoppingListItemForm>::from_request(request, &state).await {
        Ok(form_data) => form_data,
        Err(e) => {
            eprintln!("error in save_shopping_list_item {e:?}");
            let notification = render_error_notification(None);
            headers.insert("hx-reswap", "none".parse().unwrap());
            return (StatusCode::BAD_REQUEST, headers, minify_html_response(notification));
        }
    };

    let shopping_list_id = if form_data.shopping_list_id.is_some() {
        form_data.shopping_list_id.unwrap()
    } else {
        match user.selected_shopping_list_id {
            Some(id) => id,
            None => {
                let notification = render_error_notification(Some("Kein Einkaufszettel ausgewählt"));
                headers.insert("hx-reswap", "none".parse().unwrap());
                return (StatusCode::BAD_REQUEST, headers, minify_html_response(notification));
            }
        }
    };

    match shopping_list::toggle_shopping_list_item(
        &state.db_pool,
        &authenticated_user_id,
        &shopping_list_id,
        form_data.product_id.as_str(),
        1,
    ).await {
        Ok(executed_op) => {
            if form_data.shopping_list_id.is_some() {
                // TODO list-item disappears, need to render whole list
                let content = render_success_notification(Some("Produkt vom Einkaufszettel entfernt"));
                return (StatusCode::OK, headers, minify_html_response(content));
            }

            let is_liked = match executed_op {
                Added => true,
                _ => false,
            };
            let rendered_content = AddProductToggle {
                action_product_id: &form_data.product_id,
                action_is_liked: is_liked,
                notification: None,
            };
            (StatusCode::OK, headers, minify_html_response(rendered_content.render().unwrap_or_default()))
        },
        Err(sqlx::Error::Database(e)) => {
            let rendered_content = if e.is_unique_violation() {
                render_error_notification(Some("Produkt ist bereits vorhanden"))
            } else {
                render_error_notification(Some("Ein unerwarteter Fehler ist aufgetreten"))
            };
            headers.insert("hx-reswap", "none".parse().unwrap());
            (StatusCode::UNPROCESSABLE_ENTITY, headers, minify_html_response(rendered_content))
        },
        Err(e) => {
            eprintln!("undefined error in controller::shopping_list::save_shopping_list_item {e:?}");
            let rendered_content = render_error_notification(Some("Ein unerwarteter Fehler ist aufgetreten"));
            headers.insert("hx-reswap", "none".parse().unwrap());
            (StatusCode::UNPROCESSABLE_ENTITY, headers, minify_html_response(rendered_content))
        },
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/einkaufszettel/:id", get(get_shopping_list_detail_page))
        .route("/einkaufszettel/create", get(get_shopping_list_detail_page))
        .route("/einkaufszettel", get(get_shopping_lists))
        .route("/shopping_list/delete/:id", delete(delete_shopping_list))
        // TODO move from PUT to PATCH and enable partial updates
        .route("/shopping_list/save", put(save_shopping_list))
        .route("/shopping_list/toggle-like", post(save_shopping_list_item))
}