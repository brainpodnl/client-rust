/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.2.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// IdentityPatch : Payload for patching an identity



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityPatch {
    #[serde(rename = "create", skip_serializing_if = "Option::is_none")]
    pub create: Option<Box<crate::models::CreateIdentityBody>>,
    /// The ID of this patch.  The patch ID is optional. If specified, the ID will be returned in the response, so consumers of this API can correlate the response with the patch.
    #[serde(rename = "patch_id", skip_serializing_if = "Option::is_none")]
    pub patch_id: Option<String>,
}

impl Default for IdentityPatch {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityPatch {
    /// Payload for patching an identity
    pub fn new() -> IdentityPatch {
        IdentityPatch {
                create: None,
                patch_id: None,
        }
    }
}


