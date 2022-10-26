// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History};
use yew_router::prelude::use_history;

use crate::router::Route;
use crate::services::auth::set_token;
use crate::types::users::{UserInfo, UserRole};

/// State handle for the [`use_user_context`] hook.
#[derive(Clone)]
pub struct UseUserContextHandle {
    inner: UseStateHandle<UserInfo>,
    history: AnyHistory,
}

impl UseUserContextHandle {
    pub fn login(&self, info: UserInfo) {
        set_token(Some(&info.token));
        self.inner.set(info);
        self.history.push(Route::Home);
    }

    pub fn logout(&self) {
        set_token(None);
        self.inner.set(UserInfo::default());
        self.history.push(Route::Home);
    }

    pub fn is_login(&self) -> bool {
        self.inner.id > 0 && self.inner.role != UserRole::Nil
    }
}

impl Deref for UseUserContextHandle {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl fmt::Debug for UseUserContextHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseUserContextHandle")
            .field("value", &format!("{:#?}", *self.inner))
            .finish()
    }
}

/// Manages user login context.
///
/// # Panics
/// Raise panic if failed to get history object.
#[must_use]
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<UserInfo>>().unwrap();
    let history = use_history().unwrap();

    UseUserContextHandle { inner, history }
}
