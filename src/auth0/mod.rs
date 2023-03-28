pub use aspiesolutions_auth0_client::*;
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct RegularWebAppConfig {
    /// the client id used for the application
    pub client_id: String,
    /// the client secret used for communicating privately
    pub client_secret: String,
    // the domain of the authorization tenant
    pub tenant_domain: String,

    /// the domain of the application (used when performing redirects or building links)
    pub app_domain: String,
    /// an optional port to append to the domain. domain:port is treated differently than domain by itelf. appends a port when not None
    pub app_port: Option<String>,
    // the protocol to use when building application links. assumes http:// when not specified
    pub app_protocol: Option<String>,
    // the api audience to use when getting api tokens
    pub api_audience: String,
}
pub const DEFAULT_APP_PROTOCOL: &str = "http://";
pub const TENANT_PROTOCOL: &str = "https://";
pub const AUTHORIZE_ENDPOINT: &str = "/authorize";



pub fn build_authorize_redirect_uri_for_regular_web_apps(
    client_id:&str,
    tenant_domain:&str,
    scope: &str,
    state: &str,
    redirect_uri: &str,
    extra_query_params: Option<&str>,
    connection:Option<&str>
) -> String {
    let connection = connection.map(|connection| "&connection=".to_string() + connection).unwrap_or_else(||String::new());
    let extra_query_params = extra_query_params.map(|extra_query_params| "&".to_string() + extra_query_params).unwrap_or_else(||String::new());
    format!("{0}{1}{2}?response_type=code&client_id={3}&scope={scope}&redirect_uri={redirect_uri}&state={state}{connection}{extra_query_params}",TENANT_PROTOCOL,tenant_domain,AUTHORIZE_ENDPOINT,client_id)
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum Config {
    RegularWebApplication(RegularWebAppConfig),
}
