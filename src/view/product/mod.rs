use askama::Template;
use crate::core::pagination::Pagination;
use crate::core::request_extension::HttpExt;
use crate::core::context::Context;
use crate::view::misc::NotificationTemplate;
use crate::model::product::Product;
use crate::model::user::User;

#[derive(Template)]
#[template(path = "views/product/detail.html")]
pub struct ProductDetailTemplate<'a> {
    pub product: &'a Product,
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub context: Context<'a>,
}

#[derive(Template)]
#[template(path = "views/product/list.html")]
pub struct ProductListTemplate<'a> {
    pub products: Vec<Product>,
    pub authenticated_user: &'a Option<User>,
    pub pagination: &'a Pagination,
    pub notification: Option<NotificationTemplate<'a>>,
    pub context: Context<'a>,
}