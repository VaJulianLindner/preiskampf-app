use std::{sync::Arc, collections::HashMap};
use askama::Template;
use axum::{
    extract::{Path, State}, http::{HeaderMap, StatusCode}, response::IntoResponse, routing::{post, put}, Extension, Form, Router
};
use serde_json::{json, Map};

use crate::AppState;
use crate::core::client_action::ClientActionResponse;
use crate::routes::{minify_html_response, render_success_notification, render_error_notification, get_value_from_path};
use crate::services::user::{update_user, update_selected_shopping_list};
use crate::model::user::{User, SessionUser, UserUpdateForm};
use crate::routes::auth::create_auth_cookie_for_user;
use crate::view::user::UserDetailTemplate;

pub async fn save_user(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    Form(form_data): Form<UserUpdateForm>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if authenticated_user.is_none() {
        return (StatusCode::FORBIDDEN, headers, minify_html_response(String::from(""))).into_response();
    }

    match *authenticated_user {
        Some(ref u) => {
            let user_id = u.get_id().expect("authenticated user must have an id");
            if user_id != form_data.id {
                return (StatusCode::FORBIDDEN, headers, minify_html_response(String::from(""))).into_response();
            }
        },
        None => {
            return (StatusCode::FORBIDDEN, headers, minify_html_response(String::from(""))).into_response();
        }
    };




    // let template = UserDetailTemplate {
    //     authenticated_user: &authenticated_user,
    //     notification: None,
    //     // navigation: &state.navigation,
    //     request: &request,
    // };
    // (StatusCode::OK, headers, minify_html_response(template.render().unwrap_or_default())).into_response()



    let mut template_data = Map::new();
    template_data.insert("navigation".to_string(), json!(state.navigation));

    match update_user(&state.db_pool, &form_data).await {
        Ok(updated_user) => {
            template_data.insert("authenticated_user".to_string(), json!(updated_user));
            let cookie_val = create_auth_cookie_for_user(&SessionUser::new(updated_user));
            headers.insert("set-cookie", cookie_val);
        },
        Err(e) => {
            eprintln!("error in save_user: {:?}", e);
            let notification = render_error_notification(None);
            headers.insert("hx-reswap", "none".parse().unwrap());
            return (StatusCode::UNPROCESSABLE_ENTITY, headers, minify_html_response(notification)).into_response();
        },
    }

    match state.engine.render("partials/elements/profile", &json!(template_data)) {
        Ok(mut rendered_content) => {
            let notification = render_success_notification(None);
            rendered_content.push_str(notification.as_str());
            (StatusCode::OK, headers, minify_html_response(rendered_content)).into_response()
        },
        Err(_) => {
            (StatusCode::NOT_FOUND, headers, minify_html_response(String::from(""))).into_response()
        },
    }
}

pub async fn save_selected_shopping_list(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    path: Path<HashMap<String, String>>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if authenticated_user.is_none() {
        return (StatusCode::FORBIDDEN, headers, minify_html_response(String::from("")));
    }

    let authenticated_user_id = match *authenticated_user {
        Some(ref u) => {
            u.get_id().expect("authenticated user must have an id")
        },
        None => {
            return (StatusCode::FORBIDDEN, headers, minify_html_response(String::from("")));
        }
    };
    let shopping_list_id = get_value_from_path(&path, "shopping_list_id").parse::<i64>().unwrap_or_default();

    match update_selected_shopping_list(
        &state.db_pool,
        authenticated_user_id,
        shopping_list_id,
    ).await {
        Ok(_) => {
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
            (StatusCode::OK, headers, minify_html_response(unprocessed_html))
        },
        Err(e) => {
            eprintln!("error in save_selected_shopping_list route: {:?}", e);
            let notification = render_error_notification(None);
            (StatusCode::UNPROCESSABLE_ENTITY, headers, minify_html_response(notification))
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user/save", post(save_user))
        .route("/user/save_selected_shopping_list/:shopping_list_id", put(save_selected_shopping_list))
}