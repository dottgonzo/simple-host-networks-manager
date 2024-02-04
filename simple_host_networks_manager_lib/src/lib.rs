#[cfg(test)]
mod tests;

pub mod structs;
use async_recursion::async_recursion;
use default_net::gateway;
use std::{
    fs,
    net::Ipv4Addr,
    os,
    path::{self, Path, PathBuf},
    process::Command,
};

#[async_recursion]
pub async fn connect(
    disconnect_before: Option<bool>,
    multiple: Option<bool>,
    interface: Option<String>,
    wifi_password: &Option<&str>,
    apn_name: &Option<&str>,
) -> anyhow::Result<()> {
    // check ping
    if multiple.is_none() {
        let connection_status = check_connection().await;
        if connection_status.is_ok() {
            return Ok(());
        }
    }

    let phisycal_interfaces = get_all_networking_phisical_interfaces().await?;
    let mut wifi_is_connected = false;
    let wifi_networks = get_wifi_networks().await?;
    let profiles = get_connection_profiles(None).await?;
    for wifi_network in &wifi_networks {
        if is_ssid_connected(&wifi_network.ssid) {
            wifi_is_connected = true;
            break;
        }
    }
    for i in phisycal_interfaces {
        let profiles = profiles.clone();

        if !profiles
            .into_iter()
            .any(|p: structs::NetworkConnectionProfile| p.interface == i.name)
        {
            continue;
        }
        if let Some(interface) = &interface {
            if &i.name != interface {
                continue;
            }
        }
        if let Some(network_type) = i.network_type {
            if let Some(connected) = i.is_connected {
                if connected && disconnect_before.is_none() {
                    continue;
                } else if connected && disconnect_before.is_some() {
                    let nmcli = Command::new("nmcli")
                        .args(["dev", "disconnect", &i.name])
                        .output()
                        .expect("failed to run nmcli");
                }
            }
            match network_type {
                structs::NetworkType::Wifi => {
                    if wifi_is_connected && disconnect_before.is_none() {
                        continue;
                    }
                    let nmcli = Command::new("nmcli")
                        .args(["dev", "wifi", "connect", &i.name])
                        .output()
                        .expect("failed to run nmcli");
                    let output = String::from_utf8_lossy(&nmcli.stdout);
                    println!("output: {:?}", output);
                }
                structs::NetworkType::Ethernet => {
                    let nmcli = Command::new("nmcli")
                        .args(["dev", "connect", &i.name])
                        .output()
                        .expect("failed to run nmcli");

                    let output = String::from_utf8_lossy(&nmcli.stdout);
                    println!("output: {:?}", output);
                }
                structs::NetworkType::Cellular => {
                    let nmcli = Command::new("nmcli")
                        .args(["dev", "connect", &i.name])
                        .output()
                        .expect("failed to run nmcli");

                    let output = String::from_utf8_lossy(&nmcli.stdout);
                    println!("output: {:?}", output);
                }
                _ => {}
            }
        }
        if multiple.is_none() {
            let connection_status = check_connection().await;
            if connection_status.is_ok() {
                return Ok(());
            }
        }
    }
    let connection_status = check_connection().await;
    if connection_status.is_ok() {
        return Ok(());
    } else if interface.is_none()
        && connection_status.is_err()
        && disconnect_before.is_none()
        && wifi_password.is_none()
        && apn_name.is_none()
    {
        return connect(Some(true), multiple, None, &None, &None).await;
    } else {
        return Err(anyhow::anyhow!("error connecting"));
    }
}
fn is_ssid_connected(ssid: &str) -> bool {
    let nmcli = Command::new("nmcli")
        .args(["-t", "-f", "active,ssid", "dev", "wifi"])
        .output()
        .expect("failed to run nmcli");

    let output = String::from_utf8_lossy(&nmcli.stdout);

    let output = output.split('\n').collect::<Vec<_>>();
    for line in output {
        if !line.contains("no") && line.contains(ssid) {
            return true;
        }
    }
    false
}
pub async fn get_all_networking_phisical_interfaces(
) -> anyhow::Result<Vec<structs::NetworkInterface>> {
    let i = default_net::get_interfaces();

    let mut filtered: Vec<structs::NetworkInterface> = Vec::new();
    let routes = get_routes().await;
    let wifi = get_wifi_networks().await;
    let mut wifi_networks: Vec<tokio_wifiscanner::Wifi> = vec![];

    if let Ok(wifi) = wifi {
        for network in wifi {
            wifi_networks.push(network);
        }
    } else {
        println!("error scanning wifi!!?? {:?}", wifi.err());
        return Err(anyhow::anyhow!("error scanning wifi!!!!!!"));
    }

    for interface in i {
        let orig = interface.clone();
        let address: Vec<default_net::ip::Ipv4Net> = interface.ipv4;
        if !orig.is_loopback()
            && !orig.is_tun()
            && !orig.name.starts_with("br")
            && !orig.name.starts_with("vnet")
        {
            let mut is_default_route: Option<bool> = None;
            let mut network_type: Option<structs::NetworkType> = None;
            let mut wifi_connection_ssid: Option<String> = None;

            if orig.name.starts_with("wlan") || orig.name.starts_with("wlp") {
                network_type = Some(structs::NetworkType::Wifi);

                for wifi_network in &wifi_networks {
                    if is_ssid_connected(&wifi_network.ssid) {
                        wifi_connection_ssid = Some(wifi_network.ssid.to_owned());
                    }
                }
            } else if orig.name.starts_with("eth") || orig.name.starts_with("enp") {
                network_type = Some(structs::NetworkType::Ethernet);
            } else if orig.name.starts_with("ppp") {
                network_type = Some(structs::NetworkType::Cellular);
            }
            if !address.is_empty() {
                let from_vec_to_ipv4net = address[0].to_string();

                if from_vec_to_ipv4net.contains(".1/") {
                    continue;
                }

                let mut is_connected: Option<bool> = None;

                if orig.gateway.is_some() {
                    is_connected = Some(true);
                    if let Ok(rr) = &routes {
                        for route in rr {
                            if let Some(route_gateway) = route.gateway {
                                if route_gateway.to_string()
                                    == orig.clone().gateway.unwrap().ip_addr.to_string()
                                {
                                    is_default_route = Some(true);
                                    break;
                                }
                            }
                        }
                    }
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

                    is_connected: None,
                    is_default_route,
                });
            }
        }
    }
    Ok(filtered)
}

