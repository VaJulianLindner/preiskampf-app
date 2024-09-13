pub mod contacts;

use std::{collections::HashMap, sync::Arc};
use askama::Template;
use axum::{
    extract::{Path, State, Request, FromRequest}, http::{HeaderMap, StatusCode}, response::IntoResponse, routing::{post, put, get}, Extension, Form, Router
};

use crate::{core::context::Context, AppState};
use crate::core::client_action::ClientActionResponse;
use crate::routes::{minify_html_response, create_success_notification, render_success_notification, render_error_notification, get_value_from_path};
use crate::services::user::{update_user, update_selected_shopping_list};
use crate::model::user::{User, SessionUser, UserUpdateForm};
use crate::routes::auth::create_auth_cookie_for_user;
use crate::view::user::UserDetailTemplate;
use contacts::{save_contact_request, get_friends_page};

pub async fn save_user(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    request: Request,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if authenticated_user.is_none() {
        return (StatusCode::FORBIDDEN, headers, String::from("")).into_response();
    }

    let uri = request.uri().clone();
    let req_headers = request.headers().clone();
    let context = Context::new(&uri, &req_headers);
    let form_data = Form::<UserUpdateForm>::from_request(request, &state).await.unwrap();

    match *authenticated_user {
        Some(ref u) => {
            let user_id = u.get_id().expect("authenticated user must have an id");
            if user_id != form_data.id {
                return (StatusCode::FORBIDDEN, headers, String::from("")).into_response();
            }
        },
        None => {
            return (StatusCode::FORBIDDEN, headers, String::from("")).into_response();
        }
    };

    let user_update_result = update_user(&state.db_pool, &form_data).await;
    if user_update_result.is_err() {
        eprintln!("error in save_user: {:?}", user_update_result.unwrap_err());
        let notification = render_error_notification(None);
        headers.insert("hx-reswap", "none".parse().unwrap());
        return (StatusCode::UNPROCESSABLE_ENTITY, headers, minify_html_response(notification)).into_response();
    }

    let updated_user = user_update_result.unwrap();
    let cookie_val = create_auth_cookie_for_user(&SessionUser::new(updated_user.clone()));
    headers.insert("set-cookie", cookie_val);

    let template = UserDetailTemplate {
        authenticated_user: &Some(updated_user),
        notification: Some(create_success_notification(None)),
        errors: &None,
        context: context,
    };
    (StatusCode::OK, headers, minify_html_response(template.render().unwrap_or_default())).into_response()
}


pub async fn get_user_page(
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    request: Request,
) -> impl IntoResponse {
    let context = Context::from_request(&request);
    let template = UserDetailTemplate {
        authenticated_user: &authenticated_user,
        notification: None,
        errors: &None,
        context: context,
    };
    (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
}

pub async fn save_selected_shopping_list(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    path: Path<HashMap<String, String>>,
) -> impl IntoResponse {
    if authenticated_user.is_none() {
        return (StatusCode::FORBIDDEN, minify_html_response(String::from(""))).into_response();
    }

    let authenticated_user_id = match *authenticated_user {
        Some(ref u) => {
            u.get_id().expect("authenticated user must have an id")
        },
        None => {
            return (StatusCode::FORBIDDEN, minify_html_response(String::from(""))).into_response();
        }
    };
    let shopping_list_id = get_value_from_path(&path, "shopping_list_id").parse::<i64>().unwrap_or_default();

    match update_selected_shopping_list(
        &state.db_pool,
        authenticated_user_id,
        shopping_list_id,
    ).await {
        Ok(_) => {
            let mut headers = HeaderMap::new();
            let unprocessed_html = render_success_notification(Some("Auswahl gespeichert"));
            match *authenticated_user {
                Some(ref user) => {
                    let mut cloned_user = user.clone();
                    let mut client_actions = ClientActionResponse::new();

                    if let Some(prev_list_id) = cloned_user.selected_shopping_list_id {
                        let selector = format!("#shopping-list-{prev_list_id}");
                        client_actions.add(selector.as_ref(), "setAttribute", ["xui-hx-disabled", "0"]);
                        client_actions.add(selector.as_ref(), "removeClass", ["selected"]);
                        client_actions.add(selector.as_ref(), "addClass", ["pulsing", "cursor-pointer"]);
                    }
                    
                    let selector = format!("#shopping-list-{shopping_list_id}");
                    client_actions.add(selector.as_ref(), "setAttribute", ["xui-hx-disabled", "1"]);
                    client_actions.add(selector.as_ref(), "removeClass", ["pulsing", "cursor-pointer"]);
                    client_actions.add(selector.as_ref(), "addClass", ["selected"]);

                    headers.append("hx-trigger", client_actions.to_header_value());

                    cloned_user.selected_shopping_list_id = Some(shopping_list_id);
                    let cookie_val = create_auth_cookie_for_user(&SessionUser::new(cloned_user));
                    headers.insert("set-cookie", cookie_val);
                },
                None => (),
            };
            (StatusCode::OK, headers, minify_html_response(unprocessed_html)).into_response()
        },
        Err(e) => {
            eprintln!("error in save_selected_shopping_list route: {:?}", e);
            let notification = render_error_notification(None);
            (StatusCode::UNPROCESSABLE_ENTITY, minify_html_response(notification)).into_response()
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user/save", post(save_user))
        .route("/user/save_selected_shopping_list/:shopping_list_id", put(save_selected_shopping_list))
        .route("/contacts/save_contact_request", put(save_contact_request))
        .route("/contacts", get(get_friends_page))
        .route("/mein-profil", get(get_user_page))
}