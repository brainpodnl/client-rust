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
pub struct Project {
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the project.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "revision_id")]
    pub revision_id: String,
    #[serde(rename = "services")]
    pub services: Box<crate::models::ProjectServices>,
    /// The project's slug
    #[serde(rename = "slug")]
    pub slug: String,
    /// The state of the project. running Running halted Halted
    #[serde(rename = "state")]
    pub state: State,
}

impl Project {
    pub fn new(id: String, name: String, revision_id: String, services: crate::models::ProjectServices, slug: String, state: State) -> Project {
        Project {
            id,
            name,
            revision_id,
            services: Box::new(services),
            slug,
            state,
        }
    }
}

/// The state of the project. running Running halted Halted
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "halted")]
    Halted,
}

