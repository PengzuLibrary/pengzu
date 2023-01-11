// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub ok_cb: Callback<bool>,
}

#[function_component(DeletePublisherModal)]
pub fn delete_publisher_modal(props: &Props) -> Html {
    let on_confirm_button_clicked = {
        let ok_cb_clone = props.ok_cb.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            ok_cb_clone.emit(true);
        })
    };

    html! {
        <div class="modal-dialog modal-dialog-centered">
            <div class="modal-content">
                <div class="modal-header">
                    <h1 class="modal-title fs-5">{ "Will delete publisher" }</h1>
                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>

                <div class="modal-body">
                    <div class="mb-3">
                        { "Please confirm to delete publisher" }
                        <span class="f-bold">{ props.name.clone() }</span>
                    </div>
                </div>

                <div class="modal-footer">
                    <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">
                        { "Cancel"}
                    </button>
                    <button type="button" class="btn btn-primary"
                        data-bs-dismiss="modal"
                        onclick={ on_confirm_button_clicked }>
                        { "Confirm" }
                    </button>
                </div>
            </div>
        </div>
    }
}
