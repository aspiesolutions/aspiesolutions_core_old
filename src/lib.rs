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
pub struct Paged<T> {
    num_items: u64,
    num_pages: u64,
    cur_page: u64,
    data: Vec<T>,
}

pub type PagedResult<T, E> = Result<Paged<T>, E>;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[derive(PartialEq, Eq)]
pub struct Recoverable<T, E> {
    data: Vec<T>,
    errors: Vec<E>,
}