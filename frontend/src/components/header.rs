// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#![allow(clippy::let_unit_value)]

use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::components::header_search::HeaderSearchComponent;
use crate::hooks::use_user_context;
use crate::route::Route;

#[function_component(HeaderComponent)]
pub fn header() -> Html {
    let user_ctx = use_user_context();

    let style_str = include_str!("header.css");
    let style_cls = Style::new(style_str).expect("Invalid style file");

    html! {
        <header class={ style_cls }>
            <Link<Route> to={ Route::Home } classes="navbar-brand">
                { "Pengzu Library" }
            </Link<Route>>

            <HeaderSearchComponent />

            <Link<Route> to={ Route::AdvancedSearch }
                classes="navbar-advanced-search">
                <span class="glyphicon glyphicon-search" />
                <span>{ "Advanced Search" }</span>
            </Link<Route>>

            <ul class="user-container">
            if user_ctx.is_login() {
                <li><Link<Route> to={ Route::UserInfo }>
                    <span class="glyphicon glyphicon-user" />
                    { &user_ctx.name }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::Logout }>
                    <span class="glyphicon glyphicon-log-out" />
                    { "Logout" }
                </Link<Route>></li>
            } else {
                <li><Link<Route> to={ Route::Login }>
                    <span class="glyphicon glyphicon-log-in" />
                    { "Login" }
                </Link<Route>></li>
            }
            </ul>
        </header>
    }
}
