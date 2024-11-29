/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.15.13
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "node_type")]
pub enum UiNodeAttributes {
    #[serde(rename="input")]
    Input(Box<models::UiNodeInputAttributes>),
    #[serde(rename="text")]
    Text(Box<models::UiNodeTextAttributes>),
    #[serde(rename="img")]
    Img(Box<models::UiNodeImageAttributes>),
    #[serde(rename="a")]
    A(Box<models::UiNodeAnchorAttributes>),
    #[serde(rename="script")]
    Script(Box<models::UiNodeScriptAttributes>),
}

impl Default for UiNodeAttributes {
    fn default() -> Self {
        Self::Input(Default::default())
    }
}

/// The autocomplete attribute for the input. email InputAttributeAutocompleteEmail tel InputAttributeAutocompleteTel url InputAttributeAutocompleteUrl current-password InputAttributeAutocompleteCurrentPassword new-password InputAttributeAutocompleteNewPassword one-time-code InputAttributeAutocompleteOneTimeCode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AutocompleteEnum {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "tel")]
    Tel,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "current-password")]
    CurrentPassword,
    #[serde(rename = "new-password")]
    NewPassword,
    #[serde(rename = "one-time-code")]
    OneTimeCode,
}

impl Default for AutocompleteEnum {
    fn default() -> AutocompleteEnum {
        Self::Email
    }
}
/// OnClickTrigger may contain a WebAuthn trigger which should be executed on click.  The trigger maps to a JavaScript function provided by Ory, which triggers actions such as PassKey registration or login. oryWebAuthnRegistration WebAuthnTriggersWebAuthnRegistration oryWebAuthnLogin WebAuthnTriggersWebAuthnLogin oryPasskeyLogin WebAuthnTriggersPasskeyLogin oryPasskeyLoginAutocompleteInit WebAuthnTriggersPasskeyLoginAutocompleteInit oryPasskeyRegistration WebAuthnTriggersPasskeyRegistration oryPasskeySettingsRegistration WebAuthnTriggersPasskeySettingsRegistration
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnclickTriggerEnum {
    #[serde(rename = "oryWebAuthnRegistration")]
    OryWebAuthnRegistration,
    #[serde(rename = "oryWebAuthnLogin")]
    OryWebAuthnLogin,
    #[serde(rename = "oryPasskeyLogin")]
    OryPasskeyLogin,
    #[serde(rename = "oryPasskeyLoginAutocompleteInit")]
    OryPasskeyLoginAutocompleteInit,
    #[serde(rename = "oryPasskeyRegistration")]
    OryPasskeyRegistration,
    #[serde(rename = "oryPasskeySettingsRegistration")]
    OryPasskeySettingsRegistration,
}

impl Default for OnclickTriggerEnum {
    fn default() -> OnclickTriggerEnum {
        Self::OryWebAuthnRegistration
    }
}
/// OnLoadTrigger may contain a WebAuthn trigger which should be executed on load.  The trigger maps to a JavaScript function provided by Ory, which triggers actions such as PassKey registration or login. oryWebAuthnRegistration WebAuthnTriggersWebAuthnRegistration oryWebAuthnLogin WebAuthnTriggersWebAuthnLogin oryPasskeyLogin WebAuthnTriggersPasskeyLogin oryPasskeyLoginAutocompleteInit WebAuthnTriggersPasskeyLoginAutocompleteInit oryPasskeyRegistration WebAuthnTriggersPasskeyRegistration oryPasskeySettingsRegistration WebAuthnTriggersPasskeySettingsRegistration
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnloadTriggerEnum {
    #[serde(rename = "oryWebAuthnRegistration")]
    OryWebAuthnRegistration,
    #[serde(rename = "oryWebAuthnLogin")]
    OryWebAuthnLogin,
    #[serde(rename = "oryPasskeyLogin")]
    OryPasskeyLogin,
    #[serde(rename = "oryPasskeyLoginAutocompleteInit")]
    OryPasskeyLoginAutocompleteInit,
    #[serde(rename = "oryPasskeyRegistration")]
    OryPasskeyRegistration,
    #[serde(rename = "oryPasskeySettingsRegistration")]
    OryPasskeySettingsRegistration,
}

impl Default for OnloadTriggerEnum {
    fn default() -> OnloadTriggerEnum {
        Self::OryWebAuthnRegistration
    }
}