pub async fn disconnect(multiple: Option<bool>, interface: String) -> anyhow::Result<()> {
    // check ping

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
        if wifi.is_empty() {
            println!("no wifi networks found");
        }
        for network in wifi {
            // if wifi_networks.contains(&network) {
            //     continue;
            // }

            wifi_networks.push(network);
        }
        Ok(wifi_networks)
    } else {
        let err = wifi.err();
        if let Some(err) = err {
            if err.to_string().contains("permitt") || err.to_string().contains("busy") {
                println!("wifi error: {:?}", err);
                Err(anyhow::anyhow!("wifi errors..."))
            } else {
                if !err.to_string().contains("such") {
                    println!("error scanning wifi... {:?}", err);
                }
                Ok(wifi_networks)
            }
        } else {
            println!("error scanning wifi {:?}", err);
            Ok(wifi_networks)
        }
    }
}

pub async fn set_connection_profile(
    connection_db_file: Option<PathBuf>,
    profile: structs::NetworkConnectionProfile,
) -> anyhow::Result<()> {
    match profile.network_type {
        structs::NetworkType::Ethernet => {
            if profile.wifi_connection_ssid.is_some() || profile.wifi_connection_password.is_some()
            {
                return Err(anyhow::anyhow!("wifi_connection_ssid and wifi_connection_password are not required for ethernet"));
            }
            if profile.ppp_apn_connection_name.is_some() {
                return Err(anyhow::anyhow!(
                    "ppp_apn_connection_name is not required for ethernet"
                ));
            }
        }
        structs::NetworkType::Wifi => {
            if profile.wifi_connection_ssid.is_none() {
                return Err(anyhow::anyhow!("wifi_connection_ssid is required"));
            }
            if profile.wifi_connection_password.is_none() {
                return Err(anyhow::anyhow!("wifi_connection_password is required"));
            }
            let wifi_networks = get_wifi_networks().await?;

            if !wifi_networks
                .into_iter()
                .any(|network| network.ssid == profile.clone().wifi_connection_ssid.unwrap())
            {
                return Err(anyhow::anyhow!("wifi_connection_ssid not found"));
            }
        }
        structs::NetworkType::Cellular => {
            if profile.ppp_apn_connection_name.is_none() {
                // TODO: check if apn exists in local database
                return Err(anyhow::anyhow!("ppp_apn_connection_name is required"));
            }
        }
        _ => {}
    }

    let connection_db_path: PathBuf = connection_db_file.unwrap_or_else(|| {
        path::Path::new("/etc/simple-host-networks-manager/profiles.json").to_path_buf()
    });
    if !connection_db_path.exists() {
        fs::write(&connection_db_path, "[]")?;
    }

    let contents = fs::read_to_string(&connection_db_path)?;

    let mut profiles: Vec<structs::NetworkConnectionProfile> = serde_json::from_str(&contents)?;

    profiles.push(profile);

    let json = serde_json::to_string(&profiles)?;

    fs::write(&connection_db_path, json)?;

    Ok(())
}

