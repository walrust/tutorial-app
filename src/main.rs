use infopage::InfoPage;
use wal_core::router::builder::RouterBuilder;
use welcomepage::WelcomePage;

mod infopage;
mod welcomepage;

fn main() {
    RouterBuilder::default()
        .add_page::<WelcomePage>("/")
        .add_page::<InfoPage>("/info")
        .build()
        .start();
}
