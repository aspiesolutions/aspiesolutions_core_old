#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JwtClaims {}

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


#[cfg(feature="tokio")]
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
            Some((*mutex_guard).clone())
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
            Some((*mutex.lock().await).clone())
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
    pub fn get(&self, key: String) -> Option<AuthState> {
        todo!()
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
pub struct Client {
    authorization_tenant_domain: String,
    client_id: String,
    client_secret: Option<String>,
    #[cfg(feature = "reqwest")]
    client: reqwest::Client,
}
impl Client {
    pub fn new(
        authorization_tenant_domain: &str,
        client_id: &str,
        client_secret: Option<&str>,
    ) -> Self {
        Self {
            authorization_tenant_domain: authorization_tenant_domain.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.map(|s| s.to_string()),
            #[cfg(feature = "reqwest")]
            client: reqwest::Client::new(),
        }
    }
}

#[cfg(all(feature = "reqwest", feature = "serde"))]
impl Client {
    pub async fn exchange_authorization_code_for_token(
        &self,
        code: &str,
        redirect_uri: Option<&str>,
        // audience: Option<&str>,
    ) -> Result<AuthorizationCodeFlowTokenExchangeResponse, crate::Error> {
        if self.client_secret.is_none() {
            return Err(crate::Error::ClientSecretNotConfigured);
        }
        let client_secret = self.client_secret.clone().unwrap();
        let params: AuthorizationCodeFlowTokenExchangeParameters =
            AuthorizationCodeFlowTokenExchangeParameters {
                client_id: self.client_id.clone(),
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
                self.authorization_tenant_domain
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
}
