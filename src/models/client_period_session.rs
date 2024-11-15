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
pub struct ClientPeriodSession {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// String representing the object's type. Objects of the same type share the same value.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Object>,
    #[serde(
        rename = "actor",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub actor: Option<Option<serde_json::Value>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "last_active_at", skip_serializing_if = "Option::is_none")]
    pub last_active_at: Option<i64>,
    /* @Nipsuli: this is not in the api doc but exists in the response data  */
    #[serde(
        rename = "last_active_organization_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_active_organization_id: Option<String>,
    #[serde(rename = "expire_at", skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<i64>,
    #[serde(rename = "abandon_at", skip_serializing_if = "Option::is_none")]
    pub abandon_at: Option<i64>,
    #[serde(
        rename = "user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<Option<Box<models::ClientPeriodUser>>>,
    #[serde(
        rename = "public_user_data",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_user_data: Option<Option<Box<models::ClientSessionAllOfPublicUserData>>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

impl ClientPeriodSession {
    pub fn new() -> ClientPeriodSession {
        ClientPeriodSession {
            id: None,
            object: None,
            actor: None,
            status: None,
            last_active_at: None,
            last_active_organization_id: None,
            expire_at: None,
            abandon_at: None,
            user: None,
            public_user_data: None,
            created_at: None,
            updated_at: None,
        }
    }
}
/// String representing the object's type. Objects of the same type share the same value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "session")]
    Session,
}

impl Default for Object {
    fn default() -> Object {
        Self::Session
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "ended")]
    Ended,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "removed")]
    Removed,
    #[serde(rename = "abandoned")]
    Abandoned,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
