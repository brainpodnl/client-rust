/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.93
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RevokedSessions {
    /// The number of sessions that were revoked.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl RevokedSessions {
    pub fn new() -> RevokedSessions {
        RevokedSessions {
            count: None,
        }
    }
}


