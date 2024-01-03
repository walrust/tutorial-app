use wal_core::component::Component;
use wal_rsx::rsx;
pub(crate) struct WelcomePage;

impl Component for WelcomePage {
    type Message = ();

    type Properties = ();

    fn new(props: Self::Properties) -> Self {
        WelcomePage
    }

    fn view(
        &self,
        behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        rsx!(<div>"This is a welcome page."</div>)
    }

    fn update(&mut self, message: Self::Message) -> bool {
        false
    }
}

impl Default for WelcomePage {
    fn default() -> Self {
        Self::new(())
    }
}
