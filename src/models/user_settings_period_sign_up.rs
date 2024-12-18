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
pub struct UserSettingsPeriodSignUp {
    #[serde(rename = "captcha_enabled")]
    pub captcha_enabled: bool,
    #[serde(
        rename = "captcha_widget_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub captcha_widget_type: Option<String>,
    #[serde(rename = "custom_action_required")]
    pub custom_action_required: bool,
    #[serde(rename = "progressive")]
    pub progressive: bool,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
}

impl UserSettingsPeriodSignUp {
    pub fn new(
        captcha_enabled: bool,
        custom_action_required: bool,
        progressive: bool,
    ) -> UserSettingsPeriodSignUp {
        UserSettingsPeriodSignUp {
            captcha_enabled,
            captcha_widget_type: None,
            custom_action_required,
            progressive,
            mode: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "restricted")]
    Restricted,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Public
    }
}
