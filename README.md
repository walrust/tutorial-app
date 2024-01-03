# Wprowadzenie

Ten dokument zawiera instrukcję instalacji oraz korzystania z narzędzia wal, przeznaczonego do tworzenia warstwy front-endowej aplikacji webowych przy pomocy języka Rust.

# Instalacja

Przed rozpoczęciem procesu tworzenia aplikacji niezbędne jest wykonanie następujących kroków:

1. Instalacja języka Rust.
2. Inicjalizacja szablonu projektu .
3. Instalacja narzędzi wasm-bindgen-cli oraz trunk.

## Instalacja języka Rust

Aby umożliwić instalację biblioteki Wal należy najpierw zainstalować kompilator języka Rust. Szczegółowe instrukcje dostępne są na oficjalnej stronie języka Rust: https://www.rust-lang.org/learn/get-started

## Inicjalizacja szablonu projektu

Aby ułatwić inicjalizację początkowego projektu należy skorzystać z [szablonu projektu](https://github.com/walrust/template). Kod można skopiować bezpośrednio z repozytorium szablonu lub wygenerować przy pomocy narzędzia [Cargo Generate](https://github.com/cargo-generate/cargo-generate) .

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

Po pobraniu szablonu należy zainstalować dodatkowe narzędzia, które potrzebne będą do uruchomienia aplikacji:

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

Aplikacja zostanie wystawiona lokalnie na porcie, który został wskazany w pliku **Trunk.toml** (domyślnie jest to port nr 3000). Kiedy proces Trunk jest uruchomiony, dokonane w projekcie zmiany po ich zapisaniu zostaną automatycznie zastosowane w uruchomionej aplikacji.

# Tworzenie aplikacji z wykorzystaniem Wal

W tej sekcji zostanie opisany krok po kroku proces tworzenia prostej aplikacji z wykorzystaniem narzędzia wal. Aplikacja ta składała się będzie z dwóch podstron: strony powitalnej oraz strony z informacjami o projekcie.

## Stworzenie komponentów stron

Pierwszym krokiem będzie stworzenie dwóch komponentów, po jednym dla każdej z podstron. W tym celu należy utworzyć nowe pliki: `welcomepage.rs` oraz `infopage.rs` w katalogu `src` projektu.
Następnie w pliku `main.rs` należy dodać nowo utworzone pliki jako moduły projektu:

```Rust
use hellopage::HelloComponent;
use wal_core::router::builder::RouterBuilder;

mod hellopage;
mod infopage;
mod welcomepage;

fn main() {
    RouterBuilder::default()
        .add_page::<HelloComponent>("/")
        .build()
        .start();
}
```

Wewnątrz każdego z nowych plików znajdować się będzie struktura o odpowiadającej nazwie będąca głównym komponentem danej podstrony `WelcomePage`. Dla tej struktury zaimplementowany musi zostać trait `Component` oraz trait `Default`.

```Rust
// --- welcomepage.rs ---

use wal_core::component::Component;
use wal_rsx::rsx;

// component struct - temporarily with no fields
pub(crate) struct WelcomePage;

impl Component for WelcomePage {
   
    // we don't need any messages and propertis for now
    type Message = ();
    type Properties = ();

	// component constructor
    fn new(props: Self::Properties) -> Self {
        WelcomePage
    }
   
	// view function - here we define the component structure using rsx syntax
    fn view(
        &self,
        behavior: &mut impl wal_core::component::behavior::Behavior<Self>,
    ) -> wal_core::virtual_dom::VNode {
        rsx!(<div>"This is a welcome page."</div>)
    }

	// update function - for now we don't need any updating
    fn update(&mut self, message: Self::Message) -> bool {
        false
    }

}

impl Default for WelcomePage {
    fn default() -> Self {
        Self::new(())
    }
}
```

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

Po stworzeniu komponentów należy dodać je jako komponenty główne dla podstron. Dla adresu URL: `/` chcemy wyświetlić komponent `HelloPage`, natomiast dla adresu `/info` komponent `InfoPage`. W tym celu dodajemy 2 wywołania funkcji `RouterBuilder::add_page` w pliku `main.rs`. Na tym etapie można pozbyć się komponentu `HelloPage` wchodzącego w skład początkowego szablonu projektu, ponieważ nie będzie on już potrzebny w dalszych etapach.

```Rust
// --- main.rs ---
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
```

W oknie przeglądarki można już zobaczyć wyświetlane komponenty po nawigacji do odpowiadających im adresów URL.

## Strona z informacjami

Na stronie z informacjami chcemy wyświetlić użytkownikowi krótkie informacje o stronie wraz z linkami do do źródeł.
