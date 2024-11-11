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
pub struct ClientPeriodWeb3Wallet {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// String representing the object's type. Objects of the same type share the same value.
    #[serde(rename = "object")]
    pub object: Object,
    #[serde(rename = "web3_wallet")]
    pub web3_wallet: String,
    #[serde(rename = "verification", deserialize_with = "Option::deserialize")]
    pub verification: Option<Box<models::ClientWeb3WalletVerification>>,
    /// Unix timestamp of creation
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// Unix timestamp of creation
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}

impl ClientPeriodWeb3Wallet {
    pub fn new(
        object: Object,
        web3_wallet: String,
        verification: Option<models::ClientWeb3WalletVerification>,
        created_at: i64,
        updated_at: i64,
    ) -> ClientPeriodWeb3Wallet {
        ClientPeriodWeb3Wallet {
            id: None,
            object,
            web3_wallet,
            verification: if let Some(x) = verification {
                Some(Box::new(x))
            } else {
                None
            },
            created_at,
            updated_at,
        }
    }
}
/// String representing the object's type. Objects of the same type share the same value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "web3_wallet")]
    Web3Wallet,
}

impl Default for Object {
    fn default() -> Object {
        Self::Web3Wallet
    }
}
