// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
#[cfg(feature = "diesel")]
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::page::Page;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(Queryable))]
pub struct Category {
    pub id: i32,
    pub order_index: i32,
    pub serial_number: String,
    pub name: String,
    pub url: String,
    pub description: Option<String>,
    pub parent: i32,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "diesel", derive(Queryable))]
pub struct CategoryAndBook {
    pub id: i32,
    pub order_index: i32,
    pub serial_number: String,
    pub name: String,
    pub parent: i32,
    /// Number of books belongs to this category.
    pub count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryAndBookList {
    pub page: Page,
    pub list: Vec<CategoryAndBook>,
}
