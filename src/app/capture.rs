use anyhow::Result;
use pnet::datalink::NetworkInterface;

pub fn get_network_interfaces() -> Result<Vec<NetworkInterface>> {
    let interfaces = pnet::datalink::interfaces();
    Ok(interfaces)
}
