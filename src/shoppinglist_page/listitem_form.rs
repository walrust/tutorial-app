use wal_core::{
    component::{callback::Callback, Component},
    events::MouseEvent,
};
use wal_rsx::rsx;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::shoppinglist_page::listitem::ListItemDetails;

pub(crate) struct ListItemForm {
    next_item_id: i32,
    item_name: String,
    item_count: i32,
    add_handler: Callback<ListItemDetails>,
}

#[derive(Hash, Clone)]
pub(crate) struct ListItemFormProps {
    pub(crate) next_id: i32,
    pub(crate) add_handler: Callback<ListItemDetails>,
}

impl Component for ListItemForm {
    type Message = ();

    type Properties = ListItemFormProps;

    fn new(props: Self::Properties) -> Self {
        ListItemForm {
            next_item_id: props.next_id,
            item_name: "".to_string(),
            item_count: 0,
            add_handler: props.add_handler,
        }
    }

    fn view(
        &self,
        _behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        let new_item_id: i32 = self.next_item_id;
        let add_handler = self.add_handler.clone();

        // create callback to handle button press
        let add_on_click = Callback::new(move |_event: MouseEvent| {
            let document = web_sys::window().unwrap().document().unwrap();

            // get name from input
            let element = document.get_element_by_id("newItemName").unwrap();
            let input_element = element.dyn_into::<HtmlInputElement>().unwrap();
            let new_item_name = input_element.value();
            input_element.set_value("");

            // get count from input
            let element = document.get_element_by_id("newItemCount").unwrap();
            let input_element = element.dyn_into::<HtmlInputElement>().unwrap();
            let new_item_count = input_element.value().parse::<i32>().unwrap_or(0);
            input_element.set_value("0");

            let message = ListItemDetails {
                id: new_item_id,
                count: new_item_count,
                name: new_item_name,
            };
            // emit callback received in props
            add_handler.emit(message);
        });

        rsx! {
            <h1>"Add new item"</h1>
            <div class="container">
                <div>
                    <label for="newItemName">"name"</label>
                    <input id="newItemName" value = {&self.item_name} />
                </div>
                <div>
                    <label for="newItemCount">"count"</label>
                    <input id="newItemCount" type="number" value = {self.item_count} />
                </div>
                <button onclick={add_on_click}>
                    "Add"
                </button>
            </div>
        }
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}
