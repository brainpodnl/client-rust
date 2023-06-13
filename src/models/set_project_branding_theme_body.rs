/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.35
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// SetProjectBrandingThemeBody : Set Project Branding Theme Request Parameters



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetProjectBrandingThemeBody {
    /// Logo type
    #[serde(rename = "logo_type", skip_serializing_if = "Option::is_none")]
    pub logo_type: Option<String>,
    /// Logo URL
    #[serde(rename = "logo_url", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Branding name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "theme", skip_serializing_if = "Option::is_none")]
    pub theme: Option<Box<crate::models::ProjectBrandingColors>>,
}

impl Default for SetProjectBrandingThemeBody {
    fn default() -> Self {
        Self::new()
    }
}

impl SetProjectBrandingThemeBody {
    /// Set Project Branding Theme Request Parameters
    pub fn new() -> SetProjectBrandingThemeBody {
        SetProjectBrandingThemeBody {
                logo_type: None,
                logo_url: None,
                name: None,
                theme: None,
        }
    }
}


