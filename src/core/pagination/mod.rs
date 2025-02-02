use std::fmt::Debug;
use askama::Template;
use axum::http::{Request, Uri};

use crate::{core::{query_params::StateParams, context::Context}, view::misc::PaginationTemplate};

pub enum _PaginationType {
    ByOffset,
    ByCursor
}

#[derive(Debug)]
pub struct Pagination {
    pub q: Option<String>,
    pub page: usize,
    pub limit: usize,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub last_page: Option<usize>,
    pub uri: Option<Uri>,
    pub has_previous_page: bool,
    pub has_next_page: bool,
}

impl Pagination {
    pub fn _from_request<T>(request: &Request<T>) -> Self {
        Pagination::_from_uri(request.uri())
    }

    pub fn _from_uri(uri: &Uri) -> Self {
        Pagination::_from_query(uri.query())
    }

    pub fn _from_query(query: Option<&str>) -> Self {
        Pagination::from_query_params(&StateParams::from_query(query))
    }

    pub fn from_query_params(query_params: &StateParams) -> Self {
        let page = query_params.get_page().unwrap_or(0);
        Self { 
            q: query_params.get_q(),
            page: page,
            limit: query_params.get_limit().unwrap_or(10),
            sort_by: query_params.get_sort_by(),
            sort_order: query_params.get_sort_order(),
            last_page: None,
            uri: None,
            has_previous_page: page != 0,
            has_next_page: false,
        }
    }

    pub fn with_total(mut self, total: u64) -> Self {
        let total = total as usize;
        self.last_page = if total == 0 {
            Some(0)
        } else {
            Some((total - 1) / self.limit)
        };
        self
    }

    pub fn with_count(mut self, count: usize) -> Self {
        self.has_next_page = count > self.limit;
        self
    }

    pub fn with_uri(mut self, uri: Uri) -> Self {
        self.uri = Some(uri);
        self
    }

    pub fn render_with_context(&self, context: &Context) -> Result<String, askama::Error> {
        // TODO this should be rendered from context/partials?
        PaginationTemplate { pagination: self, context }.render()
    }
}