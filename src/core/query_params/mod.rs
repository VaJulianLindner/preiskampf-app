use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::routes::{render_success_notification, render_error_notification};

#[derive(Deserialize, Debug)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl SortOrder {
    pub fn to_string(&self) -> String {
        match self {
            SortOrder::Asc => "ASC".to_string(),
            SortOrder::Desc => "DESC".to_string(),
        }
    }

    pub fn from_str(str: &str) -> Self {
        match str {
            "ASC" => SortOrder::Asc,
            "asc" => SortOrder::Asc,
            "DESC" => SortOrder::Desc,
            "desc" => SortOrder::Desc,
            _ => SortOrder::Desc,
        }
    }
}


#[derive(Deserialize, Debug)]
pub enum RedirectSuccessState {
    Success,
    Error,
}

impl RedirectSuccessState {
    pub fn from_str(str: &str) -> Self {
        match str {
            "success" => RedirectSuccessState::Success,
            _ => RedirectSuccessState::Error,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StateParams {
    q: Option<String>,
    page: Option<usize>,
    limit: Option<usize>,
    sort_by: Option<String>,
    sort_order: Option<String>,
    is: Option<String>,
}

impl StateParams {
    pub fn from_query(query: Option<&str>) -> Self {
        let mut state_params = Self {
            q: None,
            page: Some(0),
            limit: Some(10),
            sort_by: None,
            sort_order: None,
            is: None,
        };

        if query.is_none() {
            return state_params;
        }

        let parts = query.unwrap().split("&");
        for part in parts {
            let mut splitted_param = part.splitn(2, |b| b == '=');
            let name = splitted_param.next().unwrap();
            let value = splitted_param.next();

            if value.is_none() {
                continue;
            }

            match name {
                "q" => state_params.q = value.unwrap().parse().ok(),
                "page" => state_params.page = value.unwrap().parse().ok(),
                "limit" => state_params.limit = value.unwrap().parse().ok(),
                "sort_by" => state_params.sort_by = value.unwrap().parse().ok(),
                "sort_order" => state_params.sort_order = value.unwrap().parse().ok(),
                "is" => state_params.is = value.unwrap().parse().ok(),
                _ => println!("no StateParam for {:?} implemented", name),
            }
        }

        state_params
    }

    pub fn success_state_notify(&self) -> String {
        if self.is.is_none() {
            return String::from("");
        }

        match RedirectSuccessState::from_str(self.is.as_ref().unwrap().as_str()) {
            RedirectSuccessState::Success => render_success_notification(None),
            RedirectSuccessState::Error => render_error_notification(None),
        }
    }

    pub fn success_state_json(&self) -> Value {
        if self.is.is_none() {
            return json!({});
        }

        match RedirectSuccessState::from_str(self.is.as_ref().unwrap().as_str()) {
            RedirectSuccessState::Success => {
                json!({
                    "is_success": true,
                    "message": "Erfolgreich gespeichert"
                })
            }
            RedirectSuccessState::Error => {
                json!({
                    "is_success": false,
                    "message": "Ein unerwarteter Fehler ist aufgetreten"
                })
            }
        }
    }

    pub fn get_q(self: &Self) -> Option<String> {
        self.q.to_owned()
    }

    pub fn get_sort_by(self: &Self) -> Option<String> {
        self.sort_by.to_owned()
    }

    pub fn get_sort_order(self: &Self) -> Option<String> {
        self.sort_order.to_owned()
    }

    pub fn get_limit(self: &Self) -> Option<usize> {
        self.limit
    }

    pub fn get_page(self: &Self) -> Option<usize> {
        self.page
    }
}

#[derive(Deserialize)]
pub struct ActivationParams {
    pub token: Option<String>,
}