/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.32
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// SelfServiceFlowExpiredError : Is sent when a flow is expired



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfServiceFlowExpiredError {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::GenericError>>,
    /// When the flow has expired
    #[serde(rename = "expired_at", skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    /// A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years.
    #[serde(rename = "since", skip_serializing_if = "Option::is_none")]
    pub since: Option<i64>,
    /// The flow ID that should be used for the new flow as it contains the correct messages.
    #[serde(rename = "use_flow_id", skip_serializing_if = "Option::is_none")]
    pub use_flow_id: Option<String>,
}

impl Default for SelfServiceFlowExpiredError {
    fn default() -> Self {
        Self::new()
    }
}

impl SelfServiceFlowExpiredError {
    /// Is sent when a flow is expired
    pub fn new() -> SelfServiceFlowExpiredError {
        SelfServiceFlowExpiredError {
                error: None,
                expired_at: None,
                since: None,
                use_flow_id: None,
        }
    }
}


