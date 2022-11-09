// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::general_query::GeneralOrder;
use stylist::Style;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub current_order: GeneralOrder,
    pub onchange: Callback<GeneralOrder>,
}

#[function_component(GeneralFilterComponent)]
pub fn general_filter(props: &Props) -> Html {
    let style_str = include_str!("general_filter.css");
    let style_cls = Style::new(style_str).expect("Invalid general_filter.css file");

    let button_onclick = |order: GeneralOrder| {
        let onchange = props.onchange.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            onchange.emit(order);
        })
    };

    let get_button_cls = {
        let active_order = props.current_order;
        move |order: GeneralOrder| {
            if order == active_order {
                "btn btn-primary active"
            } else {
                "btn btn-primary"
            }
        }
    };

    html! {
        <div class={ style_cls }>

        <button class={ get_button_cls(GeneralOrder::IdAsc) }
            title={ "Sort according to book date, newest first" }
            onclick={ button_onclick(GeneralOrder::IdAsc) }>
            <span class="glyphicon glyphicon-sort-by-order" />
        </button>
        <button class={ get_button_cls(GeneralOrder::IdDesc) }
            title={ "Sort according to book date, oldest first" }
            onclick={ button_onclick(GeneralOrder::IdDesc) }>
            <span class="glyphicon glyphicon-sort-by-order-alt" />
        </button>

        <button class={ get_button_cls(GeneralOrder::TitleAsc) }
            title={ "Sort titles in alphabetical order" }
            onclick={ button_onclick(GeneralOrder::TitleAsc) }>
            <span class="glyphicon glyphicon-sort-by-alphabet" />
        </button>
        <button class={ get_button_cls(GeneralOrder::TitleDesc) }
            title={ "Sort titles in reverse alphabetical order" }
            onclick={ button_onclick(GeneralOrder::TitleDesc) }>
            <span class="glyphicon glyphicon-sort-by-alphabet-alt" />
        </button>

        <button class={ get_button_cls(GeneralOrder::NumberAsc) }
            title={ "Sort according to number of books, newest first" }
            onclick={ button_onclick(GeneralOrder::NumberAsc) }>
            <span class="glyphicon glyphicon-book" />
            <span class="glyphicon glyphicon-sort-by-order" />
        </button>
        <button class={ get_button_cls(GeneralOrder::NumberDesc) }
            title={ "Sort according to number of books, oldest first" }
            onclick={ button_onclick(GeneralOrder::NumberDesc) }>
            <span class="glyphicon glyphicon-book" />
            <span class="glyphicon glyphicon-sort-by-order-alt" />
        </button>

        </div>
    }
}