// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

pub const BOOKS_EACH_PAGE: i64 = 50;
pub const CATEGORIES_EACH_PAGE: i64 = 90;

pub type PageId = i64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    pub page_num: PageId,
    pub each_page: i64,
    pub total: i64,
}

#[must_use]
pub const fn default_page_id() -> PageId {
    1
}

// TODO(Shaohua): Remove this struct.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageQuery {
    #[serde(default = "default_page_id")]
    pub page: PageId,
}
