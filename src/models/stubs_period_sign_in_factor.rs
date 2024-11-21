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
pub struct StubsPeriodSignInFactor {
    #[serde(rename = "strategy")]
    pub strategy: Strategy,
    #[serde(rename = "safe_identifier", skip_serializing_if = "Option::is_none")]
    pub safe_identifier: Option<String>,
    #[serde(rename = "email_address_id", skip_serializing_if = "Option::is_none")]
    pub email_address_id: Option<String>,
    #[serde(rename = "phone_number_id", skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
    #[serde(rename = "web3_wallet_id", skip_serializing_if = "Option::is_none")]
    pub web3_wallet_id: Option<String>,
    #[serde(rename = "passkey_id", skip_serializing_if = "Option::is_none")]
    pub passkey_id: Option<String>,
    #[serde(
        rename = "primary",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary: Option<Option<bool>>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

impl StubsPeriodSignInFactor {
    pub fn new(strategy: Strategy) -> StubsPeriodSignInFactor {
        StubsPeriodSignInFactor {
            strategy,
            safe_identifier: None,
            email_address_id: None,
            phone_number_id: None,
            web3_wallet_id: None,
            passkey_id: None,
            primary: None,
            default: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "ticket")]
    Ticket,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "email_code")]
    EmailCode,
    #[serde(rename = "email_link")]
    EmailLink,
    #[serde(rename = "phone_code")]
    PhoneCode,
    #[serde(rename = "web3_metamask_signature")]
    Web3MetamaskSignature,
    #[serde(rename = "web3_coinbase_wallet_signature")]
    Web3CoinbaseWalletSignature,
    #[serde(rename = "totp")]
    Totp,
    #[serde(rename = "backup_code")]
    BackupCode,
    #[serde(rename = "oauth_apple")]
    OauthApple,
    #[serde(rename = "oauth_google")]
    OauthGoogle,
    #[serde(rename = "oauth_facebook")]
    OauthFacebook,
    #[serde(rename = "oauth_hubspot")]
    OauthHubspot,
    #[serde(rename = "oauth_github")]
    OauthGithub,
    #[serde(rename = "oauth_mock")]
    OauthMock,
    #[serde(rename = "oauth_custom_mock")]
    OauthCustomMock,
    #[serde(rename = "oauth_token_mock")]
    OauthTokenMock,
    #[serde(rename = "saml")]
    Saml,
    #[serde(rename = "enterprise_sso")]
    EnterpriseSso,
    #[serde(rename = "reset_password_email_code")]
    ResetPasswordEmailCode,
    #[serde(rename = "reset_password_phone_code")]
    ResetPasswordPhoneCode,
    #[serde(rename = "passkey")]
    Passkey,
    #[serde(rename = "google_one_tap")]
    GoogleOneTap,
}

impl Default for Strategy {
    fn default() -> Strategy {
        Self::Ticket
    }
}