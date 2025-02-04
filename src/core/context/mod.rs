use axum::http::Request;
use crate::core::path::DetailOperations;
use crate::core::query_params::StateParams;

// TODO context holds form-errors
// TODO context holds pagination
// TODO Box context in templates to prevent copy when transferring data ownership
// TODO clever use of Boxes, Rc/Arc and RefCell/Mutex
pub struct Context<'a> {
    pub uri: &'a axum::http::Uri,
    pub headers: &'a axum::http::HeaderMap,
    pub query_params: StateParams,
}

// TODO for more convenience
// impl axum::extract::FromRequestParts<AppState> for Context {}
impl<'a> Context<'a> {
    pub fn new(uri: &'a axum::http::Uri, headers: &'a axum::http::HeaderMap) -> Self {
        Self {
            uri: uri,
            headers: headers,
            query_params: StateParams::from_query(uri.query()),
        }
    }

    pub fn from_request<T>(request: &'a Request<T>) -> Self {
        Self {
            uri: request.uri(),
            headers: request.headers(),
            query_params: StateParams::from_query(request.uri().query()),
        }
    }

    pub fn is_create_operation(&self) -> bool {
        let last_path_part = self.uri.path().split("/").last().unwrap_or_default().to_string().to_lowercase();
        let detail_operation = DetailOperations::from_string(last_path_part);
        match detail_operation {
            Some(DetailOperations::Create) => true,
            _ => false,
        }
    }

    pub fn get_current_page(&self) -> usize {
        self.query_params.get_page().unwrap_or(0)
    }

    pub fn preserve_query_state(&self, page: &usize, with_pathname: bool) -> String {
        let query = self.uri.query().unwrap_or_default();

        let preserved_query = if query.len() == 0 {
            format!("page={}", page)
        } else {
            let updated_query = query.split("&").map(|v| {
                if v.starts_with("page=") {
                    format!("page={page}")
                } else {
                    v.to_string()
                }
            }).reduce(|acc, e| {
                if acc.len() == 0 {
                    format!("{acc}{e}")
                } else {
                    format!("{acc}&{e}")
                }
            });

            match updated_query {
                Some(query) => {
                    if !query.contains("page") {
                        format!("{}&page={}", query, page)
                    } else {                               
                        query
                    }
                },
                None => query.to_string(),
            }                   
        };

        if with_pathname {
            format!("{}?{}", self.uri.path(), preserved_query)
        } else {
            preserved_query
        }     
    }
}