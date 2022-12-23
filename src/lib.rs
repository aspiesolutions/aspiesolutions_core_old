#![deny(warnings)]
#![allow(dead_code)]
pub mod server;
pub mod forms;
pub mod error;

// pub mod user;

pub type DataWithError<T> = (T,crate::error::Error);
pub type Recoverable<T> = Result<T,DataWithError<T>>;
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
impl<'a,'b,'c,'d,T> Paged<T> {
    pub fn new(num_items:u64,num_pages:u64,cur_page:u64,data:Vec<T>) -> Self {
        Self {
            num_items,
            num_pages,
            cur_page,
            data
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

pub struct ErrorWithOptionalData<T,E> {
    data:Option<T>,
    error:E
}

impl<T,E> std::convert::From<(Option<T>,E)> for ErrorWithOptionalData<T,E> {
    fn from((data,error): (Option<T>,E)) -> Self {
        Self{data,error}
    }
}


impl<'a,'b,T,E> ErrorWithOptionalData<T,E> {
    pub fn new(data:Option<T>,error:E) -> Self {
        Self { data, error }
    }
    pub fn data(&'a self) -> Option<&'a T> {
        self.data.as_ref()
    }
    pub fn error(&'b self) -> &'b E {
        &self.error
    }

}

pub type CreateManyUser<'a> = Result<Vec<Result<aspiesolutions_entity::user::Model, (crate::forms::CreateUserForm<'a>,crate::error::Error)>>,crate::error::Error>;

pub type DeleteManyUser =  Result<RecoverableMany<aspiesolutions_entity::user::Id>,crate::error::Error>;
pub type SearchUser = PagedResult<aspiesolutions_entity::user::Model,crate::error::Error>;