use askama::Template;
use crate::core::context::Context;
use crate::core::request_extension::HttpExt;
use crate::model::user::User;
use crate::view::misc::NotificationTemplate;

#[derive(Template)]
#[template(path = "views/static_page/home.html")]
pub struct HomeTemplate<'a> {
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub context: Context<'a>,
}

#[derive(Template)]
#[template(path = "views/static_page/imprint.html")]
pub struct ImprintTemplate<'a> {
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub context: Context<'a>,
}

#[derive(Template)]
#[template(path = "views/static_page/about.html")]
pub struct AboutTemplate<'a> {
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub context: Context<'a>,
}

#[derive(Template)]
#[template(path = "views/static_page/404.html")]
pub struct NotFoundTemplate<'a> {
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub context: Context<'a>,
}