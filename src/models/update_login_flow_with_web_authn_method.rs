/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.2.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// UpdateLoginFlowWithWebAuthnMethod : Update Login Flow with WebAuthn Method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateLoginFlowWithWebAuthnMethod {
    /// Sending the anti-csrf token is only required for browser login flows.
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Identifier is the email or username of the user trying to log in.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// Method should be set to \"webAuthn\" when logging in using the WebAuthn strategy.
    #[serde(rename = "method")]
    pub method: String,
    /// Login a WebAuthn Security Key  This must contain the ID of the WebAuthN connection.
    #[serde(rename = "webauthn_login", skip_serializing_if = "Option::is_none")]
    pub webauthn_login: Option<String>,
}


impl UpdateLoginFlowWithWebAuthnMethod {
    /// Update Login Flow with WebAuthn Method
    pub fn new(identifier: String, method: String) -> UpdateLoginFlowWithWebAuthnMethod {
        UpdateLoginFlowWithWebAuthnMethod {
                csrf_token: None,
                identifier,
                method,
                webauthn_login: None,
        }
    }
}


