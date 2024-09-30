use std::{collections::HashMap, sync::Arc};
use futures::try_join;
use askama::Template;
use axum::{
    extract::{FromRequest, Path, Request, State}, http::{HeaderMap, StatusCode}, response::{Html, IntoResponse}, Extension, Form
};

use crate::{core::context::Context, routes::{create_notification, get_value_from_path, minify_html_response, render_error_notification, render_success_notification}, AppState};
use crate::model::user::{User, contacts::AddContactRequestForm};
use crate::services::user::contacts::{
    add_contact_request_by_email,
    add_contact_request,
    find_contacts,
    find_requested_contacts,
    find_pending_contacts,
    delete_request,
    confirm_request,
};
use crate::view::user::contacts::{
    ContactPageTemplate,
    AddContactFormTemplate,
    ContactListEntryTemplate,
};

pub async fn get_friends_page(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    request: Request,
) -> impl IntoResponse {
    let context = Context::from_request(&request);
    let authenticated_user_id = match *authenticated_user {
        Some(ref u) => {
            u.get_id().expect("authenticated user must have an id")
        },
        None => {
            return (StatusCode::FORBIDDEN, [("Hx-Reswap", "none")], minify_html_response(String::from(""))).into_response();
        }
    };

    // TODO use transaction to reduce network-trips?
    // let mut transaction = state.db_pool.begin().await.unwrap();
    let future_result = try_join!(
        find_contacts(&state.db_pool, &authenticated_user_id),
        find_requested_contacts(&state.db_pool, &authenticated_user_id),
        find_pending_contacts(&state.db_pool, &authenticated_user_id),
    );
    let (
        contacts,
        requested_contacts,
        pending_contacts,
    ) = match future_result {
        Ok(val) => val,
        Err(e) => {
            eprintln!("error in controller::contacts::get_friends_page {:?}", e);
            (vec![], vec![], vec![])
        }
    };

    let template = ContactPageTemplate {
        authenticated_user: &authenticated_user,
        contacts: &contacts,
        requested_contacts: &requested_contacts,
        pending_contacts: &pending_contacts,
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

    // TODO automatically accept contact, if you want to create request to a user, that already has an open request to you
    match add_contact_request_by_email(
        &state.db_pool,
        &authenticated_user_id,
        &form_data,
        None,
    ).await {
        Ok(linked_contact) => {
            let template = ContactListEntryTemplate {
                authenticated_user: &authenticated_user,
                notification: Some(create_notification("Die Kontaktanfrage wurde versendet", true)),
                contact_entry: &linked_contact,
                // TODO confirmed-contacts-list etc should be statics in the contact module to not accidentally change them in the templates
                oob_swap_target: &Some("sent-requests-list"),
                context: context
            };
            (
                StatusCode::OK,
                [("Hx-Reswap", "none")],
                minify_html_response(template.render().unwrap_or_default()),
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

pub async fn remove_contact(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    path: Path<HashMap<String, String>>,
) -> impl IntoResponse {
    let contact_id = get_value_from_path(&path, "contact_id").parse::<i64>().unwrap_or_default();

    let authenticated_user_id = match *authenticated_user {
        Some(ref u) => {
            u.get_id().expect("authenticated user must have an id")
        },
        None => {
            return (StatusCode::FORBIDDEN, [("Hx-Reswap", "none")], minify_html_response(String::from(""))).into_response();
        }
    };

    match delete_request(&state.db_pool, &authenticated_user_id, &contact_id).await {
        Ok(_) => (
            StatusCode::NO_CONTENT,
            [("Xui-Deleted", "yes")],
            minify_html_response(render_success_notification(Some("Der Kontakt wurde entfernt")))
        ).into_response(),
        Err(e) => {
            eprintln!("unexpected error in controller::contacts::remove_contact {:?}", e);
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                [("Hx-Reswap", "none")],
                minify_html_response(render_error_notification(Some("Ein unerwarteter Fehler ist aufgetreten")))
            ).into_response()
        }
    }
}

pub async fn confirm_contact(
    state: State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    path: Path<HashMap<String, String>>,
    request: Request,
) -> impl IntoResponse {
    let by_user_id = get_value_from_path(&path, "by_user_id").parse::<i64>().unwrap_or_default();
    let authenticated_user_id = match *authenticated_user {
        Some(ref u) => {
            u.get_id().expect("authenticated user must have an id")
        },
        None => {
            return (StatusCode::FORBIDDEN, [("Hx-Reswap", "none")], minify_html_response(String::from(""))).into_response();
        }
    };

    // TODO should be transaction, because it should fail or succeed only entirely
    println!("confirm_contact by_user_id {by_user_id}, authenticated_user_id {authenticated_user_id}");
    let future_result = try_join!(
        add_contact_request(&state.db_pool, &authenticated_user_id, &by_user_id, Some("confirmed")),
        confirm_request(&state.db_pool, &by_user_id)
    );
    let confirmed_contact = match future_result {
        Ok(val) => val.1,
        Err(e) => {
            eprintln!("unexpected error in controller::contacts::confirm_contact {:?}", e);
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                [("Hx-Reswap", "none")],
                minify_html_response(render_error_notification(Some("Ein unerwarteter Fehler ist aufgetreten")))
            ).into_response();
        }
    };

    let template = ContactListEntryTemplate {
        authenticated_user: &authenticated_user,
        notification: Some(create_notification("Die Kontaktanfrage wurde bestätigt", true)),
        contact_entry: &confirmed_contact,
        // TODO confirmed-contacts-list etc should be statics in the contact module to not accidentally change them in the templates
        oob_swap_target: &Some("confirmed-contacts-list"),
        context: Context::from_request(&request),
    };
    (
        StatusCode::UNPROCESSABLE_ENTITY,
        [("Xui-Confirmed", "yes")],
        minify_html_response(template.render().unwrap_or_default())
    ).into_response()
}