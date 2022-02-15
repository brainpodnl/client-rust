/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.94
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// SubmitSelfServiceLoginFlowWithOidcMethodBody : SubmitSelfServiceLoginFlowWithOidcMethodBody is used to decode the login form payload when using the oidc method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitSelfServiceLoginFlowWithOidcMethodBody {
    /// The CSRF Token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method to use  This field must be set to `oidc` when using the oidc method.
    #[serde(rename = "method")]
    pub method: String,
    /// The provider to register with
    #[serde(rename = "provider")]
    pub provider: String,
    /// The identity traits. This is a placeholder for the registration flow.
    #[serde(rename = "traits", skip_serializing_if = "Option::is_none")]
    pub traits: Option<serde_json::Value>,
}

impl SubmitSelfServiceLoginFlowWithOidcMethodBody {
    /// SubmitSelfServiceLoginFlowWithOidcMethodBody is used to decode the login form payload when using the oidc method.
    pub fn new(method: String, provider: String) -> SubmitSelfServiceLoginFlowWithOidcMethodBody {
        SubmitSelfServiceLoginFlowWithOidcMethodBody {
            csrf_token: None,
            method,
            provider,
            traits: None,
        }
    }
}


