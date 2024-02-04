use std::net::Ipv4Addr;

use default_net::{
    interface::InterfaceType,
    ip::{Ipv4Net, Ipv6Net},
    mac::MacAddr,
    Gateway,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum NetworkType {
    Wifi,
    Ethernet,
    Bluetooth,
    Cellular,
}

#[derive(Debug)]
pub struct NetworkInterface {
    /// Index of network interface
    pub index: u32,
    /// Name of network interface
    pub name: String,
    /// Friendly Name of network interface
    pub friendly_name: Option<String>,
    /// Description of the network interface
    pub description: Option<String>,
    /// Interface Type
    pub if_type: InterfaceType,
    /// MAC address of network interface
    pub mac_addr: Option<MacAddr>,
    /// List of Ipv4Net for the network interface
    pub ipv4: Vec<Ipv4Net>,
    /// List of Ipv6Net for the network interface
    pub ipv6: Vec<Ipv6Net>,
    /// Flags for the network interface (OS Specific)
    pub flags: u32,
    /// Speed in bits per second of the transmit for the network interface
    pub transmit_speed: Option<u64>,
    /// Speed in bits per second of the receive for the network interface
    pub receive_speed: Option<u64>,
    /// Default gateway for the network interface
    pub gateway: Option<Gateway>,

    pub is_connected: Option<bool>,

    pub is_default_route: Option<bool>,

    pub network_type: Option<NetworkType>,

    pub wifi_connection_ssid: Option<String>,

    pub ppp_apn_connection_name: Option<String>,
}

// serde support for NetworkInterface
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkConnectionProfile {
    pub interface: String,
    pub ipv4: Option<Ipv4Addr>,
    pub gateway: Option<Ipv4Addr>,
    pub priority: Option<u8>,
    pub name: String,
    pub network_type: NetworkType,
    pub wifi_connection_ssid: Option<String>,
    pub wifi_connection_password: Option<String>,
    pub ppp_apn_connection_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APN {
    pub name: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub apn: String,
    pub auth_type: Option<String>,
    pub ip_type: Option<String>,
    pub dns1: Option<String>,
    pub dns2: Option<String>,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
}
