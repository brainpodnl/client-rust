/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.1.0-alpha.3
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalRelationTuple {
    /// Namespace of the Relation Tuple
    #[serde(rename = "namespace")]
    pub namespace: String,
    /// Object of the Relation Tuple
    #[serde(rename = "object")]
    pub object: String,
    /// Relation of the Relation Tuple
    #[serde(rename = "relation")]
    pub relation: String,
    /// SubjectID of the Relation Tuple  Either SubjectSet or SubjectID are required.
    #[serde(rename = "subject_id", skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<String>,
    #[serde(rename = "subject_set", skip_serializing_if = "Option::is_none")]
    pub subject_set: Option<Box<crate::models::SubjectSet>>,
}

impl InternalRelationTuple {
    pub fn new(namespace: String, object: String, relation: String) -> InternalRelationTuple {
        InternalRelationTuple {
            namespace,
            object,
            relation,
            subject_id: None,
            subject_set: None,
        }
    }
}


