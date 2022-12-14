// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::categories::{CategoryAndBook, CategoryAndBookList};
use shared::recursive_query::RecursiveQuery;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::router::Route;
use crate::services::categories::fetch_categories;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub category: CategoryAndBook,
}

pub fn generate_category_list(category_list: &CategoryAndBookList) -> Html {
    html! {
        <ol class="">
        {for category_list.list.iter().map(|category| html!{
            <li class="mb-2" key={ category.id }>
                <CategoryItemComponent category={ category.clone() } />
            </li>
        })}
        </ol>
    }
}

#[function_component(CategoryItemComponent)]
pub fn category_item(props: &Props) -> Html {
    let category = &props.category;
    let parent_id = category.id;

    let child_categories = use_async(async move {
        let query = RecursiveQuery {
            parent: parent_id,
            fetch_all: true,
            ..RecursiveQuery::default()
        };
        fetch_categories(&query).await
    });
    let onclick = {
        let child_categories = child_categories.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            child_categories.run();
        })
    };

    let child_items = child_categories
        .data
        .as_ref()
        .map_or_else(|| html! {}, generate_category_list);

    html! {
        <>
            <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ category.count }</span>
            <span class="me-1">{ "[" }{ &category.serial_number }{ "]" }</span>
            <Link<Route> to={ Route::BooksOfCategory { category_id: category.id }}>
                { &category.name }
            </Link<Route>>
            <a href="#" {onclick} ><i class="bi bi-caret-right"></i></a>
            { child_items }
        </>
    }
}
