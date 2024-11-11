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
pub struct ClientPeriodAccountPortal {
    #[serde(rename = "object")]
    pub object: Object,
    #[serde(rename = "allowed", skip_serializing_if = "Option::is_none")]
    pub allowed: Option<bool>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "internal_linking")]
    pub internal_linking: bool,
    #[serde(rename = "after_sign_in_url")]
    pub after_sign_in_url: String,
    #[serde(rename = "after_sign_up_url")]
    pub after_sign_up_url: String,
    #[serde(rename = "after_create_organization_url")]
    pub after_create_organization_url: String,
    #[serde(rename = "after_leave_organization_url")]
    pub after_leave_organization_url: String,
    #[serde(rename = "logo_link_url")]
    pub logo_link_url: String,
}

impl ClientPeriodAccountPortal {
    pub fn new(
        object: Object,
        enabled: bool,
        internal_linking: bool,
        after_sign_in_url: String,
        after_sign_up_url: String,
        after_create_organization_url: String,
        after_leave_organization_url: String,
        logo_link_url: String,
    ) -> ClientPeriodAccountPortal {
        ClientPeriodAccountPortal {
            object,
            allowed: None,
            enabled,
            internal_linking,
            after_sign_in_url,
            after_sign_up_url,
            after_create_organization_url,
            after_leave_organization_url,
            logo_link_url,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "account_portal")]
    AccountPortal,
}

impl Default for Object {
    fn default() -> Object {
        Self::AccountPortal
    }
}
