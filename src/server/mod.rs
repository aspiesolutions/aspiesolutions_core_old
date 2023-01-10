use rocket::request::{FromRequest, Outcome, Request};
// use rocket::outcome::{};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::State;
// use rocket::http::Status;
use std::str::FromStr;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[derive(Debug)]
pub struct ServerConfig {
    domain: String,
    database_url: String,
    auth0: crate::auth0::Auth0Config,
}
impl ServerConfig {
    pub fn database_url(&self) -> &str {
        &self.database_url
    }
    pub fn domain(&self) -> &str {
        &self.domain
    }
    pub fn auth0(&self) -> &crate::auth0::Auth0Config {
        &self.auth0
    }
}
// create a request guard that represents a user whos browser sends us an encrypted "session_id" token
pub struct SessionIdCookie(uuid::Uuid);

impl SessionIdCookie {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
    pub fn get_id(&self) -> &uuid::Uuid {
        &self.0
    }
}

pub const SESSION_COOKIE_NAME: &str = "session_id";
#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionIdCookie {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // let cookie_jar = ;
        let session_uuid: uuid::Uuid = match request
            .guard::<&CookieJar<'_>>()
            .await
            .succeeded()
            .and_then(|cookie_jar| cookie_jar.get_private(SESSION_COOKIE_NAME))
            .and_then(|session_cookie| uuid::Uuid::from_str(session_cookie.value()).ok())
        {
            Some(session_uuid) => session_uuid,
            None => return Outcome::Forward(()),
        };

        return Outcome::Success(SessionIdCookie(session_uuid));
    }
}
impl<'r> std::convert::From<SessionIdCookie> for Cookie<'r> {
    fn from(sid: SessionIdCookie) -> Cookie<'r> {
        let mut cookie: Cookie = Cookie::named(SESSION_COOKIE_NAME);
        cookie.set_value(sid.0.to_string());
        cookie
    }
}
pub struct Session(pub crate::entity::session::Model);
// find a way to accept a session id cookie in FromRequest and use that session id cookie to
// look up a database session
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        use crate::entity;
        use sea_orm::prelude::*;
        // first we need the session id cookie
        let maybe_session_id_cookie = request.guard::<SessionIdCookie>().await.succeeded();
        if maybe_session_id_cookie.is_none() {
            // forward if the session id cookie guard forwards, fails, or errors
            return Outcome::Forward(());
        }
        // now we need a databse connection
        let maybe_database_connection = request
            .guard::<&State<DatabaseConnection>>()
            .await
            .succeeded();
        if maybe_database_connection.is_none() {
            return Outcome::Failure((
                Status::InternalServerError,
                "failed to get database connection".to_string(),
            ));
        }
        let session_id = maybe_session_id_cookie.unwrap().get_id().to_owned();
        let database_connection = maybe_database_connection.unwrap();
        let session_search_result = entity::session::Entity::find()
            .filter(entity::session::Column::Uuid.eq(session_id))
            .one(database_connection.inner())
            .await;
        if session_search_result.is_err() {
            return Outcome::Failure((
                Status::InternalServerError,
                session_search_result.unwrap_err().to_string(),
            ));
        }
        match session_search_result.unwrap() {
            Some(session) => Outcome::Success(Session(session)),
            None => Outcome::Forward(()),
        }
    }
}
