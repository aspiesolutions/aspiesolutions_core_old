// an instance of a client used to communicate with the auth0 management api
// management clients should be accessed currently using client id and client secret


/// The default api endpoint prefix to append
const API_ENDPOINT_PREFIX:&str="/api/v2/";
/// the protocol to use when communicating (https is required)
const PROTOCOL:&str = "https://";
const BEARER_PREFIX: &str = "Bearer ";



pub struct ManagementApiV2Config {
    client_id:String,
    client_secret:String,
    // the tenant domain to use when using back-channel communication.
    // make sure to use the domain provided by auth0 instead of custom domains
    // because of how audience values are formatted

}



pub struct ManagementApiV2Client{
    #[cfg(feature="reqwest")]
    client:reqwest::Client
}