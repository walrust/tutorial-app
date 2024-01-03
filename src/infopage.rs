use wal_core::component::Component;
use wal_rsx::rsx;
pub(crate) struct InfoPage;

impl Component for InfoPage {
    type Message = ();

    type Properties = ();

    fn new(props: Self::Properties) -> Self {
        InfoPage
    }

    fn view(
        &self,
        behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        rsx!(<div>"This is an info page."</div>)
    }

    fn update(&mut self, message: Self::Message) -> bool {
        false
    }
}

impl Default for InfoPage {
    fn default() -> Self {
        Self::new(())
    }
}
