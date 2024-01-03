use wal_core::component::Component;
use wal_rsx::rsx;

pub(crate) struct ShoppingListPage;

impl Component for ShoppingListPage {
    type Message = ();
    type Properties = ();

    // component constructor
    fn new(props: Self::Properties) -> Self {
        ShoppingListPage
    }

    // view function - here we define the component structure using rsx syntax
    fn view(
        &self,
        behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        rsx!(<div>"This is a shopping list page."</div>)
    }

    // update function - for now we don't need any updating
    fn update(&mut self, message: Self::Message) -> bool {
        false
    }
}

impl Default for ShoppingListPage {
    fn default() -> Self {
        Self::new(())
    }
}
