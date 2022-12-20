// use serde::{Deserialize, Serialize};
pub mod user;
use std::borrow::Cow;
pub use user::{
    CreateOrUpdateUserForm, CreateOrUpdateUserFormData, CreateUserForm, DeleteUserForm,
    UpdateUserForm, UserSearchForm, UserSearchFormData,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[derive(PartialEq, Eq, Clone)]
pub struct StringSearchFilter<'r> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    contains: Cow<'r, str>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    equals: Cow<'r, str>,
}
impl<'r> StringSearchFilter<'r> {
    pub fn contains(&'r self) -> &Cow<'r, str> {
        &self.contains
    }
    pub fn equals(&'r self) -> &Cow<'r, str> {
        &self.equals
    }
}
