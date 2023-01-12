use crate::Error;
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
        self.client_secret.as_ref().map(|s| s.as_str())
    }
    pub fn get_response_type(&self) -> &'static str {
        if self.client_secret.is_some() {
            "code"
        } else {
            "token"
        }
    }
}
pub fn generate_state_key() -> String {
    use rand::distributions::{Alphanumeric,DistString};
    Alphanumeric.sample_string(&mut rand::thread_rng(),256)
}
#[cfg(feature="tokio")]
pub async fn generate_random_state_key_async() -> Result<String,Error> {
    tokio::task::spawn(async move {
       generate_state_key()
    }).await.map_err(|e| e.into())
}


#[derive(Clone)]
#[cfg_attr(feature="debug", derive(Debug))]
pub struct AuthState {
    /// where to send the user after the auth flow completes
    return_to: Option<String>,
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
    pub async fn get(&self,key:String) -> Option<AuthState> {
            let read_guard = self.0.read().await;
            if let Some(mutex) = read_guard.get(&key) {
                let mutex_guard = mutex.lock().await;
                Some((*mutex_guard).clone())
            }
            else {
                None
            }
        
    }
    pub async fn insert(&self,key:String,state:AuthState) {
        let mut write_guard = self.0.write().await;
        write_guard.insert(key,tokio::sync::Mutex::new(state));
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
    pub fn get(&self,key:String) -> Option<AuthState> {
        todo!()
    }
  
}
// an enum that represents the valid values of the 'authentication header'

pub enum AuthenticationHeader {
    Bearer(String),
}

pub struct Jwt(String);
