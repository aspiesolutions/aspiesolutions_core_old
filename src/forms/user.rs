use super::StringSearchFilter;
use std::borrow::Cow;
// use this type alias to allow for defaults while also allowing to specify a different type in case the type changes
type UserId = aspiesolutions_entity::user::Id;
#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq)]
pub struct CreateUserForm<'r> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    name: Cow<'r, str>,
}

impl<'r> CreateUserForm<'r> {
    pub fn new() -> Self {
        Self {
            name: Cow::Owned(String::new()),
        }
    }
    pub fn set_name(mut self, str: &'r str) -> Self {
        self.name = Cow::Borrowed(str);
        self
    }
    pub fn name(&'r self) -> Cow<'r, str> {
        self.name.clone()
    }
}

#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug,PartialEq, Eq, Clone)]
pub struct CreateOrUpdateUserFormData<Id = UserId> {
    id: Option<Id>,
    name: String,
}
impl<Id> CreateOrUpdateUserFormData<Id> {
    pub fn id(self) -> Option<Id> {
        self.id
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}


#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone)]
pub struct CreateOrUpdateUserForm<Id: Clone = UserId> {
    user: CreateOrUpdateUserFormData<Id>,
}
impl<'r> CreateOrUpdateUserForm {
    pub fn user(&'r self) -> &'r CreateOrUpdateUserFormData {
        &self.user
    }
}

#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone)]
pub struct UpdateUserForm<'r, Id = UserId> {
    id: Id,
    name: &'r str,
}
impl<'r, Id> UpdateUserForm<'r, Id> {
    pub fn id(&'r self) -> &'r Id {
        &self.id
    }
    pub fn name(&'r self) -> &'r str {
        &self.name
    }
}

#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone)]
pub struct DeleteUserForm<Id = UserId> {
    id: Vec<Id>,
    cascade_on_delete: bool,
}
impl<Id> DeleteUserForm<Id> {
    pub fn get_ids<'a>(&'a self) -> &'a Vec<Id> {
        self.id.as_ref()
    }
    pub fn do_cascade_on_delete<'a>(&'a self) -> &'a bool {
        &self.cascade_on_delete
    }
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[derive(PartialEq, Eq, Clone)]
pub struct UserSearchFormData<'r> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    name: StringSearchFilter<'r>,
}
impl<'r> UserSearchFormData<'r> {
    pub fn name(&'r self) -> &'r StringSearchFilter<'r> {
        &self.name
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[derive(PartialEq, Eq, Clone)]
pub struct UserSearchForm<'r> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    user: Option<UserSearchFormData<'r>>,
    page: Option<u32>,
    limit: Option<u16>,
}
impl<'r> UserSearchForm<'r> {
    pub fn user(&'r self) -> Option<&'r UserSearchFormData<'r>> {
        self.user.as_ref()
    }
    pub fn limit(&'r self) -> Option<&'r u16> {
        self.limit.as_ref()
    }
    pub fn page(&'r self) -> Option<&'r u32> {
        self.page.as_ref()
    }
}
