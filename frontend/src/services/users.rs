// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::users::{LoginForm, NewUserReq, UserInfo};

use crate::error::FetchError;
use crate::services::fetch::{request_get, request_post};

/// Send login form to server and sign in.
///
/// # Errors
/// Returns error if
/// - invalid username or password
/// - server error
pub async fn login(form: &LoginForm) -> Result<UserInfo, FetchError> {
    let url = "/api/user/login";
    request_post(url, form).await
}

/// Get all user list.
///
/// # Errors
/// Returns error if:
/// - Current user has no root-rule
/// - server error
pub async fn fetch_users() -> Result<Vec<UserInfo>, FetchError> {
    request_get("/api/user").await
}

/// Get current user info.
///
/// # Errors
/// Returns error if:
/// - invalid token
/// - invalid user
/// - server error
pub async fn get_user_info() -> Result<UserInfo, FetchError> {
    let url = "/api/user/self";
    request_get(url).await
}

/// Create a new user account.
///
/// # Errors
/// Returns error if:
/// - required field is missing
/// - invalid user role
/// - server error
pub async fn add_user(query: &NewUserReq) -> Result<UserInfo, FetchError> {
    let url = "/api/user";
    request_post(url, query).await
}
