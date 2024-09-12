use askama::Template;
use crate::core::context::Context;
use crate::core::request_extension::HttpExt;
use crate::view::misc::NotificationTemplate;
use crate::model::user::User;

#[derive(Template)]
#[template(path = "views/user/contacts.html")]
pub struct ContactPageTemplate<'a> {
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub errors: &'a Option<Vec<String>>,
    pub context: Context<'a>,
}

#[derive(Template)]
#[template(path = "views/user/contacts_form.html")]
pub struct AddContactFormTemplate<'a> {
    pub notification: Option<NotificationTemplate<'a>>,
    pub errors: &'a Option<Vec<String>>,
    pub context: Context<'a>,
}