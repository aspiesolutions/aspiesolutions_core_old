use std::collections::HashMap;

use reqwest::{Response, StatusCode};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, thiserror::Error)]
pub enum ManagementApiV2Error {
    #[error("ReqwestError: {0}")]
    ReqwestError(String),
    #[error("Slow Down! Too many requests")]
    SlowDown,
    #[error("Not Found. Check your configuration and try again")]
    NotFound,
    #[error("Unauthorized. Check that you have a valid client id and client secret, and that your client has the correct permissions")]
    Unauthorized,
    #[error("Forbidden.")]
    Forbidden
}
impl std::convert::From<reqwest::Error> for ManagementApiV2Error {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(error.to_string())
    }
}
// an instance of a client used to communicate with the auth0 management api
// management clients should be accessed currently using client id and client secret

/// The default api endpoint prefix to append
const API_ENDPOINT_PREFIX: &str = "/api/v2/";
const TOKEN_ENDPOINT: &str = "/oauth/token";
/// the protocol to use when communicating (https is required)
const PROTOCOL: &str = "https://";
const BEARER_PREFIX: &str = "Bearer ";

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ManagementApiV2Config {
    pub client_id: String,
    pub client_secret: String,
    pub domain: String,
    // the tenant domain to use when using back-channel communication.
    // make sure to use the domain provided by auth0 instead of custom domains
    // because of how audience values are formatted
}
#[derive(serde::Serialize,serde::Deserialize,Clone,Debug)]
pub struct AccessToken {
    access_token:String,
    scope:String,
    iat:usize,
    token_type:String
    
}



pub struct ManagementApiV2Client {
    client: reqwest::Client,
    config: ManagementApiV2Config,
    access_token:String
}

impl ManagementApiV2Client {
    pub async fn new(config: &ManagementApiV2Config) -> Result<Self, ManagementApiV2Error> {
        let client = reqwest::Client::new();
        let token = Self::get_management_token(&client, &config).await?;
        Ok(Self {
            client,
            config: config.to_owned(),
            access_token:String::new()
        })
    }
    async fn get_management_token(
        client: &reqwest::Client,
        config: &ManagementApiV2Config,
    ) -> Result<AccessToken, ManagementApiV2Error> {
        let mut form_data: HashMap<&str, &str> = HashMap::new();
        form_data.insert("client_id", &config.client_id);
        form_data.insert("client_secret", &config.client_secret);
        form_data.insert("grant_type", "client_credentials");
        let audience = format!("{PROTOCOL}{0}{API_ENDPOINT_PREFIX}", config.domain);
        form_data.insert("audience", &audience);
        let response = client
            .post(format!("{PROTOCOL}{0}{TOKEN_ENDPOINT}", config.domain))
            .form(&form_data)
            .send()
            .await?;
        let _headers = response.headers();
        let status = response.status();
        match status {
            StatusCode::OK => {
                let token: AccessToken = response.json().await?;
                Ok(token)
            },
            StatusCode::TOO_MANY_REQUESTS => Err(ManagementApiV2Error::SlowDown),
            StatusCode::NOT_FOUND => Err(ManagementApiV2Error::NotFound),
            StatusCode::UNAUTHORIZED=> Err(ManagementApiV2Error::Unauthorized),
            unimplemented => {
                log::debug!("unimplemented {unimplemented:#?}");
                todo!("not yet implemented {unimplemented:#?}")
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use figment::{providers::Env, Figment};
    pub fn set_up_test() -> ManagementApiV2Config {
        let _ = dotenv::dotenv().ok();
        simple_logger::init_with_env().unwrap();
        let config: ManagementApiV2Config = Figment::new()
            .merge(Env::prefixed("AUTH0_"))
            .extract()
            .unwrap();

        config
    }
    #[tokio::test]
    pub async fn auth0_management_v2_client_test_create_client() {
        let config = set_up_test();
        let managemnt_api_v2_client = ManagementApiV2Client::new(&config).await;
    }
}
