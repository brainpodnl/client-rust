/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.25
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// UpdateSettingsFlowWithProfileMethod : Update Settings Flow with Profile Method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSettingsFlowWithProfileMethod {
    /// The Anti-CSRF Token  This token is only required when performing browser flows.
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method  Should be set to profile when trying to update a profile.
    #[serde(rename = "method")]
    pub method: String,
    /// Traits  The identity's traits.
    #[serde(rename = "traits")]
    pub traits: serde_json::Value,
}


impl UpdateSettingsFlowWithProfileMethod {
    /// Update Settings Flow with Profile Method
    pub fn new(method: String, traits: serde_json::Value) -> UpdateSettingsFlowWithProfileMethod {
        UpdateSettingsFlowWithProfileMethod {
                csrf_token: None,
                method,
                traits,
        }
    }
}


