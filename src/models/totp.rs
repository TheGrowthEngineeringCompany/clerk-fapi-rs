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
pub struct Totp {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "secret", deserialize_with = "Option::deserialize")]
    pub secret: Option<String>,
    #[serde(rename = "uri", deserialize_with = "Option::deserialize")]
    pub uri: Option<String>,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(
        rename = "backup_codes",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_codes: Option<Option<Vec<String>>>,
}

impl Totp {
    pub fn new(
        object: String,
        id: String,
        secret: Option<String>,
        uri: Option<String>,
        verified: bool,
    ) -> Totp {
        Totp {
            object,
            id,
            secret,
            uri,
            verified,
            backup_codes: None,
        }
    }
}
