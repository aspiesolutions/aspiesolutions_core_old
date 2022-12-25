use super::StringSearchFilter;
// use this type alias to allow for defaults while also allowing to specify a different type in case the type changes
type UserId = aspiesolutions_entity::user::Id;
#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone)]
pub struct CreateUserForm {
    // #[cfg_attr(feature = "serde", serde(borrow))]
    id: uuid::Uuid,
    data:CreateUserFormData,
}
impl CreateUserForm {
    pub fn new(data: CreateUserFormData) -> Self {
        let id = uuid::Uuid::new_v4();
        Self {
            id,
            data
        }
    }
    pub fn data(&self) -> &CreateUserFormData {
        &self.data
    }
}
#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone)]
pub struct CreateUserFormData {
    name:String
}

impl CreateUserFormData {
    pub fn new() -> Self {
        Self {
            name: String::new(),
        }
    }
    pub fn set_name(mut self, s: &str) -> Self {
        self.name = s.to_string();
        self
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
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
impl CreateOrUpdateUserForm {
    pub fn user(&self) -> &CreateOrUpdateUserFormData {
        &self.user
    }
}

#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone)]
pub struct UpdateUserForm<Id = UserId> {
    id: Id,
    name: String,
}
impl<Id> UpdateUserForm<Id> {
    pub fn id(&self) -> &Id {
        &self.id
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
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
pub struct UserSearchFormData {
    name: StringSearchFilter,
}
impl UserSearchFormData {
    pub fn name<'r>(&'r self) -> &'r StringSearchFilter {
        &self.name
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
#[derive(PartialEq, Eq, Clone)]
pub struct UserSearchForm {
    user: Option<UserSearchFormData>,
    page: Option<u32>,
    limit: Option<u16>,
}
impl UserSearchForm {
    pub fn user(&self) -> Option<&UserSearchFormData> {
        self.user.as_ref()
    }
    pub fn limit<'b>(&self) -> Option<&u16> {
        self.limit.as_ref()
    }
    pub fn page(&self) -> Option<&u32> {
        self.page.as_ref()
    }
}
