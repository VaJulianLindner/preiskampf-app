use axum::http::{HeaderMap, Request};
use crate::core::context::Context;

pub trait HttpExt {
    fn is_hx_request(&self) -> bool;

    fn is_boosted_request(&self) -> bool;
}

impl HttpExt for HeaderMap {
    fn is_hx_request(&self) -> bool {
        self.get("hx-request").is_some()
    }

    fn is_boosted_request(&self) -> bool {
        self.get("hx-boosted").is_some()
    }
}

impl HttpExt for Context<'_> {
    fn is_hx_request(&self) -> bool {
        self.headers.is_hx_request()
    }

    fn is_boosted_request(&self) -> bool {
        self.headers.is_boosted_request()
    }
}

impl<T> HttpExt for Request<T> {
    fn is_hx_request(&self) -> bool {
        self.headers().is_hx_request()
    }

    fn is_boosted_request(&self) -> bool {
        self.headers().is_boosted_request()
    }
}

pub fn strip_query_from_uri(path_and_query: &str) -> &str {
    let path = path_and_query.splitn(2, "?").next();
    path.unwrap_or(path_and_query)
}