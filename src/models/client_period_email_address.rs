/*
 * Clerk Frontend API
 *
 * The Clerk REST Frontend API, meant to be accessed from a browser or native environment.  This is a Form Based API and all the data must be sent and formatted according to the `application/x-www-form-urlencoded` content type.  ### Versions  When the API changes in a way that isn't compatible with older versions, a new version is released. Each version is identified by its release date, e.g. `2021-02-05`. For more information, please see [Clerk API Versions](https://clerk.com/docs/backend-requests/versioning/overview).  ### Using the Try It Console  The `Try It` feature of the docs only works for **Development Instances** when using the `DevBrowser` security scheme. To use it, first generate a dev instance token from the `/v1/dev_browser` endpoint.  Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientPeriodEmailAddress {
    #[serde(rename = "id")]
    pub id: String,
    /// String representing the object's type. Objects of the same type share the same value.
    #[serde(rename = "object")]
    pub object: Object,
    #[serde(rename = "email_address")]
    pub email_address: String,
    #[serde(rename = "reserved")]
    pub reserved: bool,
    #[serde(rename = "verification", deserialize_with = "Option::deserialize")]
    pub verification: Option<Box<models::ClientEmailAddressVerification>>,
    #[serde(rename = "linked_to")]
    pub linked_to: Vec<models::StubsPeriodIdentificationPeriodLink>,
    /// Indicates whether this email address domain matches an active enterprise connection.
    #[serde(
        rename = "matches_sso_connection",
        skip_serializing_if = "Option::is_none"
    )]
    pub matches_sso_connection: Option<bool>,
    /// Unix timestamp of creation
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// Unix timestamp of creation
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}

impl ClientPeriodEmailAddress {
    pub fn new(
        id: String,
        object: Object,
        email_address: String,
        reserved: bool,
        verification: Option<models::ClientEmailAddressVerification>,
        linked_to: Vec<models::StubsPeriodIdentificationPeriodLink>,
        created_at: i64,
        updated_at: i64,
    ) -> ClientPeriodEmailAddress {
        ClientPeriodEmailAddress {
            id,
            object,
            email_address,
            reserved,
            verification: verification.map(Box::new),
            linked_to,
            matches_sso_connection: None,
            created_at,
            updated_at,
        }
    }
}
/// String representing the object's type. Objects of the same type share the same value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "email_address")]
    EmailAddress,
}

impl Default for Object {
    fn default() -> Object {
        Self::EmailAddress
    }
}
