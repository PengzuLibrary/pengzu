// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::Serialize;

#[must_use]
pub const fn default_page_id() -> i64 {
    0
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PageQuery {
    #[serde(default = "default_page_id")]
    pub page: i64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum GetBooksOrder {
    IdDesc,
    IdAsc,
    TitleDesc,
    TitleAsc,
    CreatedDesc,
    CreatedAsc,
    LastModifiedDesc,
    LastModifiedAsc,
    PubdateDesc,
    PubdateAsc,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetBooksQuery {
    #[serde(default = "default_page_id")]
    pub page: i64,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,
}

#[must_use]
pub fn append_query_to_url(url: &str, query: Option<GetBooksQuery>) -> String {
    match query {
        Some(q) => {
            format!("{}?page={}&order={:?}", url, q.page, q.order)
        }
        None => url.to_string(),
    }
}