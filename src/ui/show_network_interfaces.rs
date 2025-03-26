use crate::app::capture::get_network_interfaces;
use dioxus::prelude::*;

#[component]
pub fn show_network_interfaces() -> Element {
    let interfaces = get_network_interfaces().unwrap_or_default();
    rsx! {
        div {
            button { "显示所有网络接口" }
            if interfaces.is_empty() {
                p { "没有找到网络接口" }
            } else {
                for iface in interfaces {
                    p { "{iface.index.to_string()}" }
                    p { "{iface.name}" }
                    p { "{iface.mac.map_or(\"N/A\".to_string(), |m| m.to_string())}" }
                    p { "{iface.ips.iter().map(|ip| ip.to_string()).collect::<Vec<String>>().join(\", \")}" }
                    p { "{format_status(iface.flags)}" }
                }
            }
        }
    }
}

fn format_status(flags: u32) -> String {
    let mut status = Vec::new();
    if flags & 0x1 != 0 {
        status.push("启用");
    }
    if flags & 0x2 != 0 {
        status.push("环回");
    }
    status.join(", ")
}
