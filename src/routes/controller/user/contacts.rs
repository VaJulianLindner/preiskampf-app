use std::{any::Any, sync::Arc};
use askama::Template;
use axum::{
    extract::{Request, State, FromRequest}, http::{HeaderMap, StatusCode}, response::IntoResponse, Extension, Form
};

use crate::{core::context::Context, routes::{create_notification, minify_html_response, render_error_notification, render_success_notification}, AppState};
use crate::model::user::{User, contacts::AddContactRequestForm};
use crate::services::user::contacts::add_contact_request;
use crate::view::user::contacts::{ContactPageTemplate, AddContactFormTemplate};

pub async fn get_friends_page(
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    request: Request,
) -> impl IntoResponse {
    let context = Context::from_request(&request);

    // TODO get contacts: confirmed, pending, requesting..

    let template = ContactPageTemplate {
        authenticated_user: &authenticated_user,
        notification: None,
        errors: &None,
        context: context,
    };

    (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
}

pub async fn save_contact_request(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    request: Request,
) -> impl IntoResponse {
    let uri = request.uri().clone();
    let req_headers = request.headers().clone();
    let context = Context::new(&uri, &req_headers);

    let form_data = Form::<AddContactRequestForm>::from_request(request, &state).await.unwrap();
    let authenticated_user_id = match *authenticated_user {
        Some(ref u) => {
            if u.get_email() == form_data.contact_email {
                return (
                    StatusCode::FORBIDDEN,
                    [("Hx-Reswap", "none")],
                    minify_html_response(render_success_notification(Some("Jeder ist sich selbst am nächsten")))
                ).into_response();
            }
            u.get_id().expect("authenticated user must have an id")
        },
        None => {
            return (StatusCode::FORBIDDEN, [("Hx-Reswap", "none")], minify_html_response(String::from(""))).into_response();
        }
    };

    match add_contact_request(
        &state.db_pool,
        authenticated_user_id,
        &form_data,
    ).await {
        Ok(_) => {
            let template = AddContactFormTemplate {
                notification: Some(create_notification("Die Kontaktanfrage wurde versendet", true)),
                errors: &None,
                context: context
            };
            (
                StatusCode::OK,
                [("Hx-Retarget", "[hx-put=\"/contacts/save_contact_request\"]")],
                minify_html_response(template.render().unwrap_or_default())
            ).into_response()
        },
        Err(sqlx::Error::PoolTimedOut) => {
            (
                StatusCode::TOO_MANY_REQUESTS,
                [("Hx-Reswap", "none")],
                minify_html_response(render_error_notification(Some("Ein unerwarteter Fehler ist aufgetreten. Bitte versuchen Sie es später erneut.")))
            ).into_response()
        },
        Err(sqlx::Error::Database(e)) => {
            let rendered_content = if e.is_unique_violation() {
                render_error_notification(Some("Kontaktanfrage ist bereits vorhanden"))
            } else {
                eprintln!("Database error in controller::contacts::add_contact_request {:?}", e);
                render_error_notification(Some("Ein unerwarteter Fehler ist aufgetreten"))
            };
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                [("Hx-Reswap", "none")],
                minify_html_response(rendered_content)
            ).into_response()
        },
        Err(e) => {
            eprintln!("unexpected error in controller::contacts::add_contact_request {:?}", e);
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                [("Hx-Reswap", "none")],
                minify_html_response(render_error_notification(Some("Ein unerwarteter Fehler ist aufgetreten")))
            ).into_response()
        }
    }

}