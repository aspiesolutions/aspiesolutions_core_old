pub use aspiesolutions_auth0_client::*;
#[cfg_attr(feature="serde", derive(serde::Deserialize))]
#[derive(Clone,Debug)]
pub struct RegularWebAppConfig {
    /// the client id used for the application
    pub client_id: String,
    /// the client secret used for communicating privately
    pub client_secret: String,
    // the domain of the authorization tenant
    pub tenant_domain:String,
    /// the protocol used to communicate with the authorization tenant
    pub tenant_protocol:String,
    /// the domain of the application (used when performing redirects or building links)
    pub app_domain:String,
    /// an optional port to append to the domain. domain:port is treated differently than domain by itelf
    pub app_port:Option<String>,
    // the protocol to use when building application links
    pub app_protocol:Option<String>
}
#[cfg_attr(feature="serde", derive(serde::Deserialize))]
#[derive(Clone,Debug)]
pub enum Config {
    RegularWebApplication(RegularWebAppConfig)
}

