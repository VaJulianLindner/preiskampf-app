use askama::Template;
use crate::core::context::Context;
use crate::core::request_extension::HttpExt;
use crate::model::user::contacts::LinkedContact;
use crate::view::misc::NotificationTemplate;
use crate::model::user::User;

#[derive(Template)]
#[template(path = "views/user/contacts.html")]
pub struct ContactPageTemplate<'a> {
    pub authenticated_user: &'a Option<User>,
    pub contacts: &'a Vec<LinkedContact>,
    pub requested_contacts: &'a Vec<LinkedContact>,
    pub pending_contacts: &'a Vec<LinkedContact>,
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

#[derive(Template)]
#[template(path = "views/user/contacts_list_entry.html")]
pub struct ContactListEntryTemplate<'a> {
    pub authenticated_user: &'a Option<User>,
    pub notification: Option<NotificationTemplate<'a>>,
    pub contact_entry: &'a LinkedContact,
    pub oob_swap_target: &'a Option<&'a str>,
    pub context: Context<'a>,
}