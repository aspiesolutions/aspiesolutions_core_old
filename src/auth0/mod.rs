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
}
impl Auth0Config {
    pub fn authorization_tenant_domain(&self) -> &str {
        &self.authorization_tenant_domain
    }
    pub fn client_id(&self) -> &str {
        &self.client_id
    }
    pub fn client_secret(&self) -> Option<&str> {
        self.client_secret()
    }
    pub fn get_response_type(&self) -> &'static str {
        if self.client_secret.is_some() {
            "code"
        } else {
            "token"
        }
    }
}

pub struct AuthState {
    state: String,
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
}
// an enum that represents the valid values of the 'authentication header'

pub enum AuthenticationHeader {
    Bearer(String),
}

pub struct Jwt(String);
