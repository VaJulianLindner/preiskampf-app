use std::fmt::Debug;

use askama::Template;
use axum::http::{Request, Uri};
use serde_json::{json, Value};
use handlebars::handlebars_helper;

use crate::{core::{query_params::StateParams, context::Context}, view::misc::PaginationTemplate};

pub enum PaginationType {
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
}

impl Pagination {
    pub fn from_request<T>(request: &Request<T>) {

    }

    pub fn from_uri(uri: &Uri) -> Self {
        Pagination::from_query_params(&StateParams::from_query(uri.query()))
    }

    pub fn from_query(query: Option<&str>) -> Self {
        Pagination::from_query_params(&StateParams::from_query(query))
    }

    pub fn from_query_params(query_params: &StateParams) -> Self {
        Self { 
            q: query_params.get_q(),
            page: query_params.get_page().unwrap_or(0),
            limit: query_params.get_limit().unwrap_or(10),
            sort_by: query_params.get_sort_by(),
            sort_order: query_params.get_sort_order(),
            last_page: None,
            uri: None,
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

    pub fn with_uri(mut self, uri: Uri) -> Self {
        self.uri = Some(uri);
        self
    }

    pub fn as_json(&self, opt_total: Option<u64>) -> Value {
        if let Some(total) = opt_total {
            let total = total as usize;
            let last_page = if total == 0 {
                0
            } else {
                (total - 1) / self.limit
            };
            json!({
                "page": self.page,
                "limit": self.limit,
                "total": total,
                "last_page": last_page
            })
        } else {
            json!({})
        }
    }

    pub fn render(&self, context: &Context) -> Result<String, askama::Error> {
        // TODO this should be rendered from context/partials?
        PaginationTemplate { pagination: self, context }.render()
    }
}

// TODO remove handlebars_helpers after move to askama
handlebars_helper!(prev_page_if_last_item: |current_page: u32, current_list: Value| {
    match current_list.as_array() {
        Some(val) => {
            if val.len() == 1 {
                if current_page > 0 {
                    current_page - 1
                } else {
                    0
                }
            } else {
                current_page
            }
        },
        None => 0
    }
});