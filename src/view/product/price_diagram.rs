use askama::Template;
use crate::core::context::Context;
use crate::model::product::price_diagram::PriceDiagram;
use crate::model::user::User;

#[derive(Template)]
#[template(path = "views/product/price_diagram.html")]
pub struct PriceDiagramTemplate<'a> {
    pub model: PriceDiagram,
    pub authenticated_user: &'a Option<User>,
    pub context: &'a Context<'a>,
}