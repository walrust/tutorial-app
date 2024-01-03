use wal_core::{
    component::{callback::Callback, Component},
    events::MouseEvent,
};
use wal_rsx::rsx;

#[derive(Hash, Clone)]
pub(crate) struct ListItemProps {
    pub(crate) details: ListItemDetails,
    pub(crate) remove_callback: Callback<i32>,
}

#[derive(Hash, Clone)]
pub(crate) struct ListItemDetails {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) count: i32,
}

pub(crate) struct ListItem {
    props: ListItemProps,
}

impl Component for ListItem {
    type Message = ();

    type Properties = ListItemProps;

    fn new(props: Self::Properties) -> Self {
        ListItem { props }
    }

    fn view(
        &self,
        behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        let props = self.props.clone();

        let delete_on_click = behavior.create_callback(move |_event: MouseEvent| {
            props.remove_callback.emit(props.details.id);
        });

        rsx! {
            <div class="container">
                <span>{&self.props.details.name}</span>
                <span>{self.props.details.count}</span>
                <button onclick={delete_on_click}>
                    "delete"
                </button>
            </div>
        }
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}
