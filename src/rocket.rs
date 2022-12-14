// implement any rocket derives here
use rocket::serde::msgpack::MsgPack;
#[cfg(feature="serde")]
impl<T, E> Recoverable<T, E>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    E: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    pub fn new(response: crate::Recoverable<T, E>) -> Self {
        Self {
            inner: MsgPack(response),
        }
    }
}

#[derive(rocket::Responder)]
/// a responder that describes an unrecoverable error response
#[response(status=200)]
pub struct Recoverable<T,E> {
    inner: MsgPack<crate::Recoverable<T,E>>,
}

#[derive(rocket::Responder)]
pub struct Paged<T> {
  inner:MsgPack<crate::Paged<T>>
}
#[cfg(feature="serde")]
impl<T> Paged<T> where T: serde::Serialize + for <'de> serde::Deserialize<'de> {
  pub fn new(r:crate::Paged<T>) -> Self {
    Self {inner: MsgPack(r)}
  }
}

impl <T,E> std::convert::From<crate::Recoverable<T,E>> for self::Recoverable<T,E> {
    fn from(r: crate::Recoverable<T,E>) -> Self {
        Self{inner:MsgPack(r)}
    }
}
type PagedResult<T,E> = Result<Paged<T>,E>;