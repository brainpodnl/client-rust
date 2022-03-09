/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.120
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "nodetype")]
pub enum UiNodeAttributes {
    #[serde(rename="anchor")]
    UiNodeAnchorAttributes {
        /// The link's href (destination) URL.  format: uri
        #[serde(rename = "href")]
        href: String,
        /// A unique identifier
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "title")]
        title: Box<crate::models::UiText>,
    },
    #[serde(rename="image")]
    UiNodeImageAttributes {
        /// Height of the image
        #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
        height: Option<i64>,
        /// A unique identifier
        #[serde(rename = "id")]
        id: String,
        /// The image's source URL.  format: uri
        #[serde(rename = "src")]
        src: String,
        /// Width of the image
        #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
        width: Option<i64>,
    },
    #[serde(rename="input")]
    UiNodeInputAttributes {
        /// Sets the input's disabled field to true or false.
        #[serde(rename = "disabled")]
        disabled: bool,
        #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
        label: Option<Box<crate::models::UiText>>,
        /// The input's element name.
        #[serde(rename = "name")]
        name: String,
        /// OnClick may contain javascript which should be executed on click. This is primarily used for WebAuthn.
        #[serde(rename = "onclick", skip_serializing_if = "Option::is_none")]
        onclick: Option<String>,
        /// The input's pattern.
        #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
        pattern: Option<String>,
        /// Mark this input field as required.
        #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
        required: Option<bool>,
        #[serde(rename = "type")]
        _type: String,
        /// The input's value.
        #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
        value: Option<serde_json::Value>,
    },
    #[serde(rename="script")]
    UiNodeScriptAttributes {
        /// The script async type
        #[serde(rename = "async")]
        _async: bool,
        /// The script cross origin policy
        #[serde(rename = "crossorigin")]
        crossorigin: String,
        /// A unique identifier
        #[serde(rename = "id")]
        id: String,
        /// The script's integrity hash
        #[serde(rename = "integrity")]
        integrity: String,
        /// Nonce for CSP  A nonce you may want to use to improve your Content Security Policy. You do not have to use this value but if you want to improve your CSP policies you may use it. You can also choose to use your own nonce value!
        #[serde(rename = "nonce")]
        nonce: String,
        /// The script referrer policy
        #[serde(rename = "referrerpolicy")]
        referrerpolicy: String,
        /// The script source
        #[serde(rename = "src")]
        src: String,
        /// The script MIME type
        #[serde(rename = "type")]
        _type: String,
    },
    #[serde(rename="text")]
    UiNodeTextAttributes {
        /// A unique identifier
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "text")]
        text: Box<crate::models::UiText>,
    },
}




