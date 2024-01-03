Karol Kącki, Wal 2024

# Wprowadzenie

Ten dokument zawiera instrukcję instalacji oraz korzystania z narzędzia Wal, przeznaczonego do tworzenia warstwy front-endowej aplikacji webowych przy pomocy języka Rust. W dokumencie przedstawiono proces tworzenia przykładowej aplikacji krok po kroku. Wynikowy kod uzyskany poprzez realizację instrukcji znajduje się w plikach repozytorium.

# Instalacja

Przed rozpoczęciem procesu tworzenia aplikacji niezbędne jest wykonanie następujących kroków:

1. Instalacja języka Rust.
2. Inicjalizacja szablonu projektu.
3. Instalacja narzędzi wasm-bindgen-cli oraz trunk.

## Instalacja języka Rust

Aby umożliwić instalację biblioteki Wal należy najpierw zainstalować kompilator języka Rust. Szczegółowe instrukcje dostępne są na oficjalnej stronie języka Rust: https://www.rust-lang.org/learn/get-started.

## Inicjalizacja szablonu projektu

Aby ułatwić inicjalizację początkowego projektu zaleca się skorzystanie z [szablonu projektu](https://github.com/walrust/template). Kod można skopiować bezpośrednio z repozytorium szablonu lub wygenerować przy pomocy narzędzia [Cargo Generate](https://github.com/cargo-generate/cargo-generate) .

### Inicjalizacja z użyciem Cargo Generate

Aby zainstalować Cargo Generate należy będąc w docelowej lokalizacji projektu wywołać polecenie:

```
cargo install cargo-generate
```

Następnie należy dokonać inicjalizacji projektu komendą:

```
cargo generate --git https://github.com/walrust/template
```

oraz podać nawę projektu, przykładowo **_tutorial-app_**. Pliki projektu zostaną wygenerowane w folderze o podanej nazwie projektu. Po zakończeniu generacji należy przejść do folderu projektu:

```
cd ./tutorial-app
```

## Instalacja wasm-bindgen-cli oraz trunk

Po inicjalizacji szablonu należy zainstalować dodatkowe narzędzia, które potrzebne będą do uruchomienia aplikacji:

1. **wasm-bindgen-cli**, które jest wymagane do umożliwienia automatycznej kompilacji z języka Rust do formatu WebAssembly.
2. **trunk**, które umożliwia wystawienie aplikacji oraz zapewnia funkcję hot-reload przydatną podczas fazy developementu.

Instalacja obydwu narzędzi odbywa się poprzez wywołanie poleceń:

```
cargo install --locked wasm-bindgen-cli
cargo install trunk
```

## Uruchomienie aplikacji

Po zainstalowaniu potrzebnych narzędzi można uruchomić podstawową aplikację stworzoną na podstawie szablonu. W tym celu należy w folderze projektu wywołać kolejno polecenia:

```
cargo build
trunk serve
```

Aplikacja zostanie wystawiona lokalnie na porcie, który został wskazany w pliku **Trunk.toml** (domyślnie jest to port nr 3000). Kiedy proces Trunk jest uruchomiony, dokonane w projekcie zmiany (po ich zapisaniu) zostaną automatycznie zastosowane w uruchomionej aplikacji.

# Tworzenie aplikacji z wykorzystaniem Wal

W tej sekcji zostanie opisany krok po kroku proces tworzenia prostej aplikacji z wykorzystaniem narzędzia Wal. Aplikacja ta składała się będzie z dwóch podstron: **strony z listą zakupów** oraz **strony z informacjami o projekcie**.

## Stworzenie komponentów stron

Pierwszym krokiem będzie stworzenie dwóch komponentów, po jednym dla każdej z podstron. Strona informacyjna będzie składała się tylko z jednego prostego komponentu, dlatego jej plik o nazwie `infopage.rs` stworzony zostanie bezpośrednio w katalogu `src`. Strona z listą zakupów będzie bardziej złożona, dlatego lepiej wyznaczyć dla niej osoby folder `src/shoppinglist_page`. W środku utworzonego folderu w pliku `mod.rs` zadeklarowany zostanie tymczasowo główny komponent strony z zakupami. Struktura plików powinna wyglądac nasteująco:

```
src
│── shoppinglist_page
│ 	└── mod.rs
├── infopage.rs
├── main.rs
└── hellopage.rs
```

Następnie w pliku `main.rs` należy dodać nowo utworzony plik `infopage.rs` oraz folder `shoppinglist_page` jako moduły projektu:

```Rust
use hellopage::HelloComponent;
use wal_core::router::builder::RouterBuilder;

mod hellopage;
mod infopage;
mod shoppinglist_page;

fn main() {
    RouterBuilder::default()
        .add_page::<HelloComponent>("/")
        .build()
        .start();
}
```

Wewnątrz każdego z nowych plików znajdować się będzie struktura o odpowiadającej folderowi nazwie będąca głównym komponentem danej podstrony. Dla każdej z tych struktur zaimplementowany musi zostać trait `Component` oraz trait `Default`.

```Rust
// --- infopage.rs ---

use wal_core::component::Component;
use wal_rsx::rsx;

// component struct - temporarily with no fields
pub(crate) struct InfoPage;

impl Component for InfoPage {
   
    // we don't need any messages and propertis for now
    type Message = ();
    type Properties = ();

	// component constructor
    fn new(props: Self::Properties) -> Self {
        InfoPage
    }
   
	// view function - here we define the component structure using rsx syntax
    fn view(
        &self,
        behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        rsx!(<div>"This is a info page."</div>)
    }

	// update function - for now we don't need any updating
    fn update(&mut self, message: Self::Message) -> bool {
        false
    }

}

impl Default for InfoPage {
    fn default() -> Self {
        Self::new(())
    }
}
```

```Rust
// --- shoppinglist_page/mod.rs ---

use wal_core::component::Component;
use wal_rsx::rsx;

// component struct - temporarily with no fields
pub(crate) struct ShoppingListPage;

impl Component for ShoppingListPage {
   
    // we don't need any messages and propertis for now
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
```

Po stworzeniu komponentów należy dodać je jako komponenty główne podstron. Dla adresu URL: `/` wyświetlany będzie komponent `ShoppingListPage`, natomiast dla adresu `/info` komponent `InfoPage`. W tym celu dodać należy dwa wywołania funkcji `RouterBuilder::add_page` w pliku `main.rs`. Na tym etapie można pozbyć się komponentu `HelloPage` wchodzącego w skład początkowego szablonu projektu, ponieważ nie będzie on potrzebny w dalszych etapach.

```Rust
// --- main.rs ---
use infopage::InfoPage;
use wal_core::router::builder::RouterBuilder;
use welcomepage::WelcomePage;

mod infopage;
mod welcomepage;

fn main() {
    RouterBuilder::default()
        .add_page::<ShoppingListPage>("/")
        .add_page::<InfoPage>("/info")
        .build()
        .start();
}
```

W oknie przeglądarki można już zobaczyć wyświetlane komponenty po nawigacji do odpowiadających im adresów URL.

## Strona z informacjami

Na stronie z informacjami wyświetlone zostaną użytkownikowi krótkie informacje o stronie wraz z linkiem do repozytorium, gdzie znajdzie on więcej przykładów. W tym celu należy zmienić strukturę komponentu w pliku `infopage.rs`. Dodatkowo użytkownik możliwość nawigacji do strony z listą zakupów. W tym celu wewnątrz komponentu umiescić należy `<Link>` pozwalający na wydajną nawigacje pomiedzy różnymi adresami URL wewnątrz aplikacji.

```Rust
// --- infopage.rs ---
// ...

 fn view(
        &self,
        behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        rsx!(<div class="container">
            <h1>"This app has been made using Wal"</h1>
            <p>"You can find more complex examples on the offical Wal GtiHub repository in the demo directory:"</p>
            <a href="https://github.com/walrust/wal">"Wal repository"</a>
            <Link to="/">"back to the shopping list"</Link>
        </div>)
    }

// ...
```

## Strona z listą zakupów

Strona z listą zakupów bedzie składała się z kilku pomniejszych komponantów: komponentu głównego zawierającego listę zakupów, komponentu z formularzem, gdzie użytkownik będzie mógł wpisać nowe rzeczy do kupienia oraz komponentu, który reprezentował będzie pojedynczą pozycję z listy.

w folderze shoppinglist_page tworzone są pliki: `listitem.rs` oraz `listitem_form.rs`.

### Komponent przedmiotów z listy

W pliku `listitem.rs` powstanie komponent reprezentujący pojedynczą pozycję z listy. Każdy przedmiot na liście posiada swój identyfikator, nazwę produktu oraz liczbę jednostek. Reprezentowane jest to przez strukturę:

```Rust
#[derive(Hash, Clone)]
pub(crate) struct ListItemDetails {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) count: i32,
}
```

Poza danymi o zakupie użytkownik powinien mieć możliwość usuwania już zakupionych produktów z listy. W tym celu należy z zewnątrz dodatkowo przekazać do komponentu `Callback`, który przymuje identyfikator elementu z listy i odpowiada za jego usunięcie.

Komponent `ListItem` wygląda następująco:

```Rust
// listitem.rs

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

        // create callback to handle button press
        let delete_on_click = behavior.create_callback(move |_event: MouseEvent| {
            // emit callback received in props
            props.remove_callback.emit(props.details.id);
        });

        rsx! {
            <div class="container">
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
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}

```

### Komponent z formulazem do dodawania nowych produktów

Kolejnym komponentem jest formularz, dzięki któremu użytkownik ma możliwość dodawania nowych porduktów do listy. Formularz posiada pola z nazwą oraz liczbą jednostek produktu.

Komponent będzie implementowny przez strukturę `ListItemForm`.

```Rust
pub(crate) struct ListItemForm {
    next_item_id: i32,
    item_name: String,
    item_count: i32,
    add_handler: Callback<ListItemDetails>,
}
```

Poza wspomnianą nazwą i liczbą jednostek produktu należy również przekazać do komponentu identyfikator, który zostanie nadany nowemu elementowi na liście po jego utworzeniu oraz `Callback` przyjmujący szczegóły nowego elementu i odpowiadający za jego dodanie do listy. Kod komponentu `ListItemForm` wygląda następująco:

```Rust

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
        behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        let new_item_id: i32 = self.next_item_id;
        let add_handler = self.add_handler.clone();

        // create callback to handle button press
        let add_on_click = behavior.create_callback(move |_event: MouseEvent| {
            let document = web_sys::window().unwrap().document().unwrap();

            // get name from input
            let element = document.get_element_by_id("newItemName").unwrap();
            let input_element = element.dyn_into::<HtmlInputElement>().unwrap();
            let new_item_name = input_element.value();

            // get count from input
            let element = document.get_element_by_id("newItemCount").unwrap();
            let input_element = element.dyn_into::<HtmlInputElement>().unwrap();
            let new_item_count = input_element.value().parse::<i32>().unwrap();

            let message = ListItemDetails {
                id: new_item_id,
                count: new_item_count,
                name: new_item_name,
            };
            // emit callback received in props
            add_handler.emit(message);
        });

        rsx! {
            <>
            <h1>"Add new item"</h1>
            <div class="container">
                <div>
                    <label>"name"</label>
                    <br/>
                    <input id="newItemName" value = {&self.item_name} />
                </div>
                <div >
                    <label>"count"</label>
                    <br/>
                    <input id="newItemCount" value = {self.item_count} />
                </div>
                <button onclick={add_on_click}>
                    "Add"
                </button>
            </div>
            </>
        }
    }

    fn update(&mut self, _message: Self::Message) -> bool {
        false
    }
}

```

### Komponent z listą zakupów

Główny komponent `ShoppingListPage` zawierający listę zakupów zadeklarowany zostanie w pliku `shoppinglist_page/mod.rs`. Zawierać on będzie w sobie jeden komponent `ListItemForm` oraz wiele komponentów `ListItem`. Jest to nadrzędny komponent całej podstrony, dlatego to w nim stworzone zostaną `Callbacki` wymagande do działania wyżej wymienionych komponentów. Kompnent `ShoppingListPage` przechowywać będzie informację o identyfikatorze dla potencjalnego nowego elementu, a także wektor elementów obecnych aktualnie na liście:

```Rust
pub(crate) struct ShoppingListPage {
    list_items: Vec<ListItemDetails>,
    next_id: i32,
}
```

Dodatkowo `ShoppingListPage` może otrzymywac wiadomości o dodaniu lub usunięciu elementu z listy reprezentowane poprzez `ShoppingListMessage`:

```Rust
pub(crate) enum ShoppingListMessage {
    AddItem(ListItemDetails),
    RemoveItem(i32),
}
```

Cały kod komponentu wygląda następująco:

```Rust

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
        // create callback which will send the AddItem message
        let add_handler_callback = behavior.create_callback(ShoppingListMessage::AddItem);

        rsx!(
            <div class="container">
                <Link to="/info">"project info"</Link>
                <ListItemForm props={ListItemFormProps {next_id: self.next_id, add_handler: add_handler_callback}}/>
                <h1>"Items on the list:"</h1>
                for { self.list_items.iter().map( |details| {
                    // create callback which will send the RemoveItem message with given id
                    let remove_callback = behavior.create_callback(ShoppingListMessage::RemoveItem);
                    rsx! { <ListItem props={ ListItemProps{details: details.clone(), remove_callback}} /> }
                })}
            </div>
        )
    }

    fn update(&mut self, message: Self::Message) -> bool {
        match message {
            ShoppingListMessage::AddItem(details) => {
                // update id stored for the next element
                self.next_id += 1;
                // update add new element to the list
                self.list_items.push(details);
                // return true to re-render
                true
            }
            ShoppingListMessage::RemoveItem(id_to_delete) => {
                // remove element with given id from the list
                self.list_items.retain(|i| i.id != id_to_delete);
                // return true to re-render
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

```

## Dopracowanie stylowania

Elementy dodawane do listy nie wylądają na obecnym etapie zbyt ładnie. Bardziej stosowne bedą elementy, których szczegóły są wyświetlone w poziomie zamiast w pionie. Obecny wygląd komponentu jest konsekwencją zastosowania klasy `container` zdefiniowanej w pliku `global.css`, a więc obowiązującej na obszarze całej aplikacji:

```css
/* global.css */

.container {
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
}
```

Możliwe jest oczywiście zadeklarowanie dodatkowej klasy o innej nazwie, jednak przy większych projektach wymyślanie skomplikowanych, unikalnych nazw może być uciążliwe. Z myślą o tym problemie Wal udostępnia rozwiązanie: **stylowanie lokalne** udostępniane przez crate `wal-css`. Aby z niego skorzystać należy zadeklarować osobny plik CSS z nową definicją klasy `container` o nazwie `listitem_styles.css`:

```css
/* shoppinglist_page/listitem_styles.css */

.container {
  display: flex;
  justify-content: space-around;
  align-items: center;
  gap: 1rem;
  width: 80%;
  background-color: #049a90a2;
  padding: 2rem;
  border-radius: 15px;
  box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);

  &:hover {
    transition: 0.5s;
    background-color: #049a8f;
  }
}
```

Następnie, aby zastosować nowy styl w komponencie `ListItem` należy zaimportować utworzony arkusz do pliku komponentu za pomocą macro `css_stylesheet!`:

```Rust
// shoppinglist_page/listitem.rs

use wal_core::{
    component::{callback::Callback, Component},
    events::MouseEvent,
};
use wal_css::{css::Css, css_stylesheet};
use wal_rsx::rsx;

thread_local! {
    static LIST_ITEM_CSS: Css = css_stylesheet!("./listitem_styles.css")
}

// ...

```

Zaimportowany obiekt `LIST_ITEM_CSS` może być wykorzystywany aby uszyskać dostęp do nowej defincji klasy `container` wewnątrz macro `rsx!` poprzez operator indeksowania:

```Rust
// shoppinglist_page/listitem.rs

// ...

fn view(
        &self,
        behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        let props = self.props.clone();

        let delete_on_click = behavior.create_callback(move |_event: MouseEvent| {
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

// ...
```

Dodatkowo nadal możliwe jest korzystanie z globalnej definicji klasy `container` w miejscach, gdzie nie jest pożądane jej nadpisywanie.

# Dodatkowe zasoby

Kod narzędzia Wal jest udostępniony publicznie poprzez oficjalne [repozytorium](https://github.com/walrust/wal). Znajdują się tam również dodatkowe przykłady zastosowania narzędzia.
