#[cfg(test)]
mod tests;
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

pub async fn get_all_networking_phisical_interfaces() -> anyhow::Result<Vec<default_net::Interface>>
{
    let i = default_net::get_interfaces();

    let mut filtered: Vec<default_net::Interface> = Vec::new();

    for interface in i {
        let orig = interface.clone();
        let address: Vec<default_net::ip::Ipv4Net> = interface.ipv4;
        if !orig.is_loopback() && !orig.is_tun() {
            if !address.is_empty() {
                let from_vec_to_ipv4net: default_net::ip::Ipv4Net = address[0];
                if !from_vec_to_ipv4net.addr.to_string().ends_with(".1") {

                    filtered.push(orig);
                }
            } else {
                filtered.push(orig);
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

pub async fn set_route() -> anyhow::Result<()> {
    Ok(())
}

pub async fn switch_route() -> anyhow::Result<()> {
    Ok(())
}

pub async fn get_eth_connections() -> anyhow::Result<()> {
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
