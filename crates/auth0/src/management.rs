use const_format::concatcp;
use std::collections::HashMap;

const SUBJECT_SIGNING_KEYS: &str = "signing_keys";
const ACTION_READ: &str = "read";
const SCOPE_SEPERATOR:&str = ":";
const SCOPE_READ_SIGING_KEYS: &str = concatcp!(ACTION_READ, SCOPE_SEPERATOR, SUBJECT_SIGNING_KEYS);

// const SCOPE_WRITE:&str = "write";

use reqwest::{Request, Response, StatusCode};

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
    Forbidden,
    #[error("We are not sure how to handle this response yet. Recieved status code {0}")]
    UnhandledStatusCode(String),
    #[error("Access token is missing a required scope {0}")]
    AccessTokenMissingRequiredScope(String),
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct AccessToken {
    access_token: String,
    scope: String,
    // iat: usize,
    expires_in: usize,
    token_type: String,
}

pub struct ManagementApiV2Client {
    client: reqwest::Client,
    config: ManagementApiV2Config,
    access_token: AccessToken,
}

impl ManagementApiV2Client {
    pub async fn new(config: &ManagementApiV2Config) -> Result<Self, ManagementApiV2Error> {
        let client = reqwest::Client::new();
        let access_token = Self::get_management_token(&client, &config).await?;
        let signing_keys = Self::get_signing_keys(&client, config, &access_token).await?;
        Ok(Self {
            client,
            config: config.to_owned(),
            access_token,
        })
    }
    fn check_scopes(
        scopes: &[&str],
        access_token: &AccessToken,
    ) -> Result<(), ManagementApiV2Error> {
        for scope in scopes {
            if !access_token.scope.contains(scope) {
                return Err(ManagementApiV2Error::AccessTokenMissingRequiredScope(
                    scope.to_string(),
                ));
            }
        }
        Ok(())
    }
    async fn get_signing_keys(
        client: &reqwest::Client,
        config: &ManagementApiV2Config,
        access_token: &AccessToken,
    ) -> Result<(), ManagementApiV2Error> {
        log::debug!("getting signing keys");
        // add some local permission checking
        Self::check_scopes(&[SCOPE_READ_SIGING_KEYS], access_token)?;
        let response = client
            .get(format!(
                "{PROTOCOL}{0}{API_ENDPOINT_PREFIX}keys/signing",
                config.domain
            ))
            .header(
                "Authorization",
                format!("{BEARER_PREFIX}{0}", access_token.access_token),
            )
            .send()
            .await?;
        let status = response.status();
        match status {
            StatusCode::OK => Ok(()),
            unhandled => Err(ManagementApiV2Error::UnhandledStatusCode(
                unhandled.to_string(),
            )),
        }
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
            }
            StatusCode::TOO_MANY_REQUESTS => Err(ManagementApiV2Error::SlowDown),
            StatusCode::NOT_FOUND => Err(ManagementApiV2Error::NotFound),
            StatusCode::UNAUTHORIZED => Err(ManagementApiV2Error::Unauthorized),
            unhandled => {
                log::debug!("unhandled status code {unhandled:#?}");
                Err(ManagementApiV2Error::UnhandledStatusCode(
                    unhandled.to_string(),
                ))
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
        let managemnt_api_v2_client = ManagementApiV2Client::new(&config).await.unwrap();
    }
}
