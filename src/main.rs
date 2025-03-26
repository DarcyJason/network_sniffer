use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

mod app;
mod ui;
mod utils;

fn main() {
    dioxus::launch(app::App);
}
