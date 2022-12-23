// use serde::{Deserialize, Serialize};
pub mod user;
pub use user::{
    CreateOrUpdateUserForm, CreateOrUpdateUserFormData, CreateUserForm, DeleteUserForm,
    UpdateUserForm, UserSearchForm, UserSearchFormData,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[derive(PartialEq, Eq, Clone)]
pub struct StringSearchFilter {
    contains: String,
    equals: String,
}
impl StringSearchFilter {
    pub fn contains(&self) -> &str {
        self.contains.as_str()
    }
    pub fn equals(&self) -> &str {
        self.equals.as_str()
    }
}
