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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaUsage {
    /// The additional price per unit in cents.
    #[serde(rename = "additional_price")]
    pub additional_price: String,
    #[serde(rename = "can_use_more")]
    pub can_use_more: bool,
    ///  production_projects ProductionProjects staging_projects StagingProjects development_projects DevelopmentProjects daily_active_users DailyActiveUsers custom_domains CustomDomains event_streams EventStreams event_stream_events EventStreamEvents sla SLA collaborator_seats CollaboratorSeats edge_cache EdgeCache branding_themes BrandingThemes zendesk_support ZendeskSupport project_metrics ProjectMetrics project_metrics_time_window ProjectMetricsTimeWindow project_metrics_events_history ProjectMetricsEventsHistory organizations Organizations rop_grant ResourceOwnerPasswordGrant concierge_onboarding ConciergeOnboarding credit Credit data_location_global DataLocationGlobal data_location_us DataLocationUS data_location_asiane DataLocationAsiaNorthEast m2m_token_issuance M2MTokenIssuance permission_checks PermissionChecks captcha Captcha data_location_regional DataLocationRegional  Required Features rate_limit_tier RateLimitTier session_rate_limit_tier RateLimitTierSessions identities_list_rate_limit_tier RateLimitTierIdentitiesList permission_checks_rate_limit_tier RateLimitTierPermissionChecks oauth2_introspect_rate_limit_tier RateLimitTierOAuth2Introspect
    #[serde(rename = "feature")]
    pub feature: FeatureEnum,
    #[serde(rename = "feature_available")]
    pub feature_available: bool,
    #[serde(rename = "included")]
    pub included: i64,
    #[serde(rename = "is_unlimited")]
    pub is_unlimited: bool,
    #[serde(rename = "used")]
    pub used: i64,
}

impl QuotaUsage {
    pub fn new(additional_price: String, can_use_more: bool, feature: FeatureEnum, feature_available: bool, included: i64, is_unlimited: bool, used: i64) -> QuotaUsage {
        QuotaUsage {
            additional_price,
            can_use_more,
            feature,
            feature_available,
            included,
            is_unlimited,
            used,
        }
    }
}
///  production_projects ProductionProjects staging_projects StagingProjects development_projects DevelopmentProjects daily_active_users DailyActiveUsers custom_domains CustomDomains event_streams EventStreams event_stream_events EventStreamEvents sla SLA collaborator_seats CollaboratorSeats edge_cache EdgeCache branding_themes BrandingThemes zendesk_support ZendeskSupport project_metrics ProjectMetrics project_metrics_time_window ProjectMetricsTimeWindow project_metrics_events_history ProjectMetricsEventsHistory organizations Organizations rop_grant ResourceOwnerPasswordGrant concierge_onboarding ConciergeOnboarding credit Credit data_location_global DataLocationGlobal data_location_us DataLocationUS data_location_asiane DataLocationAsiaNorthEast m2m_token_issuance M2MTokenIssuance permission_checks PermissionChecks captcha Captcha data_location_regional DataLocationRegional  Required Features rate_limit_tier RateLimitTier session_rate_limit_tier RateLimitTierSessions identities_list_rate_limit_tier RateLimitTierIdentitiesList permission_checks_rate_limit_tier RateLimitTierPermissionChecks oauth2_introspect_rate_limit_tier RateLimitTierOAuth2Introspect
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeatureEnum {
    #[serde(rename = "production_projects")]
    ProductionProjects,
    #[serde(rename = "staging_projects")]
    StagingProjects,
    #[serde(rename = "development_projects")]
    DevelopmentProjects,
    #[serde(rename = "daily_active_users")]
    DailyActiveUsers,
    #[serde(rename = "custom_domains")]
    CustomDomains,
    #[serde(rename = "event_streams")]
    EventStreams,
    #[serde(rename = "event_stream_events")]
    EventStreamEvents,
    #[serde(rename = "sla")]
    Sla,
    #[serde(rename = "collaborator_seats")]
    CollaboratorSeats,
    #[serde(rename = "edge_cache")]
    EdgeCache,
    #[serde(rename = "branding_themes")]
    BrandingThemes,
    #[serde(rename = "zendesk_support")]
    ZendeskSupport,
    #[serde(rename = "project_metrics")]
    ProjectMetrics,
    #[serde(rename = "project_metrics_time_window")]
    ProjectMetricsTimeWindow,
    #[serde(rename = "project_metrics_events_history")]
    ProjectMetricsEventsHistory,
    #[serde(rename = "organizations")]
    Organizations,
    #[serde(rename = "rop_grant")]
    RopGrant,
    #[serde(rename = "concierge_onboarding")]
    ConciergeOnboarding,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "data_location_global")]
    DataLocationGlobal,
    #[serde(rename = "data_location_us")]
    DataLocationUs,
    #[serde(rename = "data_location_asiane")]
    DataLocationAsiane,
    #[serde(rename = "m2m_token_issuance")]
    M2mTokenIssuance,
    #[serde(rename = "permission_checks")]
    PermissionChecks,
    #[serde(rename = "captcha")]
    Captcha,
    #[serde(rename = "data_location_regional")]
    DataLocationRegional,
    #[serde(rename = "rate_limit_tier")]
    RateLimitTier,
    #[serde(rename = "session_rate_limit_tier")]
    SessionRateLimitTier,
    #[serde(rename = "identities_list_rate_limit_tier")]
    IdentitiesListRateLimitTier,
    #[serde(rename = "permission_checks_rate_limit_tier")]
    PermissionChecksRateLimitTier,
    #[serde(rename = "oauth2_introspect_rate_limit_tier")]
    Oauth2IntrospectRateLimitTier,
}

impl Default for FeatureEnum {
    fn default() -> FeatureEnum {
        Self::ProductionProjects
    }
}

