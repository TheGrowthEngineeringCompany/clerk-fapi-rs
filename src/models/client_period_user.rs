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
pub struct ClientPeriodUser {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// String representing the object's type. Objects of the same type share the same value.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Object>,
    #[serde(
        rename = "external_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_id: Option<Option<String>>,
    #[serde(
        rename = "primary_email_address_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_email_address_id: Option<Option<String>>,
    #[serde(
        rename = "primary_phone_number_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_phone_number_id: Option<Option<String>>,
    #[serde(
        rename = "primary_web3_wallet_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_web3_wallet_id: Option<Option<String>>,
    #[serde(
        rename = "username",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub username: Option<Option<String>>,
    #[serde(
        rename = "first_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub first_name: Option<Option<String>>,
    #[serde(
        rename = "last_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_name: Option<Option<String>>,
    #[serde(rename = "profile_image_url", skip_serializing_if = "Option::is_none")]
    pub profile_image_url: Option<String>,
    #[serde(rename = "image_url", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "has_image", skip_serializing_if = "Option::is_none")]
    pub has_image: Option<bool>,
    #[serde(rename = "public_metadata", skip_serializing_if = "Option::is_none")]
    pub public_metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(
        rename = "private_metadata",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub private_metadata: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "unsafe_metadata", skip_serializing_if = "Option::is_none")]
    pub unsafe_metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "email_addresses", skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<models::ClientPeriodEmailAddress>>,
    #[serde(rename = "phone_numbers", skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<models::ClientPeriodPhoneNumber>>,
    #[serde(rename = "web3_wallets", skip_serializing_if = "Option::is_none")]
    pub web3_wallets: Option<Vec<models::ClientPeriodWeb3Wallet>>,
    #[serde(rename = "passkeys", skip_serializing_if = "Option::is_none")]
    pub passkeys: Option<Vec<models::ClientPeriodPasskey>>,
    #[serde(
        rename = "organization_memberships",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_memberships: Option<Vec<models::ClientPeriodOrganizationMembership>>,
    #[serde(rename = "password_enabled", skip_serializing_if = "Option::is_none")]
    pub password_enabled: Option<bool>,
    #[serde(rename = "two_factor_enabled", skip_serializing_if = "Option::is_none")]
    pub two_factor_enabled: Option<bool>,
    #[serde(rename = "totp_enabled", skip_serializing_if = "Option::is_none")]
    pub totp_enabled: Option<bool>,
    #[serde(
        rename = "backup_code_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_code_enabled: Option<bool>,
    #[serde(rename = "external_accounts", skip_serializing_if = "Option::is_none")]
    pub external_accounts: Option<Vec<models::ExternalAccountWithVerification>>,
    #[serde(rename = "saml_accounts", skip_serializing_if = "Option::is_none")]
    pub saml_accounts: Option<Vec<models::ClientPeriodSamlAccount>>,
    /// Unix timestamp of last sign-in.
    #[serde(
        rename = "last_sign_in_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_sign_in_at: Option<Option<i64>>,
    /// Flag to denote whether user is banned or not.
    #[serde(rename = "banned", skip_serializing_if = "Option::is_none")]
    pub banned: Option<bool>,
    /// Flag to denote whether user is currently locked, i.e. restricted from signing in or not.
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// The number of seconds remaining until the lockout period expires for a locked user. A null value for a locked user indicates that lockout never expires.
    #[serde(
        rename = "lockout_expires_in_seconds",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub lockout_expires_in_seconds: Option<Option<i64>>,
    /// The number of verification attempts remaining until the user is locked. Null if account lockout is not enabled. Note: if a user is locked explicitly via the Backend API, they may still have verification attempts remaining.
    #[serde(
        rename = "verification_attempts_remaining",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_attempts_remaining: Option<Option<i64>>,
    /// Unix timestamp of last update.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// Unix timestamp of creation.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// If enabled, user can delete themselves via FAPI.
    #[serde(
        rename = "delete_self_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_self_enabled: Option<bool>,
    /// If enabled, user can create organizations via FAPI.
    #[serde(
        rename = "create_organization_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_organization_enabled: Option<bool>,
    /// The maximum number of organizations the user can create. 0 means unlimited.
    #[serde(
        rename = "create_organizations_limit",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_organizations_limit: Option<Option<i64>>,
    /// Unix timestamp of the latest session activity, with day precision.
    #[serde(
        rename = "last_active_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_active_at: Option<Option<i64>>,
}

impl ClientPeriodUser {
    pub fn new() -> ClientPeriodUser {
        ClientPeriodUser {
            id: None,
            object: None,
            external_id: None,
            primary_email_address_id: None,
            primary_phone_number_id: None,
            primary_web3_wallet_id: None,
            username: None,
            first_name: None,
            last_name: None,
            profile_image_url: None,
            image_url: None,
            has_image: None,
            public_metadata: None,
            private_metadata: None,
            unsafe_metadata: None,
            email_addresses: None,
            phone_numbers: None,
            web3_wallets: None,
            passkeys: None,
            organization_memberships: None,
            password_enabled: None,
            two_factor_enabled: None,
            totp_enabled: None,
            backup_code_enabled: None,
            external_accounts: None,
            saml_accounts: None,
            last_sign_in_at: None,
            banned: None,
            locked: None,
            lockout_expires_in_seconds: None,
            verification_attempts_remaining: None,
            updated_at: None,
            created_at: None,
            delete_self_enabled: None,
            create_organization_enabled: None,
            create_organizations_limit: None,
            last_active_at: None,
        }
    }
}
/// String representing the object's type. Objects of the same type share the same value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "user")]
    User,
}

impl Default for Object {
    fn default() -> Object {
        Self::User
    }
}