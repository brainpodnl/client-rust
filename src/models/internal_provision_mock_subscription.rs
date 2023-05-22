/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.32
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// InternalProvisionMockSubscription : Internal Provision Mock Subscription Request Body



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalProvisionMockSubscription {
    /// Identity ID
    #[serde(rename = "identity_id")]
    pub identity_id: String,
    /// Billing Interval monthly Monthly yearly Yearly
    #[serde(rename = "interval")]
    pub interval: IntervalEnum,
    /// Plan ID
    #[serde(rename = "plan")]
    pub plan: String,
}


impl InternalProvisionMockSubscription {
    /// Internal Provision Mock Subscription Request Body
    pub fn new(identity_id: String, interval: IntervalEnum, plan: String) -> InternalProvisionMockSubscription {
        InternalProvisionMockSubscription {
                identity_id,
                interval,
                plan,
        }
    }
}

/// Billing Interval monthly Monthly yearly Yearly
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IntervalEnum {
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "yearly")]
    Yearly,
}

