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
pub struct ResponsesPeriodClientPeriodSignUp {
    #[serde(rename = "response")]
    pub response: Box<models::ClientPeriodSignUp>,
    #[serde(rename = "client")]
    pub client: Box<models::ClientPeriodClient>,
}

impl ResponsesPeriodClientPeriodSignUp {
    pub fn new(
        response: models::ClientPeriodSignUp,
        client: models::ClientPeriodClient,
    ) -> ResponsesPeriodClientPeriodSignUp {
        ResponsesPeriodClientPeriodSignUp {
            response: Box::new(response),
            client: Box::new(client),
        }
    }
}
