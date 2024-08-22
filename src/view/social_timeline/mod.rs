use askama::Template;
use crate::core::{
    context::Context, pagination::Pagination
};
use crate::core::request_extension::HttpExt;
use crate::model::social_timeline::Post;
use crate::model::user::User;

use super::misc::NotificationTemplate;

#[derive(Template)]
#[template(path = "views/social_timeline/detail.html")]
pub struct PostDetailTemplate<'a> {
    pub post: &'a Post,
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub context: Context<'a>,
}

#[derive(Template)]
#[template(path = "views/social_timeline/list.html")]
pub struct PostListTemplate<'a> {
    pub posts: Vec<Post>,
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub pagination: &'a Pagination,
    pub context: Context<'a>,
}