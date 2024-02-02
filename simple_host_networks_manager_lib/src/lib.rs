#[cfg(test)]
mod tests;

mod structs;
use default_net;
use tokio_wifiscanner;

pub async fn connect(force: Option<bool>, interface: String) -> anyhow::Result<()> {
    // check ping
    if force.is_none() {
        let connection_status = check_connection().await;
        if connection_status.is_ok() {
            return Ok(());
        }
    }
    Ok(())
}

pub async fn get_all_networking_phisical_interfaces(
) -> anyhow::Result<Vec<structs::NetworkInterface>> {
    let i = default_net::get_interfaces();

    let mut filtered: Vec<structs::NetworkInterface> = Vec::new();

    for interface in i {
        let orig = interface.clone();
        let address: Vec<default_net::ip::Ipv4Net> = interface.ipv4;
        if !orig.is_loopback() && !orig.is_tun() {
            let mut is_default_route = false;

            let routes = get_routes().await;
            if let Ok(routes) = routes {
                for route in routes {
                    if let Some(route_gateway) = route.gateway {
                        if route_gateway.to_string() == address[0].addr.to_string() {
                            is_default_route = true;
                            break;
                        }
                    }
                }
            }

            if !address.is_empty() && orig.gateway.is_some() {
                let from_vec_to_ipv4net: default_net::ip::Ipv4Net = address[0];
                if !from_vec_to_ipv4net.addr.to_string().ends_with(".1") {
                    let is_connected = true;

                    let mut network_type: Option<structs::NetworkType> = None;
                    let mut wifi_connection_ssid: Option<String> = None;

                    if orig.name.contains("wlan") || orig.name.contains("wlp") {
                        network_type = Some(structs::NetworkType::Wifi);

                        let wifi_networks = get_wifi_networks().await;
                        if let Ok(wifi_networks) = wifi_networks {
                            for wifi_network in wifi_networks {
                                if wifi_network.mac == orig.mac_addr.unwrap().to_string() {
                                    wifi_connection_ssid = Some(wifi_network.ssid);
                                    break;
                                }
                            }
                        }
                    } else if orig.name.contains("eth") || orig.name.contains("enp") {
                        network_type = Some(structs::NetworkType::Ethernet);
                    } else if orig.name.contains("ppp") {
                        network_type = Some(structs::NetworkType::Cellular);
                    }

                    filtered.push(structs::NetworkInterface {
                        index: orig.index,
                        name: orig.name,
                        friendly_name: orig.friendly_name,
                        description: orig.description,
                        if_type: orig.if_type,
                        mac_addr: orig.mac_addr,
                        ipv4: orig.ipv4,
                        ipv6: orig.ipv6,
                        flags: orig.flags,
                        transmit_speed: orig.transmit_speed,
                        receive_speed: orig.receive_speed,
                        gateway: orig.gateway,

                        network_type,

                        wifi_connection_ssid,
                        ppp_apn_connection_name: None,

                        is_connected,
                        is_default_route,
                    });
                }
            } else {
                filtered.push(structs::NetworkInterface {
                    index: orig.index,
                    name: orig.name,
                    friendly_name: orig.friendly_name,
                    description: orig.description,
                    if_type: orig.if_type,
                    mac_addr: orig.mac_addr,
                    ipv4: orig.ipv4,
                    ipv6: orig.ipv6,
                    flags: orig.flags,
                    transmit_speed: orig.transmit_speed,
                    receive_speed: orig.receive_speed,
                    gateway: orig.gateway,

                    network_type: None,

                    wifi_connection_ssid: None,
                    ppp_apn_connection_name: None,

                    is_connected: false,
                    is_default_route,
                });
            }
        }
    }
    Ok(filtered)
}

pub async fn disconnect(force: Option<bool>, interface: String) -> anyhow::Result<()> {
    // check ping

    Ok(())
}
pub async fn reset(force: Option<bool>, interface: String) -> anyhow::Result<()> {
    // check ping
    let _ = disconnect(None, interface.clone()).await;
    let _ = connect(None, interface).await;
    Ok(())
}

pub async fn check_connection() -> anyhow::Result<()> {
    Ok(())
}

pub async fn get_routes() -> anyhow::Result<Vec<net_route::Route>> {
    let handle = net_route::Handle::new()?;
    let routes = handle.list().await;

    let mut filtered_routes: Vec<net_route::Route> = Vec::new();

    if let Ok(routes) = routes {
        for route in routes {
            filtered_routes.push(route);
        }
        Ok(filtered_routes)
    } else {
        println!("error getting routes {:?}", routes.err());

        Err(anyhow::anyhow!("error getting routes"))
    }
}
pub async fn set_route() -> anyhow::Result<()> {
    Ok(())
}

pub async fn switch_route() -> anyhow::Result<()> {
    Ok(())
}

pub async fn get_wifi_networks() -> anyhow::Result<Vec<tokio_wifiscanner::Wifi>> {
    let mut wifi_networks: Vec<tokio_wifiscanner::Wifi> = Vec::new();

    let wifi = tokio_wifiscanner::scan().await;

    if let Ok(wifi) = wifi {
        if wifi.len() == 0 {
            println!("no wifi networks found");
        }
        for network in wifi {
            println!("{:?}", network);

            if wifi_networks.contains(&network) {
                continue;
            }

            wifi_networks.push(network);
        }
    } else {
        println!("error scanning wifi {:?}", wifi.err());
    }

    Ok(wifi_networks)
}

pub async fn set_wifi_password(apn_name: &str, password: &str) -> anyhow::Result<()> {
    Ok(())
}

pub async fn get_ppp_apns_list() -> anyhow::Result<()> {
    Ok(())
}

pub async fn add_ppp_apn(apn_name: &str, apn_data: &str) -> anyhow::Result<()> {
    Ok(())
}

pub async fn remove_ppp_apn(apn_name: &str) -> anyhow::Result<()> {
    Ok(())
}

pub async fn switch_to_ppp_apn(apn_name: &str) -> anyhow::Result<()> {
    Ok(())
}
