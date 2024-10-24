use askama::Template;
use crate::core::pagination::Pagination;
use crate::core::request_extension::HttpExt;
use crate::core::context::Context;
use crate::view::misc::NotificationTemplate;
use crate::model::product::{ListProduct, Price, Product};
use crate::model::user::User;

#[derive(Template)]
#[template(path = "views/product/detail.html")]
pub struct ProductDetailTemplate<'a> {
    pub product: &'a Product,
    pub prices: &'a Vec<Price>,
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub context: Context<'a>,
}

#[derive(Template)]
#[template(path = "views/product/list.html")]
pub struct ProductListTemplate<'a> {
    pub products: Vec<ListProduct<'a>>,
    pub authenticated_user: &'a Option<User>,
    pub pagination: &'a Pagination,
    pub notification: Option<NotificationTemplate<'a>>,
    pub errors: &'a Option<Vec<String>>,
    pub context: Context<'a>,
}

#[derive(Template)]
#[template(path = "views/product/like_toggle.html")]
pub struct AddProductToggle<'a> {
    pub action_product_id: &'a String,
    pub action_is_liked: bool,
    pub notification: Option<NotificationTemplate<'a>>,
}