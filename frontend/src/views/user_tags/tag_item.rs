// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::route::Route;
use crate::services::user_tags::fetch_user_tags;
use crate::types::user_tags::{UserTagAndBook, UserTagList};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub tag: UserTagAndBook,
}

pub fn generate_tag_list(tag_list: &UserTagList) -> Html {
    html! {
        <ul>
        {for tag_list.list.iter().map(|tag| html!{
            <li class="tag-item" key={ tag.id }>
            <UserTagItemComponent tag={ tag.clone() } />
            </li>
        })}
        </ul>
    }
}

#[function_component(UserTagItemComponent)]
pub fn tag_item(props: &Props) -> Html {
    let tag = &props.tag;
    let parent_id = tag.id;

    let child_tags = { use_async(async move { fetch_user_tags(parent_id).await }) };
    let onclick = {
        let child_tags = child_tags.clone();
        Callback::from(move |_event| {
            child_tags.run();
        })
    };

    let child_items = if let Some(tag_list) = &child_tags.data {
        generate_tag_list(tag_list)
    } else {
        html! {}
    };

    // TODO(Shaohua): Replace with BooksOfUserTag
    return html! {
        <>
            <span class="badge">{ tag.count }</span>
            <Link<Route> to={ Route::BooksOfTag { tag_id: tag.id }}>
                { &tag.name }
            </Link<Route>>
            <a href="#" {onclick}>{ "˃" }</a>
            { child_items }
        </>
    };
}