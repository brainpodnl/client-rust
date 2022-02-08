/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.76
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CnameSettings {
    #[serde(rename = "cookie_domain", skip_serializing_if = "Option::is_none")]
    pub cookie_domain: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "verification_errors", skip_serializing_if = "Option::is_none")]
    pub verification_errors: Option<Vec<String>>,
    /// CustomHostnameStatus is the enumeration of valid state values in the CustomHostnameSSL
    #[serde(rename = "verification_status", skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
}

impl CnameSettings {
    pub fn new() -> CnameSettings {
        CnameSettings {
            cookie_domain: None,
            created_at: None,
            hostname: None,
            id: None,
            updated_at: None,
            verification_errors: None,
            verification_status: None,
        }
    }
}


