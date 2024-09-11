use std::sync::Arc;
use askama::Template;
use axum::{
    extract::{Request, State, FromRequest}, http::{HeaderMap, StatusCode}, response::IntoResponse, Extension, Form
};

use crate::{core::context::Context, routes::{render_error_notification, render_success_notification, minify_html_response}, AppState};
use crate::model::user::{User, contacts::AddContactRequestForm};
use crate::services::user::add_contact_request;
use crate::view::user::contacts::{ContactPageTemplate, AddContactFormTemplate};

pub async fn get_friends_page(
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    request: Request,
) -> impl IntoResponse {
    let context = Context::from_request(&request);
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

    let authenticated_user_id = match *authenticated_user {
        Some(ref u) => u.get_id().expect("authenticated user must have an id"),
        None => {
            return (StatusCode::FORBIDDEN, minify_html_response(String::from(""))).into_response();
        }
    };
    let form_data = Form::<AddContactRequestForm>::from_request(request, &state).await.unwrap();

    match add_contact_request(
        &state.db_pool,
        authenticated_user_id,
        &form_data.contact_email,
    ).await {
        Ok(_) => {
            (
                StatusCode::OK,
                // [("Hx-Reswap", "[hx-put=\"/contacts/save_contact_request\")]")],
                render_success_notification(Some("Die Kontaktanfrage wurde versendet"))
            ).into_response()
        },
        Err(sqlx::Error::PoolTimedOut) => {
            (
                StatusCode::TOO_MANY_REQUESTS,
                [("Hx-Reswap", "none")],
                render_error_notification(Some("Ein unerwarteter Fehler ist aufgetreten. Bitte versuchen Sie es später erneut."))
            ).into_response()
        },
        Err(e) => {
            eprintln!("unexpected error in controller::contacts::add_contact_request {:?}", e);
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                [("Hx-Reswap", "none")],
                render_error_notification(Some("Ein unerwarteter Fehler ist aufgetreten"))
            ).into_response()
        }
    }

}