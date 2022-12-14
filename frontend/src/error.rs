// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use std::error;
use std::fmt;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    // For status == 400.
    BadRequest,

    // For status == 401.
    Unauthorized,

    // For status == 403.
    Forbidden,

    // For status == 404.
    NotFound,

    // For status == 500.
    InternalServerError,

    JsError,
    DeserializeError,
    RequestError,
    ResponseError,
    UrlParamError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchError {
    kind: ErrorKind,
    message: String,
}

impl FetchError {
    #[must_use]
    pub const fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            message: String::new(),
        }
    }

    #[must_use]
    pub const fn from_string(kind: ErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl error::Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self::from_string(ErrorKind::JsError, format!("{value:?}"))
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> Self {
        Self::from_string(ErrorKind::DeserializeError, err.to_string())
    }
}

impl From<serde_urlencoded::ser::Error> for FetchError {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        Self::from_string(ErrorKind::UrlParamError, err.to_string())
    }
}
