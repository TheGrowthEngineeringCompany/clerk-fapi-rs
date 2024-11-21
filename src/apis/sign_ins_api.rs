/*
 * Clerk Frontend API
 *
 * The Clerk REST Frontend API, meant to be accessed from a browser or native environment.  This is a Form Based API and all the data must be sent and formatted according to the `application/x-www-form-urlencoded` content type.  ### Versions  When the API changes in a way that isn't compatible with older versions, a new version is released. Each version is identified by its release date, e.g. `2021-02-05`. For more information, please see [Clerk API Versions](https://clerk.com/docs/backend-requests/versioning/overview).  ### Using the Try It Console  The `Try It` feature of the docs only works for **Development Instances** when using the `DevBrowser` security scheme. To use it, first generate a dev instance token from the `/v1/dev_browser` endpoint.  Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`accept_ticket`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AcceptTicketError {
    Status400(models::ClerkErrors),
    Status404(models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`attempt_sign_in_factor_one`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttemptSignInFactorOneError {
    Status400(models::ClerkErrors),
    Status403(models::ClerkErrors),
    Status404(models::ClerkErrors),
    Status422(models::ClerkErrors),
    Status429(models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`attempt_sign_in_factor_two`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttemptSignInFactorTwoError {
    Status400(models::ClerkErrors),
    Status403(models::ClerkErrors),
    Status422(models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_sign_in`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSignInError {
    Status400(models::ClerkErrors),
    Status403(models::ClerkErrors),
    Status404(models::ClerkErrors),
    Status409(models::ClerkErrors),
    Status422(models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sign_in`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSignInError {
    Status400(models::ClerkErrors),
    Status401(models::ClerkErrors),
    Status404(models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`prepare_sign_in_factor_one`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrepareSignInFactorOneError {
    Status400(models::ClerkErrors),
    Status403(models::ClerkErrors),
    Status404(models::ClerkErrors),
    Status422(models::ClerkErrors),
    Status429(models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`prepare_sign_in_factor_two`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrepareSignInFactorTwoError {
    Status400(models::ClerkErrors),
    Status403(models::ClerkErrors),
    Status422(models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`reset_password`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetPasswordError {
    Status400(models::ClerkErrors),
    Status422(models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`verify`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyError {
    Status400(models::ClerkErrors),
    UnknownValue(serde_json::Value),
}

/// Parses a ticket JWT and performs the necessary actions depending on the ticket's source type. Depending on the ticket source type, a successful response can either redirect to a new location with the ticket in the query string, or respond directly with a text/html content type for the response body.
pub async fn accept_ticket(
    configuration: &configuration::Configuration,
    ticket: &str,
) -> Result<(), Error<AcceptTicketError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/tickets/accept", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("ticket", &ticket.to_string())]);
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("_is_native", local_var_value)]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("__dev_session", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AcceptTicketError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Attempt the first verification. Requires the sign in attempt to be identified, and the first factor verification to be prepared, unless you're using a password.  Parameter rules: If the strategy equals `email_code` or `phone_code` then a code is required. If the strategy equals `password` then a password is required.
pub async fn attempt_sign_in_factor_one(
    configuration: &configuration::Configuration,
    sign_in_id: &str,
    strategy: Option<&str>,
    code: Option<&str>,
    password: Option<&str>,
    signature: Option<&str>,
    redirect_url: Option<&str>,
    action_complete_redirect_url: Option<&str>,
    ticket: Option<&str>,
) -> Result<models::ResponsesPeriodClientPeriodSignIn, Error<AttemptSignInFactorOneError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/client/sign_ins/{sign_in_id}/attempt_first_factor",
        local_var_configuration.base_path,
        sign_in_id = crate::apis::urlencode(sign_in_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("_is_native", local_var_value)]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("__dev_session", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = strategy {
        local_var_form_params.insert("strategy", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = code {
        local_var_form_params.insert("code", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = password {
        local_var_form_params.insert("password", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = signature {
        local_var_form_params.insert("signature", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = redirect_url {
        local_var_form_params.insert("redirect_url", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = action_complete_redirect_url {
        local_var_form_params.insert(
            "action_complete_redirect_url",
            local_var_param_value.to_string(),
        );
    }
    if let Some(local_var_param_value) = ticket {
        local_var_form_params.insert("ticket", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AttemptSignInFactorOneError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Attempt the second verification. Requires the sign in attempt `status` to be equal to `needs_second_factor`, and for the preparation step to have been called.
pub async fn attempt_sign_in_factor_two(
    configuration: &configuration::Configuration,
    sign_in_id: &str,
    strategy: Option<&str>,
    code: Option<&str>,
) -> Result<models::ResponsesPeriodClientPeriodSignIn, Error<AttemptSignInFactorTwoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/client/sign_ins/{sign_in_id}/attempt_second_factor",
        local_var_configuration.base_path,
        sign_in_id = crate::apis::urlencode(sign_in_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("_is_native", local_var_value)]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("__dev_session", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = strategy {
        local_var_form_params.insert("strategy", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = code {
        local_var_form_params.insert("code", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AttemptSignInFactorTwoError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates or replaces the current Sign in object. In order to authenticate a Sign in in as few requests as possible, you can pass in parameters to this request that can identify and verify the Sign in.  Parameter rules:  If the strategy equals `phone_code`, `email_code`, `web3_[provider]_signature`, `reset_password_code` or `reset_password_phone_code` then an identifier is required.  If the strategy equals `email_link` then an identifier is required and optionally redirect_url can be supplied.  If the strategy equals `password` then both an identifier and a password is required.  If the strategy equals `oauth_[provider]` or `saml` then a redirect_url is required, and an action_complete_redirect_url is optional.  If the strategy equals `oauth_token_[provider]` then at least one of code (grant code) or token (openID token) is required. Passing only the token will probably retrieve minimal information about the user from the OAuth provider. You can pass both code and token for the best results.  If the strategy equals `ticket` then ticket is required.  If the strategy equals `passkey` then no identifier is provided.  If the strategy equals `google_one_tap` then token is required.
pub async fn create_sign_in(
    configuration: &configuration::Configuration,
    strategy: Option<&str>,
    identifier: Option<&str>,
    password: Option<&str>,
    ticket: Option<&str>,
    redirect_url: Option<&str>,
    action_complete_redirect_url: Option<&str>,
    transfer: Option<bool>,
    code: Option<&str>,
    token: Option<&str>,
) -> Result<models::ResponsesPeriodClientPeriodSignIn, Error<CreateSignInError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/client/sign_ins", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("_is_native", local_var_value)]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("__dev_session", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = strategy {
        local_var_form_params.insert("strategy", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = identifier {
        local_var_form_params.insert("identifier", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = password {
        local_var_form_params.insert("password", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ticket {
        local_var_form_params.insert("ticket", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = redirect_url {
        local_var_form_params.insert("redirect_url", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = action_complete_redirect_url {
        local_var_form_params.insert(
            "action_complete_redirect_url",
            local_var_param_value.to_string(),
        );
    }
    if let Some(local_var_param_value) = transfer {
        local_var_form_params.insert("transfer", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = code {
        local_var_form_params.insert("code", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = token {
        local_var_form_params.insert("token", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateSignInError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the sign-in with the given id. The sign in is returned only if it belongs to the requesting client and is not abandoned.
pub async fn get_sign_in(
    configuration: &configuration::Configuration,
    sign_in_id: &str,
) -> Result<models::ResponsesPeriodClientPeriodSignIn, Error<GetSignInError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/client/sign_ins/{sign_in_id}",
        local_var_configuration.base_path,
        sign_in_id = crate::apis::urlencode(sign_in_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("_is_native", local_var_value)]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("__dev_session", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSignInError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Prepares the verification object for the identified Sign in. This step authenticates that the user is who they say they are. Depending on the strategy, this request request will do something different.  Parameter actions: If the strategy equals email_code then this request will send an email with an OTP code. If the strategy equals phone_code then this request will send an SMS with an OTP code. If the strategy equals oauth_[provider] then this request generate a URL that the User needs to visit in order to authenticate. If the strategy equals passkey then this request will begin the passkey registration flow.  Parameter rules: If the strategy equals `oauth_[provider]` then a redirect_url is required, and an action_complete_redirect_url is optional.
pub async fn prepare_sign_in_factor_one(
    configuration: &configuration::Configuration,
    sign_in_id: &str,
    strategy: Option<&str>,
    email_address_id: Option<&str>,
    phone_number_id: Option<&str>,
    web3_wallet_id: Option<&str>,
    passkey_id: Option<&str>,
    redirect_url: Option<&str>,
    action_complete_redirect_url: Option<&str>,
) -> Result<models::ResponsesPeriodClientPeriodSignIn, Error<PrepareSignInFactorOneError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/client/sign_ins/{sign_in_id}/prepare_first_factor",
        local_var_configuration.base_path,
        sign_in_id = crate::apis::urlencode(sign_in_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("_is_native", local_var_value)]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("__dev_session", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = strategy {
        local_var_form_params.insert("strategy", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = email_address_id {
        local_var_form_params.insert("email_address_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = phone_number_id {
        local_var_form_params.insert("phone_number_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = web3_wallet_id {
        local_var_form_params.insert("web3_wallet_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = passkey_id {
        local_var_form_params.insert("passkey_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = redirect_url {
        local_var_form_params.insert("redirect_url", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = action_complete_redirect_url {
        local_var_form_params.insert(
            "action_complete_redirect_url",
            local_var_param_value.to_string(),
        );
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PrepareSignInFactorOneError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Prepare the second verification. Requires the sign in attempt `status` to be equal to `needs_second_factor`.
pub async fn prepare_sign_in_factor_two(
    configuration: &configuration::Configuration,
    sign_in_id: &str,
    strategy: Option<&str>,
    phone_number_id: Option<&str>,
) -> Result<models::ResponsesPeriodClientPeriodSignIn, Error<PrepareSignInFactorTwoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/client/sign_ins/{sign_in_id}/prepare_second_factor",
        local_var_configuration.base_path,
        sign_in_id = crate::apis::urlencode(sign_in_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("_is_native", local_var_value)]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("__dev_session", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = strategy {
        local_var_form_params.insert("strategy", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = phone_number_id {
        local_var_form_params.insert("phone_number_id", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PrepareSignInFactorTwoError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Reset password on sign-in.
pub async fn reset_password(
    configuration: &configuration::Configuration,
    sign_in_id: &str,
    password: Option<&str>,
    sign_out_of_other_sessions: Option<bool>,
) -> Result<models::ResponsesPeriodClientPeriodSignIn, Error<ResetPasswordError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/client/sign_ins/{sign_in_id}/reset_password",
        local_var_configuration.base_path,
        sign_in_id = crate::apis::urlencode(sign_in_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("_is_native", local_var_value)]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("__dev_session", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = password {
        local_var_form_params.insert("password", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = sign_out_of_other_sessions {
        local_var_form_params.insert(
            "sign_out_of_other_sessions",
            local_var_param_value.to_string(),
        );
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ResetPasswordError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Attempt to verify a verification with email_link strategy.
pub async fn verify(
    configuration: &configuration::Configuration,
    token: &str,
) -> Result<(), Error<VerifyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/verify", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("_is_native", local_var_value)]);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.query(&[("__dev_session", local_var_value)]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<VerifyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}