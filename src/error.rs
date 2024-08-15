use axum::http::{StatusCode};

pub struct NotFoundError(anyhow::Error);

impl IntoResponse for NotFoundError {
    fn into_response(self) -> Response {
        (
            StatusCode::NOT_FOUND,
            format!("Not Found"),
        )
            .into_response()
    }
}

impl<E> From<E> for NotFoundError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}