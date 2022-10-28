// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_router::history::{History, Location};
use yew_router::hooks::{use_history, use_location};

use crate::components::book_filter::BookFilterComponent;
use crate::components::book_list::BookListComponent;
use crate::components::book_pagination::BookPaginationComponent;
use crate::router::Route;
use crate::services::books::fetch_books;
use crate::types::books::{GetBooksOrder, GetBooksQuery};
use crate::types::page::PageId;
use crate::views::util;

#[function_component(BooksComponent)]
pub fn books() -> Html {
    util::set_document_title("Books");

    let history = use_history().unwrap();
    let location = use_location().unwrap();
    let query = location.query::<GetBooksQuery>().unwrap_or_default();
    let book_list = {
        let query_clone = query.clone();
        use_async_with_options(
            async move { fetch_books(&query_clone).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    let book_filter_onchange = {
        Callback::from(|order: GetBooksOrder| {
            log::info!("new order: {:?}", order);
        })
    };

    let pagination_onclick = {
        Callback::from(move |page_id: PageId| {
            let new_query = GetBooksQuery {
                page: page_id,
                ..query
            };
            let ret = history.push_with_query(Route::Book, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    book_list.data.as_ref().map_or_else(
        || html! {},
        |book_list| {
            html! {
                <>
                <h2>{ "Books" }</h2>
                <BookFilterComponent onchange={ book_filter_onchange }/>
                <BookListComponent books={ book_list.list.clone() } />
                <BookPaginationComponent current_page={ book_list.page.page_num }
                    total_pages={ book_list.page.total_pages() }
                    onclick={ pagination_onclick } />
                </>
            }
        },
    )
}
