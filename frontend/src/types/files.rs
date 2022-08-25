// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct File {
    pub id: i32,
    pub book: i32,
    pub format: i32,
    pub size: i32,
    pub name: String,
    pub sha: String,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}