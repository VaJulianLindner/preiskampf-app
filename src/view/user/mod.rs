pub mod contacts;

use askama::Template;
use crate::core::context::Context;
use crate::core::request_extension::HttpExt;
use crate::view::misc::NotificationTemplate;
use crate::model::user::User;

#[derive(Template)]
#[template(path = "views/user/detail.html")]
pub struct UserDetailTemplate<'a> {
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub errors: &'a Option<Vec<String>>,
    pub context: Context<'a>,
}