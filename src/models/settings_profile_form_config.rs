/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.148
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettingsProfileFormConfig {
    /// Action should be used as the form action URL `<form action=\"{{ .Action }}\" method=\"post\">`.
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<crate::models::UiText>>,
    /// Method is the form method (e.g. POST)
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "nodes")]
    pub nodes: Vec<crate::models::UiNode>,
}

impl SettingsProfileFormConfig {
    pub fn new(action: String, method: String, nodes: Vec<crate::models::UiNode>) -> SettingsProfileFormConfig {
        SettingsProfileFormConfig {
            action,
            messages: None,
            method,
            nodes,
        }
    }
}


