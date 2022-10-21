// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::services::users::fetch_users;
use crate::views::util;

#[function_component(UsersComponent)]
pub fn users_page() -> Html {
    util::set_document_title("Users");

    let user_list = use_async_with_options(
        async move { fetch_users().await },
        UseAsyncOptions::enable_auto(),
    );

    user_list.data.as_ref().map_or_else(
        || html! {},
        |user_list| {
            html! {
                <>
                <h2>{ "Users" }</h2>
                <ul>
                {for user_list.iter().map(|user_info| html!{
                    <li key={ user_info.id }>
                    <span>{ &user_info.name }</span>
                    </li>
                })}
                </ul>
                </>
            }
        },
    )
}