pub async fn get_connection_profiles(
    connection_db_file: Option<PathBuf>,
) -> anyhow::Result<Vec<structs::NetworkConnectionProfile>> {
    let connection_db_path: PathBuf = connection_db_file.unwrap_or_else(|| {
        path::Path::new("/etc/simple-host-networks-manager/profiles.json").to_path_buf()
    });
    // let phisi = get_all_networking_phisical_interfaces().await?;
    let mut profiles: Vec<structs::NetworkConnectionProfile> = Vec::new();
    if !connection_db_path.exists() {
        // if phisi.is_empty() {
        fs::write(&connection_db_path, "[]")?;
        // }
    } else {
        let contents = fs::read_to_string(&connection_db_path)?;
        profiles = serde_json::from_str(&contents)?;

        // write back to file
    }

    // let mut connection_profiles: Vec<structs::NetworkConnectionProfile> = Vec::new();

    // for phisical in phisi {
    //     let profiles = profiles.clone();
    //     if phisical.is_connected.is_some() {
    //         match phisical.network_type {
    //             Some(structs::NetworkType::Ethernet) => {}
    //             Some(structs::NetworkType::Wifi) => {}
    //             Some(structs::NetworkType::Cellular) => {}
    //             _ => {}
    //         }
    //     }
    // }
    // let phisi_connected = phisi.into_iter().filter(|p| p.is_connected.unwrap());
    // for p in phisi_connected {
    //     let profile_for_connection = profiles
    //         .clone()
    //         .into_iter()
    //         .find(|profile| profile.interface == p.name);

    //     let profile = structs::NetworkConnectionProfile {
    //         interface: p.name,
    //         ipv4: p.ipv4[0].ip_addr,
    //         gateway: p.gateway.unwrap().ip_addr,
    //         priority: Some(1),
    //         name: p.name,
    //         network_type: p.network_type.unwrap(),
    //         wifi_connection_ssid: p.wifi_connection_ssid,
    //         wifi_connection_password: None,
    //         ppp_apn_connection_name: p.ppp_apn_connection_name,
    //     };
    //     profiles.push(profile);
    // }
    Ok(profiles)
}

pub async fn remove_connection_profile(
    connection_db_file: Option<PathBuf>,
    interface: &str,
) -> anyhow::Result<()> {
    let connection_db_path: PathBuf = connection_db_file.unwrap_or_else(|| {
        path::Path::new("/etc/simple-host-networks-manager/profiles.json").to_path_buf()
    });
    let contents = fs::read_to_string(&connection_db_path)?;

    let mut profiles: Vec<structs::NetworkConnectionProfile> = serde_json::from_str(&contents)?;

    profiles.retain(|profile| profile.interface != interface);

    let json = serde_json::to_string(&profiles)?;

    fs::write(&connection_db_path, json)?;

    Ok(())
}

pub async fn get_ppp_apns_list(apn_db_file: Option<PathBuf>) -> anyhow::Result<()> {
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
