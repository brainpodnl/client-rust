/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.2.16
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// UpdateRegistrationFlowWithCodeMethod : Update Registration Flow with Code Method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateRegistrationFlowWithCodeMethod {
    /// The OTP Code sent to the user
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The CSRF Token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method to use  This field must be set to `code` when using the code method.
    #[serde(rename = "method")]
    pub method: String,
    /// Resend restarts the flow with a new code
    #[serde(rename = "resend", skip_serializing_if = "Option::is_none")]
    pub resend: Option<String>,
    /// The identity's traits
    #[serde(rename = "traits")]
    pub traits: serde_json::Value,
    /// Transient data to pass along to any webhooks
    #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
    pub transient_payload: Option<serde_json::Value>,
}


impl UpdateRegistrationFlowWithCodeMethod {
    /// Update Registration Flow with Code Method
    pub fn new(method: String, traits: serde_json::Value) -> UpdateRegistrationFlowWithCodeMethod {
        UpdateRegistrationFlowWithCodeMethod {
                code: None,
                csrf_token: None,
                method,
                resend: None,
                traits,
                transient_payload: None,
        }
    }
}


