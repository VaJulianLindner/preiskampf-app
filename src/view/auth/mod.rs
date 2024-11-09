use askama::Template;
use crate::core::context::Context;
use crate::core::request_extension::HttpExt;
use crate::view::misc::NotificationTemplate;
use crate::model::user::User;

#[derive(Template)]
#[template(path = "views/auth/login_page.html")]
pub struct LoginPageTemplate<'a> {
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub errors: &'a Option<Vec<String>>,
    pub context: Context<'a>,
}

#[derive(Template)]
#[template(path = "views/auth/register_page.html")]
pub struct RegisterPageTemplate<'a> {
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub success: bool,
    pub errors: &'a Option<Vec<String>>,
    pub context: Context<'a>,
}