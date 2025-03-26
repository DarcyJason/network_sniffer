use crate::ui::show_network_interfaces::show_network_interfaces;
use crate::MAIN_CSS;
use dioxus::prelude::*;

pub mod analysis;
pub mod capture;
pub mod models;
pub mod parser;

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        h1 { "欢迎使用 Network Sniffer" }
        show_network_interfaces {}
        button { "开始捕获" }
    }
}
