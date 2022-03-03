/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.112
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitSelfServiceSettingsFlowWithLookupMethodBody {
    /// CSRFToken is the anti-CSRF token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// If set to true will save the regenerated lookup secrets
    #[serde(rename = "lookup_secret_confirm", skip_serializing_if = "Option::is_none")]
    pub lookup_secret_confirm: Option<bool>,
    /// Disables this method if true.
    #[serde(rename = "lookup_secret_disable", skip_serializing_if = "Option::is_none")]
    pub lookup_secret_disable: Option<bool>,
    /// If set to true will regenerate the lookup secrets
    #[serde(rename = "lookup_secret_regenerate", skip_serializing_if = "Option::is_none")]
    pub lookup_secret_regenerate: Option<bool>,
    /// If set to true will reveal the lookup secrets
    #[serde(rename = "lookup_secret_reveal", skip_serializing_if = "Option::is_none")]
    pub lookup_secret_reveal: Option<bool>,
    /// Method  Should be set to \"lookup\" when trying to add, update, or remove a lookup pairing.
    #[serde(rename = "method")]
    pub method: String,
}

impl SubmitSelfServiceSettingsFlowWithLookupMethodBody {
    pub fn new(method: String) -> SubmitSelfServiceSettingsFlowWithLookupMethodBody {
        SubmitSelfServiceSettingsFlowWithLookupMethodBody {
            csrf_token: None,
            lookup_secret_confirm: None,
            lookup_secret_disable: None,
            lookup_secret_regenerate: None,
            lookup_secret_reveal: None,
            method,
        }
    }
}


