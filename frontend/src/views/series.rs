// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_router::prelude::Link;

use crate::route::Route;
use crate::services::series::fetch_series;

#[function_component(SeriesComponent)]
pub fn series_page() -> Html {
    let series_list = use_async_with_options(
        async move { fetch_series().await },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(series_list) = &series_list.data {
        return html! {
            <ul>
            {for series_list.list.iter().map(|series| html!{
                <li class="series-item" key={ series.id }>
                <span class="badge">{ series.count }</span>
                <Link<Route> to={ Route::BooksOfSeries { series_id: series.id } }>
                { &series.name }
                </Link<Route>>
                </li>
            })}
            </ul>
        };
    } else {
        return html! {};
    }
}
