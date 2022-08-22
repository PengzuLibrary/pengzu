// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::services::fetch::fetch;
use crate::types::error::FetchError;
use crate::types::publishers::GetPublishersResp;

/// Get publisher list.
///
/// # Error
///
/// Returns error if server fails.
pub async fn fetch_publishers() -> Result<GetPublishersResp, FetchError> {
    let url = "/api/publisher";
    let text = fetch(url).await?;
    let obj: GetPublishersResp = serde_json::from_str(&text)?;
    Ok(obj)
}