use std::sync::Arc;
use std::collections::HashMap;
use askama::Template;
use axum::{
    extract::{Path, State, Query, Request, FromRequest},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Extension,
    RequestExt,
    Router,
};

use crate::{core::{context::Context, query_params::StateParams, pagination::Pagination}, AppState};
use crate::routes::{minify_html_response, create_success_notification, render_success_notification, render_error_notification, get_value_from_path};
use crate::model::{user::User, social_timeline::Post};
use crate::view::social_timeline::{PostDetailTemplate, PostListTemplate};

pub async fn get_post_list(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    mut request: Request,
) -> impl IntoResponse {
    let query_params: Query<StateParams> = request.extract_parts_with_state::<Query::<StateParams>, _>(&state).await.unwrap();
    
    let context = Context::new(request.uri(), request.headers());
    let pagination = Pagination::from_query_params(&query_params).with_uri(request.uri().clone());

    let posts = vec![Post {id: 1}, Post {id: 545}];

    let template = PostListTemplate {
        posts: posts,
        authenticated_user: &authenticated_user,
        pagination: &pagination,
        notification: None,
        context: context,
    };
    (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
}

pub async fn get_post_detail(
    Extension(authenticated_user): Extension<Arc<Option<User>>>,
    path: Path<HashMap<String, String>>,
    request: Request,
) -> impl IntoResponse {
    let post_id = get_value_from_path(&path, "id");
    let post = Post { id: post_id.parse::<i64>().unwrap_or_default() };
    let context = Context::from_request(&request);
    let template = PostDetailTemplate {
        post: &post,
        authenticated_user: &authenticated_user,
        notification: None,
        context: context,
    };
    (StatusCode::OK, minify_html_response(template.render().unwrap_or_default())).into_response()
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/posts/:id", get(get_post_detail))
        .route("/posts", get(get_post_list))
}