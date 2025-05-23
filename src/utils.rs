use std::{error::Error, fmt};

use log::error;

use crate::{
    clerk_fapi::ClerkFapiClient,
    models::{
        ClientClientWrappedOrganizationMembershipsResponse, ClientPeriodClient,
        ClientPeriodOrganization, ClientPeriodOrganizationMembership, ClientPeriodSession,
    },
};

pub fn find_organization_id_from_memberships(
    memberships: Vec<ClientPeriodOrganizationMembership>,
    organization_id_or_slug: String,
) -> Option<ClientPeriodOrganizationMembership> {
    if organization_id_or_slug.starts_with("org_") {
        // It's an organization ID - verify it exists in memberships
        memberships
            .into_iter()
            .find(|m| m.organization.id == organization_id_or_slug)
    } else {
        // It's a slug
        memberships
            .into_iter()
            .find(|m| m.organization.slug == organization_id_or_slug)
    }
}

#[derive(Debug)]
pub enum ClerkOrgFindingError {
    NoUserFound,
    NoMatchFound,
    ClerkApiError,
}
impl fmt::Display for ClerkOrgFindingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClerkOrgFindingError::NoUserFound => write!(f, "No user found"),
            ClerkOrgFindingError::NoMatchFound => write!(f, "No match found"),
            ClerkOrgFindingError::ClerkApiError => write!(f, "Clerk API error"),
        }
    }
}
impl Error for ClerkOrgFindingError {}

pub async fn find_target_organization(
    fapi: &ClerkFapiClient,
    session: ClientPeriodSession,
    organization_id_or_slug: String,
) -> Result<ClientPeriodOrganization, ClerkOrgFindingError> {
    let user = match session.user {
        Some(user) => *user.clone(),
        None => return Err(ClerkOrgFindingError::NoUserFound),
    };

    if let Some(user_org_memberships) = user.organization_memberships {
        if let Some(org) = find_organization_id_from_memberships(
            user_org_memberships,
            organization_id_or_slug.clone(),
        ) {
            return Ok(*org.organization.clone());
        }
    }

    // The organization didn't exist in the current session
    // let's try to find it from the API
    // Let's start by refreshing user
    let user = *fapi
        .get_user(Some(&session.id))
        .await
        .map_err(|e| {
            error!("Failed to get user: {}", e);
            ClerkOrgFindingError::ClerkApiError
        })?
        .response;

    if let Some(user_org_memberships) = user.organization_memberships {
        if let Some(org) = find_organization_id_from_memberships(
            user_org_memberships,
            organization_id_or_slug.clone(),
        ) {
            return Ok(*org.organization.clone());
        }
    }

    // Still no matching organization found!
    // let's try one more time, le'ts pull the org memberships!

    let org_memberships = *fapi
        .get_organization_memberships(
            None, // limit
            None, // offset
            None, // paginated
        )
        .await
        .map_err(|e| {
            error!("Failed to get org memberships: {}", e);
            ClerkOrgFindingError::ClerkApiError
        })?
        .response;

    let user_org_memberships = match org_memberships {
        ClientClientWrappedOrganizationMembershipsResponse::Array(memberships) => {
            memberships
        },
        ClientClientWrappedOrganizationMembershipsResponse::ClientClientWrappedOrganizationMembershipsResponseOneOf(memberships) => {
            memberships.data.unwrap_or(Vec::new())
        }
    };

    if let Some(org) =
        find_organization_id_from_memberships(user_org_memberships, organization_id_or_slug.clone())
    {
        return Ok(*org.organization.clone());
    }

    // We tried our best, we couldn't find organization
    // user was trying to find
    Err(ClerkOrgFindingError::NoMatchFound)
}

#[derive(Debug)]
pub enum ClerkSessionFindingError {
    NoMatchFound,
    NoSession,
}
impl fmt::Display for ClerkSessionFindingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClerkSessionFindingError::NoMatchFound => write!(f, "No match found"),
            ClerkSessionFindingError::NoSession => write!(f, "No session found"),
        }
    }
}
impl Error for ClerkSessionFindingError {}

pub fn find_target_session(
    client: ClientPeriodClient,
    session_id: String,
) -> Result<ClientPeriodSession, ClerkSessionFindingError> {
    client
        .sessions
        .iter()
        .find(|s| s.id == session_id)
        .cloned()
        .ok_or(ClerkSessionFindingError::NoMatchFound)
}
