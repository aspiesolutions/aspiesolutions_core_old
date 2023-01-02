// use serde::{Deserialize, Serialize};
pub mod user;
pub use user::{
    CreateOrUpdateUserForm, CreateOrUpdateUserFormData, CreateUserForm, CreateUserFormData, DeleteUserForm,
    UpdateUserForm, UserSearchForm, UserSearchFormData,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[derive(PartialEq, Eq, Clone)]
pub struct StringSearchFilter {
    contains: Option<String>,
    equals: Option<String>,
}
impl StringSearchFilter {
    pub fn contains(&self) -> Option<&str> {
        self.contains.as_ref().map(|s|s.as_str())
    }
    pub fn equals(&self) -> Option<&str> {
        self.equals.as_ref().map(|s|s.as_str())
    }
}
