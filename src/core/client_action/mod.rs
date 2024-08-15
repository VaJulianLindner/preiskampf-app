use serde_json::{json, Value};
use axum::http::HeaderValue;

pub struct ClientActionResponse {
    streamable_client_actions: Vec<Value>,
}

impl ClientActionResponse {
    pub fn new() -> Self {
        Self {
            streamable_client_actions: vec![],
        }
    }

    pub fn add<const SIZE: usize>(self: &mut Self, selector: &str, method: &str, args: [&str; SIZE]) {
        self.streamable_client_actions.push(json!({
            "selector": selector,
            "method": method,
            "args": args.to_vec(),
        }))
    }

    pub fn to_header_value(self: &Self) -> HeaderValue {
        format!("{}", json!({
            "xui:clientAction": self.streamable_client_actions
        })).parse().unwrap_or_else(|e| {
            eprintln!("error in client_action::to_header_value {}", e);
            "[]".parse().unwrap()
        })
    }
}