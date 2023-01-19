#[cfg(feature = "sea-orm")]
pub use crate::entity::sea_orm;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[cfg_attr(feature="rocket",derive(rocket::response::Responder))]
pub enum Error {
    // for internal rust errors
    #[error("{0}")]
    PoisonError(String),

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
    PasswordHashError(String),
    // below errors are for client errors
    #[cfg(feature = "reqwest")]
    #[cfg_attr(feature = "reqwest", error("{0}"))]
    ReqwestError(String),

    #[error("unhandled response code {0}")]
    UnhandledResponseStatusCode(String),
    #[error("The operation failed because the server said that the request body was malformed")]
    ServerRespondedWithUnprocessableEntity,
    #[error("The operation failed because the server refused the request or did not find the requested resource.")]
    NotFoundOrRefused,
    #[error("The operation failed because the server says that you are not authorized to perform this request: '{0}'")]
    Unauthorized(String),
    #[error("The operation failed because the server forbids it '{0}'")]
    Forbidden(String),
    #[error("{0}")]
    TokioJoinError(String),
    //below errors are for apis
    #[error("Failed To Create User in the system")]
    CreateUserFailure {
        form_data: crate::forms::CreateOrUpdateUserFormData,
    },
    // below errors for auth0
    #[error("Calling this endpoint requires a client secret to be configured")]
    ClientSecretNotConfigured,
    #[error("{0}")]
    FigmentError(String),
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

#[cfg(feature = "reqwest")]
impl std::convert::From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::ReqwestError(e.to_string())
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

#[cfg(feature = "rust-argon2")]
impl std::convert::From<argon2::Error> for Error {
    fn from(e: argon2::Error) -> Self {
        Self::PasswordHashError(e.to_string())
    }
}
#[cfg(feature = "tokio")]
impl std::convert::From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Self {
        Self::TokioJoinError(e.to_string())
    }
}
impl<T> std::convert::From<std::sync::PoisonError<T>> for Error {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        Self::PoisonError(e.to_string())
    }
}
#[cfg(feature = "figment")]
impl std::convert::From<figment::Error> for Error {
    fn from(e: figment::Error) -> Self {
        Self::FigmentError(e.to_string())
    }
}
