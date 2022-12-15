#![deny(rust_2018_idioms)]
#![deny(unused_imports)]
#![allow(dead_code)]
#![deny(unused_assignments)]

#[cfg(feature = "rocket")]
pub mod rocket;


type Results<T,E> = Vec<Result<T,E>>; 
type ErrorsWithOptionalData<T,E> = Vec<ErrorWithOptionalData<T,E>>;
type Errors<E> = Vec<E>;

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
pub struct Recoverable<T, E> {
    data: Vec<T>,
    errors: Errors<E>,
}
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

impl<T,E> Recoverable<T,E> {
    pub fn new(data:Vec<T>,errors: Vec<E>) -> Self {
        Self{data,errors}
    }
    pub fn data<'a>(&'a self) -> &'a Vec<T> {
        self.data.as_ref()
    }
    pub fn errors<'a>(&'a self) -> &'a Vec<E>{
        self.errors.as_ref()
    }
}

impl<T,E> std::convert::From<Results<T,E>> for Recoverable<T,E> {
    fn from(results: Results<T,E>) -> Self {
        let mut data = Vec::<T>::new();
        let mut errors = Vec::<E>::new();
        for result in results {
            match result {
                Ok(t) => data.push(t),
                Err(e) => errors.push(e)
            }
        }
        Self {
            data,
            errors
        }
        
    }
}
// impl<T,E> std::convert::From<(T,E)> for Recoverable<T,E> {
//     fn from((t,errors): (T,E)) -> Self {
//         Self {
//             data:vec![t],
//             errors
//         }
//     }
// }
// impl<T,E> std::convert::From<(Vec<T>,E)> for Recoverable<T,E> {
//     fn from((data,errors): (Vec<T>,E)) -> Self {
//         Self {
//             data,
//             errors
//         }
//     }
// }
type MaybeRecoverable<T,E> = Result<Recoverable<T,E>,E>;