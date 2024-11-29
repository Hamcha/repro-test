use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! {
        h1 { "Hello world!" }
    }
}
