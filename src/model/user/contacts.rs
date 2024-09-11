use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddContactRequestForm {
    pub contact_email: String,
}