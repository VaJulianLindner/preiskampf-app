use askama::Template;
use crate::{
    core::pagination::Pagination,
    model::misc::Notification,
    core::context::Context,
};

#[derive(Template)]
#[template(path = "partials/notifications/message_template.html")]
pub struct NotificationTemplate<'a> {
    pub notification: Notification<'a>,
}

#[derive(Template)]
#[template(path = "partials/pagination/base.html")]
pub struct PaginationTemplate<'a> {
    pub pagination: &'a Pagination,
    pub context: &'a Context<'a>,
}