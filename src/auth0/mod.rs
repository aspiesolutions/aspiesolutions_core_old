#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JwtClaims {}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Auth0Config {
    base_url: String,
    auth: AuthenticationMethod,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AuthenticationMethod {
    ClientIdAndSecret(ClientIdAndSecret),
    AccessToken { access_token: String },
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClientIdAndSecret {
    client_id: String,
    client_secret: String,
}

// pub fn get_authorization_endpoint(config: &Config) -> String {
//     match config.auth {
//         AuthenticationMethod::ClientIdAndSecret(ClientIdAndSecret{client_id,client_secret}) =>{

//             format!("https://{}/authorize?client_id={}&client_secret",config.base_url)
//         },
//         AuthenticationMethod::AccessToken { access_token } => {

//         }
//     }
// }

// an enum that represents the valid values of the 'authentication header'

pub enum AuthenticationHeader {
    Bearer(String),
}

pub struct Jwt(String);
