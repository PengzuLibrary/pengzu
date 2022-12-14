// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

use crate::general_query::GeneralOrder;
use crate::page::{default_page_id, PageId};

#[must_use]
pub const fn default_parent_id() -> i32 {
    0
}

#[must_use]
pub const fn default_fetch_all() -> bool {
    false
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RecursiveQuery {
    /// Returns root items if parent id is 0.
    #[serde(default = "default_parent_id")]
    pub parent: i32,

    #[serde(default = "default_page_id")]
    pub page: PageId,
    #[serde(default = "GeneralOrder::default")]
    pub order: GeneralOrder,

    /// Returns all records or not.
    #[serde(default = "default_fetch_all")]
    pub fetch_all: bool,
}

impl Default for RecursiveQuery {
    fn default() -> Self {
        Self {
            parent: default_parent_id(),
            page: default_page_id(),
            order: GeneralOrder::default(),
            fetch_all: default_fetch_all(),
        }
    }
}

impl RecursiveQuery {
    pub fn backend_page_id(&self) -> PageId {
        if self.page < 1 {
            0
        } else {
            self.page - 1
        }
    }

    pub fn frontend_page_id(&self) -> PageId {
        if self.page < 1 {
            1
        } else {
            self.page
        }
    }
}
