#![deny(warnings)]
#![allow(dead_code)]
pub mod auth0;
#[cfg(feature = "sea-orm")]
pub mod db;
pub mod entity;
pub mod error;
pub mod forms;
#[cfg(feature = "rocket")]
pub mod server;

pub use forms::{
    CreateOrUpdateUserForm, CreateUserForm, CreateUserFormData, DeleteUserForm, UpdateUserForm,
};

pub use crate::error::Error;
// pub mod user;

pub type DataWithError<T> = (T, Error);
pub type Recoverable<T> = Result<T, DataWithError<T>>;
pub type RecoverableMany<T> = Vec<Recoverable<T>>;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[derive(PartialEq, Eq)]
pub struct Paged<T> {
    num_items: u64,
    num_pages: u64,
    cur_page: u64,
    data: Vec<T>,
}
impl<'a, 'b, 'c, 'd, T> Paged<T> {
    pub fn new(num_items: u64, num_pages: u64, cur_page: u64, data: Vec<T>) -> Self {
        Self {
            num_items,
            num_pages,
            cur_page,
            data,
        }
    }
    pub fn num_items(&'a self) -> &'a u64 {
        &self.num_items
    }
    pub fn num_pages(&'b self) -> &'b u64 {
        &self.num_pages
    }
    pub fn cur_page(&'c self) -> &'c u64 {
        &self.cur_page
    }
    pub fn data(&'d self) -> &'d Vec<T> {
        self.data.as_ref()
    }
}
pub type PagedResult<T, E> = Result<Paged<T>, E>;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[derive(PartialEq, Eq)]

pub struct ErrorWithOptionalData<T, E> {
    data: Option<T>,
    error: E,
}

impl<T, E> std::convert::From<(Option<T>, E)> for ErrorWithOptionalData<T, E> {
    fn from((data, error): (Option<T>, E)) -> Self {
        Self { data, error }
    }
}

impl<'a, 'b, T, E> ErrorWithOptionalData<T, E> {
    pub fn new(data: Option<T>, error: E) -> Self {
        Self { data, error }
    }
    pub fn data(&'a self) -> Option<&'a T> {
        self.data.as_ref()
    }
    pub fn error(&'b self) -> &'b E {
        &self.error
    }
}

// TYPEALIASES
pub type ManyData<Input, Output> = Vec<Result<Output, (Input, Error)>>;
pub type ManyResult<Input, Output> = Result<ManyData<Input, Output>, Error>;
pub type SingleData<Input, Output> = Result<Output, (Input, Error)>;
pub type SingleResult<Input, Output> = Result<SingleData<Input, Output>, Error>;

// SEARCH
pub type SearchUserData = Paged<crate::entity::user::Model>;
pub type SearchUserResult = Result<SearchUserData, Error>;
//CREATE
pub type CreateManyUserData = ManyData<CreateUserForm, crate::entity::user::Model>;
pub type CreateManyUserResult = ManyResult<CreateUserForm, crate::entity::user::Model>;
pub type CreateUserData = SingleData<CreateUserForm, crate::entity::user::Model>;
pub type CreateUserResult = SingleResult<CreateUserForm, crate::entity::user::Model>;
// CREATE OR UPDATE
pub type CreateOrUpdateManyUserData = ManyData<CreateOrUpdateUserForm, crate::entity::user::Model>;
pub type CreateOrUpdateManyUserResult =
    ManyResult<CreateOrUpdateUserForm, crate::entity::user::Model>;
pub type CreateOrUpdateUserData = ManyData<CreateOrUpdateUserForm, crate::entity::user::Model>;
pub type CreateOrUpdateUserResult =
    SingleResult<CreateOrUpdateUserForm, crate::entity::user::Model>;
// UPDATE
pub type UpdateManyUserData = ManyData<UpdateUserForm, crate::entity::user::Model>;
pub type UpdateManyUserResult = ManyResult<UpdateUserForm, crate::entity::user::Model>;
pub type UpdateUserData = ManyData<UpdateUserForm, crate::entity::user::Model>;
pub type UpdateUserResult = SingleResult<UpdateUserForm, crate::entity::user::Model>;
// DELETE
pub type DeleteManyUserData = ManyData<DeleteUserForm, crate::entity::user::Model>;
pub type DeleteManyUserResult = ManyResult<DeleteUserForm, crate::entity::user::Model>;
pub type DeleteUserData = ManyData<DeleteUserForm, crate::entity::user::Model>;
pub type DeleteUserResult = SingleResult<DeleteUserForm, crate::entity::user::Model>;
