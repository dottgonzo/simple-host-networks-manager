use default_net::{interface::InterfaceType, ip::{Ipv4Net, Ipv6Net}, mac::MacAddr, Gateway};

pub enum NetworkType {
    Wifi,
    Ethernet,
    Bluetooth,
    Cellular,
}

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

        pub is_connected: bool,

        pub is_default_route: bool,

        pub network_type: Option<NetworkType>,

        pub wifi_connection_ssid: Option<String>,

        pub ppp_apn_connection_name: Option<String>,
    
}