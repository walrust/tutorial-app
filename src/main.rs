use infopage::InfoPage;
use shoppinglist_page::ShoppingListPage;
use wal_core::router::builder::RouterBuilder;

mod infopage;
mod shoppinglist_page;

fn main() {
    RouterBuilder::default()
        .add_page::<ShoppingListPage>("/")
        .add_page::<InfoPage>("/info")
        .build()
        .start();
}
