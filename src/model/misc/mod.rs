use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Notification<'a> {
    pub is_oob_swap: bool,
    pub is_success: bool,
    pub message: &'a str,
    pub hint: Option<&'a str>,
}