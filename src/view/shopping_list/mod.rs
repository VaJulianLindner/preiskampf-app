use askama::Template;
use crate::core::{
    context::Context, pagination::Pagination
};
use crate::core::request_extension::HttpExt;
use crate::model::shopping_list::ShoppingList;
use crate::model::user::User;

use super::misc::NotificationTemplate;

const EMOJI_LIST: [u32; 35] = [
    128525, 128526, 129303, 129322, 128571, 9757, 9996, 128513,
    128020, 128022, 128025, 128035, 128048, 129424, 129445, 128106,
    128103, 129492, 128170, 128150, 9749, 127864, 129346, 129475,
    127829, 127791, 127831, 127843, 127847, 129360, 129386, 127814,
    129361, 129382, 128293,
];

#[derive(Template)]
#[template(path = "views/shopping_list/detail.html")]
pub struct ShoppingListDetailTemplate<'a> {
    pub shopping_list: &'a ShoppingList,
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub errors: &'a Option<Vec<String>>,
    pub context: Context<'a>,
}

impl<'a> ShoppingListDetailTemplate<'a> {
    fn get_reset_url(&self) -> String {
        if self.shopping_list.id != 0 {
            format!("/einkaufszettel/{}", self.shopping_list.id)
        } else {
            "/einkaufszettel/anlegen".to_string()
        }
    }
}

#[derive(Template)]
#[template(path = "views/shopping_list/list.html")]
pub struct ShoppingListsTemplate<'a> {
    pub shopping_lists: Vec<ShoppingList>,
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub pagination: &'a Pagination,
    pub errors: &'a Option<Vec<String>>,
    pub context: Context<'a>,
}