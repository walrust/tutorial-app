use wal_core::{
    component::{callback::Callback, Component},
    events::MouseEvent,
};
use wal_css::{css::Css, css_stylesheet};
use wal_rsx::rsx;

thread_local! {
    static LIST_ITEM_CSS: Css = css_stylesheet!("./listitem_styles.css")
}

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
        _behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        let props = self.props.clone();

        let delete_on_click = Callback::new(move |_event: MouseEvent| {
            props.remove_callback.emit(props.details.id);
        });

        LIST_ITEM_CSS.with(|css| {
            rsx! {
                <div class={&css["container"]}>
                    <div class="container">
                        <p>"name"</p>
                        <p>{&self.props.details.name}</p>
                    </div>
                    <div class="container">
                        <p>"count"</p>
                        <p>{self.props.details.count}</p>
                    </div>
                    <button onclick={delete_on_click}>
                        "delete"
                    </button>
                </div>
            }
        })
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}
