/*
 * Ory APIs
 *
 * # Introduction Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers.  ## SDKs This document describes the APIs available in the Ory Network. The APIs are available as SDKs for the following languages:  | Language       | Download SDK                                                     | Documentation                                                                        | | -------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------ | | Dart           | [pub.dev](https://pub.dev/packages/ory_client)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/dart/README.md)       | | .NET           | [nuget.org](https://www.nuget.org/packages/Ory.Client/)          | [README](https://github.com/ory/sdk/blob/master/clients/client/dotnet/README.md)     | | Elixir         | [hex.pm](https://hex.pm/packages/ory_client)                     | [README](https://github.com/ory/sdk/blob/master/clients/client/elixir/README.md)     | | Go             | [github.com](https://github.com/ory/client-go)                   | [README](https://github.com/ory/sdk/blob/master/clients/client/go/README.md)         | | Java           | [maven.org](https://search.maven.org/artifact/sh.ory/ory-client) | [README](https://github.com/ory/sdk/blob/master/clients/client/java/README.md)       | | JavaScript     | [npmjs.com](https://www.npmjs.com/package/@ory/client)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript/README.md) | | JavaScript (With fetch) | [npmjs.com](https://www.npmjs.com/package/@ory/client-fetch)           | [README](https://github.com/ory/sdk/blob/master/clients/client/typescript-fetch/README.md) |  | PHP            | [packagist.org](https://packagist.org/packages/ory/client)       | [README](https://github.com/ory/sdk/blob/master/clients/client/php/README.md)        | | Python         | [pypi.org](https://pypi.org/project/ory-client/)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/python/README.md)     | | Ruby           | [rubygems.org](https://rubygems.org/gems/ory-client)             | [README](https://github.com/ory/sdk/blob/master/clients/client/ruby/README.md)       | | Rust           | [crates.io](https://crates.io/crates/ory-client)                 | [README](https://github.com/ory/sdk/blob/master/clients/client/rust/README.md)       | 
 *
 * The version of the OpenAPI document: v1.15.12
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// OAuth2Client : OAuth 2.0 Clients are used to perform OAuth 2.0 and OpenID Connect flows. Usually, OAuth 2.0 clients are generated for applications which want to consume your OAuth 2.0 or OpenID Connect capabilities.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuth2Client {
    /// OAuth 2.0 Access Token Strategy  AccessTokenStrategy is the strategy used to generate access tokens. Valid options are `jwt` and `opaque`. `jwt` is a bad idea, see https://www.ory.sh/docs/hydra/advanced#json-web-tokens Setting the stragegy here overrides the global setting in `strategies.access_token`.
    #[serde(rename = "access_token_strategy", skip_serializing_if = "Option::is_none")]
    pub access_token_strategy: Option<String>,
    #[serde(rename = "allowed_cors_origins", skip_serializing_if = "Option::is_none")]
    pub allowed_cors_origins: Option<Vec<String>>,
    #[serde(rename = "audience", skip_serializing_if = "Option::is_none")]
    pub audience: Option<Vec<String>>,
    #[serde(rename = "authorization_code_grant_access_token_lifespan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub authorization_code_grant_access_token_lifespan: Option<Option<String>>,
    #[serde(rename = "authorization_code_grant_id_token_lifespan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub authorization_code_grant_id_token_lifespan: Option<Option<String>>,
    #[serde(rename = "authorization_code_grant_refresh_token_lifespan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub authorization_code_grant_refresh_token_lifespan: Option<Option<String>>,
    /// OpenID Connect Back-Channel Logout Session Required  Boolean value specifying whether the RP requires that a sid (session ID) Claim be included in the Logout Token to identify the RP session with the OP when the backchannel_logout_uri is used. If omitted, the default value is false.
    #[serde(rename = "backchannel_logout_session_required", skip_serializing_if = "Option::is_none")]
    pub backchannel_logout_session_required: Option<bool>,
    /// OpenID Connect Back-Channel Logout URI  RP URL that will cause the RP to log itself out when sent a Logout Token by the OP.
    #[serde(rename = "backchannel_logout_uri", skip_serializing_if = "Option::is_none")]
    pub backchannel_logout_uri: Option<String>,
    #[serde(rename = "client_credentials_grant_access_token_lifespan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub client_credentials_grant_access_token_lifespan: Option<Option<String>>,
    /// OAuth 2.0 Client ID  The ID is immutable. If no ID is provided, a UUID4 will be generated.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// OAuth 2.0 Client Name  The human-readable name of the client to be presented to the end-user during authorization.
    #[serde(rename = "client_name", skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    /// OAuth 2.0 Client Secret  The secret will be included in the create request as cleartext, and then never again. The secret is kept in hashed format and is not recoverable once lost.
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// OAuth 2.0 Client Secret Expires At  The field is currently not supported and its value is always 0.
    #[serde(rename = "client_secret_expires_at", skip_serializing_if = "Option::is_none")]
    pub client_secret_expires_at: Option<i64>,
    /// OAuth 2.0 Client URI  ClientURI is a URL string of a web page providing information about the client. If present, the server SHOULD display this URL to the end-user in a clickable fashion.
    #[serde(rename = "client_uri", skip_serializing_if = "Option::is_none")]
    pub client_uri: Option<String>,
    #[serde(rename = "contacts", skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<String>>,
    /// OAuth 2.0 Client Creation Date  CreatedAt returns the timestamp of the client's creation.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// OpenID Connect Front-Channel Logout Session Required  Boolean value specifying whether the RP requires that iss (issuer) and sid (session ID) query parameters be included to identify the RP session with the OP when the frontchannel_logout_uri is used. If omitted, the default value is false.
    #[serde(rename = "frontchannel_logout_session_required", skip_serializing_if = "Option::is_none")]
    pub frontchannel_logout_session_required: Option<bool>,
    /// OpenID Connect Front-Channel Logout URI  RP URL that will cause the RP to log itself out when rendered in an iframe by the OP. An iss (issuer) query parameter and a sid (session ID) query parameter MAY be included by the OP to enable the RP to validate the request and to determine which of the potentially multiple sessions is to be logged out; if either is included, both MUST be.
    #[serde(rename = "frontchannel_logout_uri", skip_serializing_if = "Option::is_none")]
    pub frontchannel_logout_uri: Option<String>,
    #[serde(rename = "grant_types", skip_serializing_if = "Option::is_none")]
    pub grant_types: Option<Vec<String>>,
    #[serde(rename = "implicit_grant_access_token_lifespan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implicit_grant_access_token_lifespan: Option<Option<String>>,
    #[serde(rename = "implicit_grant_id_token_lifespan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implicit_grant_id_token_lifespan: Option<Option<String>>,
    /// OAuth 2.0 Client JSON Web Key Set  Client's JSON Web Key Set [JWK] document, passed by value. The semantics of the jwks parameter are the same as the jwks_uri parameter, other than that the JWK Set is passed by value, rather than by reference. This parameter is intended only to be used by Clients that, for some reason, are unable to use the jwks_uri parameter, for instance, by native applications that might not have a location to host the contents of the JWK Set. If a Client can use jwks_uri, it MUST NOT use jwks. One significant downside of jwks is that it does not enable key rotation (which jwks_uri does, as described in Section 10 of OpenID Connect Core 1.0 [OpenID.Core]). The jwks_uri and jwks parameters MUST NOT be used together.
    #[serde(rename = "jwks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub jwks: Option<Option<serde_json::Value>>,
    /// OAuth 2.0 Client JSON Web Key Set URL  URL for the Client's JSON Web Key Set [JWK] document. If the Client signs requests to the Server, it contains the signing key(s) the Server uses to validate signatures from the Client. The JWK Set MAY also contain the Client's encryption keys(s), which are used by the Server to encrypt responses to the Client. When both signing and encryption keys are made available, a use (Key Use) parameter value is REQUIRED for all keys in the referenced JWK Set to indicate each key's intended usage. Although some algorithms allow the same key to be used for both signatures and encryption, doing so is NOT RECOMMENDED, as it is less secure. The JWK x5c parameter MAY be used to provide X.509 representations of keys provided. When used, the bare key values MUST still be present and MUST match those in the certificate.
    #[serde(rename = "jwks_uri", skip_serializing_if = "Option::is_none")]
    pub jwks_uri: Option<String>,
    #[serde(rename = "jwt_bearer_grant_access_token_lifespan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub jwt_bearer_grant_access_token_lifespan: Option<Option<String>>,
    /// OAuth 2.0 Client Logo URI  A URL string referencing the client's logo.
    #[serde(rename = "logo_uri", skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// OAuth 2.0 Client Owner  Owner is a string identifying the owner of the OAuth 2.0 Client.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// OAuth 2.0 Client Policy URI  PolicyURI is a URL string that points to a human-readable privacy policy document that describes how the deployment organization collects, uses, retains, and discloses personal data.
    #[serde(rename = "policy_uri", skip_serializing_if = "Option::is_none")]
    pub policy_uri: Option<String>,
    #[serde(rename = "post_logout_redirect_uris", skip_serializing_if = "Option::is_none")]
    pub post_logout_redirect_uris: Option<Vec<String>>,
    #[serde(rename = "redirect_uris", skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
    #[serde(rename = "refresh_token_grant_access_token_lifespan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub refresh_token_grant_access_token_lifespan: Option<Option<String>>,
    #[serde(rename = "refresh_token_grant_id_token_lifespan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub refresh_token_grant_id_token_lifespan: Option<Option<String>>,
    #[serde(rename = "refresh_token_grant_refresh_token_lifespan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub refresh_token_grant_refresh_token_lifespan: Option<Option<String>>,
    /// OpenID Connect Dynamic Client Registration Access Token  RegistrationAccessToken can be used to update, get, or delete the OAuth2 Client. It is sent when creating a client using Dynamic Client Registration.
    #[serde(rename = "registration_access_token", skip_serializing_if = "Option::is_none")]
    pub registration_access_token: Option<String>,
    /// OpenID Connect Dynamic Client Registration URL  RegistrationClientURI is the URL used to update, get, or delete the OAuth2 Client.
    #[serde(rename = "registration_client_uri", skip_serializing_if = "Option::is_none")]
    pub registration_client_uri: Option<String>,
    /// OpenID Connect Request Object Signing Algorithm  JWS [JWS] alg algorithm [JWA] that MUST be used for signing Request Objects sent to the OP. All Request Objects from this Client MUST be rejected, if not signed with this algorithm.
    #[serde(rename = "request_object_signing_alg", skip_serializing_if = "Option::is_none")]
    pub request_object_signing_alg: Option<String>,
    #[serde(rename = "request_uris", skip_serializing_if = "Option::is_none")]
    pub request_uris: Option<Vec<String>>,
    #[serde(rename = "response_types", skip_serializing_if = "Option::is_none")]
    pub response_types: Option<Vec<String>>,
    /// OAuth 2.0 Client Scope  Scope is a string containing a space-separated list of scope values (as described in Section 3.3 of OAuth 2.0 [RFC6749]) that the client can use when requesting access tokens.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// OpenID Connect Sector Identifier URI  URL using the https scheme to be used in calculating Pseudonymous Identifiers by the OP. The URL references a file with a single JSON array of redirect_uri values.
    #[serde(rename = "sector_identifier_uri", skip_serializing_if = "Option::is_none")]
    pub sector_identifier_uri: Option<String>,
    /// SkipConsent skips the consent screen for this client. This field can only be set from the admin API.
    #[serde(rename = "skip_consent", skip_serializing_if = "Option::is_none")]
    pub skip_consent: Option<bool>,
    /// SkipLogoutConsent skips the logout consent screen for this client. This field can only be set from the admin API.
    #[serde(rename = "skip_logout_consent", skip_serializing_if = "Option::is_none")]
    pub skip_logout_consent: Option<bool>,
    /// OpenID Connect Subject Type  The `subject_types_supported` Discovery parameter contains a list of the supported subject_type values for this server. Valid types include `pairwise` and `public`.
    #[serde(rename = "subject_type", skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<String>,
    /// OAuth 2.0 Token Endpoint Authentication Method  Requested Client Authentication method for the Token Endpoint. The options are:  `client_secret_basic`: (default) Send `client_id` and `client_secret` as `application/x-www-form-urlencoded` encoded in the HTTP Authorization header. `client_secret_post`: Send `client_id` and `client_secret` as `application/x-www-form-urlencoded` in the HTTP body. `private_key_jwt`: Use JSON Web Tokens to authenticate the client. `none`: Used for public clients (native apps, mobile apps) which can not have secrets.
    #[serde(rename = "token_endpoint_auth_method", skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_method: Option<String>,
    /// OAuth 2.0 Token Endpoint Signing Algorithm  Requested Client Authentication signing algorithm for the Token Endpoint.
    #[serde(rename = "token_endpoint_auth_signing_alg", skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_signing_alg: Option<String>,
    /// OAuth 2.0 Client Terms of Service URI  A URL string pointing to a human-readable terms of service document for the client that describes a contractual relationship between the end-user and the client that the end-user accepts when authorizing the client.
    #[serde(rename = "tos_uri", skip_serializing_if = "Option::is_none")]
    pub tos_uri: Option<String>,
    /// OAuth 2.0 Client Last Update Date  UpdatedAt returns the timestamp of the last update.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// OpenID Connect Request Userinfo Signed Response Algorithm  JWS alg algorithm [JWA] REQUIRED for signing UserInfo Responses. If this is specified, the response will be JWT [JWT] serialized, and signed using JWS. The default, if omitted, is for the UserInfo Response to return the Claims as a UTF-8 encoded JSON object using the application/json content-type.
    #[serde(rename = "userinfo_signed_response_alg", skip_serializing_if = "Option::is_none")]
    pub userinfo_signed_response_alg: Option<String>,
}

impl OAuth2Client {
    /// OAuth 2.0 Clients are used to perform OAuth 2.0 and OpenID Connect flows. Usually, OAuth 2.0 clients are generated for applications which want to consume your OAuth 2.0 or OpenID Connect capabilities.
    pub fn new() -> OAuth2Client {
        OAuth2Client {
            access_token_strategy: None,
            allowed_cors_origins: None,
            audience: None,
            authorization_code_grant_access_token_lifespan: None,
            authorization_code_grant_id_token_lifespan: None,
            authorization_code_grant_refresh_token_lifespan: None,
            backchannel_logout_session_required: None,
            backchannel_logout_uri: None,
            client_credentials_grant_access_token_lifespan: None,
            client_id: None,
            client_name: None,
            client_secret: None,
            client_secret_expires_at: None,
            client_uri: None,
            contacts: None,
            created_at: None,
            frontchannel_logout_session_required: None,
            frontchannel_logout_uri: None,
            grant_types: None,
            implicit_grant_access_token_lifespan: None,
            implicit_grant_id_token_lifespan: None,
            jwks: None,
            jwks_uri: None,
            jwt_bearer_grant_access_token_lifespan: None,
            logo_uri: None,
            metadata: None,
            owner: None,
            policy_uri: None,
            post_logout_redirect_uris: None,
            redirect_uris: None,
            refresh_token_grant_access_token_lifespan: None,
            refresh_token_grant_id_token_lifespan: None,
            refresh_token_grant_refresh_token_lifespan: None,
            registration_access_token: None,
            registration_client_uri: None,
            request_object_signing_alg: None,
            request_uris: None,
            response_types: None,
            scope: None,
            sector_identifier_uri: None,
            skip_consent: None,
            skip_logout_consent: None,
            subject_type: None,
            token_endpoint_auth_method: None,
            token_endpoint_auth_signing_alg: None,
            tos_uri: None,
            updated_at: None,
            userinfo_signed_response_alg: None,
        }
    }
}

