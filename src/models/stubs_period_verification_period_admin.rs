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
pub struct StubsPeriodVerificationPeriodAdmin {
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "strategy")]
    pub strategy: Strategy,
    #[serde(
        rename = "attempts",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub attempts: Option<Option<i32>>,
    #[serde(
        rename = "expire_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub expire_at: Option<Option<i32>>,
}

impl StubsPeriodVerificationPeriodAdmin {
    pub fn new(status: Status, strategy: Strategy) -> StubsPeriodVerificationPeriodAdmin {
        StubsPeriodVerificationPeriodAdmin {
            status,
            strategy,
            attempts: None,
            expire_at: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "verified")]
    Verified,
}

impl Default for Status {
    fn default() -> Status {
        Self::Verified
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "admin")]
    Admin,
}

impl Default for Strategy {
    fn default() -> Strategy {
        Self::Admin
    }
}
