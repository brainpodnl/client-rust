/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.39-alpha.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuth2LogoutRequest {
    /// Challenge is the identifier (\"logout challenge\") of the logout authentication request. It is used to identify the session.
    #[serde(rename = "challenge", skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,
    #[serde(rename = "client", skip_serializing_if = "Option::is_none")]
    pub client: Option<Box<crate::models::OAuth2Client>>,
    /// RequestURL is the original Logout URL requested.
    #[serde(rename = "request_url", skip_serializing_if = "Option::is_none")]
    pub request_url: Option<String>,
    /// RPInitiated is set to true if the request was initiated by a Relying Party (RP), also known as an OAuth 2.0 Client.
    #[serde(rename = "rp_initiated", skip_serializing_if = "Option::is_none")]
    pub rp_initiated: Option<bool>,
    /// SessionID is the login session ID that was requested to log out.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// Subject is the user for whom the logout was request.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

impl Default for OAuth2LogoutRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl OAuth2LogoutRequest {
    pub fn new() -> OAuth2LogoutRequest {
        OAuth2LogoutRequest {
                challenge: None,
                client: None,
                request_url: None,
                rp_initiated: None,
                sid: None,
                subject: None,
        }
    }
}


