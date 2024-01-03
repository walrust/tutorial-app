use wal_core::component::Component;
use wal_rsx::rsx;

use crate::shoppinglist_page::{
    listitem::{ListItem, ListItemProps},
    listitem_form::ListItemFormProps,
};

use self::{listitem::ListItemDetails, listitem_form::ListItemForm};

mod listitem;
mod listitem_form;

pub(crate) enum ShoppingListMessage {
    AddItem(ListItemDetails),
    RemoveItem(i32),
}

pub(crate) struct ShoppingListPage {
    list_items: Vec<ListItemDetails>,
    next_id: i32,
}

impl Component for ShoppingListPage {
    type Message = ShoppingListMessage;
    type Properties = ();

    fn new(_props: Self::Properties) -> Self {
        ShoppingListPage {
            list_items: vec![],
            next_id: 0,
        }
    }

    fn view(
        &self,
        behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        let add_handler_callback = behavior.create_callback(ShoppingListMessage::AddItem);

        rsx!(
            <div class="container">
                <ListItemForm props={ListItemFormProps {next_id: self.next_id, add_handler: add_handler_callback}}/>
                <h1>"Items on the list:"</h1>
                for { self.list_items.iter().map( |details| {
                    let remove_callback = behavior.create_callback(ShoppingListMessage::RemoveItem);
                    rsx! { <ListItem props={ ListItemProps{details: details.clone(), remove_callback}} /> }
                })}
            </div>
        )
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            ShoppingListMessage::AddItem(details) => {
                self.next_id += 1;
                self.list_items.push(details);
                true
            }
            ShoppingListMessage::RemoveItem(id_to_delete) => {
                self.list_items.retain(|i| i.id != id_to_delete);
                true
            }
        }
    }
}

impl Default for ShoppingListPage {
    fn default() -> Self {
        Self::new(())
    }
}
