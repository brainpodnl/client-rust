/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.189
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityCredentialsOidc {
    #[serde(rename = "providers", skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<crate::models::IdentityCredentialsOidcProvider>>,
}

impl IdentityCredentialsOidc {
    pub fn new() -> IdentityCredentialsOidc {
        IdentityCredentialsOidc {
            providers: None,
        }
    }
}


