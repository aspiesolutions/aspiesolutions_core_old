#![deny(rust_2018_idioms)]
#![deny(unused_imports)]
#![allow(dead_code)]
#![deny(unused_assignments)]

#[cfg(feature = "rocket")]
pub mod rocket;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[derive(PartialEq, Eq)]
pub struct PagedResponse<T> {
    num_items: u64,
    num_pages: u64,
    cur_page: u64,
    data: Vec<T>,
}

pub type PagedResponseResult<T, E> = Result<PagedResponse<T>, E>;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[derive(PartialEq, Eq)]
pub struct BatchOperationResponse<T, E> {
    data: Vec<T>,
    errors: Vec<E>,
}
