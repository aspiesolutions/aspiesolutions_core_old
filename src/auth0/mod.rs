#[cfg_attr(feature="serde",derive(serde::Serialize,serde::Deserialize))]
pub struct JwtClaims {
}




pub struct Config {
    base_url:String,
    auth:AuthenticationMethod
}
pub enum AuthenticationMethod {
    ClientIdAndSecret{client_id:String,client_secret:String},
    AccessToken{access_token:String}
}

// an enum that represents the valid values of the 'authentication header'

pub enum AuthenticationHeader {
    Bearer(String),
}

pub struct Jwt(String);