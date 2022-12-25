#![deny(warnings)]
#![allow(dead_code)]
pub mod error;
pub mod forms;
pub mod server;


pub use forms::{CreateUserForm,CreateOrUpdateUserForm, UpdateUserForm,DeleteUserForm};

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
pub type ManyData<Input,Output> = Vec<Result<Output,(Input,Error)>>;
pub type ManyResult<Input,Output> = Result<ManyData<Input,Output>,Error>;
pub type SingleData<Input,Output> = Result<Output,(Input,Error)>;
pub type SingleResult<Input,Output> = Result<SingleData<Input,Output>,Error>;

// SEARCH
pub type SearchUserData = Paged<aspiesolutions_entity::user::Model>;
pub type SearchUserResult = Result<SearchUserData,Error>;
//CREATE
pub type CreateManyUserData = ManyData<CreateUserForm,aspiesolutions_entity::user::Model>;
pub type CreateManyUserResult = ManyResult<CreateUserForm,aspiesolutions_entity::user::Model>;
pub type CreateUserData = SingleData<CreateUserForm,aspiesolutions_entity::user::Model>;
pub type CreateUserResult = SingleResult<CreateUserForm,aspiesolutions_entity::user::Model>;
// CREATE OR UPDATE
pub type CreateOrUpdateManyUserData = ManyData<aspiesolutions_entity::user::Model,CreateOrUpdateUserForm>;
pub type CreateOrUpdateManyUserResult = ManyResult<aspiesolutions_entity::user::Model,CreateOrUpdateUserForm>;
pub type CreateOrUpdateUserData= ManyData<aspiesolutions_entity::user::Model,CreateOrUpdateUserForm>;
pub type CreateOrUpdateUserResult = SingleResult<aspiesolutions_entity::user::Model,CreateOrUpdateUserForm>;
// UPDATE
pub type UpdateManyUserData = ManyData<aspiesolutions_entity::user::Model,UpdateUserForm>;
pub type UpdateManyUserResult = ManyResult<aspiesolutions_entity::user::Model,UpdateUserForm>;
pub type UpdateUserData= ManyData<aspiesolutions_entity::user::Model,UpdateUserForm>;
pub type UpdateUserResult = SingleResult<aspiesolutions_entity::user::Model,UpdateUserForm>;
// DELETE
pub type DeleteManyUserData = ManyData<aspiesolutions_entity::user::Model,DeleteUserForm>;
pub type DeleteManyUserResult = ManyResult<aspiesolutions_entity::user::Model,DeleteUserForm>;
pub type DeleteUserData= ManyData<aspiesolutions_entity::user::Model,DeleteUserForm>;
pub type DeleteUserResult = SingleResult<aspiesolutions_entity::user::Model,DeleteUserForm>;


