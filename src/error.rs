#[cfg(feature = "sea-orm")]
pub use aspiesolutions_entity::sea_orm;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature="rocket",derive(rocket::response::Responder))]
pub enum Error {
    #[error("A Database Error Occurred '{0}'")]
    // #[cfg_attr(feature="rocket",response(status=500))]
    DbError(String),
    #[error("Failed to encode msgpack {0}")]
    MsgPackEncodeError(String),
    #[error("Failed to decode msgpack {0}")]
    MsgPackDecodeError(String),
    #[error("A Database Error Occurred while processing a transaction '{0}' ")]
    // #[cfg_attr(feature="rocket",response(status=500))]
    TransactionError(String),
    #[error("{0}")]
    ClientError(ClientError),
    #[error("{0}")]
    ApiError(ApiError),
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature="rocket",derive(rocket::response::Responder))]
pub enum ApiError {
    // CreateOrUpdateUserFailure()
    #[error("Failed To Create User in the system")]
    CreateUserFailure {
        form_data: crate::forms::CreateOrUpdateUserFormData,
    },
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub enum ClientError {
    #[error("{0}")]
    GeneralError(String),
    #[error("unhandled response code {0}")]
    UnhandledResponseStatusCode(String),
    #[error("The operation failed because the server said that the request body was malformed")]
    ServerRespondedWithUnprocessableEntity,
    #[error("The operation failed because the server refused the request or did not find the requested resource.")]
    NotFoundOrRefused,
}

#[cfg(feature = "sea-orm")]
impl std::convert::From<sea_orm::DbErr> for Error {
    fn from(e: sea_orm::DbErr) -> Self {
        Self::DbError(e.to_string())
    }
}
#[cfg(feature = "sea-orm")]
impl std::convert::From<sea_orm::TransactionError<sea_orm::DbErr>> for Error {
    fn from(e: sea_orm::TransactionError<sea_orm::DbErr>) -> Self {
        Self::DbError(e.to_string())
    }
}

impl std::convert::From<ClientError> for Error {
    fn from(e: ClientError) -> Self {
        Self::ClientError(e)
    }
}
impl std::convert::From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        Self::ApiError(e)
    }
}

#[cfg(feature = "reqwest")]
impl std::convert::From<reqwest::Error> for ClientError {
    fn from(e: reqwest::Error) -> Self {
        Self::GeneralError(e.to_string())
    }
}
#[cfg(feature = "reqwest")]
impl std::convert::From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::ClientError(e.into())
    }
}
#[cfg(feature = "rmp-serde")]
impl std::convert::From<rmp_serde::encode::Error> for Error {
    fn from(e: rmp_serde::encode::Error) -> Self {
        Self::MsgPackEncodeError(e.to_string())
    }
}
#[cfg(feature = "rmp-serde")]
impl std::convert::From<rmp_serde::decode::Error> for Error {
    fn from(e: rmp_serde::decode::Error) -> Self {
        Self::MsgPackDecodeError(e.to_string())
    }
}
