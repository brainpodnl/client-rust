/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.32
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetProject {
    /// The name of the project.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "services")]
    pub services: Box<crate::models::ProjectServices>,
}


impl SetProject {
    pub fn new(name: String, services: crate::models::ProjectServices) -> SetProject {
        SetProject {
                name,
                services: Box::new(services),
        }
    }
}


