#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JwtClaims {}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[allow(non_snake_case)]
pub enum SigningAlgorythm {
    RS256,
    HS256,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[derive(Debug)]
pub struct Auth0Config {
    /// the domain where your Auth0 Instance is located
    authorization_tenant_domain: String,
    client_id: String,
    client_secret: Option<String>,
    /// the default api identifier used when requesting access tokens
    api_audience: String,
    management_api_audience: Option<String>,
}
impl Auth0Config {
    pub fn authorization_tenant_domain(&self) -> &str {
        &self.authorization_tenant_domain
    }
    pub fn client_id(&self) -> &str {
        &self.client_id
    }
    pub fn client_secret(&self) -> Option<&str> {
        self.client_secret.as_ref().map(|s| s.as_str())
    }
    pub fn api_audience(&self) -> &str {
        &self.api_audience
    }
    pub fn get_response_type(&self) -> &'static str {
        if self.client_secret.is_some() {
            "code"
        } else {
            "token"
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct AuthState {
    /// where to send the user after the auth flow completes
    return_to: Option<String>,
    // the callback url that was sent in the original request
    redirect_uri: Option<String>,
    grant_type: Option<String>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            return_to: None,
            grant_type: None,
            redirect_uri: None,
        }
    }
    pub fn set_return_to(&mut self, return_to: Option<&str>) {
        self.return_to = return_to.map(|s| s.to_string())
    }
    pub fn return_to(&self) -> Option<&str> {
        self.return_to.as_ref().map(|s| s.as_str())
    }
    pub fn set_grant_type(&mut self, grant_type: Option<&str>) {
        self.grant_type = grant_type.map(|s| s.to_string())
    }
    pub fn grant_type(&self) -> Option<&str> {
        self.grant_type.as_ref().map(|s| s.as_str())
    }
    pub fn set_redirect_uri(&mut self, redirect_uri: Option<&str>) {
        self.redirect_uri = redirect_uri.map(|s| s.to_string())
    }
    pub fn redirect_uri(&self) -> Option<&str> {
        self.redirect_uri.as_ref().map(|s| s.as_str())
    }
    pub fn generate_state_key() -> String {
        use rand::distributions::{Alphanumeric, DistString};
        Alphanumeric.sample_string(&mut rand::thread_rng(), 128)
    }
}

/// A guard that represents an access token in a cookie
pub struct AccessTokenCookieGuard(String);
/// A guard that represents an access token in the authentication header
pub struct AccessTokenAuthenticationHeaderGuard(String);
/// a guard that represents all valid methods of authentication for this provider
pub enum AccessTokenGuard {
    AuthenticationHeader(AccessTokenAuthenticationHeaderGuard),
    Cookie(AccessTokenCookieGuard),
}
#[cfg(feature = "tokio")]
impl AuthState {
    pub async fn generate_state_key_async() -> Result<String, crate::Error> {
        tokio::task::spawn(async move { Self::generate_state_key() })
            .await
            .map_err(|e| e.into())
    }
}

#[cfg(feature = "tokio")]
pub struct AuthStateHashMap(
    tokio::sync::RwLock<std::collections::HashMap<String, tokio::sync::Mutex<AuthState>>>,
);

#[cfg(feature = "tokio")]
impl AuthStateHashMap {
    pub fn new() -> Self {
        Self(tokio::sync::RwLock::new(std::collections::HashMap::<
            String,
            tokio::sync::Mutex<AuthState>,
        >::new()))
    }
    pub async fn get(&self, key: &str) -> Option<AuthState> {
        let read_guard = self.0.read().await;
        if let Some(mutex) = read_guard.get(key) {
            let mutex_guard = mutex.lock().await;
            Some((*mutex_guard).to_owned())
        } else {
            None
        }
    }
    pub async fn insert(&self, key: &str, state: AuthState) {
        let mut write_guard = self.0.write().await;
        write_guard.insert(key.to_string(), tokio::sync::Mutex::new(state));
    }
    pub async fn remove(&self, key: &str) -> Option<AuthState> {
        let mut write_guard = self.0.write().await;
        if let Some(mutex) = write_guard.remove(key) {
            Some((*mutex.lock().await).to_owned())
        } else {
            None
        }
    }
}
#[cfg(not(feature = "tokio"))]
pub struct AuthStateHashMap(
    std::sync::RwLock<std::collections::HashMap<String, std::sync::Mutex<AuthState>>>,
);

#[cfg(not(feature = "tokio"))]
impl AuthStateHashMap {
    pub fn new() -> Self {
        Self(std::sync::RwLock::new(std::collections::HashMap::<
            String,
            std::sync::Mutex<AuthState>,
        >::new()))
    }
    pub fn get(&self, key: &str) -> Result<Option<AuthState>, crate::Error> {
        let read_guard = self.0.read()?;
        if let Some(mutex_guard) = read_guard.get(key) {
            Ok(Some((*mutex_guard.lock()?).to_owned()))
        } else {
            Ok(None)
        }
    }
}
// an enum that represents the valid values of the 'authentication header'

pub enum AuthenticationHeader {
    Bearer(String),
}

pub struct Jwt(String);

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct AuthorizationCodeFlowTokenExchangeParameters {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    // pub audience:String,
    pub redirect_uri: Option<String>,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct AuthorizationCodeFlowTokenExchangeResponse {
    access_token: String,
    token_type: String,
    expires_in: usize,
}
impl AuthorizationCodeFlowTokenExchangeResponse {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
    pub fn token_type(&self) -> &str {
        &self.token_type
    }
    pub fn expires_in(&self) -> usize {
        self.expires_in
    }
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Auth0ErrorResponse {
    error: String,
    error_description: String,
}

impl Auth0ErrorResponse {
    pub fn error(&self) -> &str {
        &self.error
    }
    pub fn error_description(&self) -> &str {
        &self.error_description
    }
}

#[cfg_attr(feature = "clone", derive(Clone))]
pub struct Client {
    config: Auth0Config,
    #[cfg(feature = "reqwest")]
    client: reqwest::Client,
}
impl Client {
    pub fn new(config: Auth0Config) -> Self {
        Self {
            config,
            #[cfg(feature = "reqwest")]
            client: reqwest::Client::new(),
        }
    }
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClientCredentialsTokenExhcangeParameters {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub audience: String,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Auth0ManagementTokenResponse {
    access_token: String,
    expires_in: usize,
    scope: String,
    token_type: String,
}

#[cfg(all(feature = "reqwest", feature = "serde"))]
impl Client {
    pub async fn exchange_authorization_code_for_token(
        &self,
        code: &str,
        redirect_uri: Option<&str>,
        // audience: Option<&str>,
    ) -> Result<AuthorizationCodeFlowTokenExchangeResponse, crate::Error> {
        if self.config.client_secret.is_none() {
            return Err(crate::Error::ClientSecretNotConfigured);
        }
        let client_secret = self.config.client_secret.clone().unwrap();
        let params: AuthorizationCodeFlowTokenExchangeParameters =
            AuthorizationCodeFlowTokenExchangeParameters {
                client_id: self.config.client_id.clone(),
                client_secret,
                code: code.to_string(),
                grant_type: "authorization_code".to_string(),
                redirect_uri: redirect_uri.map(|s| s.to_string()),
                // audience: audience.map(|s| s.to_string())
            };
        let response = self
            .client
            .post(format!(
                "https://{}/oauth/token",
                self.config.authorization_tenant_domain
            ))
            .json(&params)
            .send()
            .await?;

        let _status = response.status();
        // let _headers = response.headers();
        match _status {
            reqwest::StatusCode::OK => response
                .json::<AuthorizationCodeFlowTokenExchangeResponse>()
                .await
                .map_err(|e| e.into()),
            reqwest::StatusCode::BAD_REQUEST => {
                println!("{}", response.text().await?);
                todo!("bad request")
            }
            not_yet_implemented => {
                println!("not yet implemented {not_yet_implemented}");
                todo!("{}", not_yet_implemented)
            }
        }
    }
    pub async fn get_jwks_from_oidc(&self) -> Result<jsonwebtoken::jwk::JwkSet, crate::Error> {
        let response = self
            .client
            .get(format!(
                "https://{}/.well-known/jwks.json",
                self.config.authorization_tenant_domain
            ))
            .send()
            .await?;
        let jwks = response.json::<jsonwebtoken::jwk::JwkSet>().await?;
        Ok(jwks)
    }
    pub async fn get_management_token(
        &self,
        audience: Option<&str>,
    ) -> Result<Auth0ManagementTokenResponse, crate::Error> {
        log::debug!("getting management token");
        if self.config.client_secret().is_none() {
            return Err(crate::Error::ClientSecretNotConfigured);
        }
        let client_secret = self.config.client_secret().unwrap().to_string();
        let params = ClientCredentialsTokenExhcangeParameters {
            grant_type: "client_credentials".to_string(),
            client_id: self.config.client_id().to_string(),
            client_secret,
            audience: format!(
                "https://{}/api/v2/",
                self.config.authorization_tenant_domain
            ),
        };
        // use the configured domain unless an alternate was specified in this function
        let response = self
            .client
            .post(audience.map(|s| s.to_string()).map_or_else(|| {
                format!(
                    "https://{}/oauth/token",
                    self.config.authorization_tenant_domain
                )
            }))
            .form(&params)
            .send()
            .await?;

        let status = response.status();
        log::debug!("status {status:?}");
        let _headers = response.headers();
        match status {
            reqwest::StatusCode::OK => {
                let management_token_response: Auth0ManagementTokenResponse =
                    response.json().await?;
                Ok(Auth0ManagementTokenResponse)
            }

            reqwest::StatusCode::UNAUTHORIZED => {
                // parse the body text
                let error_response: Auth0ErrorResponse = response.json().await?;
                log::debug!(
                    "status: unauthorzed. error: \n{0}, error message: {1}",
                    error_response.error(),
                    error_response.error_description()
                );
                Err(crate::Error::Unauthorized(
                    error_response.error_description().to_string(),
                ))
            }
            reqwest::StatusCode::FORBIDDEN => {
                let error_response: Auth0ErrorResponse = response.json().await?;
                log::debug!(
                    "status: forbidden. error: '{0}' error_message: '{1}' ",
                    error_response.error(),
                    error_response.error_description()
                );
                Err(crate::error::Error::Forbidden(
                    error_response.error_description().to_string(),
                ))
            }
            not_yet_implemented => {
                log::debug!("{not_yet_implemented}");
                todo!()
            }
        }
    }
    pub async fn verify_access_token(&self) -> Result<(), crate::Error> {
        // find the decoding key
        let _jwks = self.get_jwks_from_oidc().await?;
        // get the signing key based on the active kid

        // jwks.find(self.kid)
        Ok(())
    }
}

#[cfg(all(test, feature = "serde", feature = "reqwest"))]
pub mod test_client {
    pub fn test_set_up() -> Result<super::Client, crate::error::Error> {
        use figment::{providers::Env, Figment};
        let _ = dotenv::dotenv().ok();
        let _ = simple_logger::init_with_env().ok();
        let config: super::Auth0Config = Figment::new().merge(Env::prefixed("AUTH0_")).extract()?;
        let client = super::Client::new(config);
        Ok(client)
    }
    #[tokio::test]
    pub async fn auth0_client_test_get_managment_token() {
        let client = test_set_up().unwrap();

        let _ = client.get_management_token().await.unwrap();
    }
}
