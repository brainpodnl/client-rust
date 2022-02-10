/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.83
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvitePayload {
    #[serde(rename = "invitee_email", skip_serializing_if = "Option::is_none")]
    pub invitee_email: Option<String>,
}

impl InvitePayload {
    pub fn new() -> InvitePayload {
        InvitePayload {
            invitee_email: None,
        }
    }
}


