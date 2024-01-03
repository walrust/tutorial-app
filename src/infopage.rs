use wal_core::component::Component;
use wal_rsx::rsx;

pub(crate) struct InfoPage;

impl Component for InfoPage {
    type Message = ();

    type Properties = ();

    fn new(_props: Self::Properties) -> Self {
        InfoPage
    }

    fn view(
        &self,
        _behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        rsx!(<div class="container">
            <h1>"This app has been made using Wal"</h1>
            <p>"You can find more complex examples on the offical Wal GtiHub repository in the demo directory:"</p>
            <a href="https://github.com/walrust/wal">"Wal repository"</a>
            <Link to="/">"back to the shopping list"</Link>
        </div>)
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}

impl Default for InfoPage {
    fn default() -> Self {
        Self::new(())
    }
}
