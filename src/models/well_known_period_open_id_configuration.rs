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
pub struct WellKnownPeriodOpenIdConfiguration {
    #[serde(rename = "issuer")]
    pub issuer: String,
    #[serde(rename = "authorization_endpoint")]
    pub authorization_endpoint: String,
    #[serde(rename = "token_endpoint")]
    pub token_endpoint: String,
    #[serde(rename = "userinfo_endpoint")]
    pub userinfo_endpoint: String,
    #[serde(rename = "jwks_uri")]
    pub jwks_uri: String,
    #[serde(rename = "scopes_supported")]
    pub scopes_supported: Vec<String>,
    #[serde(rename = "response_types_supported")]
    pub response_types_supported: Vec<String>,
    #[serde(rename = "response_modes_supported")]
    pub response_modes_supported: Vec<String>,
    #[serde(rename = "grant_types_supported")]
    pub grant_types_supported: Vec<String>,
    #[serde(rename = "subject_types_supported")]
    pub subject_types_supported: Vec<String>,
    #[serde(rename = "id_token_signing_alg_values_supported")]
    pub id_token_signing_alg_values_supported: Vec<String>,
    #[serde(rename = "token_endpoint_auth_methods_supported")]
    pub token_endpoint_auth_methods_supported: Vec<String>,
    #[serde(rename = "claims_supported")]
    pub claims_supported: Vec<String>,
    #[serde(rename = "code_challenge_methods_supported")]
    pub code_challenge_methods_supported: Vec<String>,
    #[serde(rename = "backchannel_logout_supported")]
    pub backchannel_logout_supported: bool,
    #[serde(rename = "frontchannel_logout_supported")]
    pub frontchannel_logout_supported: bool,
}

impl WellKnownPeriodOpenIdConfiguration {
    pub fn new(
        issuer: String,
        authorization_endpoint: String,
        token_endpoint: String,
        userinfo_endpoint: String,
        jwks_uri: String,
        scopes_supported: Vec<String>,
        response_types_supported: Vec<String>,
        response_modes_supported: Vec<String>,
        grant_types_supported: Vec<String>,
        subject_types_supported: Vec<String>,
        id_token_signing_alg_values_supported: Vec<String>,
        token_endpoint_auth_methods_supported: Vec<String>,
        claims_supported: Vec<String>,
        code_challenge_methods_supported: Vec<String>,
        backchannel_logout_supported: bool,
        frontchannel_logout_supported: bool,
    ) -> WellKnownPeriodOpenIdConfiguration {
        WellKnownPeriodOpenIdConfiguration {
            issuer,
            authorization_endpoint,
            token_endpoint,
            userinfo_endpoint,
            jwks_uri,
            scopes_supported,
            response_types_supported,
            response_modes_supported,
            grant_types_supported,
            subject_types_supported,
            id_token_signing_alg_values_supported,
            token_endpoint_auth_methods_supported,
            claims_supported,
            code_challenge_methods_supported,
            backchannel_logout_supported,
            frontchannel_logout_supported,
        }
    }
}
