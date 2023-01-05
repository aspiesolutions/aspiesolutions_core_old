use rocket::request::{FromRequest, Outcome, Request};
// use rocket::outcome::{};
use rocket::http::{CookieJar,Cookie
};
// use rocket::http::Status;
use std::str::FromStr;

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
        let mut cookie:Cookie =  Cookie::named(SESSION_COOKIE_NAME);
        cookie.set_value(sid.0.to_string());
        cookie
    }
}
