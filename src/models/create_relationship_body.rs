/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.1
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// CreateRelationshipBody : Create Relationship Request Body



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRelationshipBody {
    /// Namespace to query
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Object to query
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// Relation to query
    #[serde(rename = "relation", skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    /// SubjectID to query  Either SubjectSet or SubjectID can be provided.
    #[serde(rename = "subject_id", skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<String>,
    #[serde(rename = "subject_set", skip_serializing_if = "Option::is_none")]
    pub subject_set: Option<Box<crate::models::SubjectSet>>,
}

impl Default for CreateRelationshipBody {
    fn default() -> Self {
        Self::new()
    }
}

impl CreateRelationshipBody {
    /// Create Relationship Request Body
    pub fn new() -> CreateRelationshipBody {
        CreateRelationshipBody {
                namespace: None,
                object: None,
                relation: None,
                subject_id: None,
                subject_set: None,
        }
    }
}


