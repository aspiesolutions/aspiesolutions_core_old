// implement any rocket derives here
use rocket::serde::msgpack::MsgPack;
#[derive(rocket::Responder)]
#[response(status = 200)]
/// a responder that describes a struct that can  collect data and all recoverable errors
/// thereby converting a failed response to a response with data and errors
pub struct BatchOperationSuccessResponse<T, E> {
    inner: MsgPack<crate::BatchOperationResponse<T, E>>,
}

#[cfg(feature="serde")]
impl<T, E> BatchOperationSuccessResponse<T, E>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    E: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    pub fn new(response: crate::BatchOperationResponse<T, E>) -> Self {
        Self {
            inner: MsgPack(response),
        }
    }
}

#[derive(rocket::Responder)]
/// a responder that describes an unrecoverable error response
pub struct BatchOperationErrorResponse<E> {
    inner: MsgPack<E>,
}
pub type BatchOperationResult<T, E> =
    Result<BatchOperationSuccessResponse<T,E>, BatchOperationErrorResponse<E>>;


#[derive(rocket::Responder)]
pub struct PagedResponse<T> {
  inner:MsgPack<crate::PagedResponse<T>>
}
#[cfg(feature="serde")]
impl<T> PagedResponse<T> where T: serde::Serialize + for <'de> serde::Deserialize<'de> {
  pub fn new(r:crate::PagedResponse<T>) -> Self {
    Self {inner: MsgPack(r)}
  }
}